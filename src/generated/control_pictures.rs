
/// An enum to represent all characters in the ControlPictures block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ControlPictures {
    /// \u{2400}: '␀'
    SymbolForNull,
    /// \u{2401}: '␁'
    SymbolForStartOfHeading,
    /// \u{2402}: '␂'
    SymbolForStartOfText,
    /// \u{2403}: '␃'
    SymbolForEndOfText,
    /// \u{2404}: '␄'
    SymbolForEndOfTransmission,
    /// \u{2405}: '␅'
    SymbolForEnquiry,
    /// \u{2406}: '␆'
    SymbolForAcknowledge,
    /// \u{2407}: '␇'
    SymbolForBell,
    /// \u{2408}: '␈'
    SymbolForBackspace,
    /// \u{2409}: '␉'
    SymbolForHorizontalTabulation,
    /// \u{240a}: '␊'
    SymbolForLineFeed,
    /// \u{240b}: '␋'
    SymbolForVerticalTabulation,
    /// \u{240c}: '␌'
    SymbolForFormFeed,
    /// \u{240d}: '␍'
    SymbolForCarriageReturn,
    /// \u{240e}: '␎'
    SymbolForShiftOut,
    /// \u{240f}: '␏'
    SymbolForShiftIn,
    /// \u{2410}: '␐'
    SymbolForDataLinkEscape,
    /// \u{2411}: '␑'
    SymbolForDeviceControlOne,
    /// \u{2412}: '␒'
    SymbolForDeviceControlTwo,
    /// \u{2413}: '␓'
    SymbolForDeviceControlThree,
    /// \u{2414}: '␔'
    SymbolForDeviceControlFour,
    /// \u{2415}: '␕'
    SymbolForNegativeAcknowledge,
    /// \u{2416}: '␖'
    SymbolForSynchronousIdle,
    /// \u{2417}: '␗'
    SymbolForEndOfTransmissionBlock,
    /// \u{2418}: '␘'
    SymbolForCancel,
    /// \u{2419}: '␙'
    SymbolForEndOfMedium,
    /// \u{241a}: '␚'
    SymbolForSubstitute,
    /// \u{241b}: '␛'
    SymbolForEscape,
    /// \u{241c}: '␜'
    SymbolForFileSeparator,
    /// \u{241d}: '␝'
    SymbolForGroupSeparator,
    /// \u{241e}: '␞'
    SymbolForRecordSeparator,
    /// \u{241f}: '␟'
    SymbolForUnitSeparator,
    /// \u{2420}: '␠'
    SymbolForSpace,
    /// \u{2421}: '␡'
    SymbolForDelete,
    /// \u{2422}: '␢'
    BlankSymbol,
    /// \u{2423}: '␣'
    OpenBox,
    /// \u{2424}: '␤'
    SymbolForNewline,
    /// \u{2425}: '␥'
    SymbolForDeleteFormTwo,
    /// \u{2426}: '␦'
    SymbolForSubstituteFormTwo,
}

impl Into<char> for ControlPictures {
    fn into(self) -> char {
        match self {
            ControlPictures::SymbolForNull => '␀',
            ControlPictures::SymbolForStartOfHeading => '␁',
            ControlPictures::SymbolForStartOfText => '␂',
            ControlPictures::SymbolForEndOfText => '␃',
            ControlPictures::SymbolForEndOfTransmission => '␄',
            ControlPictures::SymbolForEnquiry => '␅',
            ControlPictures::SymbolForAcknowledge => '␆',
            ControlPictures::SymbolForBell => '␇',
            ControlPictures::SymbolForBackspace => '␈',
            ControlPictures::SymbolForHorizontalTabulation => '␉',
            ControlPictures::SymbolForLineFeed => '␊',
            ControlPictures::SymbolForVerticalTabulation => '␋',
            ControlPictures::SymbolForFormFeed => '␌',
            ControlPictures::SymbolForCarriageReturn => '␍',
            ControlPictures::SymbolForShiftOut => '␎',
            ControlPictures::SymbolForShiftIn => '␏',
            ControlPictures::SymbolForDataLinkEscape => '␐',
            ControlPictures::SymbolForDeviceControlOne => '␑',
            ControlPictures::SymbolForDeviceControlTwo => '␒',
            ControlPictures::SymbolForDeviceControlThree => '␓',
            ControlPictures::SymbolForDeviceControlFour => '␔',
            ControlPictures::SymbolForNegativeAcknowledge => '␕',
            ControlPictures::SymbolForSynchronousIdle => '␖',
            ControlPictures::SymbolForEndOfTransmissionBlock => '␗',
            ControlPictures::SymbolForCancel => '␘',
            ControlPictures::SymbolForEndOfMedium => '␙',
            ControlPictures::SymbolForSubstitute => '␚',
            ControlPictures::SymbolForEscape => '␛',
            ControlPictures::SymbolForFileSeparator => '␜',
            ControlPictures::SymbolForGroupSeparator => '␝',
            ControlPictures::SymbolForRecordSeparator => '␞',
            ControlPictures::SymbolForUnitSeparator => '␟',
            ControlPictures::SymbolForSpace => '␠',
            ControlPictures::SymbolForDelete => '␡',
            ControlPictures::BlankSymbol => '␢',
            ControlPictures::OpenBox => '␣',
            ControlPictures::SymbolForNewline => '␤',
            ControlPictures::SymbolForDeleteFormTwo => '␥',
            ControlPictures::SymbolForSubstituteFormTwo => '␦',
        }
    }
}

impl std::convert::TryFrom<char> for ControlPictures {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '␀' => Ok(ControlPictures::SymbolForNull),
            '␁' => Ok(ControlPictures::SymbolForStartOfHeading),
            '␂' => Ok(ControlPictures::SymbolForStartOfText),
            '␃' => Ok(ControlPictures::SymbolForEndOfText),
            '␄' => Ok(ControlPictures::SymbolForEndOfTransmission),
            '␅' => Ok(ControlPictures::SymbolForEnquiry),
            '␆' => Ok(ControlPictures::SymbolForAcknowledge),
            '␇' => Ok(ControlPictures::SymbolForBell),
            '␈' => Ok(ControlPictures::SymbolForBackspace),
            '␉' => Ok(ControlPictures::SymbolForHorizontalTabulation),
            '␊' => Ok(ControlPictures::SymbolForLineFeed),
            '␋' => Ok(ControlPictures::SymbolForVerticalTabulation),
            '␌' => Ok(ControlPictures::SymbolForFormFeed),
            '␍' => Ok(ControlPictures::SymbolForCarriageReturn),
            '␎' => Ok(ControlPictures::SymbolForShiftOut),
            '␏' => Ok(ControlPictures::SymbolForShiftIn),
            '␐' => Ok(ControlPictures::SymbolForDataLinkEscape),
            '␑' => Ok(ControlPictures::SymbolForDeviceControlOne),
            '␒' => Ok(ControlPictures::SymbolForDeviceControlTwo),
            '␓' => Ok(ControlPictures::SymbolForDeviceControlThree),
            '␔' => Ok(ControlPictures::SymbolForDeviceControlFour),
            '␕' => Ok(ControlPictures::SymbolForNegativeAcknowledge),
            '␖' => Ok(ControlPictures::SymbolForSynchronousIdle),
            '␗' => Ok(ControlPictures::SymbolForEndOfTransmissionBlock),
            '␘' => Ok(ControlPictures::SymbolForCancel),
            '␙' => Ok(ControlPictures::SymbolForEndOfMedium),
            '␚' => Ok(ControlPictures::SymbolForSubstitute),
            '␛' => Ok(ControlPictures::SymbolForEscape),
            '␜' => Ok(ControlPictures::SymbolForFileSeparator),
            '␝' => Ok(ControlPictures::SymbolForGroupSeparator),
            '␞' => Ok(ControlPictures::SymbolForRecordSeparator),
            '␟' => Ok(ControlPictures::SymbolForUnitSeparator),
            '␠' => Ok(ControlPictures::SymbolForSpace),
            '␡' => Ok(ControlPictures::SymbolForDelete),
            '␢' => Ok(ControlPictures::BlankSymbol),
            '␣' => Ok(ControlPictures::OpenBox),
            '␤' => Ok(ControlPictures::SymbolForNewline),
            '␥' => Ok(ControlPictures::SymbolForDeleteFormTwo),
            '␦' => Ok(ControlPictures::SymbolForSubstituteFormTwo),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ControlPictures {
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

impl std::convert::TryFrom<u32> for ControlPictures {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ControlPictures {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ControlPictures {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ControlPictures::SymbolForNull
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ControlPictures{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
