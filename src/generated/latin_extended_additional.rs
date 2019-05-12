
/// An enum to represent all characters in the LatinExtendedAdditional block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LatinExtendedAdditional {
    /// \u{1e00}: 'Ḁ'
    LatinCapitalLetterAWithRingBelow,
    /// \u{1e01}: 'ḁ'
    LatinSmallLetterAWithRingBelow,
    /// \u{1e02}: 'Ḃ'
    LatinCapitalLetterBWithDotAbove,
    /// \u{1e03}: 'ḃ'
    LatinSmallLetterBWithDotAbove,
    /// \u{1e04}: 'Ḅ'
    LatinCapitalLetterBWithDotBelow,
    /// \u{1e05}: 'ḅ'
    LatinSmallLetterBWithDotBelow,
    /// \u{1e06}: 'Ḇ'
    LatinCapitalLetterBWithLineBelow,
    /// \u{1e07}: 'ḇ'
    LatinSmallLetterBWithLineBelow,
    /// \u{1e08}: 'Ḉ'
    LatinCapitalLetterCWithCedillaAndAcute,
    /// \u{1e09}: 'ḉ'
    LatinSmallLetterCWithCedillaAndAcute,
    /// \u{1e0a}: 'Ḋ'
    LatinCapitalLetterDWithDotAbove,
    /// \u{1e0b}: 'ḋ'
    LatinSmallLetterDWithDotAbove,
    /// \u{1e0c}: 'Ḍ'
    LatinCapitalLetterDWithDotBelow,
    /// \u{1e0d}: 'ḍ'
    LatinSmallLetterDWithDotBelow,
    /// \u{1e0e}: 'Ḏ'
    LatinCapitalLetterDWithLineBelow,
    /// \u{1e0f}: 'ḏ'
    LatinSmallLetterDWithLineBelow,
    /// \u{1e10}: 'Ḑ'
    LatinCapitalLetterDWithCedilla,
    /// \u{1e11}: 'ḑ'
    LatinSmallLetterDWithCedilla,
    /// \u{1e12}: 'Ḓ'
    LatinCapitalLetterDWithCircumflexBelow,
    /// \u{1e13}: 'ḓ'
    LatinSmallLetterDWithCircumflexBelow,
    /// \u{1e14}: 'Ḕ'
    LatinCapitalLetterEWithMacronAndGrave,
    /// \u{1e15}: 'ḕ'
    LatinSmallLetterEWithMacronAndGrave,
    /// \u{1e16}: 'Ḗ'
    LatinCapitalLetterEWithMacronAndAcute,
    /// \u{1e17}: 'ḗ'
    LatinSmallLetterEWithMacronAndAcute,
    /// \u{1e18}: 'Ḙ'
    LatinCapitalLetterEWithCircumflexBelow,
    /// \u{1e19}: 'ḙ'
    LatinSmallLetterEWithCircumflexBelow,
    /// \u{1e1a}: 'Ḛ'
    LatinCapitalLetterEWithTildeBelow,
    /// \u{1e1b}: 'ḛ'
    LatinSmallLetterEWithTildeBelow,
    /// \u{1e1c}: 'Ḝ'
    LatinCapitalLetterEWithCedillaAndBreve,
    /// \u{1e1d}: 'ḝ'
    LatinSmallLetterEWithCedillaAndBreve,
    /// \u{1e1e}: 'Ḟ'
    LatinCapitalLetterFWithDotAbove,
    /// \u{1e1f}: 'ḟ'
    LatinSmallLetterFWithDotAbove,
    /// \u{1e20}: 'Ḡ'
    LatinCapitalLetterGWithMacron,
    /// \u{1e21}: 'ḡ'
    LatinSmallLetterGWithMacron,
    /// \u{1e22}: 'Ḣ'
    LatinCapitalLetterHWithDotAbove,
    /// \u{1e23}: 'ḣ'
    LatinSmallLetterHWithDotAbove,
    /// \u{1e24}: 'Ḥ'
    LatinCapitalLetterHWithDotBelow,
    /// \u{1e25}: 'ḥ'
    LatinSmallLetterHWithDotBelow,
    /// \u{1e26}: 'Ḧ'
    LatinCapitalLetterHWithDiaeresis,
    /// \u{1e27}: 'ḧ'
    LatinSmallLetterHWithDiaeresis,
    /// \u{1e28}: 'Ḩ'
    LatinCapitalLetterHWithCedilla,
    /// \u{1e29}: 'ḩ'
    LatinSmallLetterHWithCedilla,
    /// \u{1e2a}: 'Ḫ'
    LatinCapitalLetterHWithBreveBelow,
    /// \u{1e2b}: 'ḫ'
    LatinSmallLetterHWithBreveBelow,
    /// \u{1e2c}: 'Ḭ'
    LatinCapitalLetterIWithTildeBelow,
    /// \u{1e2d}: 'ḭ'
    LatinSmallLetterIWithTildeBelow,
    /// \u{1e2e}: 'Ḯ'
    LatinCapitalLetterIWithDiaeresisAndAcute,
    /// \u{1e2f}: 'ḯ'
    LatinSmallLetterIWithDiaeresisAndAcute,
    /// \u{1e30}: 'Ḱ'
    LatinCapitalLetterKWithAcute,
    /// \u{1e31}: 'ḱ'
    LatinSmallLetterKWithAcute,
    /// \u{1e32}: 'Ḳ'
    LatinCapitalLetterKWithDotBelow,
    /// \u{1e33}: 'ḳ'
    LatinSmallLetterKWithDotBelow,
    /// \u{1e34}: 'Ḵ'
    LatinCapitalLetterKWithLineBelow,
    /// \u{1e35}: 'ḵ'
    LatinSmallLetterKWithLineBelow,
    /// \u{1e36}: 'Ḷ'
    LatinCapitalLetterLWithDotBelow,
    /// \u{1e37}: 'ḷ'
    LatinSmallLetterLWithDotBelow,
    /// \u{1e38}: 'Ḹ'
    LatinCapitalLetterLWithDotBelowAndMacron,
    /// \u{1e39}: 'ḹ'
    LatinSmallLetterLWithDotBelowAndMacron,
    /// \u{1e3a}: 'Ḻ'
    LatinCapitalLetterLWithLineBelow,
    /// \u{1e3b}: 'ḻ'
    LatinSmallLetterLWithLineBelow,
    /// \u{1e3c}: 'Ḽ'
    LatinCapitalLetterLWithCircumflexBelow,
    /// \u{1e3d}: 'ḽ'
    LatinSmallLetterLWithCircumflexBelow,
    /// \u{1e3e}: 'Ḿ'
    LatinCapitalLetterMWithAcute,
    /// \u{1e3f}: 'ḿ'
    LatinSmallLetterMWithAcute,
    /// \u{1e40}: 'Ṁ'
    LatinCapitalLetterMWithDotAbove,
    /// \u{1e41}: 'ṁ'
    LatinSmallLetterMWithDotAbove,
    /// \u{1e42}: 'Ṃ'
    LatinCapitalLetterMWithDotBelow,
    /// \u{1e43}: 'ṃ'
    LatinSmallLetterMWithDotBelow,
    /// \u{1e44}: 'Ṅ'
    LatinCapitalLetterNWithDotAbove,
    /// \u{1e45}: 'ṅ'
    LatinSmallLetterNWithDotAbove,
    /// \u{1e46}: 'Ṇ'
    LatinCapitalLetterNWithDotBelow,
    /// \u{1e47}: 'ṇ'
    LatinSmallLetterNWithDotBelow,
    /// \u{1e48}: 'Ṉ'
    LatinCapitalLetterNWithLineBelow,
    /// \u{1e49}: 'ṉ'
    LatinSmallLetterNWithLineBelow,
    /// \u{1e4a}: 'Ṋ'
    LatinCapitalLetterNWithCircumflexBelow,
    /// \u{1e4b}: 'ṋ'
    LatinSmallLetterNWithCircumflexBelow,
    /// \u{1e4c}: 'Ṍ'
    LatinCapitalLetterOWithTildeAndAcute,
    /// \u{1e4d}: 'ṍ'
    LatinSmallLetterOWithTildeAndAcute,
    /// \u{1e4e}: 'Ṏ'
    LatinCapitalLetterOWithTildeAndDiaeresis,
    /// \u{1e4f}: 'ṏ'
    LatinSmallLetterOWithTildeAndDiaeresis,
    /// \u{1e50}: 'Ṑ'
    LatinCapitalLetterOWithMacronAndGrave,
    /// \u{1e51}: 'ṑ'
    LatinSmallLetterOWithMacronAndGrave,
    /// \u{1e52}: 'Ṓ'
    LatinCapitalLetterOWithMacronAndAcute,
    /// \u{1e53}: 'ṓ'
    LatinSmallLetterOWithMacronAndAcute,
    /// \u{1e54}: 'Ṕ'
    LatinCapitalLetterPWithAcute,
    /// \u{1e55}: 'ṕ'
    LatinSmallLetterPWithAcute,
    /// \u{1e56}: 'Ṗ'
    LatinCapitalLetterPWithDotAbove,
    /// \u{1e57}: 'ṗ'
    LatinSmallLetterPWithDotAbove,
    /// \u{1e58}: 'Ṙ'
    LatinCapitalLetterRWithDotAbove,
    /// \u{1e59}: 'ṙ'
    LatinSmallLetterRWithDotAbove,
    /// \u{1e5a}: 'Ṛ'
    LatinCapitalLetterRWithDotBelow,
    /// \u{1e5b}: 'ṛ'
    LatinSmallLetterRWithDotBelow,
    /// \u{1e5c}: 'Ṝ'
    LatinCapitalLetterRWithDotBelowAndMacron,
    /// \u{1e5d}: 'ṝ'
    LatinSmallLetterRWithDotBelowAndMacron,
    /// \u{1e5e}: 'Ṟ'
    LatinCapitalLetterRWithLineBelow,
    /// \u{1e5f}: 'ṟ'
    LatinSmallLetterRWithLineBelow,
    /// \u{1e60}: 'Ṡ'
    LatinCapitalLetterSWithDotAbove,
    /// \u{1e61}: 'ṡ'
    LatinSmallLetterSWithDotAbove,
    /// \u{1e62}: 'Ṣ'
    LatinCapitalLetterSWithDotBelow,
    /// \u{1e63}: 'ṣ'
    LatinSmallLetterSWithDotBelow,
    /// \u{1e64}: 'Ṥ'
    LatinCapitalLetterSWithAcuteAndDotAbove,
    /// \u{1e65}: 'ṥ'
    LatinSmallLetterSWithAcuteAndDotAbove,
    /// \u{1e66}: 'Ṧ'
    LatinCapitalLetterSWithCaronAndDotAbove,
    /// \u{1e67}: 'ṧ'
    LatinSmallLetterSWithCaronAndDotAbove,
    /// \u{1e68}: 'Ṩ'
    LatinCapitalLetterSWithDotBelowAndDotAbove,
    /// \u{1e69}: 'ṩ'
    LatinSmallLetterSWithDotBelowAndDotAbove,
    /// \u{1e6a}: 'Ṫ'
    LatinCapitalLetterTWithDotAbove,
    /// \u{1e6b}: 'ṫ'
    LatinSmallLetterTWithDotAbove,
    /// \u{1e6c}: 'Ṭ'
    LatinCapitalLetterTWithDotBelow,
    /// \u{1e6d}: 'ṭ'
    LatinSmallLetterTWithDotBelow,
    /// \u{1e6e}: 'Ṯ'
    LatinCapitalLetterTWithLineBelow,
    /// \u{1e6f}: 'ṯ'
    LatinSmallLetterTWithLineBelow,
    /// \u{1e70}: 'Ṱ'
    LatinCapitalLetterTWithCircumflexBelow,
    /// \u{1e71}: 'ṱ'
    LatinSmallLetterTWithCircumflexBelow,
    /// \u{1e72}: 'Ṳ'
    LatinCapitalLetterUWithDiaeresisBelow,
    /// \u{1e73}: 'ṳ'
    LatinSmallLetterUWithDiaeresisBelow,
    /// \u{1e74}: 'Ṵ'
    LatinCapitalLetterUWithTildeBelow,
    /// \u{1e75}: 'ṵ'
    LatinSmallLetterUWithTildeBelow,
    /// \u{1e76}: 'Ṷ'
    LatinCapitalLetterUWithCircumflexBelow,
    /// \u{1e77}: 'ṷ'
    LatinSmallLetterUWithCircumflexBelow,
    /// \u{1e78}: 'Ṹ'
    LatinCapitalLetterUWithTildeAndAcute,
    /// \u{1e79}: 'ṹ'
    LatinSmallLetterUWithTildeAndAcute,
    /// \u{1e7a}: 'Ṻ'
    LatinCapitalLetterUWithMacronAndDiaeresis,
    /// \u{1e7b}: 'ṻ'
    LatinSmallLetterUWithMacronAndDiaeresis,
    /// \u{1e7c}: 'Ṽ'
    LatinCapitalLetterVWithTilde,
    /// \u{1e7d}: 'ṽ'
    LatinSmallLetterVWithTilde,
    /// \u{1e7e}: 'Ṿ'
    LatinCapitalLetterVWithDotBelow,
    /// \u{1e7f}: 'ṿ'
    LatinSmallLetterVWithDotBelow,
    /// \u{1e80}: 'Ẁ'
    LatinCapitalLetterWWithGrave,
    /// \u{1e81}: 'ẁ'
    LatinSmallLetterWWithGrave,
    /// \u{1e82}: 'Ẃ'
    LatinCapitalLetterWWithAcute,
    /// \u{1e83}: 'ẃ'
    LatinSmallLetterWWithAcute,
    /// \u{1e84}: 'Ẅ'
    LatinCapitalLetterWWithDiaeresis,
    /// \u{1e85}: 'ẅ'
    LatinSmallLetterWWithDiaeresis,
    /// \u{1e86}: 'Ẇ'
    LatinCapitalLetterWWithDotAbove,
    /// \u{1e87}: 'ẇ'
    LatinSmallLetterWWithDotAbove,
    /// \u{1e88}: 'Ẉ'
    LatinCapitalLetterWWithDotBelow,
    /// \u{1e89}: 'ẉ'
    LatinSmallLetterWWithDotBelow,
    /// \u{1e8a}: 'Ẋ'
    LatinCapitalLetterXWithDotAbove,
    /// \u{1e8b}: 'ẋ'
    LatinSmallLetterXWithDotAbove,
    /// \u{1e8c}: 'Ẍ'
    LatinCapitalLetterXWithDiaeresis,
    /// \u{1e8d}: 'ẍ'
    LatinSmallLetterXWithDiaeresis,
    /// \u{1e8e}: 'Ẏ'
    LatinCapitalLetterYWithDotAbove,
    /// \u{1e8f}: 'ẏ'
    LatinSmallLetterYWithDotAbove,
    /// \u{1e90}: 'Ẑ'
    LatinCapitalLetterZWithCircumflex,
    /// \u{1e91}: 'ẑ'
    LatinSmallLetterZWithCircumflex,
    /// \u{1e92}: 'Ẓ'
    LatinCapitalLetterZWithDotBelow,
    /// \u{1e93}: 'ẓ'
    LatinSmallLetterZWithDotBelow,
    /// \u{1e94}: 'Ẕ'
    LatinCapitalLetterZWithLineBelow,
    /// \u{1e95}: 'ẕ'
    LatinSmallLetterZWithLineBelow,
    /// \u{1e96}: 'ẖ'
    LatinSmallLetterHWithLineBelow,
    /// \u{1e97}: 'ẗ'
    LatinSmallLetterTWithDiaeresis,
    /// \u{1e98}: 'ẘ'
    LatinSmallLetterWWithRingAbove,
    /// \u{1e99}: 'ẙ'
    LatinSmallLetterYWithRingAbove,
    /// \u{1e9a}: 'ẚ'
    LatinSmallLetterAWithRightHalfRing,
    /// \u{1e9b}: 'ẛ'
    LatinSmallLetterLongSWithDotAbove,
    /// \u{1e9c}: 'ẜ'
    LatinSmallLetterLongSWithDiagonalStroke,
    /// \u{1e9d}: 'ẝ'
    LatinSmallLetterLongSWithHighStroke,
    /// \u{1e9e}: 'ẞ'
    LatinCapitalLetterSharpS,
    /// \u{1e9f}: 'ẟ'
    LatinSmallLetterDelta,
    /// \u{1ea0}: 'Ạ'
    LatinCapitalLetterAWithDotBelow,
    /// \u{1ea1}: 'ạ'
    LatinSmallLetterAWithDotBelow,
    /// \u{1ea2}: 'Ả'
    LatinCapitalLetterAWithHookAbove,
    /// \u{1ea3}: 'ả'
    LatinSmallLetterAWithHookAbove,
    /// \u{1ea4}: 'Ấ'
    LatinCapitalLetterAWithCircumflexAndAcute,
    /// \u{1ea5}: 'ấ'
    LatinSmallLetterAWithCircumflexAndAcute,
    /// \u{1ea6}: 'Ầ'
    LatinCapitalLetterAWithCircumflexAndGrave,
    /// \u{1ea7}: 'ầ'
    LatinSmallLetterAWithCircumflexAndGrave,
    /// \u{1ea8}: 'Ẩ'
    LatinCapitalLetterAWithCircumflexAndHookAbove,
    /// \u{1ea9}: 'ẩ'
    LatinSmallLetterAWithCircumflexAndHookAbove,
    /// \u{1eaa}: 'Ẫ'
    LatinCapitalLetterAWithCircumflexAndTilde,
    /// \u{1eab}: 'ẫ'
    LatinSmallLetterAWithCircumflexAndTilde,
    /// \u{1eac}: 'Ậ'
    LatinCapitalLetterAWithCircumflexAndDotBelow,
    /// \u{1ead}: 'ậ'
    LatinSmallLetterAWithCircumflexAndDotBelow,
    /// \u{1eae}: 'Ắ'
    LatinCapitalLetterAWithBreveAndAcute,
    /// \u{1eaf}: 'ắ'
    LatinSmallLetterAWithBreveAndAcute,
    /// \u{1eb0}: 'Ằ'
    LatinCapitalLetterAWithBreveAndGrave,
    /// \u{1eb1}: 'ằ'
    LatinSmallLetterAWithBreveAndGrave,
    /// \u{1eb2}: 'Ẳ'
    LatinCapitalLetterAWithBreveAndHookAbove,
    /// \u{1eb3}: 'ẳ'
    LatinSmallLetterAWithBreveAndHookAbove,
    /// \u{1eb4}: 'Ẵ'
    LatinCapitalLetterAWithBreveAndTilde,
    /// \u{1eb5}: 'ẵ'
    LatinSmallLetterAWithBreveAndTilde,
    /// \u{1eb6}: 'Ặ'
    LatinCapitalLetterAWithBreveAndDotBelow,
    /// \u{1eb7}: 'ặ'
    LatinSmallLetterAWithBreveAndDotBelow,
    /// \u{1eb8}: 'Ẹ'
    LatinCapitalLetterEWithDotBelow,
    /// \u{1eb9}: 'ẹ'
    LatinSmallLetterEWithDotBelow,
    /// \u{1eba}: 'Ẻ'
    LatinCapitalLetterEWithHookAbove,
    /// \u{1ebb}: 'ẻ'
    LatinSmallLetterEWithHookAbove,
    /// \u{1ebc}: 'Ẽ'
    LatinCapitalLetterEWithTilde,
    /// \u{1ebd}: 'ẽ'
    LatinSmallLetterEWithTilde,
    /// \u{1ebe}: 'Ế'
    LatinCapitalLetterEWithCircumflexAndAcute,
    /// \u{1ebf}: 'ế'
    LatinSmallLetterEWithCircumflexAndAcute,
    /// \u{1ec0}: 'Ề'
    LatinCapitalLetterEWithCircumflexAndGrave,
    /// \u{1ec1}: 'ề'
    LatinSmallLetterEWithCircumflexAndGrave,
    /// \u{1ec2}: 'Ể'
    LatinCapitalLetterEWithCircumflexAndHookAbove,
    /// \u{1ec3}: 'ể'
    LatinSmallLetterEWithCircumflexAndHookAbove,
    /// \u{1ec4}: 'Ễ'
    LatinCapitalLetterEWithCircumflexAndTilde,
    /// \u{1ec5}: 'ễ'
    LatinSmallLetterEWithCircumflexAndTilde,
    /// \u{1ec6}: 'Ệ'
    LatinCapitalLetterEWithCircumflexAndDotBelow,
    /// \u{1ec7}: 'ệ'
    LatinSmallLetterEWithCircumflexAndDotBelow,
    /// \u{1ec8}: 'Ỉ'
    LatinCapitalLetterIWithHookAbove,
    /// \u{1ec9}: 'ỉ'
    LatinSmallLetterIWithHookAbove,
    /// \u{1eca}: 'Ị'
    LatinCapitalLetterIWithDotBelow,
    /// \u{1ecb}: 'ị'
    LatinSmallLetterIWithDotBelow,
    /// \u{1ecc}: 'Ọ'
    LatinCapitalLetterOWithDotBelow,
    /// \u{1ecd}: 'ọ'
    LatinSmallLetterOWithDotBelow,
    /// \u{1ece}: 'Ỏ'
    LatinCapitalLetterOWithHookAbove,
    /// \u{1ecf}: 'ỏ'
    LatinSmallLetterOWithHookAbove,
    /// \u{1ed0}: 'Ố'
    LatinCapitalLetterOWithCircumflexAndAcute,
    /// \u{1ed1}: 'ố'
    LatinSmallLetterOWithCircumflexAndAcute,
    /// \u{1ed2}: 'Ồ'
    LatinCapitalLetterOWithCircumflexAndGrave,
    /// \u{1ed3}: 'ồ'
    LatinSmallLetterOWithCircumflexAndGrave,
    /// \u{1ed4}: 'Ổ'
    LatinCapitalLetterOWithCircumflexAndHookAbove,
    /// \u{1ed5}: 'ổ'
    LatinSmallLetterOWithCircumflexAndHookAbove,
    /// \u{1ed6}: 'Ỗ'
    LatinCapitalLetterOWithCircumflexAndTilde,
    /// \u{1ed7}: 'ỗ'
    LatinSmallLetterOWithCircumflexAndTilde,
    /// \u{1ed8}: 'Ộ'
    LatinCapitalLetterOWithCircumflexAndDotBelow,
    /// \u{1ed9}: 'ộ'
    LatinSmallLetterOWithCircumflexAndDotBelow,
    /// \u{1eda}: 'Ớ'
    LatinCapitalLetterOWithHornAndAcute,
    /// \u{1edb}: 'ớ'
    LatinSmallLetterOWithHornAndAcute,
    /// \u{1edc}: 'Ờ'
    LatinCapitalLetterOWithHornAndGrave,
    /// \u{1edd}: 'ờ'
    LatinSmallLetterOWithHornAndGrave,
    /// \u{1ede}: 'Ở'
    LatinCapitalLetterOWithHornAndHookAbove,
    /// \u{1edf}: 'ở'
    LatinSmallLetterOWithHornAndHookAbove,
    /// \u{1ee0}: 'Ỡ'
    LatinCapitalLetterOWithHornAndTilde,
    /// \u{1ee1}: 'ỡ'
    LatinSmallLetterOWithHornAndTilde,
    /// \u{1ee2}: 'Ợ'
    LatinCapitalLetterOWithHornAndDotBelow,
    /// \u{1ee3}: 'ợ'
    LatinSmallLetterOWithHornAndDotBelow,
    /// \u{1ee4}: 'Ụ'
    LatinCapitalLetterUWithDotBelow,
    /// \u{1ee5}: 'ụ'
    LatinSmallLetterUWithDotBelow,
    /// \u{1ee6}: 'Ủ'
    LatinCapitalLetterUWithHookAbove,
    /// \u{1ee7}: 'ủ'
    LatinSmallLetterUWithHookAbove,
    /// \u{1ee8}: 'Ứ'
    LatinCapitalLetterUWithHornAndAcute,
    /// \u{1ee9}: 'ứ'
    LatinSmallLetterUWithHornAndAcute,
    /// \u{1eea}: 'Ừ'
    LatinCapitalLetterUWithHornAndGrave,
    /// \u{1eeb}: 'ừ'
    LatinSmallLetterUWithHornAndGrave,
    /// \u{1eec}: 'Ử'
    LatinCapitalLetterUWithHornAndHookAbove,
    /// \u{1eed}: 'ử'
    LatinSmallLetterUWithHornAndHookAbove,
    /// \u{1eee}: 'Ữ'
    LatinCapitalLetterUWithHornAndTilde,
    /// \u{1eef}: 'ữ'
    LatinSmallLetterUWithHornAndTilde,
    /// \u{1ef0}: 'Ự'
    LatinCapitalLetterUWithHornAndDotBelow,
    /// \u{1ef1}: 'ự'
    LatinSmallLetterUWithHornAndDotBelow,
    /// \u{1ef2}: 'Ỳ'
    LatinCapitalLetterYWithGrave,
    /// \u{1ef3}: 'ỳ'
    LatinSmallLetterYWithGrave,
    /// \u{1ef4}: 'Ỵ'
    LatinCapitalLetterYWithDotBelow,
    /// \u{1ef5}: 'ỵ'
    LatinSmallLetterYWithDotBelow,
    /// \u{1ef6}: 'Ỷ'
    LatinCapitalLetterYWithHookAbove,
    /// \u{1ef7}: 'ỷ'
    LatinSmallLetterYWithHookAbove,
    /// \u{1ef8}: 'Ỹ'
    LatinCapitalLetterYWithTilde,
    /// \u{1ef9}: 'ỹ'
    LatinSmallLetterYWithTilde,
    /// \u{1efa}: 'Ỻ'
    LatinCapitalLetterMiddleDashWelshLl,
    /// \u{1efb}: 'ỻ'
    LatinSmallLetterMiddleDashWelshLl,
    /// \u{1efc}: 'Ỽ'
    LatinCapitalLetterMiddleDashWelshV,
    /// \u{1efd}: 'ỽ'
    LatinSmallLetterMiddleDashWelshV,
    /// \u{1efe}: 'Ỿ'
    LatinCapitalLetterYWithLoop,
}

impl Into<char> for LatinExtendedAdditional {
    fn into(self) -> char {
        match self {
            LatinExtendedAdditional::LatinCapitalLetterAWithRingBelow => 'Ḁ',
            LatinExtendedAdditional::LatinSmallLetterAWithRingBelow => 'ḁ',
            LatinExtendedAdditional::LatinCapitalLetterBWithDotAbove => 'Ḃ',
            LatinExtendedAdditional::LatinSmallLetterBWithDotAbove => 'ḃ',
            LatinExtendedAdditional::LatinCapitalLetterBWithDotBelow => 'Ḅ',
            LatinExtendedAdditional::LatinSmallLetterBWithDotBelow => 'ḅ',
            LatinExtendedAdditional::LatinCapitalLetterBWithLineBelow => 'Ḇ',
            LatinExtendedAdditional::LatinSmallLetterBWithLineBelow => 'ḇ',
            LatinExtendedAdditional::LatinCapitalLetterCWithCedillaAndAcute => 'Ḉ',
            LatinExtendedAdditional::LatinSmallLetterCWithCedillaAndAcute => 'ḉ',
            LatinExtendedAdditional::LatinCapitalLetterDWithDotAbove => 'Ḋ',
            LatinExtendedAdditional::LatinSmallLetterDWithDotAbove => 'ḋ',
            LatinExtendedAdditional::LatinCapitalLetterDWithDotBelow => 'Ḍ',
            LatinExtendedAdditional::LatinSmallLetterDWithDotBelow => 'ḍ',
            LatinExtendedAdditional::LatinCapitalLetterDWithLineBelow => 'Ḏ',
            LatinExtendedAdditional::LatinSmallLetterDWithLineBelow => 'ḏ',
            LatinExtendedAdditional::LatinCapitalLetterDWithCedilla => 'Ḑ',
            LatinExtendedAdditional::LatinSmallLetterDWithCedilla => 'ḑ',
            LatinExtendedAdditional::LatinCapitalLetterDWithCircumflexBelow => 'Ḓ',
            LatinExtendedAdditional::LatinSmallLetterDWithCircumflexBelow => 'ḓ',
            LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndGrave => 'Ḕ',
            LatinExtendedAdditional::LatinSmallLetterEWithMacronAndGrave => 'ḕ',
            LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndAcute => 'Ḗ',
            LatinExtendedAdditional::LatinSmallLetterEWithMacronAndAcute => 'ḗ',
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexBelow => 'Ḙ',
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexBelow => 'ḙ',
            LatinExtendedAdditional::LatinCapitalLetterEWithTildeBelow => 'Ḛ',
            LatinExtendedAdditional::LatinSmallLetterEWithTildeBelow => 'ḛ',
            LatinExtendedAdditional::LatinCapitalLetterEWithCedillaAndBreve => 'Ḝ',
            LatinExtendedAdditional::LatinSmallLetterEWithCedillaAndBreve => 'ḝ',
            LatinExtendedAdditional::LatinCapitalLetterFWithDotAbove => 'Ḟ',
            LatinExtendedAdditional::LatinSmallLetterFWithDotAbove => 'ḟ',
            LatinExtendedAdditional::LatinCapitalLetterGWithMacron => 'Ḡ',
            LatinExtendedAdditional::LatinSmallLetterGWithMacron => 'ḡ',
            LatinExtendedAdditional::LatinCapitalLetterHWithDotAbove => 'Ḣ',
            LatinExtendedAdditional::LatinSmallLetterHWithDotAbove => 'ḣ',
            LatinExtendedAdditional::LatinCapitalLetterHWithDotBelow => 'Ḥ',
            LatinExtendedAdditional::LatinSmallLetterHWithDotBelow => 'ḥ',
            LatinExtendedAdditional::LatinCapitalLetterHWithDiaeresis => 'Ḧ',
            LatinExtendedAdditional::LatinSmallLetterHWithDiaeresis => 'ḧ',
            LatinExtendedAdditional::LatinCapitalLetterHWithCedilla => 'Ḩ',
            LatinExtendedAdditional::LatinSmallLetterHWithCedilla => 'ḩ',
            LatinExtendedAdditional::LatinCapitalLetterHWithBreveBelow => 'Ḫ',
            LatinExtendedAdditional::LatinSmallLetterHWithBreveBelow => 'ḫ',
            LatinExtendedAdditional::LatinCapitalLetterIWithTildeBelow => 'Ḭ',
            LatinExtendedAdditional::LatinSmallLetterIWithTildeBelow => 'ḭ',
            LatinExtendedAdditional::LatinCapitalLetterIWithDiaeresisAndAcute => 'Ḯ',
            LatinExtendedAdditional::LatinSmallLetterIWithDiaeresisAndAcute => 'ḯ',
            LatinExtendedAdditional::LatinCapitalLetterKWithAcute => 'Ḱ',
            LatinExtendedAdditional::LatinSmallLetterKWithAcute => 'ḱ',
            LatinExtendedAdditional::LatinCapitalLetterKWithDotBelow => 'Ḳ',
            LatinExtendedAdditional::LatinSmallLetterKWithDotBelow => 'ḳ',
            LatinExtendedAdditional::LatinCapitalLetterKWithLineBelow => 'Ḵ',
            LatinExtendedAdditional::LatinSmallLetterKWithLineBelow => 'ḵ',
            LatinExtendedAdditional::LatinCapitalLetterLWithDotBelow => 'Ḷ',
            LatinExtendedAdditional::LatinSmallLetterLWithDotBelow => 'ḷ',
            LatinExtendedAdditional::LatinCapitalLetterLWithDotBelowAndMacron => 'Ḹ',
            LatinExtendedAdditional::LatinSmallLetterLWithDotBelowAndMacron => 'ḹ',
            LatinExtendedAdditional::LatinCapitalLetterLWithLineBelow => 'Ḻ',
            LatinExtendedAdditional::LatinSmallLetterLWithLineBelow => 'ḻ',
            LatinExtendedAdditional::LatinCapitalLetterLWithCircumflexBelow => 'Ḽ',
            LatinExtendedAdditional::LatinSmallLetterLWithCircumflexBelow => 'ḽ',
            LatinExtendedAdditional::LatinCapitalLetterMWithAcute => 'Ḿ',
            LatinExtendedAdditional::LatinSmallLetterMWithAcute => 'ḿ',
            LatinExtendedAdditional::LatinCapitalLetterMWithDotAbove => 'Ṁ',
            LatinExtendedAdditional::LatinSmallLetterMWithDotAbove => 'ṁ',
            LatinExtendedAdditional::LatinCapitalLetterMWithDotBelow => 'Ṃ',
            LatinExtendedAdditional::LatinSmallLetterMWithDotBelow => 'ṃ',
            LatinExtendedAdditional::LatinCapitalLetterNWithDotAbove => 'Ṅ',
            LatinExtendedAdditional::LatinSmallLetterNWithDotAbove => 'ṅ',
            LatinExtendedAdditional::LatinCapitalLetterNWithDotBelow => 'Ṇ',
            LatinExtendedAdditional::LatinSmallLetterNWithDotBelow => 'ṇ',
            LatinExtendedAdditional::LatinCapitalLetterNWithLineBelow => 'Ṉ',
            LatinExtendedAdditional::LatinSmallLetterNWithLineBelow => 'ṉ',
            LatinExtendedAdditional::LatinCapitalLetterNWithCircumflexBelow => 'Ṋ',
            LatinExtendedAdditional::LatinSmallLetterNWithCircumflexBelow => 'ṋ',
            LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndAcute => 'Ṍ',
            LatinExtendedAdditional::LatinSmallLetterOWithTildeAndAcute => 'ṍ',
            LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndDiaeresis => 'Ṏ',
            LatinExtendedAdditional::LatinSmallLetterOWithTildeAndDiaeresis => 'ṏ',
            LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndGrave => 'Ṑ',
            LatinExtendedAdditional::LatinSmallLetterOWithMacronAndGrave => 'ṑ',
            LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndAcute => 'Ṓ',
            LatinExtendedAdditional::LatinSmallLetterOWithMacronAndAcute => 'ṓ',
            LatinExtendedAdditional::LatinCapitalLetterPWithAcute => 'Ṕ',
            LatinExtendedAdditional::LatinSmallLetterPWithAcute => 'ṕ',
            LatinExtendedAdditional::LatinCapitalLetterPWithDotAbove => 'Ṗ',
            LatinExtendedAdditional::LatinSmallLetterPWithDotAbove => 'ṗ',
            LatinExtendedAdditional::LatinCapitalLetterRWithDotAbove => 'Ṙ',
            LatinExtendedAdditional::LatinSmallLetterRWithDotAbove => 'ṙ',
            LatinExtendedAdditional::LatinCapitalLetterRWithDotBelow => 'Ṛ',
            LatinExtendedAdditional::LatinSmallLetterRWithDotBelow => 'ṛ',
            LatinExtendedAdditional::LatinCapitalLetterRWithDotBelowAndMacron => 'Ṝ',
            LatinExtendedAdditional::LatinSmallLetterRWithDotBelowAndMacron => 'ṝ',
            LatinExtendedAdditional::LatinCapitalLetterRWithLineBelow => 'Ṟ',
            LatinExtendedAdditional::LatinSmallLetterRWithLineBelow => 'ṟ',
            LatinExtendedAdditional::LatinCapitalLetterSWithDotAbove => 'Ṡ',
            LatinExtendedAdditional::LatinSmallLetterSWithDotAbove => 'ṡ',
            LatinExtendedAdditional::LatinCapitalLetterSWithDotBelow => 'Ṣ',
            LatinExtendedAdditional::LatinSmallLetterSWithDotBelow => 'ṣ',
            LatinExtendedAdditional::LatinCapitalLetterSWithAcuteAndDotAbove => 'Ṥ',
            LatinExtendedAdditional::LatinSmallLetterSWithAcuteAndDotAbove => 'ṥ',
            LatinExtendedAdditional::LatinCapitalLetterSWithCaronAndDotAbove => 'Ṧ',
            LatinExtendedAdditional::LatinSmallLetterSWithCaronAndDotAbove => 'ṧ',
            LatinExtendedAdditional::LatinCapitalLetterSWithDotBelowAndDotAbove => 'Ṩ',
            LatinExtendedAdditional::LatinSmallLetterSWithDotBelowAndDotAbove => 'ṩ',
            LatinExtendedAdditional::LatinCapitalLetterTWithDotAbove => 'Ṫ',
            LatinExtendedAdditional::LatinSmallLetterTWithDotAbove => 'ṫ',
            LatinExtendedAdditional::LatinCapitalLetterTWithDotBelow => 'Ṭ',
            LatinExtendedAdditional::LatinSmallLetterTWithDotBelow => 'ṭ',
            LatinExtendedAdditional::LatinCapitalLetterTWithLineBelow => 'Ṯ',
            LatinExtendedAdditional::LatinSmallLetterTWithLineBelow => 'ṯ',
            LatinExtendedAdditional::LatinCapitalLetterTWithCircumflexBelow => 'Ṱ',
            LatinExtendedAdditional::LatinSmallLetterTWithCircumflexBelow => 'ṱ',
            LatinExtendedAdditional::LatinCapitalLetterUWithDiaeresisBelow => 'Ṳ',
            LatinExtendedAdditional::LatinSmallLetterUWithDiaeresisBelow => 'ṳ',
            LatinExtendedAdditional::LatinCapitalLetterUWithTildeBelow => 'Ṵ',
            LatinExtendedAdditional::LatinSmallLetterUWithTildeBelow => 'ṵ',
            LatinExtendedAdditional::LatinCapitalLetterUWithCircumflexBelow => 'Ṷ',
            LatinExtendedAdditional::LatinSmallLetterUWithCircumflexBelow => 'ṷ',
            LatinExtendedAdditional::LatinCapitalLetterUWithTildeAndAcute => 'Ṹ',
            LatinExtendedAdditional::LatinSmallLetterUWithTildeAndAcute => 'ṹ',
            LatinExtendedAdditional::LatinCapitalLetterUWithMacronAndDiaeresis => 'Ṻ',
            LatinExtendedAdditional::LatinSmallLetterUWithMacronAndDiaeresis => 'ṻ',
            LatinExtendedAdditional::LatinCapitalLetterVWithTilde => 'Ṽ',
            LatinExtendedAdditional::LatinSmallLetterVWithTilde => 'ṽ',
            LatinExtendedAdditional::LatinCapitalLetterVWithDotBelow => 'Ṿ',
            LatinExtendedAdditional::LatinSmallLetterVWithDotBelow => 'ṿ',
            LatinExtendedAdditional::LatinCapitalLetterWWithGrave => 'Ẁ',
            LatinExtendedAdditional::LatinSmallLetterWWithGrave => 'ẁ',
            LatinExtendedAdditional::LatinCapitalLetterWWithAcute => 'Ẃ',
            LatinExtendedAdditional::LatinSmallLetterWWithAcute => 'ẃ',
            LatinExtendedAdditional::LatinCapitalLetterWWithDiaeresis => 'Ẅ',
            LatinExtendedAdditional::LatinSmallLetterWWithDiaeresis => 'ẅ',
            LatinExtendedAdditional::LatinCapitalLetterWWithDotAbove => 'Ẇ',
            LatinExtendedAdditional::LatinSmallLetterWWithDotAbove => 'ẇ',
            LatinExtendedAdditional::LatinCapitalLetterWWithDotBelow => 'Ẉ',
            LatinExtendedAdditional::LatinSmallLetterWWithDotBelow => 'ẉ',
            LatinExtendedAdditional::LatinCapitalLetterXWithDotAbove => 'Ẋ',
            LatinExtendedAdditional::LatinSmallLetterXWithDotAbove => 'ẋ',
            LatinExtendedAdditional::LatinCapitalLetterXWithDiaeresis => 'Ẍ',
            LatinExtendedAdditional::LatinSmallLetterXWithDiaeresis => 'ẍ',
            LatinExtendedAdditional::LatinCapitalLetterYWithDotAbove => 'Ẏ',
            LatinExtendedAdditional::LatinSmallLetterYWithDotAbove => 'ẏ',
            LatinExtendedAdditional::LatinCapitalLetterZWithCircumflex => 'Ẑ',
            LatinExtendedAdditional::LatinSmallLetterZWithCircumflex => 'ẑ',
            LatinExtendedAdditional::LatinCapitalLetterZWithDotBelow => 'Ẓ',
            LatinExtendedAdditional::LatinSmallLetterZWithDotBelow => 'ẓ',
            LatinExtendedAdditional::LatinCapitalLetterZWithLineBelow => 'Ẕ',
            LatinExtendedAdditional::LatinSmallLetterZWithLineBelow => 'ẕ',
            LatinExtendedAdditional::LatinSmallLetterHWithLineBelow => 'ẖ',
            LatinExtendedAdditional::LatinSmallLetterTWithDiaeresis => 'ẗ',
            LatinExtendedAdditional::LatinSmallLetterWWithRingAbove => 'ẘ',
            LatinExtendedAdditional::LatinSmallLetterYWithRingAbove => 'ẙ',
            LatinExtendedAdditional::LatinSmallLetterAWithRightHalfRing => 'ẚ',
            LatinExtendedAdditional::LatinSmallLetterLongSWithDotAbove => 'ẛ',
            LatinExtendedAdditional::LatinSmallLetterLongSWithDiagonalStroke => 'ẜ',
            LatinExtendedAdditional::LatinSmallLetterLongSWithHighStroke => 'ẝ',
            LatinExtendedAdditional::LatinCapitalLetterSharpS => 'ẞ',
            LatinExtendedAdditional::LatinSmallLetterDelta => 'ẟ',
            LatinExtendedAdditional::LatinCapitalLetterAWithDotBelow => 'Ạ',
            LatinExtendedAdditional::LatinSmallLetterAWithDotBelow => 'ạ',
            LatinExtendedAdditional::LatinCapitalLetterAWithHookAbove => 'Ả',
            LatinExtendedAdditional::LatinSmallLetterAWithHookAbove => 'ả',
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndAcute => 'Ấ',
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndAcute => 'ấ',
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndGrave => 'Ầ',
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndGrave => 'ầ',
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndHookAbove => 'Ẩ',
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndHookAbove => 'ẩ',
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndTilde => 'Ẫ',
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndTilde => 'ẫ',
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndDotBelow => 'Ậ',
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndDotBelow => 'ậ',
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndAcute => 'Ắ',
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndAcute => 'ắ',
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndGrave => 'Ằ',
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndGrave => 'ằ',
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndHookAbove => 'Ẳ',
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndHookAbove => 'ẳ',
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndTilde => 'Ẵ',
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndTilde => 'ẵ',
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndDotBelow => 'Ặ',
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndDotBelow => 'ặ',
            LatinExtendedAdditional::LatinCapitalLetterEWithDotBelow => 'Ẹ',
            LatinExtendedAdditional::LatinSmallLetterEWithDotBelow => 'ẹ',
            LatinExtendedAdditional::LatinCapitalLetterEWithHookAbove => 'Ẻ',
            LatinExtendedAdditional::LatinSmallLetterEWithHookAbove => 'ẻ',
            LatinExtendedAdditional::LatinCapitalLetterEWithTilde => 'Ẽ',
            LatinExtendedAdditional::LatinSmallLetterEWithTilde => 'ẽ',
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndAcute => 'Ế',
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndAcute => 'ế',
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndGrave => 'Ề',
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndGrave => 'ề',
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndHookAbove => 'Ể',
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndHookAbove => 'ể',
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndTilde => 'Ễ',
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndTilde => 'ễ',
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndDotBelow => 'Ệ',
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndDotBelow => 'ệ',
            LatinExtendedAdditional::LatinCapitalLetterIWithHookAbove => 'Ỉ',
            LatinExtendedAdditional::LatinSmallLetterIWithHookAbove => 'ỉ',
            LatinExtendedAdditional::LatinCapitalLetterIWithDotBelow => 'Ị',
            LatinExtendedAdditional::LatinSmallLetterIWithDotBelow => 'ị',
            LatinExtendedAdditional::LatinCapitalLetterOWithDotBelow => 'Ọ',
            LatinExtendedAdditional::LatinSmallLetterOWithDotBelow => 'ọ',
            LatinExtendedAdditional::LatinCapitalLetterOWithHookAbove => 'Ỏ',
            LatinExtendedAdditional::LatinSmallLetterOWithHookAbove => 'ỏ',
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndAcute => 'Ố',
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndAcute => 'ố',
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndGrave => 'Ồ',
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndGrave => 'ồ',
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndHookAbove => 'Ổ',
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndHookAbove => 'ổ',
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndTilde => 'Ỗ',
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndTilde => 'ỗ',
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndDotBelow => 'Ộ',
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndDotBelow => 'ộ',
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndAcute => 'Ớ',
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndAcute => 'ớ',
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndGrave => 'Ờ',
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndGrave => 'ờ',
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndHookAbove => 'Ở',
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndHookAbove => 'ở',
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndTilde => 'Ỡ',
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndTilde => 'ỡ',
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndDotBelow => 'Ợ',
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndDotBelow => 'ợ',
            LatinExtendedAdditional::LatinCapitalLetterUWithDotBelow => 'Ụ',
            LatinExtendedAdditional::LatinSmallLetterUWithDotBelow => 'ụ',
            LatinExtendedAdditional::LatinCapitalLetterUWithHookAbove => 'Ủ',
            LatinExtendedAdditional::LatinSmallLetterUWithHookAbove => 'ủ',
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndAcute => 'Ứ',
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndAcute => 'ứ',
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndGrave => 'Ừ',
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndGrave => 'ừ',
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndHookAbove => 'Ử',
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndHookAbove => 'ử',
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndTilde => 'Ữ',
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndTilde => 'ữ',
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndDotBelow => 'Ự',
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndDotBelow => 'ự',
            LatinExtendedAdditional::LatinCapitalLetterYWithGrave => 'Ỳ',
            LatinExtendedAdditional::LatinSmallLetterYWithGrave => 'ỳ',
            LatinExtendedAdditional::LatinCapitalLetterYWithDotBelow => 'Ỵ',
            LatinExtendedAdditional::LatinSmallLetterYWithDotBelow => 'ỵ',
            LatinExtendedAdditional::LatinCapitalLetterYWithHookAbove => 'Ỷ',
            LatinExtendedAdditional::LatinSmallLetterYWithHookAbove => 'ỷ',
            LatinExtendedAdditional::LatinCapitalLetterYWithTilde => 'Ỹ',
            LatinExtendedAdditional::LatinSmallLetterYWithTilde => 'ỹ',
            LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshLl => 'Ỻ',
            LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshLl => 'ỻ',
            LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshV => 'Ỽ',
            LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshV => 'ỽ',
            LatinExtendedAdditional::LatinCapitalLetterYWithLoop => 'Ỿ',
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedAdditional {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ḁ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithRingBelow),
            'ḁ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithRingBelow),
            'Ḃ' => Ok(LatinExtendedAdditional::LatinCapitalLetterBWithDotAbove),
            'ḃ' => Ok(LatinExtendedAdditional::LatinSmallLetterBWithDotAbove),
            'Ḅ' => Ok(LatinExtendedAdditional::LatinCapitalLetterBWithDotBelow),
            'ḅ' => Ok(LatinExtendedAdditional::LatinSmallLetterBWithDotBelow),
            'Ḇ' => Ok(LatinExtendedAdditional::LatinCapitalLetterBWithLineBelow),
            'ḇ' => Ok(LatinExtendedAdditional::LatinSmallLetterBWithLineBelow),
            'Ḉ' => Ok(LatinExtendedAdditional::LatinCapitalLetterCWithCedillaAndAcute),
            'ḉ' => Ok(LatinExtendedAdditional::LatinSmallLetterCWithCedillaAndAcute),
            'Ḋ' => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithDotAbove),
            'ḋ' => Ok(LatinExtendedAdditional::LatinSmallLetterDWithDotAbove),
            'Ḍ' => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithDotBelow),
            'ḍ' => Ok(LatinExtendedAdditional::LatinSmallLetterDWithDotBelow),
            'Ḏ' => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithLineBelow),
            'ḏ' => Ok(LatinExtendedAdditional::LatinSmallLetterDWithLineBelow),
            'Ḑ' => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithCedilla),
            'ḑ' => Ok(LatinExtendedAdditional::LatinSmallLetterDWithCedilla),
            'Ḓ' => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithCircumflexBelow),
            'ḓ' => Ok(LatinExtendedAdditional::LatinSmallLetterDWithCircumflexBelow),
            'Ḕ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndGrave),
            'ḕ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithMacronAndGrave),
            'Ḗ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndAcute),
            'ḗ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithMacronAndAcute),
            'Ḙ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexBelow),
            'ḙ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexBelow),
            'Ḛ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithTildeBelow),
            'ḛ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithTildeBelow),
            'Ḝ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCedillaAndBreve),
            'ḝ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCedillaAndBreve),
            'Ḟ' => Ok(LatinExtendedAdditional::LatinCapitalLetterFWithDotAbove),
            'ḟ' => Ok(LatinExtendedAdditional::LatinSmallLetterFWithDotAbove),
            'Ḡ' => Ok(LatinExtendedAdditional::LatinCapitalLetterGWithMacron),
            'ḡ' => Ok(LatinExtendedAdditional::LatinSmallLetterGWithMacron),
            'Ḣ' => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithDotAbove),
            'ḣ' => Ok(LatinExtendedAdditional::LatinSmallLetterHWithDotAbove),
            'Ḥ' => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithDotBelow),
            'ḥ' => Ok(LatinExtendedAdditional::LatinSmallLetterHWithDotBelow),
            'Ḧ' => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithDiaeresis),
            'ḧ' => Ok(LatinExtendedAdditional::LatinSmallLetterHWithDiaeresis),
            'Ḩ' => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithCedilla),
            'ḩ' => Ok(LatinExtendedAdditional::LatinSmallLetterHWithCedilla),
            'Ḫ' => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithBreveBelow),
            'ḫ' => Ok(LatinExtendedAdditional::LatinSmallLetterHWithBreveBelow),
            'Ḭ' => Ok(LatinExtendedAdditional::LatinCapitalLetterIWithTildeBelow),
            'ḭ' => Ok(LatinExtendedAdditional::LatinSmallLetterIWithTildeBelow),
            'Ḯ' => Ok(LatinExtendedAdditional::LatinCapitalLetterIWithDiaeresisAndAcute),
            'ḯ' => Ok(LatinExtendedAdditional::LatinSmallLetterIWithDiaeresisAndAcute),
            'Ḱ' => Ok(LatinExtendedAdditional::LatinCapitalLetterKWithAcute),
            'ḱ' => Ok(LatinExtendedAdditional::LatinSmallLetterKWithAcute),
            'Ḳ' => Ok(LatinExtendedAdditional::LatinCapitalLetterKWithDotBelow),
            'ḳ' => Ok(LatinExtendedAdditional::LatinSmallLetterKWithDotBelow),
            'Ḵ' => Ok(LatinExtendedAdditional::LatinCapitalLetterKWithLineBelow),
            'ḵ' => Ok(LatinExtendedAdditional::LatinSmallLetterKWithLineBelow),
            'Ḷ' => Ok(LatinExtendedAdditional::LatinCapitalLetterLWithDotBelow),
            'ḷ' => Ok(LatinExtendedAdditional::LatinSmallLetterLWithDotBelow),
            'Ḹ' => Ok(LatinExtendedAdditional::LatinCapitalLetterLWithDotBelowAndMacron),
            'ḹ' => Ok(LatinExtendedAdditional::LatinSmallLetterLWithDotBelowAndMacron),
            'Ḻ' => Ok(LatinExtendedAdditional::LatinCapitalLetterLWithLineBelow),
            'ḻ' => Ok(LatinExtendedAdditional::LatinSmallLetterLWithLineBelow),
            'Ḽ' => Ok(LatinExtendedAdditional::LatinCapitalLetterLWithCircumflexBelow),
            'ḽ' => Ok(LatinExtendedAdditional::LatinSmallLetterLWithCircumflexBelow),
            'Ḿ' => Ok(LatinExtendedAdditional::LatinCapitalLetterMWithAcute),
            'ḿ' => Ok(LatinExtendedAdditional::LatinSmallLetterMWithAcute),
            'Ṁ' => Ok(LatinExtendedAdditional::LatinCapitalLetterMWithDotAbove),
            'ṁ' => Ok(LatinExtendedAdditional::LatinSmallLetterMWithDotAbove),
            'Ṃ' => Ok(LatinExtendedAdditional::LatinCapitalLetterMWithDotBelow),
            'ṃ' => Ok(LatinExtendedAdditional::LatinSmallLetterMWithDotBelow),
            'Ṅ' => Ok(LatinExtendedAdditional::LatinCapitalLetterNWithDotAbove),
            'ṅ' => Ok(LatinExtendedAdditional::LatinSmallLetterNWithDotAbove),
            'Ṇ' => Ok(LatinExtendedAdditional::LatinCapitalLetterNWithDotBelow),
            'ṇ' => Ok(LatinExtendedAdditional::LatinSmallLetterNWithDotBelow),
            'Ṉ' => Ok(LatinExtendedAdditional::LatinCapitalLetterNWithLineBelow),
            'ṉ' => Ok(LatinExtendedAdditional::LatinSmallLetterNWithLineBelow),
            'Ṋ' => Ok(LatinExtendedAdditional::LatinCapitalLetterNWithCircumflexBelow),
            'ṋ' => Ok(LatinExtendedAdditional::LatinSmallLetterNWithCircumflexBelow),
            'Ṍ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndAcute),
            'ṍ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithTildeAndAcute),
            'Ṏ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndDiaeresis),
            'ṏ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithTildeAndDiaeresis),
            'Ṑ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndGrave),
            'ṑ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithMacronAndGrave),
            'Ṓ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndAcute),
            'ṓ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithMacronAndAcute),
            'Ṕ' => Ok(LatinExtendedAdditional::LatinCapitalLetterPWithAcute),
            'ṕ' => Ok(LatinExtendedAdditional::LatinSmallLetterPWithAcute),
            'Ṗ' => Ok(LatinExtendedAdditional::LatinCapitalLetterPWithDotAbove),
            'ṗ' => Ok(LatinExtendedAdditional::LatinSmallLetterPWithDotAbove),
            'Ṙ' => Ok(LatinExtendedAdditional::LatinCapitalLetterRWithDotAbove),
            'ṙ' => Ok(LatinExtendedAdditional::LatinSmallLetterRWithDotAbove),
            'Ṛ' => Ok(LatinExtendedAdditional::LatinCapitalLetterRWithDotBelow),
            'ṛ' => Ok(LatinExtendedAdditional::LatinSmallLetterRWithDotBelow),
            'Ṝ' => Ok(LatinExtendedAdditional::LatinCapitalLetterRWithDotBelowAndMacron),
            'ṝ' => Ok(LatinExtendedAdditional::LatinSmallLetterRWithDotBelowAndMacron),
            'Ṟ' => Ok(LatinExtendedAdditional::LatinCapitalLetterRWithLineBelow),
            'ṟ' => Ok(LatinExtendedAdditional::LatinSmallLetterRWithLineBelow),
            'Ṡ' => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithDotAbove),
            'ṡ' => Ok(LatinExtendedAdditional::LatinSmallLetterSWithDotAbove),
            'Ṣ' => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithDotBelow),
            'ṣ' => Ok(LatinExtendedAdditional::LatinSmallLetterSWithDotBelow),
            'Ṥ' => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithAcuteAndDotAbove),
            'ṥ' => Ok(LatinExtendedAdditional::LatinSmallLetterSWithAcuteAndDotAbove),
            'Ṧ' => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithCaronAndDotAbove),
            'ṧ' => Ok(LatinExtendedAdditional::LatinSmallLetterSWithCaronAndDotAbove),
            'Ṩ' => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithDotBelowAndDotAbove),
            'ṩ' => Ok(LatinExtendedAdditional::LatinSmallLetterSWithDotBelowAndDotAbove),
            'Ṫ' => Ok(LatinExtendedAdditional::LatinCapitalLetterTWithDotAbove),
            'ṫ' => Ok(LatinExtendedAdditional::LatinSmallLetterTWithDotAbove),
            'Ṭ' => Ok(LatinExtendedAdditional::LatinCapitalLetterTWithDotBelow),
            'ṭ' => Ok(LatinExtendedAdditional::LatinSmallLetterTWithDotBelow),
            'Ṯ' => Ok(LatinExtendedAdditional::LatinCapitalLetterTWithLineBelow),
            'ṯ' => Ok(LatinExtendedAdditional::LatinSmallLetterTWithLineBelow),
            'Ṱ' => Ok(LatinExtendedAdditional::LatinCapitalLetterTWithCircumflexBelow),
            'ṱ' => Ok(LatinExtendedAdditional::LatinSmallLetterTWithCircumflexBelow),
            'Ṳ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithDiaeresisBelow),
            'ṳ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithDiaeresisBelow),
            'Ṵ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithTildeBelow),
            'ṵ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithTildeBelow),
            'Ṷ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithCircumflexBelow),
            'ṷ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithCircumflexBelow),
            'Ṹ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithTildeAndAcute),
            'ṹ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithTildeAndAcute),
            'Ṻ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithMacronAndDiaeresis),
            'ṻ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithMacronAndDiaeresis),
            'Ṽ' => Ok(LatinExtendedAdditional::LatinCapitalLetterVWithTilde),
            'ṽ' => Ok(LatinExtendedAdditional::LatinSmallLetterVWithTilde),
            'Ṿ' => Ok(LatinExtendedAdditional::LatinCapitalLetterVWithDotBelow),
            'ṿ' => Ok(LatinExtendedAdditional::LatinSmallLetterVWithDotBelow),
            'Ẁ' => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithGrave),
            'ẁ' => Ok(LatinExtendedAdditional::LatinSmallLetterWWithGrave),
            'Ẃ' => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithAcute),
            'ẃ' => Ok(LatinExtendedAdditional::LatinSmallLetterWWithAcute),
            'Ẅ' => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithDiaeresis),
            'ẅ' => Ok(LatinExtendedAdditional::LatinSmallLetterWWithDiaeresis),
            'Ẇ' => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithDotAbove),
            'ẇ' => Ok(LatinExtendedAdditional::LatinSmallLetterWWithDotAbove),
            'Ẉ' => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithDotBelow),
            'ẉ' => Ok(LatinExtendedAdditional::LatinSmallLetterWWithDotBelow),
            'Ẋ' => Ok(LatinExtendedAdditional::LatinCapitalLetterXWithDotAbove),
            'ẋ' => Ok(LatinExtendedAdditional::LatinSmallLetterXWithDotAbove),
            'Ẍ' => Ok(LatinExtendedAdditional::LatinCapitalLetterXWithDiaeresis),
            'ẍ' => Ok(LatinExtendedAdditional::LatinSmallLetterXWithDiaeresis),
            'Ẏ' => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithDotAbove),
            'ẏ' => Ok(LatinExtendedAdditional::LatinSmallLetterYWithDotAbove),
            'Ẑ' => Ok(LatinExtendedAdditional::LatinCapitalLetterZWithCircumflex),
            'ẑ' => Ok(LatinExtendedAdditional::LatinSmallLetterZWithCircumflex),
            'Ẓ' => Ok(LatinExtendedAdditional::LatinCapitalLetterZWithDotBelow),
            'ẓ' => Ok(LatinExtendedAdditional::LatinSmallLetterZWithDotBelow),
            'Ẕ' => Ok(LatinExtendedAdditional::LatinCapitalLetterZWithLineBelow),
            'ẕ' => Ok(LatinExtendedAdditional::LatinSmallLetterZWithLineBelow),
            'ẖ' => Ok(LatinExtendedAdditional::LatinSmallLetterHWithLineBelow),
            'ẗ' => Ok(LatinExtendedAdditional::LatinSmallLetterTWithDiaeresis),
            'ẘ' => Ok(LatinExtendedAdditional::LatinSmallLetterWWithRingAbove),
            'ẙ' => Ok(LatinExtendedAdditional::LatinSmallLetterYWithRingAbove),
            'ẚ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithRightHalfRing),
            'ẛ' => Ok(LatinExtendedAdditional::LatinSmallLetterLongSWithDotAbove),
            'ẜ' => Ok(LatinExtendedAdditional::LatinSmallLetterLongSWithDiagonalStroke),
            'ẝ' => Ok(LatinExtendedAdditional::LatinSmallLetterLongSWithHighStroke),
            'ẞ' => Ok(LatinExtendedAdditional::LatinCapitalLetterSharpS),
            'ẟ' => Ok(LatinExtendedAdditional::LatinSmallLetterDelta),
            'Ạ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithDotBelow),
            'ạ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithDotBelow),
            'Ả' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithHookAbove),
            'ả' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithHookAbove),
            'Ấ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndAcute),
            'ấ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndAcute),
            'Ầ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndGrave),
            'ầ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndGrave),
            'Ẩ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndHookAbove),
            'ẩ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndHookAbove),
            'Ẫ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndTilde),
            'ẫ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndTilde),
            'Ậ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndDotBelow),
            'ậ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndDotBelow),
            'Ắ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndAcute),
            'ắ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndAcute),
            'Ằ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndGrave),
            'ằ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndGrave),
            'Ẳ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndHookAbove),
            'ẳ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndHookAbove),
            'Ẵ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndTilde),
            'ẵ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndTilde),
            'Ặ' => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndDotBelow),
            'ặ' => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndDotBelow),
            'Ẹ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithDotBelow),
            'ẹ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithDotBelow),
            'Ẻ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithHookAbove),
            'ẻ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithHookAbove),
            'Ẽ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithTilde),
            'ẽ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithTilde),
            'Ế' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndAcute),
            'ế' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndAcute),
            'Ề' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndGrave),
            'ề' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndGrave),
            'Ể' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndHookAbove),
            'ể' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndHookAbove),
            'Ễ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndTilde),
            'ễ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndTilde),
            'Ệ' => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndDotBelow),
            'ệ' => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndDotBelow),
            'Ỉ' => Ok(LatinExtendedAdditional::LatinCapitalLetterIWithHookAbove),
            'ỉ' => Ok(LatinExtendedAdditional::LatinSmallLetterIWithHookAbove),
            'Ị' => Ok(LatinExtendedAdditional::LatinCapitalLetterIWithDotBelow),
            'ị' => Ok(LatinExtendedAdditional::LatinSmallLetterIWithDotBelow),
            'Ọ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithDotBelow),
            'ọ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithDotBelow),
            'Ỏ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHookAbove),
            'ỏ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHookAbove),
            'Ố' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndAcute),
            'ố' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndAcute),
            'Ồ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndGrave),
            'ồ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndGrave),
            'Ổ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndHookAbove),
            'ổ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndHookAbove),
            'Ỗ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndTilde),
            'ỗ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndTilde),
            'Ộ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndDotBelow),
            'ộ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndDotBelow),
            'Ớ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndAcute),
            'ớ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndAcute),
            'Ờ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndGrave),
            'ờ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndGrave),
            'Ở' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndHookAbove),
            'ở' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndHookAbove),
            'Ỡ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndTilde),
            'ỡ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndTilde),
            'Ợ' => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndDotBelow),
            'ợ' => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndDotBelow),
            'Ụ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithDotBelow),
            'ụ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithDotBelow),
            'Ủ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHookAbove),
            'ủ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHookAbove),
            'Ứ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndAcute),
            'ứ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndAcute),
            'Ừ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndGrave),
            'ừ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndGrave),
            'Ử' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndHookAbove),
            'ử' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndHookAbove),
            'Ữ' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndTilde),
            'ữ' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndTilde),
            'Ự' => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndDotBelow),
            'ự' => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndDotBelow),
            'Ỳ' => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithGrave),
            'ỳ' => Ok(LatinExtendedAdditional::LatinSmallLetterYWithGrave),
            'Ỵ' => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithDotBelow),
            'ỵ' => Ok(LatinExtendedAdditional::LatinSmallLetterYWithDotBelow),
            'Ỷ' => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithHookAbove),
            'ỷ' => Ok(LatinExtendedAdditional::LatinSmallLetterYWithHookAbove),
            'Ỹ' => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithTilde),
            'ỹ' => Ok(LatinExtendedAdditional::LatinSmallLetterYWithTilde),
            'Ỻ' => Ok(LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshLl),
            'ỻ' => Ok(LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshLl),
            'Ỽ' => Ok(LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshV),
            'ỽ' => Ok(LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshV),
            'Ỿ' => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithLoop),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LatinExtendedAdditional {
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

impl std::convert::TryFrom<u32> for LatinExtendedAdditional {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LatinExtendedAdditional {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LatinExtendedAdditional {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LatinExtendedAdditional::LatinCapitalLetterAWithRingBelow
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("LatinExtendedAdditional{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
