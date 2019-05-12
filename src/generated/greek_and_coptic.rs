
/// An enum to represent all characters in the GreekandCoptic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GreekandCoptic {
    /// \u{370}: 'Ͱ'
    GreekCapitalLetterHeta,
    /// \u{371}: 'ͱ'
    GreekSmallLetterHeta,
    /// \u{372}: 'Ͳ'
    GreekCapitalLetterArchaicSampi,
    /// \u{373}: 'ͳ'
    GreekSmallLetterArchaicSampi,
    /// \u{374}: 'ʹ'
    GreekNumeralSign,
    /// \u{375}: '͵'
    GreekLowerNumeralSign,
    /// \u{376}: 'Ͷ'
    GreekCapitalLetterPamphylianDigamma,
    /// \u{377}: 'ͷ'
    GreekSmallLetterPamphylianDigamma,
    /// \u{37a}: 'ͺ'
    GreekYpogegrammeni,
    /// \u{37b}: 'ͻ'
    GreekSmallReversedLunateSigmaSymbol,
    /// \u{37c}: 'ͼ'
    GreekSmallDottedLunateSigmaSymbol,
    /// \u{37d}: 'ͽ'
    GreekSmallReversedDottedLunateSigmaSymbol,
    /// \u{37e}: ';'
    GreekQuestionMark,
    /// \u{37f}: 'Ϳ'
    GreekCapitalLetterYot,
    /// \u{384}: '΄'
    GreekTonos,
    /// \u{385}: '΅'
    GreekDialytikaTonos,
    /// \u{386}: 'Ά'
    GreekCapitalLetterAlphaWithTonos,
    /// \u{387}: '·'
    GreekAnoTeleia,
    /// \u{388}: 'Έ'
    GreekCapitalLetterEpsilonWithTonos,
    /// \u{389}: 'Ή'
    GreekCapitalLetterEtaWithTonos,
    /// \u{38a}: 'Ί'
    GreekCapitalLetterIotaWithTonos,
    /// \u{38c}: 'Ό'
    GreekCapitalLetterOmicronWithTonos,
    /// \u{38e}: 'Ύ'
    GreekCapitalLetterUpsilonWithTonos,
    /// \u{38f}: 'Ώ'
    GreekCapitalLetterOmegaWithTonos,
    /// \u{390}: 'ΐ'
    GreekSmallLetterIotaWithDialytikaAndTonos,
    /// \u{391}: 'Α'
    GreekCapitalLetterAlpha,
    /// \u{392}: 'Β'
    GreekCapitalLetterBeta,
    /// \u{393}: 'Γ'
    GreekCapitalLetterGamma,
    /// \u{394}: 'Δ'
    GreekCapitalLetterDelta,
    /// \u{395}: 'Ε'
    GreekCapitalLetterEpsilon,
    /// \u{396}: 'Ζ'
    GreekCapitalLetterZeta,
    /// \u{397}: 'Η'
    GreekCapitalLetterEta,
    /// \u{398}: 'Θ'
    GreekCapitalLetterTheta,
    /// \u{399}: 'Ι'
    GreekCapitalLetterIota,
    /// \u{39a}: 'Κ'
    GreekCapitalLetterKappa,
    /// \u{39b}: 'Λ'
    GreekCapitalLetterLamda,
    /// \u{39c}: 'Μ'
    GreekCapitalLetterMu,
    /// \u{39d}: 'Ν'
    GreekCapitalLetterNu,
    /// \u{39e}: 'Ξ'
    GreekCapitalLetterXi,
    /// \u{39f}: 'Ο'
    GreekCapitalLetterOmicron,
    /// \u{3a0}: 'Π'
    GreekCapitalLetterPi,
    /// \u{3a1}: 'Ρ'
    GreekCapitalLetterRho,
    /// \u{3a3}: 'Σ'
    GreekCapitalLetterSigma,
    /// \u{3a4}: 'Τ'
    GreekCapitalLetterTau,
    /// \u{3a5}: 'Υ'
    GreekCapitalLetterUpsilon,
    /// \u{3a6}: 'Φ'
    GreekCapitalLetterPhi,
    /// \u{3a7}: 'Χ'
    GreekCapitalLetterChi,
    /// \u{3a8}: 'Ψ'
    GreekCapitalLetterPsi,
    /// \u{3a9}: 'Ω'
    GreekCapitalLetterOmega,
    /// \u{3aa}: 'Ϊ'
    GreekCapitalLetterIotaWithDialytika,
    /// \u{3ab}: 'Ϋ'
    GreekCapitalLetterUpsilonWithDialytika,
    /// \u{3ac}: 'ά'
    GreekSmallLetterAlphaWithTonos,
    /// \u{3ad}: 'έ'
    GreekSmallLetterEpsilonWithTonos,
    /// \u{3ae}: 'ή'
    GreekSmallLetterEtaWithTonos,
    /// \u{3af}: 'ί'
    GreekSmallLetterIotaWithTonos,
    /// \u{3b0}: 'ΰ'
    GreekSmallLetterUpsilonWithDialytikaAndTonos,
    /// \u{3b1}: 'α'
    GreekSmallLetterAlpha,
    /// \u{3b2}: 'β'
    GreekSmallLetterBeta,
    /// \u{3b3}: 'γ'
    GreekSmallLetterGamma,
    /// \u{3b4}: 'δ'
    GreekSmallLetterDelta,
    /// \u{3b5}: 'ε'
    GreekSmallLetterEpsilon,
    /// \u{3b6}: 'ζ'
    GreekSmallLetterZeta,
    /// \u{3b7}: 'η'
    GreekSmallLetterEta,
    /// \u{3b8}: 'θ'
    GreekSmallLetterTheta,
    /// \u{3b9}: 'ι'
    GreekSmallLetterIota,
    /// \u{3ba}: 'κ'
    GreekSmallLetterKappa,
    /// \u{3bb}: 'λ'
    GreekSmallLetterLamda,
    /// \u{3bc}: 'μ'
    GreekSmallLetterMu,
    /// \u{3bd}: 'ν'
    GreekSmallLetterNu,
    /// \u{3be}: 'ξ'
    GreekSmallLetterXi,
    /// \u{3bf}: 'ο'
    GreekSmallLetterOmicron,
    /// \u{3c0}: 'π'
    GreekSmallLetterPi,
    /// \u{3c1}: 'ρ'
    GreekSmallLetterRho,
    /// \u{3c2}: 'ς'
    GreekSmallLetterFinalSigma,
    /// \u{3c3}: 'σ'
    GreekSmallLetterSigma,
    /// \u{3c4}: 'τ'
    GreekSmallLetterTau,
    /// \u{3c5}: 'υ'
    GreekSmallLetterUpsilon,
    /// \u{3c6}: 'φ'
    GreekSmallLetterPhi,
    /// \u{3c7}: 'χ'
    GreekSmallLetterChi,
    /// \u{3c8}: 'ψ'
    GreekSmallLetterPsi,
    /// \u{3c9}: 'ω'
    GreekSmallLetterOmega,
    /// \u{3ca}: 'ϊ'
    GreekSmallLetterIotaWithDialytika,
    /// \u{3cb}: 'ϋ'
    GreekSmallLetterUpsilonWithDialytika,
    /// \u{3cc}: 'ό'
    GreekSmallLetterOmicronWithTonos,
    /// \u{3cd}: 'ύ'
    GreekSmallLetterUpsilonWithTonos,
    /// \u{3ce}: 'ώ'
    GreekSmallLetterOmegaWithTonos,
    /// \u{3cf}: 'Ϗ'
    GreekCapitalKaiSymbol,
    /// \u{3d0}: 'ϐ'
    GreekBetaSymbol,
    /// \u{3d1}: 'ϑ'
    GreekThetaSymbol,
    /// \u{3d2}: 'ϒ'
    GreekUpsilonWithHookSymbol,
    /// \u{3d3}: 'ϓ'
    GreekUpsilonWithAcuteAndHookSymbol,
    /// \u{3d4}: 'ϔ'
    GreekUpsilonWithDiaeresisAndHookSymbol,
    /// \u{3d5}: 'ϕ'
    GreekPhiSymbol,
    /// \u{3d6}: 'ϖ'
    GreekPiSymbol,
    /// \u{3d7}: 'ϗ'
    GreekKaiSymbol,
    /// \u{3d8}: 'Ϙ'
    GreekLetterArchaicKoppa,
    /// \u{3d9}: 'ϙ'
    GreekSmallLetterArchaicKoppa,
    /// \u{3da}: 'Ϛ'
    GreekLetterStigma,
    /// \u{3db}: 'ϛ'
    GreekSmallLetterStigma,
    /// \u{3dc}: 'Ϝ'
    GreekLetterDigamma,
    /// \u{3dd}: 'ϝ'
    GreekSmallLetterDigamma,
    /// \u{3de}: 'Ϟ'
    GreekLetterKoppa,
    /// \u{3df}: 'ϟ'
    GreekSmallLetterKoppa,
    /// \u{3e0}: 'Ϡ'
    GreekLetterSampi,
    /// \u{3e1}: 'ϡ'
    GreekSmallLetterSampi,
    /// \u{3e2}: 'Ϣ'
    CopticCapitalLetterShei,
    /// \u{3e3}: 'ϣ'
    CopticSmallLetterShei,
    /// \u{3e4}: 'Ϥ'
    CopticCapitalLetterFei,
    /// \u{3e5}: 'ϥ'
    CopticSmallLetterFei,
    /// \u{3e6}: 'Ϧ'
    CopticCapitalLetterKhei,
    /// \u{3e7}: 'ϧ'
    CopticSmallLetterKhei,
    /// \u{3e8}: 'Ϩ'
    CopticCapitalLetterHori,
    /// \u{3e9}: 'ϩ'
    CopticSmallLetterHori,
    /// \u{3ea}: 'Ϫ'
    CopticCapitalLetterGangia,
    /// \u{3eb}: 'ϫ'
    CopticSmallLetterGangia,
    /// \u{3ec}: 'Ϭ'
    CopticCapitalLetterShima,
    /// \u{3ed}: 'ϭ'
    CopticSmallLetterShima,
    /// \u{3ee}: 'Ϯ'
    CopticCapitalLetterDei,
    /// \u{3ef}: 'ϯ'
    CopticSmallLetterDei,
    /// \u{3f0}: 'ϰ'
    GreekKappaSymbol,
    /// \u{3f1}: 'ϱ'
    GreekRhoSymbol,
    /// \u{3f2}: 'ϲ'
    GreekLunateSigmaSymbol,
    /// \u{3f3}: 'ϳ'
    GreekLetterYot,
    /// \u{3f4}: 'ϴ'
    GreekCapitalThetaSymbol,
    /// \u{3f5}: 'ϵ'
    GreekLunateEpsilonSymbol,
    /// \u{3f6}: '϶'
    GreekReversedLunateEpsilonSymbol,
    /// \u{3f7}: 'Ϸ'
    GreekCapitalLetterSho,
    /// \u{3f8}: 'ϸ'
    GreekSmallLetterSho,
    /// \u{3f9}: 'Ϲ'
    GreekCapitalLunateSigmaSymbol,
    /// \u{3fa}: 'Ϻ'
    GreekCapitalLetterSan,
    /// \u{3fb}: 'ϻ'
    GreekSmallLetterSan,
    /// \u{3fc}: 'ϼ'
    GreekRhoWithStrokeSymbol,
    /// \u{3fd}: 'Ͻ'
    GreekCapitalReversedLunateSigmaSymbol,
    /// \u{3fe}: 'Ͼ'
    GreekCapitalDottedLunateSigmaSymbol,
}

impl Into<char> for GreekandCoptic {
    fn into(self) -> char {
        match self {
            GreekandCoptic::GreekCapitalLetterHeta => 'Ͱ',
            GreekandCoptic::GreekSmallLetterHeta => 'ͱ',
            GreekandCoptic::GreekCapitalLetterArchaicSampi => 'Ͳ',
            GreekandCoptic::GreekSmallLetterArchaicSampi => 'ͳ',
            GreekandCoptic::GreekNumeralSign => 'ʹ',
            GreekandCoptic::GreekLowerNumeralSign => '͵',
            GreekandCoptic::GreekCapitalLetterPamphylianDigamma => 'Ͷ',
            GreekandCoptic::GreekSmallLetterPamphylianDigamma => 'ͷ',
            GreekandCoptic::GreekYpogegrammeni => 'ͺ',
            GreekandCoptic::GreekSmallReversedLunateSigmaSymbol => 'ͻ',
            GreekandCoptic::GreekSmallDottedLunateSigmaSymbol => 'ͼ',
            GreekandCoptic::GreekSmallReversedDottedLunateSigmaSymbol => 'ͽ',
            GreekandCoptic::GreekQuestionMark => ';',
            GreekandCoptic::GreekCapitalLetterYot => 'Ϳ',
            GreekandCoptic::GreekTonos => '΄',
            GreekandCoptic::GreekDialytikaTonos => '΅',
            GreekandCoptic::GreekCapitalLetterAlphaWithTonos => 'Ά',
            GreekandCoptic::GreekAnoTeleia => '·',
            GreekandCoptic::GreekCapitalLetterEpsilonWithTonos => 'Έ',
            GreekandCoptic::GreekCapitalLetterEtaWithTonos => 'Ή',
            GreekandCoptic::GreekCapitalLetterIotaWithTonos => 'Ί',
            GreekandCoptic::GreekCapitalLetterOmicronWithTonos => 'Ό',
            GreekandCoptic::GreekCapitalLetterUpsilonWithTonos => 'Ύ',
            GreekandCoptic::GreekCapitalLetterOmegaWithTonos => 'Ώ',
            GreekandCoptic::GreekSmallLetterIotaWithDialytikaAndTonos => 'ΐ',
            GreekandCoptic::GreekCapitalLetterAlpha => 'Α',
            GreekandCoptic::GreekCapitalLetterBeta => 'Β',
            GreekandCoptic::GreekCapitalLetterGamma => 'Γ',
            GreekandCoptic::GreekCapitalLetterDelta => 'Δ',
            GreekandCoptic::GreekCapitalLetterEpsilon => 'Ε',
            GreekandCoptic::GreekCapitalLetterZeta => 'Ζ',
            GreekandCoptic::GreekCapitalLetterEta => 'Η',
            GreekandCoptic::GreekCapitalLetterTheta => 'Θ',
            GreekandCoptic::GreekCapitalLetterIota => 'Ι',
            GreekandCoptic::GreekCapitalLetterKappa => 'Κ',
            GreekandCoptic::GreekCapitalLetterLamda => 'Λ',
            GreekandCoptic::GreekCapitalLetterMu => 'Μ',
            GreekandCoptic::GreekCapitalLetterNu => 'Ν',
            GreekandCoptic::GreekCapitalLetterXi => 'Ξ',
            GreekandCoptic::GreekCapitalLetterOmicron => 'Ο',
            GreekandCoptic::GreekCapitalLetterPi => 'Π',
            GreekandCoptic::GreekCapitalLetterRho => 'Ρ',
            GreekandCoptic::GreekCapitalLetterSigma => 'Σ',
            GreekandCoptic::GreekCapitalLetterTau => 'Τ',
            GreekandCoptic::GreekCapitalLetterUpsilon => 'Υ',
            GreekandCoptic::GreekCapitalLetterPhi => 'Φ',
            GreekandCoptic::GreekCapitalLetterChi => 'Χ',
            GreekandCoptic::GreekCapitalLetterPsi => 'Ψ',
            GreekandCoptic::GreekCapitalLetterOmega => 'Ω',
            GreekandCoptic::GreekCapitalLetterIotaWithDialytika => 'Ϊ',
            GreekandCoptic::GreekCapitalLetterUpsilonWithDialytika => 'Ϋ',
            GreekandCoptic::GreekSmallLetterAlphaWithTonos => 'ά',
            GreekandCoptic::GreekSmallLetterEpsilonWithTonos => 'έ',
            GreekandCoptic::GreekSmallLetterEtaWithTonos => 'ή',
            GreekandCoptic::GreekSmallLetterIotaWithTonos => 'ί',
            GreekandCoptic::GreekSmallLetterUpsilonWithDialytikaAndTonos => 'ΰ',
            GreekandCoptic::GreekSmallLetterAlpha => 'α',
            GreekandCoptic::GreekSmallLetterBeta => 'β',
            GreekandCoptic::GreekSmallLetterGamma => 'γ',
            GreekandCoptic::GreekSmallLetterDelta => 'δ',
            GreekandCoptic::GreekSmallLetterEpsilon => 'ε',
            GreekandCoptic::GreekSmallLetterZeta => 'ζ',
            GreekandCoptic::GreekSmallLetterEta => 'η',
            GreekandCoptic::GreekSmallLetterTheta => 'θ',
            GreekandCoptic::GreekSmallLetterIota => 'ι',
            GreekandCoptic::GreekSmallLetterKappa => 'κ',
            GreekandCoptic::GreekSmallLetterLamda => 'λ',
            GreekandCoptic::GreekSmallLetterMu => 'μ',
            GreekandCoptic::GreekSmallLetterNu => 'ν',
            GreekandCoptic::GreekSmallLetterXi => 'ξ',
            GreekandCoptic::GreekSmallLetterOmicron => 'ο',
            GreekandCoptic::GreekSmallLetterPi => 'π',
            GreekandCoptic::GreekSmallLetterRho => 'ρ',
            GreekandCoptic::GreekSmallLetterFinalSigma => 'ς',
            GreekandCoptic::GreekSmallLetterSigma => 'σ',
            GreekandCoptic::GreekSmallLetterTau => 'τ',
            GreekandCoptic::GreekSmallLetterUpsilon => 'υ',
            GreekandCoptic::GreekSmallLetterPhi => 'φ',
            GreekandCoptic::GreekSmallLetterChi => 'χ',
            GreekandCoptic::GreekSmallLetterPsi => 'ψ',
            GreekandCoptic::GreekSmallLetterOmega => 'ω',
            GreekandCoptic::GreekSmallLetterIotaWithDialytika => 'ϊ',
            GreekandCoptic::GreekSmallLetterUpsilonWithDialytika => 'ϋ',
            GreekandCoptic::GreekSmallLetterOmicronWithTonos => 'ό',
            GreekandCoptic::GreekSmallLetterUpsilonWithTonos => 'ύ',
            GreekandCoptic::GreekSmallLetterOmegaWithTonos => 'ώ',
            GreekandCoptic::GreekCapitalKaiSymbol => 'Ϗ',
            GreekandCoptic::GreekBetaSymbol => 'ϐ',
            GreekandCoptic::GreekThetaSymbol => 'ϑ',
            GreekandCoptic::GreekUpsilonWithHookSymbol => 'ϒ',
            GreekandCoptic::GreekUpsilonWithAcuteAndHookSymbol => 'ϓ',
            GreekandCoptic::GreekUpsilonWithDiaeresisAndHookSymbol => 'ϔ',
            GreekandCoptic::GreekPhiSymbol => 'ϕ',
            GreekandCoptic::GreekPiSymbol => 'ϖ',
            GreekandCoptic::GreekKaiSymbol => 'ϗ',
            GreekandCoptic::GreekLetterArchaicKoppa => 'Ϙ',
            GreekandCoptic::GreekSmallLetterArchaicKoppa => 'ϙ',
            GreekandCoptic::GreekLetterStigma => 'Ϛ',
            GreekandCoptic::GreekSmallLetterStigma => 'ϛ',
            GreekandCoptic::GreekLetterDigamma => 'Ϝ',
            GreekandCoptic::GreekSmallLetterDigamma => 'ϝ',
            GreekandCoptic::GreekLetterKoppa => 'Ϟ',
            GreekandCoptic::GreekSmallLetterKoppa => 'ϟ',
            GreekandCoptic::GreekLetterSampi => 'Ϡ',
            GreekandCoptic::GreekSmallLetterSampi => 'ϡ',
            GreekandCoptic::CopticCapitalLetterShei => 'Ϣ',
            GreekandCoptic::CopticSmallLetterShei => 'ϣ',
            GreekandCoptic::CopticCapitalLetterFei => 'Ϥ',
            GreekandCoptic::CopticSmallLetterFei => 'ϥ',
            GreekandCoptic::CopticCapitalLetterKhei => 'Ϧ',
            GreekandCoptic::CopticSmallLetterKhei => 'ϧ',
            GreekandCoptic::CopticCapitalLetterHori => 'Ϩ',
            GreekandCoptic::CopticSmallLetterHori => 'ϩ',
            GreekandCoptic::CopticCapitalLetterGangia => 'Ϫ',
            GreekandCoptic::CopticSmallLetterGangia => 'ϫ',
            GreekandCoptic::CopticCapitalLetterShima => 'Ϭ',
            GreekandCoptic::CopticSmallLetterShima => 'ϭ',
            GreekandCoptic::CopticCapitalLetterDei => 'Ϯ',
            GreekandCoptic::CopticSmallLetterDei => 'ϯ',
            GreekandCoptic::GreekKappaSymbol => 'ϰ',
            GreekandCoptic::GreekRhoSymbol => 'ϱ',
            GreekandCoptic::GreekLunateSigmaSymbol => 'ϲ',
            GreekandCoptic::GreekLetterYot => 'ϳ',
            GreekandCoptic::GreekCapitalThetaSymbol => 'ϴ',
            GreekandCoptic::GreekLunateEpsilonSymbol => 'ϵ',
            GreekandCoptic::GreekReversedLunateEpsilonSymbol => '϶',
            GreekandCoptic::GreekCapitalLetterSho => 'Ϸ',
            GreekandCoptic::GreekSmallLetterSho => 'ϸ',
            GreekandCoptic::GreekCapitalLunateSigmaSymbol => 'Ϲ',
            GreekandCoptic::GreekCapitalLetterSan => 'Ϻ',
            GreekandCoptic::GreekSmallLetterSan => 'ϻ',
            GreekandCoptic::GreekRhoWithStrokeSymbol => 'ϼ',
            GreekandCoptic::GreekCapitalReversedLunateSigmaSymbol => 'Ͻ',
            GreekandCoptic::GreekCapitalDottedLunateSigmaSymbol => 'Ͼ',
        }
    }
}

impl std::convert::TryFrom<char> for GreekandCoptic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ͱ' => Ok(GreekandCoptic::GreekCapitalLetterHeta),
            'ͱ' => Ok(GreekandCoptic::GreekSmallLetterHeta),
            'Ͳ' => Ok(GreekandCoptic::GreekCapitalLetterArchaicSampi),
            'ͳ' => Ok(GreekandCoptic::GreekSmallLetterArchaicSampi),
            'ʹ' => Ok(GreekandCoptic::GreekNumeralSign),
            '͵' => Ok(GreekandCoptic::GreekLowerNumeralSign),
            'Ͷ' => Ok(GreekandCoptic::GreekCapitalLetterPamphylianDigamma),
            'ͷ' => Ok(GreekandCoptic::GreekSmallLetterPamphylianDigamma),
            'ͺ' => Ok(GreekandCoptic::GreekYpogegrammeni),
            'ͻ' => Ok(GreekandCoptic::GreekSmallReversedLunateSigmaSymbol),
            'ͼ' => Ok(GreekandCoptic::GreekSmallDottedLunateSigmaSymbol),
            'ͽ' => Ok(GreekandCoptic::GreekSmallReversedDottedLunateSigmaSymbol),
            ';' => Ok(GreekandCoptic::GreekQuestionMark),
            'Ϳ' => Ok(GreekandCoptic::GreekCapitalLetterYot),
            '΄' => Ok(GreekandCoptic::GreekTonos),
            '΅' => Ok(GreekandCoptic::GreekDialytikaTonos),
            'Ά' => Ok(GreekandCoptic::GreekCapitalLetterAlphaWithTonos),
            '·' => Ok(GreekandCoptic::GreekAnoTeleia),
            'Έ' => Ok(GreekandCoptic::GreekCapitalLetterEpsilonWithTonos),
            'Ή' => Ok(GreekandCoptic::GreekCapitalLetterEtaWithTonos),
            'Ί' => Ok(GreekandCoptic::GreekCapitalLetterIotaWithTonos),
            'Ό' => Ok(GreekandCoptic::GreekCapitalLetterOmicronWithTonos),
            'Ύ' => Ok(GreekandCoptic::GreekCapitalLetterUpsilonWithTonos),
            'Ώ' => Ok(GreekandCoptic::GreekCapitalLetterOmegaWithTonos),
            'ΐ' => Ok(GreekandCoptic::GreekSmallLetterIotaWithDialytikaAndTonos),
            'Α' => Ok(GreekandCoptic::GreekCapitalLetterAlpha),
            'Β' => Ok(GreekandCoptic::GreekCapitalLetterBeta),
            'Γ' => Ok(GreekandCoptic::GreekCapitalLetterGamma),
            'Δ' => Ok(GreekandCoptic::GreekCapitalLetterDelta),
            'Ε' => Ok(GreekandCoptic::GreekCapitalLetterEpsilon),
            'Ζ' => Ok(GreekandCoptic::GreekCapitalLetterZeta),
            'Η' => Ok(GreekandCoptic::GreekCapitalLetterEta),
            'Θ' => Ok(GreekandCoptic::GreekCapitalLetterTheta),
            'Ι' => Ok(GreekandCoptic::GreekCapitalLetterIota),
            'Κ' => Ok(GreekandCoptic::GreekCapitalLetterKappa),
            'Λ' => Ok(GreekandCoptic::GreekCapitalLetterLamda),
            'Μ' => Ok(GreekandCoptic::GreekCapitalLetterMu),
            'Ν' => Ok(GreekandCoptic::GreekCapitalLetterNu),
            'Ξ' => Ok(GreekandCoptic::GreekCapitalLetterXi),
            'Ο' => Ok(GreekandCoptic::GreekCapitalLetterOmicron),
            'Π' => Ok(GreekandCoptic::GreekCapitalLetterPi),
            'Ρ' => Ok(GreekandCoptic::GreekCapitalLetterRho),
            'Σ' => Ok(GreekandCoptic::GreekCapitalLetterSigma),
            'Τ' => Ok(GreekandCoptic::GreekCapitalLetterTau),
            'Υ' => Ok(GreekandCoptic::GreekCapitalLetterUpsilon),
            'Φ' => Ok(GreekandCoptic::GreekCapitalLetterPhi),
            'Χ' => Ok(GreekandCoptic::GreekCapitalLetterChi),
            'Ψ' => Ok(GreekandCoptic::GreekCapitalLetterPsi),
            'Ω' => Ok(GreekandCoptic::GreekCapitalLetterOmega),
            'Ϊ' => Ok(GreekandCoptic::GreekCapitalLetterIotaWithDialytika),
            'Ϋ' => Ok(GreekandCoptic::GreekCapitalLetterUpsilonWithDialytika),
            'ά' => Ok(GreekandCoptic::GreekSmallLetterAlphaWithTonos),
            'έ' => Ok(GreekandCoptic::GreekSmallLetterEpsilonWithTonos),
            'ή' => Ok(GreekandCoptic::GreekSmallLetterEtaWithTonos),
            'ί' => Ok(GreekandCoptic::GreekSmallLetterIotaWithTonos),
            'ΰ' => Ok(GreekandCoptic::GreekSmallLetterUpsilonWithDialytikaAndTonos),
            'α' => Ok(GreekandCoptic::GreekSmallLetterAlpha),
            'β' => Ok(GreekandCoptic::GreekSmallLetterBeta),
            'γ' => Ok(GreekandCoptic::GreekSmallLetterGamma),
            'δ' => Ok(GreekandCoptic::GreekSmallLetterDelta),
            'ε' => Ok(GreekandCoptic::GreekSmallLetterEpsilon),
            'ζ' => Ok(GreekandCoptic::GreekSmallLetterZeta),
            'η' => Ok(GreekandCoptic::GreekSmallLetterEta),
            'θ' => Ok(GreekandCoptic::GreekSmallLetterTheta),
            'ι' => Ok(GreekandCoptic::GreekSmallLetterIota),
            'κ' => Ok(GreekandCoptic::GreekSmallLetterKappa),
            'λ' => Ok(GreekandCoptic::GreekSmallLetterLamda),
            'μ' => Ok(GreekandCoptic::GreekSmallLetterMu),
            'ν' => Ok(GreekandCoptic::GreekSmallLetterNu),
            'ξ' => Ok(GreekandCoptic::GreekSmallLetterXi),
            'ο' => Ok(GreekandCoptic::GreekSmallLetterOmicron),
            'π' => Ok(GreekandCoptic::GreekSmallLetterPi),
            'ρ' => Ok(GreekandCoptic::GreekSmallLetterRho),
            'ς' => Ok(GreekandCoptic::GreekSmallLetterFinalSigma),
            'σ' => Ok(GreekandCoptic::GreekSmallLetterSigma),
            'τ' => Ok(GreekandCoptic::GreekSmallLetterTau),
            'υ' => Ok(GreekandCoptic::GreekSmallLetterUpsilon),
            'φ' => Ok(GreekandCoptic::GreekSmallLetterPhi),
            'χ' => Ok(GreekandCoptic::GreekSmallLetterChi),
            'ψ' => Ok(GreekandCoptic::GreekSmallLetterPsi),
            'ω' => Ok(GreekandCoptic::GreekSmallLetterOmega),
            'ϊ' => Ok(GreekandCoptic::GreekSmallLetterIotaWithDialytika),
            'ϋ' => Ok(GreekandCoptic::GreekSmallLetterUpsilonWithDialytika),
            'ό' => Ok(GreekandCoptic::GreekSmallLetterOmicronWithTonos),
            'ύ' => Ok(GreekandCoptic::GreekSmallLetterUpsilonWithTonos),
            'ώ' => Ok(GreekandCoptic::GreekSmallLetterOmegaWithTonos),
            'Ϗ' => Ok(GreekandCoptic::GreekCapitalKaiSymbol),
            'ϐ' => Ok(GreekandCoptic::GreekBetaSymbol),
            'ϑ' => Ok(GreekandCoptic::GreekThetaSymbol),
            'ϒ' => Ok(GreekandCoptic::GreekUpsilonWithHookSymbol),
            'ϓ' => Ok(GreekandCoptic::GreekUpsilonWithAcuteAndHookSymbol),
            'ϔ' => Ok(GreekandCoptic::GreekUpsilonWithDiaeresisAndHookSymbol),
            'ϕ' => Ok(GreekandCoptic::GreekPhiSymbol),
            'ϖ' => Ok(GreekandCoptic::GreekPiSymbol),
            'ϗ' => Ok(GreekandCoptic::GreekKaiSymbol),
            'Ϙ' => Ok(GreekandCoptic::GreekLetterArchaicKoppa),
            'ϙ' => Ok(GreekandCoptic::GreekSmallLetterArchaicKoppa),
            'Ϛ' => Ok(GreekandCoptic::GreekLetterStigma),
            'ϛ' => Ok(GreekandCoptic::GreekSmallLetterStigma),
            'Ϝ' => Ok(GreekandCoptic::GreekLetterDigamma),
            'ϝ' => Ok(GreekandCoptic::GreekSmallLetterDigamma),
            'Ϟ' => Ok(GreekandCoptic::GreekLetterKoppa),
            'ϟ' => Ok(GreekandCoptic::GreekSmallLetterKoppa),
            'Ϡ' => Ok(GreekandCoptic::GreekLetterSampi),
            'ϡ' => Ok(GreekandCoptic::GreekSmallLetterSampi),
            'Ϣ' => Ok(GreekandCoptic::CopticCapitalLetterShei),
            'ϣ' => Ok(GreekandCoptic::CopticSmallLetterShei),
            'Ϥ' => Ok(GreekandCoptic::CopticCapitalLetterFei),
            'ϥ' => Ok(GreekandCoptic::CopticSmallLetterFei),
            'Ϧ' => Ok(GreekandCoptic::CopticCapitalLetterKhei),
            'ϧ' => Ok(GreekandCoptic::CopticSmallLetterKhei),
            'Ϩ' => Ok(GreekandCoptic::CopticCapitalLetterHori),
            'ϩ' => Ok(GreekandCoptic::CopticSmallLetterHori),
            'Ϫ' => Ok(GreekandCoptic::CopticCapitalLetterGangia),
            'ϫ' => Ok(GreekandCoptic::CopticSmallLetterGangia),
            'Ϭ' => Ok(GreekandCoptic::CopticCapitalLetterShima),
            'ϭ' => Ok(GreekandCoptic::CopticSmallLetterShima),
            'Ϯ' => Ok(GreekandCoptic::CopticCapitalLetterDei),
            'ϯ' => Ok(GreekandCoptic::CopticSmallLetterDei),
            'ϰ' => Ok(GreekandCoptic::GreekKappaSymbol),
            'ϱ' => Ok(GreekandCoptic::GreekRhoSymbol),
            'ϲ' => Ok(GreekandCoptic::GreekLunateSigmaSymbol),
            'ϳ' => Ok(GreekandCoptic::GreekLetterYot),
            'ϴ' => Ok(GreekandCoptic::GreekCapitalThetaSymbol),
            'ϵ' => Ok(GreekandCoptic::GreekLunateEpsilonSymbol),
            '϶' => Ok(GreekandCoptic::GreekReversedLunateEpsilonSymbol),
            'Ϸ' => Ok(GreekandCoptic::GreekCapitalLetterSho),
            'ϸ' => Ok(GreekandCoptic::GreekSmallLetterSho),
            'Ϲ' => Ok(GreekandCoptic::GreekCapitalLunateSigmaSymbol),
            'Ϻ' => Ok(GreekandCoptic::GreekCapitalLetterSan),
            'ϻ' => Ok(GreekandCoptic::GreekSmallLetterSan),
            'ϼ' => Ok(GreekandCoptic::GreekRhoWithStrokeSymbol),
            'Ͻ' => Ok(GreekandCoptic::GreekCapitalReversedLunateSigmaSymbol),
            'Ͼ' => Ok(GreekandCoptic::GreekCapitalDottedLunateSigmaSymbol),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GreekandCoptic {
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

impl std::convert::TryFrom<u32> for GreekandCoptic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GreekandCoptic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GreekandCoptic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GreekandCoptic::GreekCapitalLetterHeta
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            GreekandCoptic::GreekCapitalLetterHeta => "greek capital letter heta",
            GreekandCoptic::GreekSmallLetterHeta => "greek small letter heta",
            GreekandCoptic::GreekCapitalLetterArchaicSampi => "greek capital letter archaic sampi",
            GreekandCoptic::GreekSmallLetterArchaicSampi => "greek small letter archaic sampi",
            GreekandCoptic::GreekNumeralSign => "greek numeral sign",
            GreekandCoptic::GreekLowerNumeralSign => "greek lower numeral sign",
            GreekandCoptic::GreekCapitalLetterPamphylianDigamma => "greek capital letter pamphylian digamma",
            GreekandCoptic::GreekSmallLetterPamphylianDigamma => "greek small letter pamphylian digamma",
            GreekandCoptic::GreekYpogegrammeni => "greek ypogegrammeni",
            GreekandCoptic::GreekSmallReversedLunateSigmaSymbol => "greek small reversed lunate sigma symbol",
            GreekandCoptic::GreekSmallDottedLunateSigmaSymbol => "greek small dotted lunate sigma symbol",
            GreekandCoptic::GreekSmallReversedDottedLunateSigmaSymbol => "greek small reversed dotted lunate sigma symbol",
            GreekandCoptic::GreekQuestionMark => "greek question mark",
            GreekandCoptic::GreekCapitalLetterYot => "greek capital letter yot",
            GreekandCoptic::GreekTonos => "greek tonos",
            GreekandCoptic::GreekDialytikaTonos => "greek dialytika tonos",
            GreekandCoptic::GreekCapitalLetterAlphaWithTonos => "greek capital letter alpha with tonos",
            GreekandCoptic::GreekAnoTeleia => "greek ano teleia",
            GreekandCoptic::GreekCapitalLetterEpsilonWithTonos => "greek capital letter epsilon with tonos",
            GreekandCoptic::GreekCapitalLetterEtaWithTonos => "greek capital letter eta with tonos",
            GreekandCoptic::GreekCapitalLetterIotaWithTonos => "greek capital letter iota with tonos",
            GreekandCoptic::GreekCapitalLetterOmicronWithTonos => "greek capital letter omicron with tonos",
            GreekandCoptic::GreekCapitalLetterUpsilonWithTonos => "greek capital letter upsilon with tonos",
            GreekandCoptic::GreekCapitalLetterOmegaWithTonos => "greek capital letter omega with tonos",
            GreekandCoptic::GreekSmallLetterIotaWithDialytikaAndTonos => "greek small letter iota with dialytika and tonos",
            GreekandCoptic::GreekCapitalLetterAlpha => "greek capital letter alpha",
            GreekandCoptic::GreekCapitalLetterBeta => "greek capital letter beta",
            GreekandCoptic::GreekCapitalLetterGamma => "greek capital letter gamma",
            GreekandCoptic::GreekCapitalLetterDelta => "greek capital letter delta",
            GreekandCoptic::GreekCapitalLetterEpsilon => "greek capital letter epsilon",
            GreekandCoptic::GreekCapitalLetterZeta => "greek capital letter zeta",
            GreekandCoptic::GreekCapitalLetterEta => "greek capital letter eta",
            GreekandCoptic::GreekCapitalLetterTheta => "greek capital letter theta",
            GreekandCoptic::GreekCapitalLetterIota => "greek capital letter iota",
            GreekandCoptic::GreekCapitalLetterKappa => "greek capital letter kappa",
            GreekandCoptic::GreekCapitalLetterLamda => "greek capital letter lamda",
            GreekandCoptic::GreekCapitalLetterMu => "greek capital letter mu",
            GreekandCoptic::GreekCapitalLetterNu => "greek capital letter nu",
            GreekandCoptic::GreekCapitalLetterXi => "greek capital letter xi",
            GreekandCoptic::GreekCapitalLetterOmicron => "greek capital letter omicron",
            GreekandCoptic::GreekCapitalLetterPi => "greek capital letter pi",
            GreekandCoptic::GreekCapitalLetterRho => "greek capital letter rho",
            GreekandCoptic::GreekCapitalLetterSigma => "greek capital letter sigma",
            GreekandCoptic::GreekCapitalLetterTau => "greek capital letter tau",
            GreekandCoptic::GreekCapitalLetterUpsilon => "greek capital letter upsilon",
            GreekandCoptic::GreekCapitalLetterPhi => "greek capital letter phi",
            GreekandCoptic::GreekCapitalLetterChi => "greek capital letter chi",
            GreekandCoptic::GreekCapitalLetterPsi => "greek capital letter psi",
            GreekandCoptic::GreekCapitalLetterOmega => "greek capital letter omega",
            GreekandCoptic::GreekCapitalLetterIotaWithDialytika => "greek capital letter iota with dialytika",
            GreekandCoptic::GreekCapitalLetterUpsilonWithDialytika => "greek capital letter upsilon with dialytika",
            GreekandCoptic::GreekSmallLetterAlphaWithTonos => "greek small letter alpha with tonos",
            GreekandCoptic::GreekSmallLetterEpsilonWithTonos => "greek small letter epsilon with tonos",
            GreekandCoptic::GreekSmallLetterEtaWithTonos => "greek small letter eta with tonos",
            GreekandCoptic::GreekSmallLetterIotaWithTonos => "greek small letter iota with tonos",
            GreekandCoptic::GreekSmallLetterUpsilonWithDialytikaAndTonos => "greek small letter upsilon with dialytika and tonos",
            GreekandCoptic::GreekSmallLetterAlpha => "greek small letter alpha",
            GreekandCoptic::GreekSmallLetterBeta => "greek small letter beta",
            GreekandCoptic::GreekSmallLetterGamma => "greek small letter gamma",
            GreekandCoptic::GreekSmallLetterDelta => "greek small letter delta",
            GreekandCoptic::GreekSmallLetterEpsilon => "greek small letter epsilon",
            GreekandCoptic::GreekSmallLetterZeta => "greek small letter zeta",
            GreekandCoptic::GreekSmallLetterEta => "greek small letter eta",
            GreekandCoptic::GreekSmallLetterTheta => "greek small letter theta",
            GreekandCoptic::GreekSmallLetterIota => "greek small letter iota",
            GreekandCoptic::GreekSmallLetterKappa => "greek small letter kappa",
            GreekandCoptic::GreekSmallLetterLamda => "greek small letter lamda",
            GreekandCoptic::GreekSmallLetterMu => "greek small letter mu",
            GreekandCoptic::GreekSmallLetterNu => "greek small letter nu",
            GreekandCoptic::GreekSmallLetterXi => "greek small letter xi",
            GreekandCoptic::GreekSmallLetterOmicron => "greek small letter omicron",
            GreekandCoptic::GreekSmallLetterPi => "greek small letter pi",
            GreekandCoptic::GreekSmallLetterRho => "greek small letter rho",
            GreekandCoptic::GreekSmallLetterFinalSigma => "greek small letter final sigma",
            GreekandCoptic::GreekSmallLetterSigma => "greek small letter sigma",
            GreekandCoptic::GreekSmallLetterTau => "greek small letter tau",
            GreekandCoptic::GreekSmallLetterUpsilon => "greek small letter upsilon",
            GreekandCoptic::GreekSmallLetterPhi => "greek small letter phi",
            GreekandCoptic::GreekSmallLetterChi => "greek small letter chi",
            GreekandCoptic::GreekSmallLetterPsi => "greek small letter psi",
            GreekandCoptic::GreekSmallLetterOmega => "greek small letter omega",
            GreekandCoptic::GreekSmallLetterIotaWithDialytika => "greek small letter iota with dialytika",
            GreekandCoptic::GreekSmallLetterUpsilonWithDialytika => "greek small letter upsilon with dialytika",
            GreekandCoptic::GreekSmallLetterOmicronWithTonos => "greek small letter omicron with tonos",
            GreekandCoptic::GreekSmallLetterUpsilonWithTonos => "greek small letter upsilon with tonos",
            GreekandCoptic::GreekSmallLetterOmegaWithTonos => "greek small letter omega with tonos",
            GreekandCoptic::GreekCapitalKaiSymbol => "greek capital kai symbol",
            GreekandCoptic::GreekBetaSymbol => "greek beta symbol",
            GreekandCoptic::GreekThetaSymbol => "greek theta symbol",
            GreekandCoptic::GreekUpsilonWithHookSymbol => "greek upsilon with hook symbol",
            GreekandCoptic::GreekUpsilonWithAcuteAndHookSymbol => "greek upsilon with acute and hook symbol",
            GreekandCoptic::GreekUpsilonWithDiaeresisAndHookSymbol => "greek upsilon with diaeresis and hook symbol",
            GreekandCoptic::GreekPhiSymbol => "greek phi symbol",
            GreekandCoptic::GreekPiSymbol => "greek pi symbol",
            GreekandCoptic::GreekKaiSymbol => "greek kai symbol",
            GreekandCoptic::GreekLetterArchaicKoppa => "greek letter archaic koppa",
            GreekandCoptic::GreekSmallLetterArchaicKoppa => "greek small letter archaic koppa",
            GreekandCoptic::GreekLetterStigma => "greek letter stigma",
            GreekandCoptic::GreekSmallLetterStigma => "greek small letter stigma",
            GreekandCoptic::GreekLetterDigamma => "greek letter digamma",
            GreekandCoptic::GreekSmallLetterDigamma => "greek small letter digamma",
            GreekandCoptic::GreekLetterKoppa => "greek letter koppa",
            GreekandCoptic::GreekSmallLetterKoppa => "greek small letter koppa",
            GreekandCoptic::GreekLetterSampi => "greek letter sampi",
            GreekandCoptic::GreekSmallLetterSampi => "greek small letter sampi",
            GreekandCoptic::CopticCapitalLetterShei => "coptic capital letter shei",
            GreekandCoptic::CopticSmallLetterShei => "coptic small letter shei",
            GreekandCoptic::CopticCapitalLetterFei => "coptic capital letter fei",
            GreekandCoptic::CopticSmallLetterFei => "coptic small letter fei",
            GreekandCoptic::CopticCapitalLetterKhei => "coptic capital letter khei",
            GreekandCoptic::CopticSmallLetterKhei => "coptic small letter khei",
            GreekandCoptic::CopticCapitalLetterHori => "coptic capital letter hori",
            GreekandCoptic::CopticSmallLetterHori => "coptic small letter hori",
            GreekandCoptic::CopticCapitalLetterGangia => "coptic capital letter gangia",
            GreekandCoptic::CopticSmallLetterGangia => "coptic small letter gangia",
            GreekandCoptic::CopticCapitalLetterShima => "coptic capital letter shima",
            GreekandCoptic::CopticSmallLetterShima => "coptic small letter shima",
            GreekandCoptic::CopticCapitalLetterDei => "coptic capital letter dei",
            GreekandCoptic::CopticSmallLetterDei => "coptic small letter dei",
            GreekandCoptic::GreekKappaSymbol => "greek kappa symbol",
            GreekandCoptic::GreekRhoSymbol => "greek rho symbol",
            GreekandCoptic::GreekLunateSigmaSymbol => "greek lunate sigma symbol",
            GreekandCoptic::GreekLetterYot => "greek letter yot",
            GreekandCoptic::GreekCapitalThetaSymbol => "greek capital theta symbol",
            GreekandCoptic::GreekLunateEpsilonSymbol => "greek lunate epsilon symbol",
            GreekandCoptic::GreekReversedLunateEpsilonSymbol => "greek reversed lunate epsilon symbol",
            GreekandCoptic::GreekCapitalLetterSho => "greek capital letter sho",
            GreekandCoptic::GreekSmallLetterSho => "greek small letter sho",
            GreekandCoptic::GreekCapitalLunateSigmaSymbol => "greek capital lunate sigma symbol",
            GreekandCoptic::GreekCapitalLetterSan => "greek capital letter san",
            GreekandCoptic::GreekSmallLetterSan => "greek small letter san",
            GreekandCoptic::GreekRhoWithStrokeSymbol => "greek rho with stroke symbol",
            GreekandCoptic::GreekCapitalReversedLunateSigmaSymbol => "greek capital reversed lunate sigma symbol",
            GreekandCoptic::GreekCapitalDottedLunateSigmaSymbol => "greek capital dotted lunate sigma symbol",
        }
    }
}
