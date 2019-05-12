
/// An enum to represent all characters in the PhoneticExtensions block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PhoneticExtensions {
    /// \u{1d00}: 'ᴀ'
    LatinLetterSmallCapitalA,
    /// \u{1d01}: 'ᴁ'
    LatinLetterSmallCapitalAe,
    /// \u{1d02}: 'ᴂ'
    LatinSmallLetterTurnedAe,
    /// \u{1d03}: 'ᴃ'
    LatinLetterSmallCapitalBarredB,
    /// \u{1d04}: 'ᴄ'
    LatinLetterSmallCapitalC,
    /// \u{1d05}: 'ᴅ'
    LatinLetterSmallCapitalD,
    /// \u{1d06}: 'ᴆ'
    LatinLetterSmallCapitalEth,
    /// \u{1d07}: 'ᴇ'
    LatinLetterSmallCapitalE,
    /// \u{1d08}: 'ᴈ'
    LatinSmallLetterTurnedOpenE,
    /// \u{1d09}: 'ᴉ'
    LatinSmallLetterTurnedI,
    /// \u{1d0a}: 'ᴊ'
    LatinLetterSmallCapitalJ,
    /// \u{1d0b}: 'ᴋ'
    LatinLetterSmallCapitalK,
    /// \u{1d0c}: 'ᴌ'
    LatinLetterSmallCapitalLWithStroke,
    /// \u{1d0d}: 'ᴍ'
    LatinLetterSmallCapitalM,
    /// \u{1d0e}: 'ᴎ'
    LatinLetterSmallCapitalReversedN,
    /// \u{1d0f}: 'ᴏ'
    LatinLetterSmallCapitalO,
    /// \u{1d10}: 'ᴐ'
    LatinLetterSmallCapitalOpenO,
    /// \u{1d11}: 'ᴑ'
    LatinSmallLetterSidewaysO,
    /// \u{1d12}: 'ᴒ'
    LatinSmallLetterSidewaysOpenO,
    /// \u{1d13}: 'ᴓ'
    LatinSmallLetterSidewaysOWithStroke,
    /// \u{1d14}: 'ᴔ'
    LatinSmallLetterTurnedOe,
    /// \u{1d15}: 'ᴕ'
    LatinLetterSmallCapitalOu,
    /// \u{1d16}: 'ᴖ'
    LatinSmallLetterTopHalfO,
    /// \u{1d17}: 'ᴗ'
    LatinSmallLetterBottomHalfO,
    /// \u{1d18}: 'ᴘ'
    LatinLetterSmallCapitalP,
    /// \u{1d19}: 'ᴙ'
    LatinLetterSmallCapitalReversedR,
    /// \u{1d1a}: 'ᴚ'
    LatinLetterSmallCapitalTurnedR,
    /// \u{1d1b}: 'ᴛ'
    LatinLetterSmallCapitalT,
    /// \u{1d1c}: 'ᴜ'
    LatinLetterSmallCapitalU,
    /// \u{1d1d}: 'ᴝ'
    LatinSmallLetterSidewaysU,
    /// \u{1d1e}: 'ᴞ'
    LatinSmallLetterSidewaysDiaeresizedU,
    /// \u{1d1f}: 'ᴟ'
    LatinSmallLetterSidewaysTurnedM,
    /// \u{1d20}: 'ᴠ'
    LatinLetterSmallCapitalV,
    /// \u{1d21}: 'ᴡ'
    LatinLetterSmallCapitalW,
    /// \u{1d22}: 'ᴢ'
    LatinLetterSmallCapitalZ,
    /// \u{1d23}: 'ᴣ'
    LatinLetterSmallCapitalEzh,
    /// \u{1d24}: 'ᴤ'
    LatinLetterVoicedLaryngealSpirant,
    /// \u{1d25}: 'ᴥ'
    LatinLetterAin,
    /// \u{1d26}: 'ᴦ'
    GreekLetterSmallCapitalGamma,
    /// \u{1d27}: 'ᴧ'
    GreekLetterSmallCapitalLamda,
    /// \u{1d28}: 'ᴨ'
    GreekLetterSmallCapitalPi,
    /// \u{1d29}: 'ᴩ'
    GreekLetterSmallCapitalRho,
    /// \u{1d2a}: 'ᴪ'
    GreekLetterSmallCapitalPsi,
    /// \u{1d2b}: 'ᴫ'
    CyrillicLetterSmallCapitalEl,
    /// \u{1d2c}: 'ᴬ'
    ModifierLetterCapitalA,
    /// \u{1d2d}: 'ᴭ'
    ModifierLetterCapitalAe,
    /// \u{1d2e}: 'ᴮ'
    ModifierLetterCapitalB,
    /// \u{1d2f}: 'ᴯ'
    ModifierLetterCapitalBarredB,
    /// \u{1d30}: 'ᴰ'
    ModifierLetterCapitalD,
    /// \u{1d31}: 'ᴱ'
    ModifierLetterCapitalE,
    /// \u{1d32}: 'ᴲ'
    ModifierLetterCapitalReversedE,
    /// \u{1d33}: 'ᴳ'
    ModifierLetterCapitalG,
    /// \u{1d34}: 'ᴴ'
    ModifierLetterCapitalH,
    /// \u{1d35}: 'ᴵ'
    ModifierLetterCapitalI,
    /// \u{1d36}: 'ᴶ'
    ModifierLetterCapitalJ,
    /// \u{1d37}: 'ᴷ'
    ModifierLetterCapitalK,
    /// \u{1d38}: 'ᴸ'
    ModifierLetterCapitalL,
    /// \u{1d39}: 'ᴹ'
    ModifierLetterCapitalM,
    /// \u{1d3a}: 'ᴺ'
    ModifierLetterCapitalN,
    /// \u{1d3b}: 'ᴻ'
    ModifierLetterCapitalReversedN,
    /// \u{1d3c}: 'ᴼ'
    ModifierLetterCapitalO,
    /// \u{1d3d}: 'ᴽ'
    ModifierLetterCapitalOu,
    /// \u{1d3e}: 'ᴾ'
    ModifierLetterCapitalP,
    /// \u{1d3f}: 'ᴿ'
    ModifierLetterCapitalR,
    /// \u{1d40}: 'ᵀ'
    ModifierLetterCapitalT,
    /// \u{1d41}: 'ᵁ'
    ModifierLetterCapitalU,
    /// \u{1d42}: 'ᵂ'
    ModifierLetterCapitalW,
    /// \u{1d43}: 'ᵃ'
    ModifierLetterSmallA,
    /// \u{1d44}: 'ᵄ'
    ModifierLetterSmallTurnedA,
    /// \u{1d45}: 'ᵅ'
    ModifierLetterSmallAlpha,
    /// \u{1d46}: 'ᵆ'
    ModifierLetterSmallTurnedAe,
    /// \u{1d47}: 'ᵇ'
    ModifierLetterSmallB,
    /// \u{1d48}: 'ᵈ'
    ModifierLetterSmallD,
    /// \u{1d49}: 'ᵉ'
    ModifierLetterSmallE,
    /// \u{1d4a}: 'ᵊ'
    ModifierLetterSmallSchwa,
    /// \u{1d4b}: 'ᵋ'
    ModifierLetterSmallOpenE,
    /// \u{1d4c}: 'ᵌ'
    ModifierLetterSmallTurnedOpenE,
    /// \u{1d4d}: 'ᵍ'
    ModifierLetterSmallG,
    /// \u{1d4e}: 'ᵎ'
    ModifierLetterSmallTurnedI,
    /// \u{1d4f}: 'ᵏ'
    ModifierLetterSmallK,
    /// \u{1d50}: 'ᵐ'
    ModifierLetterSmallM,
    /// \u{1d51}: 'ᵑ'
    ModifierLetterSmallEng,
    /// \u{1d52}: 'ᵒ'
    ModifierLetterSmallO,
    /// \u{1d53}: 'ᵓ'
    ModifierLetterSmallOpenO,
    /// \u{1d54}: 'ᵔ'
    ModifierLetterSmallTopHalfO,
    /// \u{1d55}: 'ᵕ'
    ModifierLetterSmallBottomHalfO,
    /// \u{1d56}: 'ᵖ'
    ModifierLetterSmallP,
    /// \u{1d57}: 'ᵗ'
    ModifierLetterSmallT,
    /// \u{1d58}: 'ᵘ'
    ModifierLetterSmallU,
    /// \u{1d59}: 'ᵙ'
    ModifierLetterSmallSidewaysU,
    /// \u{1d5a}: 'ᵚ'
    ModifierLetterSmallTurnedM,
    /// \u{1d5b}: 'ᵛ'
    ModifierLetterSmallV,
    /// \u{1d5c}: 'ᵜ'
    ModifierLetterSmallAin,
    /// \u{1d5d}: 'ᵝ'
    ModifierLetterSmallBeta,
    /// \u{1d5e}: 'ᵞ'
    ModifierLetterSmallGreekGamma,
    /// \u{1d5f}: 'ᵟ'
    ModifierLetterSmallDelta,
    /// \u{1d60}: 'ᵠ'
    ModifierLetterSmallGreekPhi,
    /// \u{1d61}: 'ᵡ'
    ModifierLetterSmallChi,
    /// \u{1d62}: 'ᵢ'
    LatinSubscriptSmallLetterI,
    /// \u{1d63}: 'ᵣ'
    LatinSubscriptSmallLetterR,
    /// \u{1d64}: 'ᵤ'
    LatinSubscriptSmallLetterU,
    /// \u{1d65}: 'ᵥ'
    LatinSubscriptSmallLetterV,
    /// \u{1d66}: 'ᵦ'
    GreekSubscriptSmallLetterBeta,
    /// \u{1d67}: 'ᵧ'
    GreekSubscriptSmallLetterGamma,
    /// \u{1d68}: 'ᵨ'
    GreekSubscriptSmallLetterRho,
    /// \u{1d69}: 'ᵩ'
    GreekSubscriptSmallLetterPhi,
    /// \u{1d6a}: 'ᵪ'
    GreekSubscriptSmallLetterChi,
    /// \u{1d6b}: 'ᵫ'
    LatinSmallLetterUe,
    /// \u{1d6c}: 'ᵬ'
    LatinSmallLetterBWithMiddleTilde,
    /// \u{1d6d}: 'ᵭ'
    LatinSmallLetterDWithMiddleTilde,
    /// \u{1d6e}: 'ᵮ'
    LatinSmallLetterFWithMiddleTilde,
    /// \u{1d6f}: 'ᵯ'
    LatinSmallLetterMWithMiddleTilde,
    /// \u{1d70}: 'ᵰ'
    LatinSmallLetterNWithMiddleTilde,
    /// \u{1d71}: 'ᵱ'
    LatinSmallLetterPWithMiddleTilde,
    /// \u{1d72}: 'ᵲ'
    LatinSmallLetterRWithMiddleTilde,
    /// \u{1d73}: 'ᵳ'
    LatinSmallLetterRWithFishhookAndMiddleTilde,
    /// \u{1d74}: 'ᵴ'
    LatinSmallLetterSWithMiddleTilde,
    /// \u{1d75}: 'ᵵ'
    LatinSmallLetterTWithMiddleTilde,
    /// \u{1d76}: 'ᵶ'
    LatinSmallLetterZWithMiddleTilde,
    /// \u{1d77}: 'ᵷ'
    LatinSmallLetterTurnedG,
    /// \u{1d78}: 'ᵸ'
    ModifierLetterCyrillicEn,
    /// \u{1d79}: 'ᵹ'
    LatinSmallLetterInsularG,
    /// \u{1d7a}: 'ᵺ'
    LatinSmallLetterThWithStrikethrough,
    /// \u{1d7b}: 'ᵻ'
    LatinSmallCapitalLetterIWithStroke,
    /// \u{1d7c}: 'ᵼ'
    LatinSmallLetterIotaWithStroke,
    /// \u{1d7d}: 'ᵽ'
    LatinSmallLetterPWithStroke,
    /// \u{1d7e}: 'ᵾ'
    LatinSmallCapitalLetterUWithStroke,
}

impl Into<char> for PhoneticExtensions {
    fn into(self) -> char {
        match self {
            PhoneticExtensions::LatinLetterSmallCapitalA => 'ᴀ',
            PhoneticExtensions::LatinLetterSmallCapitalAe => 'ᴁ',
            PhoneticExtensions::LatinSmallLetterTurnedAe => 'ᴂ',
            PhoneticExtensions::LatinLetterSmallCapitalBarredB => 'ᴃ',
            PhoneticExtensions::LatinLetterSmallCapitalC => 'ᴄ',
            PhoneticExtensions::LatinLetterSmallCapitalD => 'ᴅ',
            PhoneticExtensions::LatinLetterSmallCapitalEth => 'ᴆ',
            PhoneticExtensions::LatinLetterSmallCapitalE => 'ᴇ',
            PhoneticExtensions::LatinSmallLetterTurnedOpenE => 'ᴈ',
            PhoneticExtensions::LatinSmallLetterTurnedI => 'ᴉ',
            PhoneticExtensions::LatinLetterSmallCapitalJ => 'ᴊ',
            PhoneticExtensions::LatinLetterSmallCapitalK => 'ᴋ',
            PhoneticExtensions::LatinLetterSmallCapitalLWithStroke => 'ᴌ',
            PhoneticExtensions::LatinLetterSmallCapitalM => 'ᴍ',
            PhoneticExtensions::LatinLetterSmallCapitalReversedN => 'ᴎ',
            PhoneticExtensions::LatinLetterSmallCapitalO => 'ᴏ',
            PhoneticExtensions::LatinLetterSmallCapitalOpenO => 'ᴐ',
            PhoneticExtensions::LatinSmallLetterSidewaysO => 'ᴑ',
            PhoneticExtensions::LatinSmallLetterSidewaysOpenO => 'ᴒ',
            PhoneticExtensions::LatinSmallLetterSidewaysOWithStroke => 'ᴓ',
            PhoneticExtensions::LatinSmallLetterTurnedOe => 'ᴔ',
            PhoneticExtensions::LatinLetterSmallCapitalOu => 'ᴕ',
            PhoneticExtensions::LatinSmallLetterTopHalfO => 'ᴖ',
            PhoneticExtensions::LatinSmallLetterBottomHalfO => 'ᴗ',
            PhoneticExtensions::LatinLetterSmallCapitalP => 'ᴘ',
            PhoneticExtensions::LatinLetterSmallCapitalReversedR => 'ᴙ',
            PhoneticExtensions::LatinLetterSmallCapitalTurnedR => 'ᴚ',
            PhoneticExtensions::LatinLetterSmallCapitalT => 'ᴛ',
            PhoneticExtensions::LatinLetterSmallCapitalU => 'ᴜ',
            PhoneticExtensions::LatinSmallLetterSidewaysU => 'ᴝ',
            PhoneticExtensions::LatinSmallLetterSidewaysDiaeresizedU => 'ᴞ',
            PhoneticExtensions::LatinSmallLetterSidewaysTurnedM => 'ᴟ',
            PhoneticExtensions::LatinLetterSmallCapitalV => 'ᴠ',
            PhoneticExtensions::LatinLetterSmallCapitalW => 'ᴡ',
            PhoneticExtensions::LatinLetterSmallCapitalZ => 'ᴢ',
            PhoneticExtensions::LatinLetterSmallCapitalEzh => 'ᴣ',
            PhoneticExtensions::LatinLetterVoicedLaryngealSpirant => 'ᴤ',
            PhoneticExtensions::LatinLetterAin => 'ᴥ',
            PhoneticExtensions::GreekLetterSmallCapitalGamma => 'ᴦ',
            PhoneticExtensions::GreekLetterSmallCapitalLamda => 'ᴧ',
            PhoneticExtensions::GreekLetterSmallCapitalPi => 'ᴨ',
            PhoneticExtensions::GreekLetterSmallCapitalRho => 'ᴩ',
            PhoneticExtensions::GreekLetterSmallCapitalPsi => 'ᴪ',
            PhoneticExtensions::CyrillicLetterSmallCapitalEl => 'ᴫ',
            PhoneticExtensions::ModifierLetterCapitalA => 'ᴬ',
            PhoneticExtensions::ModifierLetterCapitalAe => 'ᴭ',
            PhoneticExtensions::ModifierLetterCapitalB => 'ᴮ',
            PhoneticExtensions::ModifierLetterCapitalBarredB => 'ᴯ',
            PhoneticExtensions::ModifierLetterCapitalD => 'ᴰ',
            PhoneticExtensions::ModifierLetterCapitalE => 'ᴱ',
            PhoneticExtensions::ModifierLetterCapitalReversedE => 'ᴲ',
            PhoneticExtensions::ModifierLetterCapitalG => 'ᴳ',
            PhoneticExtensions::ModifierLetterCapitalH => 'ᴴ',
            PhoneticExtensions::ModifierLetterCapitalI => 'ᴵ',
            PhoneticExtensions::ModifierLetterCapitalJ => 'ᴶ',
            PhoneticExtensions::ModifierLetterCapitalK => 'ᴷ',
            PhoneticExtensions::ModifierLetterCapitalL => 'ᴸ',
            PhoneticExtensions::ModifierLetterCapitalM => 'ᴹ',
            PhoneticExtensions::ModifierLetterCapitalN => 'ᴺ',
            PhoneticExtensions::ModifierLetterCapitalReversedN => 'ᴻ',
            PhoneticExtensions::ModifierLetterCapitalO => 'ᴼ',
            PhoneticExtensions::ModifierLetterCapitalOu => 'ᴽ',
            PhoneticExtensions::ModifierLetterCapitalP => 'ᴾ',
            PhoneticExtensions::ModifierLetterCapitalR => 'ᴿ',
            PhoneticExtensions::ModifierLetterCapitalT => 'ᵀ',
            PhoneticExtensions::ModifierLetterCapitalU => 'ᵁ',
            PhoneticExtensions::ModifierLetterCapitalW => 'ᵂ',
            PhoneticExtensions::ModifierLetterSmallA => 'ᵃ',
            PhoneticExtensions::ModifierLetterSmallTurnedA => 'ᵄ',
            PhoneticExtensions::ModifierLetterSmallAlpha => 'ᵅ',
            PhoneticExtensions::ModifierLetterSmallTurnedAe => 'ᵆ',
            PhoneticExtensions::ModifierLetterSmallB => 'ᵇ',
            PhoneticExtensions::ModifierLetterSmallD => 'ᵈ',
            PhoneticExtensions::ModifierLetterSmallE => 'ᵉ',
            PhoneticExtensions::ModifierLetterSmallSchwa => 'ᵊ',
            PhoneticExtensions::ModifierLetterSmallOpenE => 'ᵋ',
            PhoneticExtensions::ModifierLetterSmallTurnedOpenE => 'ᵌ',
            PhoneticExtensions::ModifierLetterSmallG => 'ᵍ',
            PhoneticExtensions::ModifierLetterSmallTurnedI => 'ᵎ',
            PhoneticExtensions::ModifierLetterSmallK => 'ᵏ',
            PhoneticExtensions::ModifierLetterSmallM => 'ᵐ',
            PhoneticExtensions::ModifierLetterSmallEng => 'ᵑ',
            PhoneticExtensions::ModifierLetterSmallO => 'ᵒ',
            PhoneticExtensions::ModifierLetterSmallOpenO => 'ᵓ',
            PhoneticExtensions::ModifierLetterSmallTopHalfO => 'ᵔ',
            PhoneticExtensions::ModifierLetterSmallBottomHalfO => 'ᵕ',
            PhoneticExtensions::ModifierLetterSmallP => 'ᵖ',
            PhoneticExtensions::ModifierLetterSmallT => 'ᵗ',
            PhoneticExtensions::ModifierLetterSmallU => 'ᵘ',
            PhoneticExtensions::ModifierLetterSmallSidewaysU => 'ᵙ',
            PhoneticExtensions::ModifierLetterSmallTurnedM => 'ᵚ',
            PhoneticExtensions::ModifierLetterSmallV => 'ᵛ',
            PhoneticExtensions::ModifierLetterSmallAin => 'ᵜ',
            PhoneticExtensions::ModifierLetterSmallBeta => 'ᵝ',
            PhoneticExtensions::ModifierLetterSmallGreekGamma => 'ᵞ',
            PhoneticExtensions::ModifierLetterSmallDelta => 'ᵟ',
            PhoneticExtensions::ModifierLetterSmallGreekPhi => 'ᵠ',
            PhoneticExtensions::ModifierLetterSmallChi => 'ᵡ',
            PhoneticExtensions::LatinSubscriptSmallLetterI => 'ᵢ',
            PhoneticExtensions::LatinSubscriptSmallLetterR => 'ᵣ',
            PhoneticExtensions::LatinSubscriptSmallLetterU => 'ᵤ',
            PhoneticExtensions::LatinSubscriptSmallLetterV => 'ᵥ',
            PhoneticExtensions::GreekSubscriptSmallLetterBeta => 'ᵦ',
            PhoneticExtensions::GreekSubscriptSmallLetterGamma => 'ᵧ',
            PhoneticExtensions::GreekSubscriptSmallLetterRho => 'ᵨ',
            PhoneticExtensions::GreekSubscriptSmallLetterPhi => 'ᵩ',
            PhoneticExtensions::GreekSubscriptSmallLetterChi => 'ᵪ',
            PhoneticExtensions::LatinSmallLetterUe => 'ᵫ',
            PhoneticExtensions::LatinSmallLetterBWithMiddleTilde => 'ᵬ',
            PhoneticExtensions::LatinSmallLetterDWithMiddleTilde => 'ᵭ',
            PhoneticExtensions::LatinSmallLetterFWithMiddleTilde => 'ᵮ',
            PhoneticExtensions::LatinSmallLetterMWithMiddleTilde => 'ᵯ',
            PhoneticExtensions::LatinSmallLetterNWithMiddleTilde => 'ᵰ',
            PhoneticExtensions::LatinSmallLetterPWithMiddleTilde => 'ᵱ',
            PhoneticExtensions::LatinSmallLetterRWithMiddleTilde => 'ᵲ',
            PhoneticExtensions::LatinSmallLetterRWithFishhookAndMiddleTilde => 'ᵳ',
            PhoneticExtensions::LatinSmallLetterSWithMiddleTilde => 'ᵴ',
            PhoneticExtensions::LatinSmallLetterTWithMiddleTilde => 'ᵵ',
            PhoneticExtensions::LatinSmallLetterZWithMiddleTilde => 'ᵶ',
            PhoneticExtensions::LatinSmallLetterTurnedG => 'ᵷ',
            PhoneticExtensions::ModifierLetterCyrillicEn => 'ᵸ',
            PhoneticExtensions::LatinSmallLetterInsularG => 'ᵹ',
            PhoneticExtensions::LatinSmallLetterThWithStrikethrough => 'ᵺ',
            PhoneticExtensions::LatinSmallCapitalLetterIWithStroke => 'ᵻ',
            PhoneticExtensions::LatinSmallLetterIotaWithStroke => 'ᵼ',
            PhoneticExtensions::LatinSmallLetterPWithStroke => 'ᵽ',
            PhoneticExtensions::LatinSmallCapitalLetterUWithStroke => 'ᵾ',
        }
    }
}

impl std::convert::TryFrom<char> for PhoneticExtensions {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᴀ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalA),
            'ᴁ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalAe),
            'ᴂ' => Ok(PhoneticExtensions::LatinSmallLetterTurnedAe),
            'ᴃ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalBarredB),
            'ᴄ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalC),
            'ᴅ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalD),
            'ᴆ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalEth),
            'ᴇ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalE),
            'ᴈ' => Ok(PhoneticExtensions::LatinSmallLetterTurnedOpenE),
            'ᴉ' => Ok(PhoneticExtensions::LatinSmallLetterTurnedI),
            'ᴊ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalJ),
            'ᴋ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalK),
            'ᴌ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalLWithStroke),
            'ᴍ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalM),
            'ᴎ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalReversedN),
            'ᴏ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalO),
            'ᴐ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalOpenO),
            'ᴑ' => Ok(PhoneticExtensions::LatinSmallLetterSidewaysO),
            'ᴒ' => Ok(PhoneticExtensions::LatinSmallLetterSidewaysOpenO),
            'ᴓ' => Ok(PhoneticExtensions::LatinSmallLetterSidewaysOWithStroke),
            'ᴔ' => Ok(PhoneticExtensions::LatinSmallLetterTurnedOe),
            'ᴕ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalOu),
            'ᴖ' => Ok(PhoneticExtensions::LatinSmallLetterTopHalfO),
            'ᴗ' => Ok(PhoneticExtensions::LatinSmallLetterBottomHalfO),
            'ᴘ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalP),
            'ᴙ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalReversedR),
            'ᴚ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalTurnedR),
            'ᴛ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalT),
            'ᴜ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalU),
            'ᴝ' => Ok(PhoneticExtensions::LatinSmallLetterSidewaysU),
            'ᴞ' => Ok(PhoneticExtensions::LatinSmallLetterSidewaysDiaeresizedU),
            'ᴟ' => Ok(PhoneticExtensions::LatinSmallLetterSidewaysTurnedM),
            'ᴠ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalV),
            'ᴡ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalW),
            'ᴢ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalZ),
            'ᴣ' => Ok(PhoneticExtensions::LatinLetterSmallCapitalEzh),
            'ᴤ' => Ok(PhoneticExtensions::LatinLetterVoicedLaryngealSpirant),
            'ᴥ' => Ok(PhoneticExtensions::LatinLetterAin),
            'ᴦ' => Ok(PhoneticExtensions::GreekLetterSmallCapitalGamma),
            'ᴧ' => Ok(PhoneticExtensions::GreekLetterSmallCapitalLamda),
            'ᴨ' => Ok(PhoneticExtensions::GreekLetterSmallCapitalPi),
            'ᴩ' => Ok(PhoneticExtensions::GreekLetterSmallCapitalRho),
            'ᴪ' => Ok(PhoneticExtensions::GreekLetterSmallCapitalPsi),
            'ᴫ' => Ok(PhoneticExtensions::CyrillicLetterSmallCapitalEl),
            'ᴬ' => Ok(PhoneticExtensions::ModifierLetterCapitalA),
            'ᴭ' => Ok(PhoneticExtensions::ModifierLetterCapitalAe),
            'ᴮ' => Ok(PhoneticExtensions::ModifierLetterCapitalB),
            'ᴯ' => Ok(PhoneticExtensions::ModifierLetterCapitalBarredB),
            'ᴰ' => Ok(PhoneticExtensions::ModifierLetterCapitalD),
            'ᴱ' => Ok(PhoneticExtensions::ModifierLetterCapitalE),
            'ᴲ' => Ok(PhoneticExtensions::ModifierLetterCapitalReversedE),
            'ᴳ' => Ok(PhoneticExtensions::ModifierLetterCapitalG),
            'ᴴ' => Ok(PhoneticExtensions::ModifierLetterCapitalH),
            'ᴵ' => Ok(PhoneticExtensions::ModifierLetterCapitalI),
            'ᴶ' => Ok(PhoneticExtensions::ModifierLetterCapitalJ),
            'ᴷ' => Ok(PhoneticExtensions::ModifierLetterCapitalK),
            'ᴸ' => Ok(PhoneticExtensions::ModifierLetterCapitalL),
            'ᴹ' => Ok(PhoneticExtensions::ModifierLetterCapitalM),
            'ᴺ' => Ok(PhoneticExtensions::ModifierLetterCapitalN),
            'ᴻ' => Ok(PhoneticExtensions::ModifierLetterCapitalReversedN),
            'ᴼ' => Ok(PhoneticExtensions::ModifierLetterCapitalO),
            'ᴽ' => Ok(PhoneticExtensions::ModifierLetterCapitalOu),
            'ᴾ' => Ok(PhoneticExtensions::ModifierLetterCapitalP),
            'ᴿ' => Ok(PhoneticExtensions::ModifierLetterCapitalR),
            'ᵀ' => Ok(PhoneticExtensions::ModifierLetterCapitalT),
            'ᵁ' => Ok(PhoneticExtensions::ModifierLetterCapitalU),
            'ᵂ' => Ok(PhoneticExtensions::ModifierLetterCapitalW),
            'ᵃ' => Ok(PhoneticExtensions::ModifierLetterSmallA),
            'ᵄ' => Ok(PhoneticExtensions::ModifierLetterSmallTurnedA),
            'ᵅ' => Ok(PhoneticExtensions::ModifierLetterSmallAlpha),
            'ᵆ' => Ok(PhoneticExtensions::ModifierLetterSmallTurnedAe),
            'ᵇ' => Ok(PhoneticExtensions::ModifierLetterSmallB),
            'ᵈ' => Ok(PhoneticExtensions::ModifierLetterSmallD),
            'ᵉ' => Ok(PhoneticExtensions::ModifierLetterSmallE),
            'ᵊ' => Ok(PhoneticExtensions::ModifierLetterSmallSchwa),
            'ᵋ' => Ok(PhoneticExtensions::ModifierLetterSmallOpenE),
            'ᵌ' => Ok(PhoneticExtensions::ModifierLetterSmallTurnedOpenE),
            'ᵍ' => Ok(PhoneticExtensions::ModifierLetterSmallG),
            'ᵎ' => Ok(PhoneticExtensions::ModifierLetterSmallTurnedI),
            'ᵏ' => Ok(PhoneticExtensions::ModifierLetterSmallK),
            'ᵐ' => Ok(PhoneticExtensions::ModifierLetterSmallM),
            'ᵑ' => Ok(PhoneticExtensions::ModifierLetterSmallEng),
            'ᵒ' => Ok(PhoneticExtensions::ModifierLetterSmallO),
            'ᵓ' => Ok(PhoneticExtensions::ModifierLetterSmallOpenO),
            'ᵔ' => Ok(PhoneticExtensions::ModifierLetterSmallTopHalfO),
            'ᵕ' => Ok(PhoneticExtensions::ModifierLetterSmallBottomHalfO),
            'ᵖ' => Ok(PhoneticExtensions::ModifierLetterSmallP),
            'ᵗ' => Ok(PhoneticExtensions::ModifierLetterSmallT),
            'ᵘ' => Ok(PhoneticExtensions::ModifierLetterSmallU),
            'ᵙ' => Ok(PhoneticExtensions::ModifierLetterSmallSidewaysU),
            'ᵚ' => Ok(PhoneticExtensions::ModifierLetterSmallTurnedM),
            'ᵛ' => Ok(PhoneticExtensions::ModifierLetterSmallV),
            'ᵜ' => Ok(PhoneticExtensions::ModifierLetterSmallAin),
            'ᵝ' => Ok(PhoneticExtensions::ModifierLetterSmallBeta),
            'ᵞ' => Ok(PhoneticExtensions::ModifierLetterSmallGreekGamma),
            'ᵟ' => Ok(PhoneticExtensions::ModifierLetterSmallDelta),
            'ᵠ' => Ok(PhoneticExtensions::ModifierLetterSmallGreekPhi),
            'ᵡ' => Ok(PhoneticExtensions::ModifierLetterSmallChi),
            'ᵢ' => Ok(PhoneticExtensions::LatinSubscriptSmallLetterI),
            'ᵣ' => Ok(PhoneticExtensions::LatinSubscriptSmallLetterR),
            'ᵤ' => Ok(PhoneticExtensions::LatinSubscriptSmallLetterU),
            'ᵥ' => Ok(PhoneticExtensions::LatinSubscriptSmallLetterV),
            'ᵦ' => Ok(PhoneticExtensions::GreekSubscriptSmallLetterBeta),
            'ᵧ' => Ok(PhoneticExtensions::GreekSubscriptSmallLetterGamma),
            'ᵨ' => Ok(PhoneticExtensions::GreekSubscriptSmallLetterRho),
            'ᵩ' => Ok(PhoneticExtensions::GreekSubscriptSmallLetterPhi),
            'ᵪ' => Ok(PhoneticExtensions::GreekSubscriptSmallLetterChi),
            'ᵫ' => Ok(PhoneticExtensions::LatinSmallLetterUe),
            'ᵬ' => Ok(PhoneticExtensions::LatinSmallLetterBWithMiddleTilde),
            'ᵭ' => Ok(PhoneticExtensions::LatinSmallLetterDWithMiddleTilde),
            'ᵮ' => Ok(PhoneticExtensions::LatinSmallLetterFWithMiddleTilde),
            'ᵯ' => Ok(PhoneticExtensions::LatinSmallLetterMWithMiddleTilde),
            'ᵰ' => Ok(PhoneticExtensions::LatinSmallLetterNWithMiddleTilde),
            'ᵱ' => Ok(PhoneticExtensions::LatinSmallLetterPWithMiddleTilde),
            'ᵲ' => Ok(PhoneticExtensions::LatinSmallLetterRWithMiddleTilde),
            'ᵳ' => Ok(PhoneticExtensions::LatinSmallLetterRWithFishhookAndMiddleTilde),
            'ᵴ' => Ok(PhoneticExtensions::LatinSmallLetterSWithMiddleTilde),
            'ᵵ' => Ok(PhoneticExtensions::LatinSmallLetterTWithMiddleTilde),
            'ᵶ' => Ok(PhoneticExtensions::LatinSmallLetterZWithMiddleTilde),
            'ᵷ' => Ok(PhoneticExtensions::LatinSmallLetterTurnedG),
            'ᵸ' => Ok(PhoneticExtensions::ModifierLetterCyrillicEn),
            'ᵹ' => Ok(PhoneticExtensions::LatinSmallLetterInsularG),
            'ᵺ' => Ok(PhoneticExtensions::LatinSmallLetterThWithStrikethrough),
            'ᵻ' => Ok(PhoneticExtensions::LatinSmallCapitalLetterIWithStroke),
            'ᵼ' => Ok(PhoneticExtensions::LatinSmallLetterIotaWithStroke),
            'ᵽ' => Ok(PhoneticExtensions::LatinSmallLetterPWithStroke),
            'ᵾ' => Ok(PhoneticExtensions::LatinSmallCapitalLetterUWithStroke),
            _ => Err(()),
        }
    }
}

impl Into<u32> for PhoneticExtensions {
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

impl std::convert::TryFrom<u32> for PhoneticExtensions {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for PhoneticExtensions {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl PhoneticExtensions {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        PhoneticExtensions::LatinLetterSmallCapitalA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("PhoneticExtensions{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
