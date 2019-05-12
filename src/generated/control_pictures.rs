/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{2400}: '␀'
    pub const SYMBOL_FOR_NULL: char = '␀';
    /// \u{2401}: '␁'
    pub const SYMBOL_FOR_START_OF_HEADING: char = '␁';
    /// \u{2402}: '␂'
    pub const SYMBOL_FOR_START_OF_TEXT: char = '␂';
    /// \u{2403}: '␃'
    pub const SYMBOL_FOR_END_OF_TEXT: char = '␃';
    /// \u{2404}: '␄'
    pub const SYMBOL_FOR_END_OF_TRANSMISSION: char = '␄';
    /// \u{2405}: '␅'
    pub const SYMBOL_FOR_ENQUIRY: char = '␅';
    /// \u{2406}: '␆'
    pub const SYMBOL_FOR_ACKNOWLEDGE: char = '␆';
    /// \u{2407}: '␇'
    pub const SYMBOL_FOR_BELL: char = '␇';
    /// \u{2408}: '␈'
    pub const SYMBOL_FOR_BACKSPACE: char = '␈';
    /// \u{2409}: '␉'
    pub const SYMBOL_FOR_HORIZONTAL_TABULATION: char = '␉';
    /// \u{240a}: '␊'
    pub const SYMBOL_FOR_LINE_FEED: char = '␊';
    /// \u{240b}: '␋'
    pub const SYMBOL_FOR_VERTICAL_TABULATION: char = '␋';
    /// \u{240c}: '␌'
    pub const SYMBOL_FOR_FORM_FEED: char = '␌';
    /// \u{240d}: '␍'
    pub const SYMBOL_FOR_CARRIAGE_RETURN: char = '␍';
    /// \u{240e}: '␎'
    pub const SYMBOL_FOR_SHIFT_OUT: char = '␎';
    /// \u{240f}: '␏'
    pub const SYMBOL_FOR_SHIFT_IN: char = '␏';
    /// \u{2410}: '␐'
    pub const SYMBOL_FOR_DATA_LINK_ESCAPE: char = '␐';
    /// \u{2411}: '␑'
    pub const SYMBOL_FOR_DEVICE_CONTROL_ONE: char = '␑';
    /// \u{2412}: '␒'
    pub const SYMBOL_FOR_DEVICE_CONTROL_TWO: char = '␒';
    /// \u{2413}: '␓'
    pub const SYMBOL_FOR_DEVICE_CONTROL_THREE: char = '␓';
    /// \u{2414}: '␔'
    pub const SYMBOL_FOR_DEVICE_CONTROL_FOUR: char = '␔';
    /// \u{2415}: '␕'
    pub const SYMBOL_FOR_NEGATIVE_ACKNOWLEDGE: char = '␕';
    /// \u{2416}: '␖'
    pub const SYMBOL_FOR_SYNCHRONOUS_IDLE: char = '␖';
    /// \u{2417}: '␗'
    pub const SYMBOL_FOR_END_OF_TRANSMISSION_BLOCK: char = '␗';
    /// \u{2418}: '␘'
    pub const SYMBOL_FOR_CANCEL: char = '␘';
    /// \u{2419}: '␙'
    pub const SYMBOL_FOR_END_OF_MEDIUM: char = '␙';
    /// \u{241a}: '␚'
    pub const SYMBOL_FOR_SUBSTITUTE: char = '␚';
    /// \u{241b}: '␛'
    pub const SYMBOL_FOR_ESCAPE: char = '␛';
    /// \u{241c}: '␜'
    pub const SYMBOL_FOR_FILE_SEPARATOR: char = '␜';
    /// \u{241d}: '␝'
    pub const SYMBOL_FOR_GROUP_SEPARATOR: char = '␝';
    /// \u{241e}: '␞'
    pub const SYMBOL_FOR_RECORD_SEPARATOR: char = '␞';
    /// \u{241f}: '␟'
    pub const SYMBOL_FOR_UNIT_SEPARATOR: char = '␟';
    /// \u{2420}: '␠'
    pub const SYMBOL_FOR_SPACE: char = '␠';
    /// \u{2421}: '␡'
    pub const SYMBOL_FOR_DELETE: char = '␡';
    /// \u{2422}: '␢'
    pub const BLANK_SYMBOL: char = '␢';
    /// \u{2423}: '␣'
    pub const OPEN_BOX: char = '␣';
    /// \u{2424}: '␤'
    pub const SYMBOL_FOR_NEWLINE: char = '␤';
    /// \u{2425}: '␥'
    pub const SYMBOL_FOR_DELETE_FORM_TWO: char = '␥';
    /// \u{2426}: '␦'
    pub const SYMBOL_FOR_SUBSTITUTE_FORM_TWO: char = '␦';
}

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
        use constants::*;
        match self {
            ControlPictures::SymbolForNull => SYMBOL_FOR_NULL,
            ControlPictures::SymbolForStartOfHeading => SYMBOL_FOR_START_OF_HEADING,
            ControlPictures::SymbolForStartOfText => SYMBOL_FOR_START_OF_TEXT,
            ControlPictures::SymbolForEndOfText => SYMBOL_FOR_END_OF_TEXT,
            ControlPictures::SymbolForEndOfTransmission => SYMBOL_FOR_END_OF_TRANSMISSION,
            ControlPictures::SymbolForEnquiry => SYMBOL_FOR_ENQUIRY,
            ControlPictures::SymbolForAcknowledge => SYMBOL_FOR_ACKNOWLEDGE,
            ControlPictures::SymbolForBell => SYMBOL_FOR_BELL,
            ControlPictures::SymbolForBackspace => SYMBOL_FOR_BACKSPACE,
            ControlPictures::SymbolForHorizontalTabulation => SYMBOL_FOR_HORIZONTAL_TABULATION,
            ControlPictures::SymbolForLineFeed => SYMBOL_FOR_LINE_FEED,
            ControlPictures::SymbolForVerticalTabulation => SYMBOL_FOR_VERTICAL_TABULATION,
            ControlPictures::SymbolForFormFeed => SYMBOL_FOR_FORM_FEED,
            ControlPictures::SymbolForCarriageReturn => SYMBOL_FOR_CARRIAGE_RETURN,
            ControlPictures::SymbolForShiftOut => SYMBOL_FOR_SHIFT_OUT,
            ControlPictures::SymbolForShiftIn => SYMBOL_FOR_SHIFT_IN,
            ControlPictures::SymbolForDataLinkEscape => SYMBOL_FOR_DATA_LINK_ESCAPE,
            ControlPictures::SymbolForDeviceControlOne => SYMBOL_FOR_DEVICE_CONTROL_ONE,
            ControlPictures::SymbolForDeviceControlTwo => SYMBOL_FOR_DEVICE_CONTROL_TWO,
            ControlPictures::SymbolForDeviceControlThree => SYMBOL_FOR_DEVICE_CONTROL_THREE,
            ControlPictures::SymbolForDeviceControlFour => SYMBOL_FOR_DEVICE_CONTROL_FOUR,
            ControlPictures::SymbolForNegativeAcknowledge => SYMBOL_FOR_NEGATIVE_ACKNOWLEDGE,
            ControlPictures::SymbolForSynchronousIdle => SYMBOL_FOR_SYNCHRONOUS_IDLE,
            ControlPictures::SymbolForEndOfTransmissionBlock => SYMBOL_FOR_END_OF_TRANSMISSION_BLOCK,
            ControlPictures::SymbolForCancel => SYMBOL_FOR_CANCEL,
            ControlPictures::SymbolForEndOfMedium => SYMBOL_FOR_END_OF_MEDIUM,
            ControlPictures::SymbolForSubstitute => SYMBOL_FOR_SUBSTITUTE,
            ControlPictures::SymbolForEscape => SYMBOL_FOR_ESCAPE,
            ControlPictures::SymbolForFileSeparator => SYMBOL_FOR_FILE_SEPARATOR,
            ControlPictures::SymbolForGroupSeparator => SYMBOL_FOR_GROUP_SEPARATOR,
            ControlPictures::SymbolForRecordSeparator => SYMBOL_FOR_RECORD_SEPARATOR,
            ControlPictures::SymbolForUnitSeparator => SYMBOL_FOR_UNIT_SEPARATOR,
            ControlPictures::SymbolForSpace => SYMBOL_FOR_SPACE,
            ControlPictures::SymbolForDelete => SYMBOL_FOR_DELETE,
            ControlPictures::BlankSymbol => BLANK_SYMBOL,
            ControlPictures::OpenBox => OPEN_BOX,
            ControlPictures::SymbolForNewline => SYMBOL_FOR_NEWLINE,
            ControlPictures::SymbolForDeleteFormTwo => SYMBOL_FOR_DELETE_FORM_TWO,
            ControlPictures::SymbolForSubstituteFormTwo => SYMBOL_FOR_SUBSTITUTE_FORM_TWO,
        }
    }
}

impl std::convert::TryFrom<char> for ControlPictures {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SYMBOL_FOR_NULL => Ok(ControlPictures::SymbolForNull),
            SYMBOL_FOR_START_OF_HEADING => Ok(ControlPictures::SymbolForStartOfHeading),
            SYMBOL_FOR_START_OF_TEXT => Ok(ControlPictures::SymbolForStartOfText),
            SYMBOL_FOR_END_OF_TEXT => Ok(ControlPictures::SymbolForEndOfText),
            SYMBOL_FOR_END_OF_TRANSMISSION => Ok(ControlPictures::SymbolForEndOfTransmission),
            SYMBOL_FOR_ENQUIRY => Ok(ControlPictures::SymbolForEnquiry),
            SYMBOL_FOR_ACKNOWLEDGE => Ok(ControlPictures::SymbolForAcknowledge),
            SYMBOL_FOR_BELL => Ok(ControlPictures::SymbolForBell),
            SYMBOL_FOR_BACKSPACE => Ok(ControlPictures::SymbolForBackspace),
            SYMBOL_FOR_HORIZONTAL_TABULATION => Ok(ControlPictures::SymbolForHorizontalTabulation),
            SYMBOL_FOR_LINE_FEED => Ok(ControlPictures::SymbolForLineFeed),
            SYMBOL_FOR_VERTICAL_TABULATION => Ok(ControlPictures::SymbolForVerticalTabulation),
            SYMBOL_FOR_FORM_FEED => Ok(ControlPictures::SymbolForFormFeed),
            SYMBOL_FOR_CARRIAGE_RETURN => Ok(ControlPictures::SymbolForCarriageReturn),
            SYMBOL_FOR_SHIFT_OUT => Ok(ControlPictures::SymbolForShiftOut),
            SYMBOL_FOR_SHIFT_IN => Ok(ControlPictures::SymbolForShiftIn),
            SYMBOL_FOR_DATA_LINK_ESCAPE => Ok(ControlPictures::SymbolForDataLinkEscape),
            SYMBOL_FOR_DEVICE_CONTROL_ONE => Ok(ControlPictures::SymbolForDeviceControlOne),
            SYMBOL_FOR_DEVICE_CONTROL_TWO => Ok(ControlPictures::SymbolForDeviceControlTwo),
            SYMBOL_FOR_DEVICE_CONTROL_THREE => Ok(ControlPictures::SymbolForDeviceControlThree),
            SYMBOL_FOR_DEVICE_CONTROL_FOUR => Ok(ControlPictures::SymbolForDeviceControlFour),
            SYMBOL_FOR_NEGATIVE_ACKNOWLEDGE => Ok(ControlPictures::SymbolForNegativeAcknowledge),
            SYMBOL_FOR_SYNCHRONOUS_IDLE => Ok(ControlPictures::SymbolForSynchronousIdle),
            SYMBOL_FOR_END_OF_TRANSMISSION_BLOCK => Ok(ControlPictures::SymbolForEndOfTransmissionBlock),
            SYMBOL_FOR_CANCEL => Ok(ControlPictures::SymbolForCancel),
            SYMBOL_FOR_END_OF_MEDIUM => Ok(ControlPictures::SymbolForEndOfMedium),
            SYMBOL_FOR_SUBSTITUTE => Ok(ControlPictures::SymbolForSubstitute),
            SYMBOL_FOR_ESCAPE => Ok(ControlPictures::SymbolForEscape),
            SYMBOL_FOR_FILE_SEPARATOR => Ok(ControlPictures::SymbolForFileSeparator),
            SYMBOL_FOR_GROUP_SEPARATOR => Ok(ControlPictures::SymbolForGroupSeparator),
            SYMBOL_FOR_RECORD_SEPARATOR => Ok(ControlPictures::SymbolForRecordSeparator),
            SYMBOL_FOR_UNIT_SEPARATOR => Ok(ControlPictures::SymbolForUnitSeparator),
            SYMBOL_FOR_SPACE => Ok(ControlPictures::SymbolForSpace),
            SYMBOL_FOR_DELETE => Ok(ControlPictures::SymbolForDelete),
            BLANK_SYMBOL => Ok(ControlPictures::BlankSymbol),
            OPEN_BOX => Ok(ControlPictures::OpenBox),
            SYMBOL_FOR_NEWLINE => Ok(ControlPictures::SymbolForNewline),
            SYMBOL_FOR_DELETE_FORM_TWO => Ok(ControlPictures::SymbolForDeleteFormTwo),
            SYMBOL_FOR_SUBSTITUTE_FORM_TWO => Ok(ControlPictures::SymbolForSubstituteFormTwo),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            ControlPictures::SymbolForNull => "symbol for null",
            ControlPictures::SymbolForStartOfHeading => "symbol for start of heading",
            ControlPictures::SymbolForStartOfText => "symbol for start of text",
            ControlPictures::SymbolForEndOfText => "symbol for end of text",
            ControlPictures::SymbolForEndOfTransmission => "symbol for end of transmission",
            ControlPictures::SymbolForEnquiry => "symbol for enquiry",
            ControlPictures::SymbolForAcknowledge => "symbol for acknowledge",
            ControlPictures::SymbolForBell => "symbol for bell",
            ControlPictures::SymbolForBackspace => "symbol for backspace",
            ControlPictures::SymbolForHorizontalTabulation => "symbol for horizontal tabulation",
            ControlPictures::SymbolForLineFeed => "symbol for line feed",
            ControlPictures::SymbolForVerticalTabulation => "symbol for vertical tabulation",
            ControlPictures::SymbolForFormFeed => "symbol for form feed",
            ControlPictures::SymbolForCarriageReturn => "symbol for carriage return",
            ControlPictures::SymbolForShiftOut => "symbol for shift out",
            ControlPictures::SymbolForShiftIn => "symbol for shift in",
            ControlPictures::SymbolForDataLinkEscape => "symbol for data link escape",
            ControlPictures::SymbolForDeviceControlOne => "symbol for device control one",
            ControlPictures::SymbolForDeviceControlTwo => "symbol for device control two",
            ControlPictures::SymbolForDeviceControlThree => "symbol for device control three",
            ControlPictures::SymbolForDeviceControlFour => "symbol for device control four",
            ControlPictures::SymbolForNegativeAcknowledge => "symbol for negative acknowledge",
            ControlPictures::SymbolForSynchronousIdle => "symbol for synchronous idle",
            ControlPictures::SymbolForEndOfTransmissionBlock => "symbol for end of transmission block",
            ControlPictures::SymbolForCancel => "symbol for cancel",
            ControlPictures::SymbolForEndOfMedium => "symbol for end of medium",
            ControlPictures::SymbolForSubstitute => "symbol for substitute",
            ControlPictures::SymbolForEscape => "symbol for escape",
            ControlPictures::SymbolForFileSeparator => "symbol for file separator",
            ControlPictures::SymbolForGroupSeparator => "symbol for group separator",
            ControlPictures::SymbolForRecordSeparator => "symbol for record separator",
            ControlPictures::SymbolForUnitSeparator => "symbol for unit separator",
            ControlPictures::SymbolForSpace => "symbol for space",
            ControlPictures::SymbolForDelete => "symbol for delete",
            ControlPictures::BlankSymbol => "blank symbol",
            ControlPictures::OpenBox => "open box",
            ControlPictures::SymbolForNewline => "symbol for newline",
            ControlPictures::SymbolForDeleteFormTwo => "symbol for delete form two",
            ControlPictures::SymbolForSubstituteFormTwo => "symbol for substitute form two",
        }
    }
}
