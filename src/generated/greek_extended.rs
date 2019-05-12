
/// An enum to represent all characters in the GreekExtended block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GreekExtended {
    /// \u{1f00}: 'ἀ'
    GreekSmallLetterAlphaWithPsili,
    /// \u{1f01}: 'ἁ'
    GreekSmallLetterAlphaWithDasia,
    /// \u{1f02}: 'ἂ'
    GreekSmallLetterAlphaWithPsiliAndVaria,
    /// \u{1f03}: 'ἃ'
    GreekSmallLetterAlphaWithDasiaAndVaria,
    /// \u{1f04}: 'ἄ'
    GreekSmallLetterAlphaWithPsiliAndOxia,
    /// \u{1f05}: 'ἅ'
    GreekSmallLetterAlphaWithDasiaAndOxia,
    /// \u{1f06}: 'ἆ'
    GreekSmallLetterAlphaWithPsiliAndPerispomeni,
    /// \u{1f07}: 'ἇ'
    GreekSmallLetterAlphaWithDasiaAndPerispomeni,
    /// \u{1f08}: 'Ἀ'
    GreekCapitalLetterAlphaWithPsili,
    /// \u{1f09}: 'Ἁ'
    GreekCapitalLetterAlphaWithDasia,
    /// \u{1f0a}: 'Ἂ'
    GreekCapitalLetterAlphaWithPsiliAndVaria,
    /// \u{1f0b}: 'Ἃ'
    GreekCapitalLetterAlphaWithDasiaAndVaria,
    /// \u{1f0c}: 'Ἄ'
    GreekCapitalLetterAlphaWithPsiliAndOxia,
    /// \u{1f0d}: 'Ἅ'
    GreekCapitalLetterAlphaWithDasiaAndOxia,
    /// \u{1f0e}: 'Ἆ'
    GreekCapitalLetterAlphaWithPsiliAndPerispomeni,
    /// \u{1f0f}: 'Ἇ'
    GreekCapitalLetterAlphaWithDasiaAndPerispomeni,
    /// \u{1f10}: 'ἐ'
    GreekSmallLetterEpsilonWithPsili,
    /// \u{1f11}: 'ἑ'
    GreekSmallLetterEpsilonWithDasia,
    /// \u{1f12}: 'ἒ'
    GreekSmallLetterEpsilonWithPsiliAndVaria,
    /// \u{1f13}: 'ἓ'
    GreekSmallLetterEpsilonWithDasiaAndVaria,
    /// \u{1f14}: 'ἔ'
    GreekSmallLetterEpsilonWithPsiliAndOxia,
    /// \u{1f15}: 'ἕ'
    GreekSmallLetterEpsilonWithDasiaAndOxia,
    /// \u{1f18}: 'Ἐ'
    GreekCapitalLetterEpsilonWithPsili,
    /// \u{1f19}: 'Ἑ'
    GreekCapitalLetterEpsilonWithDasia,
    /// \u{1f1a}: 'Ἒ'
    GreekCapitalLetterEpsilonWithPsiliAndVaria,
    /// \u{1f1b}: 'Ἓ'
    GreekCapitalLetterEpsilonWithDasiaAndVaria,
    /// \u{1f1c}: 'Ἔ'
    GreekCapitalLetterEpsilonWithPsiliAndOxia,
    /// \u{1f1d}: 'Ἕ'
    GreekCapitalLetterEpsilonWithDasiaAndOxia,
    /// \u{1f20}: 'ἠ'
    GreekSmallLetterEtaWithPsili,
    /// \u{1f21}: 'ἡ'
    GreekSmallLetterEtaWithDasia,
    /// \u{1f22}: 'ἢ'
    GreekSmallLetterEtaWithPsiliAndVaria,
    /// \u{1f23}: 'ἣ'
    GreekSmallLetterEtaWithDasiaAndVaria,
    /// \u{1f24}: 'ἤ'
    GreekSmallLetterEtaWithPsiliAndOxia,
    /// \u{1f25}: 'ἥ'
    GreekSmallLetterEtaWithDasiaAndOxia,
    /// \u{1f26}: 'ἦ'
    GreekSmallLetterEtaWithPsiliAndPerispomeni,
    /// \u{1f27}: 'ἧ'
    GreekSmallLetterEtaWithDasiaAndPerispomeni,
    /// \u{1f28}: 'Ἠ'
    GreekCapitalLetterEtaWithPsili,
    /// \u{1f29}: 'Ἡ'
    GreekCapitalLetterEtaWithDasia,
    /// \u{1f2a}: 'Ἢ'
    GreekCapitalLetterEtaWithPsiliAndVaria,
    /// \u{1f2b}: 'Ἣ'
    GreekCapitalLetterEtaWithDasiaAndVaria,
    /// \u{1f2c}: 'Ἤ'
    GreekCapitalLetterEtaWithPsiliAndOxia,
    /// \u{1f2d}: 'Ἥ'
    GreekCapitalLetterEtaWithDasiaAndOxia,
    /// \u{1f2e}: 'Ἦ'
    GreekCapitalLetterEtaWithPsiliAndPerispomeni,
    /// \u{1f2f}: 'Ἧ'
    GreekCapitalLetterEtaWithDasiaAndPerispomeni,
    /// \u{1f30}: 'ἰ'
    GreekSmallLetterIotaWithPsili,
    /// \u{1f31}: 'ἱ'
    GreekSmallLetterIotaWithDasia,
    /// \u{1f32}: 'ἲ'
    GreekSmallLetterIotaWithPsiliAndVaria,
    /// \u{1f33}: 'ἳ'
    GreekSmallLetterIotaWithDasiaAndVaria,
    /// \u{1f34}: 'ἴ'
    GreekSmallLetterIotaWithPsiliAndOxia,
    /// \u{1f35}: 'ἵ'
    GreekSmallLetterIotaWithDasiaAndOxia,
    /// \u{1f36}: 'ἶ'
    GreekSmallLetterIotaWithPsiliAndPerispomeni,
    /// \u{1f37}: 'ἷ'
    GreekSmallLetterIotaWithDasiaAndPerispomeni,
    /// \u{1f38}: 'Ἰ'
    GreekCapitalLetterIotaWithPsili,
    /// \u{1f39}: 'Ἱ'
    GreekCapitalLetterIotaWithDasia,
    /// \u{1f3a}: 'Ἲ'
    GreekCapitalLetterIotaWithPsiliAndVaria,
    /// \u{1f3b}: 'Ἳ'
    GreekCapitalLetterIotaWithDasiaAndVaria,
    /// \u{1f3c}: 'Ἴ'
    GreekCapitalLetterIotaWithPsiliAndOxia,
    /// \u{1f3d}: 'Ἵ'
    GreekCapitalLetterIotaWithDasiaAndOxia,
    /// \u{1f3e}: 'Ἶ'
    GreekCapitalLetterIotaWithPsiliAndPerispomeni,
    /// \u{1f3f}: 'Ἷ'
    GreekCapitalLetterIotaWithDasiaAndPerispomeni,
    /// \u{1f40}: 'ὀ'
    GreekSmallLetterOmicronWithPsili,
    /// \u{1f41}: 'ὁ'
    GreekSmallLetterOmicronWithDasia,
    /// \u{1f42}: 'ὂ'
    GreekSmallLetterOmicronWithPsiliAndVaria,
    /// \u{1f43}: 'ὃ'
    GreekSmallLetterOmicronWithDasiaAndVaria,
    /// \u{1f44}: 'ὄ'
    GreekSmallLetterOmicronWithPsiliAndOxia,
    /// \u{1f45}: 'ὅ'
    GreekSmallLetterOmicronWithDasiaAndOxia,
    /// \u{1f48}: 'Ὀ'
    GreekCapitalLetterOmicronWithPsili,
    /// \u{1f49}: 'Ὁ'
    GreekCapitalLetterOmicronWithDasia,
    /// \u{1f4a}: 'Ὂ'
    GreekCapitalLetterOmicronWithPsiliAndVaria,
    /// \u{1f4b}: 'Ὃ'
    GreekCapitalLetterOmicronWithDasiaAndVaria,
    /// \u{1f4c}: 'Ὄ'
    GreekCapitalLetterOmicronWithPsiliAndOxia,
    /// \u{1f4d}: 'Ὅ'
    GreekCapitalLetterOmicronWithDasiaAndOxia,
    /// \u{1f50}: 'ὐ'
    GreekSmallLetterUpsilonWithPsili,
    /// \u{1f51}: 'ὑ'
    GreekSmallLetterUpsilonWithDasia,
    /// \u{1f52}: 'ὒ'
    GreekSmallLetterUpsilonWithPsiliAndVaria,
    /// \u{1f53}: 'ὓ'
    GreekSmallLetterUpsilonWithDasiaAndVaria,
    /// \u{1f54}: 'ὔ'
    GreekSmallLetterUpsilonWithPsiliAndOxia,
    /// \u{1f55}: 'ὕ'
    GreekSmallLetterUpsilonWithDasiaAndOxia,
    /// \u{1f56}: 'ὖ'
    GreekSmallLetterUpsilonWithPsiliAndPerispomeni,
    /// \u{1f57}: 'ὗ'
    GreekSmallLetterUpsilonWithDasiaAndPerispomeni,
    /// \u{1f59}: 'Ὑ'
    GreekCapitalLetterUpsilonWithDasia,
    /// \u{1f5b}: 'Ὓ'
    GreekCapitalLetterUpsilonWithDasiaAndVaria,
    /// \u{1f5d}: 'Ὕ'
    GreekCapitalLetterUpsilonWithDasiaAndOxia,
    /// \u{1f5f}: 'Ὗ'
    GreekCapitalLetterUpsilonWithDasiaAndPerispomeni,
    /// \u{1f60}: 'ὠ'
    GreekSmallLetterOmegaWithPsili,
    /// \u{1f61}: 'ὡ'
    GreekSmallLetterOmegaWithDasia,
    /// \u{1f62}: 'ὢ'
    GreekSmallLetterOmegaWithPsiliAndVaria,
    /// \u{1f63}: 'ὣ'
    GreekSmallLetterOmegaWithDasiaAndVaria,
    /// \u{1f64}: 'ὤ'
    GreekSmallLetterOmegaWithPsiliAndOxia,
    /// \u{1f65}: 'ὥ'
    GreekSmallLetterOmegaWithDasiaAndOxia,
    /// \u{1f66}: 'ὦ'
    GreekSmallLetterOmegaWithPsiliAndPerispomeni,
    /// \u{1f67}: 'ὧ'
    GreekSmallLetterOmegaWithDasiaAndPerispomeni,
    /// \u{1f68}: 'Ὠ'
    GreekCapitalLetterOmegaWithPsili,
    /// \u{1f69}: 'Ὡ'
    GreekCapitalLetterOmegaWithDasia,
    /// \u{1f6a}: 'Ὢ'
    GreekCapitalLetterOmegaWithPsiliAndVaria,
    /// \u{1f6b}: 'Ὣ'
    GreekCapitalLetterOmegaWithDasiaAndVaria,
    /// \u{1f6c}: 'Ὤ'
    GreekCapitalLetterOmegaWithPsiliAndOxia,
    /// \u{1f6d}: 'Ὥ'
    GreekCapitalLetterOmegaWithDasiaAndOxia,
    /// \u{1f6e}: 'Ὦ'
    GreekCapitalLetterOmegaWithPsiliAndPerispomeni,
    /// \u{1f6f}: 'Ὧ'
    GreekCapitalLetterOmegaWithDasiaAndPerispomeni,
    /// \u{1f70}: 'ὰ'
    GreekSmallLetterAlphaWithVaria,
    /// \u{1f71}: 'ά'
    GreekSmallLetterAlphaWithOxia,
    /// \u{1f72}: 'ὲ'
    GreekSmallLetterEpsilonWithVaria,
    /// \u{1f73}: 'έ'
    GreekSmallLetterEpsilonWithOxia,
    /// \u{1f74}: 'ὴ'
    GreekSmallLetterEtaWithVaria,
    /// \u{1f75}: 'ή'
    GreekSmallLetterEtaWithOxia,
    /// \u{1f76}: 'ὶ'
    GreekSmallLetterIotaWithVaria,
    /// \u{1f77}: 'ί'
    GreekSmallLetterIotaWithOxia,
    /// \u{1f78}: 'ὸ'
    GreekSmallLetterOmicronWithVaria,
    /// \u{1f79}: 'ό'
    GreekSmallLetterOmicronWithOxia,
    /// \u{1f7a}: 'ὺ'
    GreekSmallLetterUpsilonWithVaria,
    /// \u{1f7b}: 'ύ'
    GreekSmallLetterUpsilonWithOxia,
    /// \u{1f7c}: 'ὼ'
    GreekSmallLetterOmegaWithVaria,
    /// \u{1f7d}: 'ώ'
    GreekSmallLetterOmegaWithOxia,
    /// \u{1f80}: 'ᾀ'
    GreekSmallLetterAlphaWithPsiliAndYpogegrammeni,
    /// \u{1f81}: 'ᾁ'
    GreekSmallLetterAlphaWithDasiaAndYpogegrammeni,
    /// \u{1f82}: 'ᾂ'
    GreekSmallLetterAlphaWithPsiliAndVariaAndYpogegrammeni,
    /// \u{1f83}: 'ᾃ'
    GreekSmallLetterAlphaWithDasiaAndVariaAndYpogegrammeni,
    /// \u{1f84}: 'ᾄ'
    GreekSmallLetterAlphaWithPsiliAndOxiaAndYpogegrammeni,
    /// \u{1f85}: 'ᾅ'
    GreekSmallLetterAlphaWithDasiaAndOxiaAndYpogegrammeni,
    /// \u{1f86}: 'ᾆ'
    GreekSmallLetterAlphaWithPsiliAndPerispomeniAndYpogegrammeni,
    /// \u{1f87}: 'ᾇ'
    GreekSmallLetterAlphaWithDasiaAndPerispomeniAndYpogegrammeni,
    /// \u{1f88}: 'ᾈ'
    GreekCapitalLetterAlphaWithPsiliAndProsgegrammeni,
    /// \u{1f89}: 'ᾉ'
    GreekCapitalLetterAlphaWithDasiaAndProsgegrammeni,
    /// \u{1f8a}: 'ᾊ'
    GreekCapitalLetterAlphaWithPsiliAndVariaAndProsgegrammeni,
    /// \u{1f8b}: 'ᾋ'
    GreekCapitalLetterAlphaWithDasiaAndVariaAndProsgegrammeni,
    /// \u{1f8c}: 'ᾌ'
    GreekCapitalLetterAlphaWithPsiliAndOxiaAndProsgegrammeni,
    /// \u{1f8d}: 'ᾍ'
    GreekCapitalLetterAlphaWithDasiaAndOxiaAndProsgegrammeni,
    /// \u{1f8e}: 'ᾎ'
    GreekCapitalLetterAlphaWithPsiliAndPerispomeniAndProsgegrammeni,
    /// \u{1f8f}: 'ᾏ'
    GreekCapitalLetterAlphaWithDasiaAndPerispomeniAndProsgegrammeni,
    /// \u{1f90}: 'ᾐ'
    GreekSmallLetterEtaWithPsiliAndYpogegrammeni,
    /// \u{1f91}: 'ᾑ'
    GreekSmallLetterEtaWithDasiaAndYpogegrammeni,
    /// \u{1f92}: 'ᾒ'
    GreekSmallLetterEtaWithPsiliAndVariaAndYpogegrammeni,
    /// \u{1f93}: 'ᾓ'
    GreekSmallLetterEtaWithDasiaAndVariaAndYpogegrammeni,
    /// \u{1f94}: 'ᾔ'
    GreekSmallLetterEtaWithPsiliAndOxiaAndYpogegrammeni,
    /// \u{1f95}: 'ᾕ'
    GreekSmallLetterEtaWithDasiaAndOxiaAndYpogegrammeni,
    /// \u{1f96}: 'ᾖ'
    GreekSmallLetterEtaWithPsiliAndPerispomeniAndYpogegrammeni,
    /// \u{1f97}: 'ᾗ'
    GreekSmallLetterEtaWithDasiaAndPerispomeniAndYpogegrammeni,
    /// \u{1f98}: 'ᾘ'
    GreekCapitalLetterEtaWithPsiliAndProsgegrammeni,
    /// \u{1f99}: 'ᾙ'
    GreekCapitalLetterEtaWithDasiaAndProsgegrammeni,
    /// \u{1f9a}: 'ᾚ'
    GreekCapitalLetterEtaWithPsiliAndVariaAndProsgegrammeni,
    /// \u{1f9b}: 'ᾛ'
    GreekCapitalLetterEtaWithDasiaAndVariaAndProsgegrammeni,
    /// \u{1f9c}: 'ᾜ'
    GreekCapitalLetterEtaWithPsiliAndOxiaAndProsgegrammeni,
    /// \u{1f9d}: 'ᾝ'
    GreekCapitalLetterEtaWithDasiaAndOxiaAndProsgegrammeni,
    /// \u{1f9e}: 'ᾞ'
    GreekCapitalLetterEtaWithPsiliAndPerispomeniAndProsgegrammeni,
    /// \u{1f9f}: 'ᾟ'
    GreekCapitalLetterEtaWithDasiaAndPerispomeniAndProsgegrammeni,
    /// \u{1fa0}: 'ᾠ'
    GreekSmallLetterOmegaWithPsiliAndYpogegrammeni,
    /// \u{1fa1}: 'ᾡ'
    GreekSmallLetterOmegaWithDasiaAndYpogegrammeni,
    /// \u{1fa2}: 'ᾢ'
    GreekSmallLetterOmegaWithPsiliAndVariaAndYpogegrammeni,
    /// \u{1fa3}: 'ᾣ'
    GreekSmallLetterOmegaWithDasiaAndVariaAndYpogegrammeni,
    /// \u{1fa4}: 'ᾤ'
    GreekSmallLetterOmegaWithPsiliAndOxiaAndYpogegrammeni,
    /// \u{1fa5}: 'ᾥ'
    GreekSmallLetterOmegaWithDasiaAndOxiaAndYpogegrammeni,
    /// \u{1fa6}: 'ᾦ'
    GreekSmallLetterOmegaWithPsiliAndPerispomeniAndYpogegrammeni,
    /// \u{1fa7}: 'ᾧ'
    GreekSmallLetterOmegaWithDasiaAndPerispomeniAndYpogegrammeni,
    /// \u{1fa8}: 'ᾨ'
    GreekCapitalLetterOmegaWithPsiliAndProsgegrammeni,
    /// \u{1fa9}: 'ᾩ'
    GreekCapitalLetterOmegaWithDasiaAndProsgegrammeni,
    /// \u{1faa}: 'ᾪ'
    GreekCapitalLetterOmegaWithPsiliAndVariaAndProsgegrammeni,
    /// \u{1fab}: 'ᾫ'
    GreekCapitalLetterOmegaWithDasiaAndVariaAndProsgegrammeni,
    /// \u{1fac}: 'ᾬ'
    GreekCapitalLetterOmegaWithPsiliAndOxiaAndProsgegrammeni,
    /// \u{1fad}: 'ᾭ'
    GreekCapitalLetterOmegaWithDasiaAndOxiaAndProsgegrammeni,
    /// \u{1fae}: 'ᾮ'
    GreekCapitalLetterOmegaWithPsiliAndPerispomeniAndProsgegrammeni,
    /// \u{1faf}: 'ᾯ'
    GreekCapitalLetterOmegaWithDasiaAndPerispomeniAndProsgegrammeni,
    /// \u{1fb0}: 'ᾰ'
    GreekSmallLetterAlphaWithVrachy,
    /// \u{1fb1}: 'ᾱ'
    GreekSmallLetterAlphaWithMacron,
    /// \u{1fb2}: 'ᾲ'
    GreekSmallLetterAlphaWithVariaAndYpogegrammeni,
    /// \u{1fb3}: 'ᾳ'
    GreekSmallLetterAlphaWithYpogegrammeni,
    /// \u{1fb4}: 'ᾴ'
    GreekSmallLetterAlphaWithOxiaAndYpogegrammeni,
    /// \u{1fb6}: 'ᾶ'
    GreekSmallLetterAlphaWithPerispomeni,
    /// \u{1fb7}: 'ᾷ'
    GreekSmallLetterAlphaWithPerispomeniAndYpogegrammeni,
    /// \u{1fb8}: 'Ᾰ'
    GreekCapitalLetterAlphaWithVrachy,
    /// \u{1fb9}: 'Ᾱ'
    GreekCapitalLetterAlphaWithMacron,
    /// \u{1fba}: 'Ὰ'
    GreekCapitalLetterAlphaWithVaria,
    /// \u{1fbb}: 'Ά'
    GreekCapitalLetterAlphaWithOxia,
    /// \u{1fbc}: 'ᾼ'
    GreekCapitalLetterAlphaWithProsgegrammeni,
    /// \u{1fbd}: '᾽'
    GreekKoronis,
    /// \u{1fbe}: 'ι'
    GreekProsgegrammeni,
    /// \u{1fbf}: '᾿'
    GreekPsili,
    /// \u{1fc0}: '῀'
    GreekPerispomeni,
    /// \u{1fc1}: '῁'
    GreekDialytikaAndPerispomeni,
    /// \u{1fc2}: 'ῂ'
    GreekSmallLetterEtaWithVariaAndYpogegrammeni,
    /// \u{1fc3}: 'ῃ'
    GreekSmallLetterEtaWithYpogegrammeni,
    /// \u{1fc4}: 'ῄ'
    GreekSmallLetterEtaWithOxiaAndYpogegrammeni,
    /// \u{1fc6}: 'ῆ'
    GreekSmallLetterEtaWithPerispomeni,
    /// \u{1fc7}: 'ῇ'
    GreekSmallLetterEtaWithPerispomeniAndYpogegrammeni,
    /// \u{1fc8}: 'Ὲ'
    GreekCapitalLetterEpsilonWithVaria,
    /// \u{1fc9}: 'Έ'
    GreekCapitalLetterEpsilonWithOxia,
    /// \u{1fca}: 'Ὴ'
    GreekCapitalLetterEtaWithVaria,
    /// \u{1fcb}: 'Ή'
    GreekCapitalLetterEtaWithOxia,
    /// \u{1fcc}: 'ῌ'
    GreekCapitalLetterEtaWithProsgegrammeni,
    /// \u{1fcd}: '῍'
    GreekPsiliAndVaria,
    /// \u{1fce}: '῎'
    GreekPsiliAndOxia,
    /// \u{1fcf}: '῏'
    GreekPsiliAndPerispomeni,
    /// \u{1fd0}: 'ῐ'
    GreekSmallLetterIotaWithVrachy,
    /// \u{1fd1}: 'ῑ'
    GreekSmallLetterIotaWithMacron,
    /// \u{1fd2}: 'ῒ'
    GreekSmallLetterIotaWithDialytikaAndVaria,
    /// \u{1fd3}: 'ΐ'
    GreekSmallLetterIotaWithDialytikaAndOxia,
    /// \u{1fd6}: 'ῖ'
    GreekSmallLetterIotaWithPerispomeni,
    /// \u{1fd7}: 'ῗ'
    GreekSmallLetterIotaWithDialytikaAndPerispomeni,
    /// \u{1fd8}: 'Ῐ'
    GreekCapitalLetterIotaWithVrachy,
    /// \u{1fd9}: 'Ῑ'
    GreekCapitalLetterIotaWithMacron,
    /// \u{1fda}: 'Ὶ'
    GreekCapitalLetterIotaWithVaria,
    /// \u{1fdb}: 'Ί'
    GreekCapitalLetterIotaWithOxia,
    /// \u{1fdd}: '῝'
    GreekDasiaAndVaria,
    /// \u{1fde}: '῞'
    GreekDasiaAndOxia,
    /// \u{1fdf}: '῟'
    GreekDasiaAndPerispomeni,
    /// \u{1fe0}: 'ῠ'
    GreekSmallLetterUpsilonWithVrachy,
    /// \u{1fe1}: 'ῡ'
    GreekSmallLetterUpsilonWithMacron,
    /// \u{1fe2}: 'ῢ'
    GreekSmallLetterUpsilonWithDialytikaAndVaria,
    /// \u{1fe3}: 'ΰ'
    GreekSmallLetterUpsilonWithDialytikaAndOxia,
    /// \u{1fe4}: 'ῤ'
    GreekSmallLetterRhoWithPsili,
    /// \u{1fe5}: 'ῥ'
    GreekSmallLetterRhoWithDasia,
    /// \u{1fe6}: 'ῦ'
    GreekSmallLetterUpsilonWithPerispomeni,
    /// \u{1fe7}: 'ῧ'
    GreekSmallLetterUpsilonWithDialytikaAndPerispomeni,
    /// \u{1fe8}: 'Ῠ'
    GreekCapitalLetterUpsilonWithVrachy,
    /// \u{1fe9}: 'Ῡ'
    GreekCapitalLetterUpsilonWithMacron,
    /// \u{1fea}: 'Ὺ'
    GreekCapitalLetterUpsilonWithVaria,
    /// \u{1feb}: 'Ύ'
    GreekCapitalLetterUpsilonWithOxia,
    /// \u{1fec}: 'Ῥ'
    GreekCapitalLetterRhoWithDasia,
    /// \u{1fed}: '῭'
    GreekDialytikaAndVaria,
    /// \u{1fee}: '΅'
    GreekDialytikaAndOxia,
    /// \u{1fef}: '`'
    GreekVaria,
    /// \u{1ff2}: 'ῲ'
    GreekSmallLetterOmegaWithVariaAndYpogegrammeni,
    /// \u{1ff3}: 'ῳ'
    GreekSmallLetterOmegaWithYpogegrammeni,
    /// \u{1ff4}: 'ῴ'
    GreekSmallLetterOmegaWithOxiaAndYpogegrammeni,
    /// \u{1ff6}: 'ῶ'
    GreekSmallLetterOmegaWithPerispomeni,
    /// \u{1ff7}: 'ῷ'
    GreekSmallLetterOmegaWithPerispomeniAndYpogegrammeni,
    /// \u{1ff8}: 'Ὸ'
    GreekCapitalLetterOmicronWithVaria,
    /// \u{1ff9}: 'Ό'
    GreekCapitalLetterOmicronWithOxia,
    /// \u{1ffa}: 'Ὼ'
    GreekCapitalLetterOmegaWithVaria,
    /// \u{1ffb}: 'Ώ'
    GreekCapitalLetterOmegaWithOxia,
    /// \u{1ffc}: 'ῼ'
    GreekCapitalLetterOmegaWithProsgegrammeni,
    /// \u{1ffd}: '´'
    GreekOxia,
    /// \u{1ffe}: '῾'
    GreekDasia,
}

impl Into<char> for GreekExtended {
    fn into(self) -> char {
        match self {
            GreekExtended::GreekSmallLetterAlphaWithPsili => 'ἀ',
            GreekExtended::GreekSmallLetterAlphaWithDasia => 'ἁ',
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndVaria => 'ἂ',
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndVaria => 'ἃ',
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxia => 'ἄ',
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxia => 'ἅ',
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeni => 'ἆ',
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeni => 'ἇ',
            GreekExtended::GreekCapitalLetterAlphaWithPsili => 'Ἀ',
            GreekExtended::GreekCapitalLetterAlphaWithDasia => 'Ἁ',
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVaria => 'Ἂ',
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVaria => 'Ἃ',
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxia => 'Ἄ',
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxia => 'Ἅ',
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeni => 'Ἆ',
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeni => 'Ἇ',
            GreekExtended::GreekSmallLetterEpsilonWithPsili => 'ἐ',
            GreekExtended::GreekSmallLetterEpsilonWithDasia => 'ἑ',
            GreekExtended::GreekSmallLetterEpsilonWithPsiliAndVaria => 'ἒ',
            GreekExtended::GreekSmallLetterEpsilonWithDasiaAndVaria => 'ἓ',
            GreekExtended::GreekSmallLetterEpsilonWithPsiliAndOxia => 'ἔ',
            GreekExtended::GreekSmallLetterEpsilonWithDasiaAndOxia => 'ἕ',
            GreekExtended::GreekCapitalLetterEpsilonWithPsili => 'Ἐ',
            GreekExtended::GreekCapitalLetterEpsilonWithDasia => 'Ἑ',
            GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndVaria => 'Ἒ',
            GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndVaria => 'Ἓ',
            GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndOxia => 'Ἔ',
            GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndOxia => 'Ἕ',
            GreekExtended::GreekSmallLetterEtaWithPsili => 'ἠ',
            GreekExtended::GreekSmallLetterEtaWithDasia => 'ἡ',
            GreekExtended::GreekSmallLetterEtaWithPsiliAndVaria => 'ἢ',
            GreekExtended::GreekSmallLetterEtaWithDasiaAndVaria => 'ἣ',
            GreekExtended::GreekSmallLetterEtaWithPsiliAndOxia => 'ἤ',
            GreekExtended::GreekSmallLetterEtaWithDasiaAndOxia => 'ἥ',
            GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeni => 'ἦ',
            GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeni => 'ἧ',
            GreekExtended::GreekCapitalLetterEtaWithPsili => 'Ἠ',
            GreekExtended::GreekCapitalLetterEtaWithDasia => 'Ἡ',
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndVaria => 'Ἢ',
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndVaria => 'Ἣ',
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxia => 'Ἤ',
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxia => 'Ἥ',
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeni => 'Ἦ',
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeni => 'Ἧ',
            GreekExtended::GreekSmallLetterIotaWithPsili => 'ἰ',
            GreekExtended::GreekSmallLetterIotaWithDasia => 'ἱ',
            GreekExtended::GreekSmallLetterIotaWithPsiliAndVaria => 'ἲ',
            GreekExtended::GreekSmallLetterIotaWithDasiaAndVaria => 'ἳ',
            GreekExtended::GreekSmallLetterIotaWithPsiliAndOxia => 'ἴ',
            GreekExtended::GreekSmallLetterIotaWithDasiaAndOxia => 'ἵ',
            GreekExtended::GreekSmallLetterIotaWithPsiliAndPerispomeni => 'ἶ',
            GreekExtended::GreekSmallLetterIotaWithDasiaAndPerispomeni => 'ἷ',
            GreekExtended::GreekCapitalLetterIotaWithPsili => 'Ἰ',
            GreekExtended::GreekCapitalLetterIotaWithDasia => 'Ἱ',
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndVaria => 'Ἲ',
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndVaria => 'Ἳ',
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndOxia => 'Ἴ',
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndOxia => 'Ἵ',
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndPerispomeni => 'Ἶ',
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndPerispomeni => 'Ἷ',
            GreekExtended::GreekSmallLetterOmicronWithPsili => 'ὀ',
            GreekExtended::GreekSmallLetterOmicronWithDasia => 'ὁ',
            GreekExtended::GreekSmallLetterOmicronWithPsiliAndVaria => 'ὂ',
            GreekExtended::GreekSmallLetterOmicronWithDasiaAndVaria => 'ὃ',
            GreekExtended::GreekSmallLetterOmicronWithPsiliAndOxia => 'ὄ',
            GreekExtended::GreekSmallLetterOmicronWithDasiaAndOxia => 'ὅ',
            GreekExtended::GreekCapitalLetterOmicronWithPsili => 'Ὀ',
            GreekExtended::GreekCapitalLetterOmicronWithDasia => 'Ὁ',
            GreekExtended::GreekCapitalLetterOmicronWithPsiliAndVaria => 'Ὂ',
            GreekExtended::GreekCapitalLetterOmicronWithDasiaAndVaria => 'Ὃ',
            GreekExtended::GreekCapitalLetterOmicronWithPsiliAndOxia => 'Ὄ',
            GreekExtended::GreekCapitalLetterOmicronWithDasiaAndOxia => 'Ὅ',
            GreekExtended::GreekSmallLetterUpsilonWithPsili => 'ὐ',
            GreekExtended::GreekSmallLetterUpsilonWithDasia => 'ὑ',
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndVaria => 'ὒ',
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndVaria => 'ὓ',
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndOxia => 'ὔ',
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndOxia => 'ὕ',
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndPerispomeni => 'ὖ',
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndPerispomeni => 'ὗ',
            GreekExtended::GreekCapitalLetterUpsilonWithDasia => 'Ὑ',
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndVaria => 'Ὓ',
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndOxia => 'Ὕ',
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndPerispomeni => 'Ὗ',
            GreekExtended::GreekSmallLetterOmegaWithPsili => 'ὠ',
            GreekExtended::GreekSmallLetterOmegaWithDasia => 'ὡ',
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndVaria => 'ὢ',
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndVaria => 'ὣ',
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxia => 'ὤ',
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxia => 'ὥ',
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeni => 'ὦ',
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeni => 'ὧ',
            GreekExtended::GreekCapitalLetterOmegaWithPsili => 'Ὠ',
            GreekExtended::GreekCapitalLetterOmegaWithDasia => 'Ὡ',
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVaria => 'Ὢ',
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVaria => 'Ὣ',
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxia => 'Ὤ',
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxia => 'Ὥ',
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeni => 'Ὦ',
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeni => 'Ὧ',
            GreekExtended::GreekSmallLetterAlphaWithVaria => 'ὰ',
            GreekExtended::GreekSmallLetterAlphaWithOxia => 'ά',
            GreekExtended::GreekSmallLetterEpsilonWithVaria => 'ὲ',
            GreekExtended::GreekSmallLetterEpsilonWithOxia => 'έ',
            GreekExtended::GreekSmallLetterEtaWithVaria => 'ὴ',
            GreekExtended::GreekSmallLetterEtaWithOxia => 'ή',
            GreekExtended::GreekSmallLetterIotaWithVaria => 'ὶ',
            GreekExtended::GreekSmallLetterIotaWithOxia => 'ί',
            GreekExtended::GreekSmallLetterOmicronWithVaria => 'ὸ',
            GreekExtended::GreekSmallLetterOmicronWithOxia => 'ό',
            GreekExtended::GreekSmallLetterUpsilonWithVaria => 'ὺ',
            GreekExtended::GreekSmallLetterUpsilonWithOxia => 'ύ',
            GreekExtended::GreekSmallLetterOmegaWithVaria => 'ὼ',
            GreekExtended::GreekSmallLetterOmegaWithOxia => 'ώ',
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndYpogegrammeni => 'ᾀ',
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndYpogegrammeni => 'ᾁ',
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndVariaAndYpogegrammeni => 'ᾂ',
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndVariaAndYpogegrammeni => 'ᾃ',
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxiaAndYpogegrammeni => 'ᾄ',
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxiaAndYpogegrammeni => 'ᾅ',
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeniAndYpogegrammeni => 'ᾆ',
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeniAndYpogegrammeni => 'ᾇ',
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndProsgegrammeni => 'ᾈ',
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndProsgegrammeni => 'ᾉ',
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVariaAndProsgegrammeni => 'ᾊ',
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVariaAndProsgegrammeni => 'ᾋ',
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxiaAndProsgegrammeni => 'ᾌ',
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxiaAndProsgegrammeni => 'ᾍ',
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeniAndProsgegrammeni => 'ᾎ',
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeniAndProsgegrammeni => 'ᾏ',
            GreekExtended::GreekSmallLetterEtaWithPsiliAndYpogegrammeni => 'ᾐ',
            GreekExtended::GreekSmallLetterEtaWithDasiaAndYpogegrammeni => 'ᾑ',
            GreekExtended::GreekSmallLetterEtaWithPsiliAndVariaAndYpogegrammeni => 'ᾒ',
            GreekExtended::GreekSmallLetterEtaWithDasiaAndVariaAndYpogegrammeni => 'ᾓ',
            GreekExtended::GreekSmallLetterEtaWithPsiliAndOxiaAndYpogegrammeni => 'ᾔ',
            GreekExtended::GreekSmallLetterEtaWithDasiaAndOxiaAndYpogegrammeni => 'ᾕ',
            GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeniAndYpogegrammeni => 'ᾖ',
            GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeniAndYpogegrammeni => 'ᾗ',
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndProsgegrammeni => 'ᾘ',
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndProsgegrammeni => 'ᾙ',
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndVariaAndProsgegrammeni => 'ᾚ',
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndVariaAndProsgegrammeni => 'ᾛ',
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxiaAndProsgegrammeni => 'ᾜ',
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxiaAndProsgegrammeni => 'ᾝ',
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeniAndProsgegrammeni => 'ᾞ',
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeniAndProsgegrammeni => 'ᾟ',
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndYpogegrammeni => 'ᾠ',
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndYpogegrammeni => 'ᾡ',
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndVariaAndYpogegrammeni => 'ᾢ',
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndVariaAndYpogegrammeni => 'ᾣ',
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxiaAndYpogegrammeni => 'ᾤ',
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxiaAndYpogegrammeni => 'ᾥ',
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeniAndYpogegrammeni => 'ᾦ',
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeniAndYpogegrammeni => 'ᾧ',
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndProsgegrammeni => 'ᾨ',
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndProsgegrammeni => 'ᾩ',
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVariaAndProsgegrammeni => 'ᾪ',
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVariaAndProsgegrammeni => 'ᾫ',
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxiaAndProsgegrammeni => 'ᾬ',
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxiaAndProsgegrammeni => 'ᾭ',
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeniAndProsgegrammeni => 'ᾮ',
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeniAndProsgegrammeni => 'ᾯ',
            GreekExtended::GreekSmallLetterAlphaWithVrachy => 'ᾰ',
            GreekExtended::GreekSmallLetterAlphaWithMacron => 'ᾱ',
            GreekExtended::GreekSmallLetterAlphaWithVariaAndYpogegrammeni => 'ᾲ',
            GreekExtended::GreekSmallLetterAlphaWithYpogegrammeni => 'ᾳ',
            GreekExtended::GreekSmallLetterAlphaWithOxiaAndYpogegrammeni => 'ᾴ',
            GreekExtended::GreekSmallLetterAlphaWithPerispomeni => 'ᾶ',
            GreekExtended::GreekSmallLetterAlphaWithPerispomeniAndYpogegrammeni => 'ᾷ',
            GreekExtended::GreekCapitalLetterAlphaWithVrachy => 'Ᾰ',
            GreekExtended::GreekCapitalLetterAlphaWithMacron => 'Ᾱ',
            GreekExtended::GreekCapitalLetterAlphaWithVaria => 'Ὰ',
            GreekExtended::GreekCapitalLetterAlphaWithOxia => 'Ά',
            GreekExtended::GreekCapitalLetterAlphaWithProsgegrammeni => 'ᾼ',
            GreekExtended::GreekKoronis => '᾽',
            GreekExtended::GreekProsgegrammeni => 'ι',
            GreekExtended::GreekPsili => '᾿',
            GreekExtended::GreekPerispomeni => '῀',
            GreekExtended::GreekDialytikaAndPerispomeni => '῁',
            GreekExtended::GreekSmallLetterEtaWithVariaAndYpogegrammeni => 'ῂ',
            GreekExtended::GreekSmallLetterEtaWithYpogegrammeni => 'ῃ',
            GreekExtended::GreekSmallLetterEtaWithOxiaAndYpogegrammeni => 'ῄ',
            GreekExtended::GreekSmallLetterEtaWithPerispomeni => 'ῆ',
            GreekExtended::GreekSmallLetterEtaWithPerispomeniAndYpogegrammeni => 'ῇ',
            GreekExtended::GreekCapitalLetterEpsilonWithVaria => 'Ὲ',
            GreekExtended::GreekCapitalLetterEpsilonWithOxia => 'Έ',
            GreekExtended::GreekCapitalLetterEtaWithVaria => 'Ὴ',
            GreekExtended::GreekCapitalLetterEtaWithOxia => 'Ή',
            GreekExtended::GreekCapitalLetterEtaWithProsgegrammeni => 'ῌ',
            GreekExtended::GreekPsiliAndVaria => '῍',
            GreekExtended::GreekPsiliAndOxia => '῎',
            GreekExtended::GreekPsiliAndPerispomeni => '῏',
            GreekExtended::GreekSmallLetterIotaWithVrachy => 'ῐ',
            GreekExtended::GreekSmallLetterIotaWithMacron => 'ῑ',
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndVaria => 'ῒ',
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndOxia => 'ΐ',
            GreekExtended::GreekSmallLetterIotaWithPerispomeni => 'ῖ',
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndPerispomeni => 'ῗ',
            GreekExtended::GreekCapitalLetterIotaWithVrachy => 'Ῐ',
            GreekExtended::GreekCapitalLetterIotaWithMacron => 'Ῑ',
            GreekExtended::GreekCapitalLetterIotaWithVaria => 'Ὶ',
            GreekExtended::GreekCapitalLetterIotaWithOxia => 'Ί',
            GreekExtended::GreekDasiaAndVaria => '῝',
            GreekExtended::GreekDasiaAndOxia => '῞',
            GreekExtended::GreekDasiaAndPerispomeni => '῟',
            GreekExtended::GreekSmallLetterUpsilonWithVrachy => 'ῠ',
            GreekExtended::GreekSmallLetterUpsilonWithMacron => 'ῡ',
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndVaria => 'ῢ',
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndOxia => 'ΰ',
            GreekExtended::GreekSmallLetterRhoWithPsili => 'ῤ',
            GreekExtended::GreekSmallLetterRhoWithDasia => 'ῥ',
            GreekExtended::GreekSmallLetterUpsilonWithPerispomeni => 'ῦ',
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndPerispomeni => 'ῧ',
            GreekExtended::GreekCapitalLetterUpsilonWithVrachy => 'Ῠ',
            GreekExtended::GreekCapitalLetterUpsilonWithMacron => 'Ῡ',
            GreekExtended::GreekCapitalLetterUpsilonWithVaria => 'Ὺ',
            GreekExtended::GreekCapitalLetterUpsilonWithOxia => 'Ύ',
            GreekExtended::GreekCapitalLetterRhoWithDasia => 'Ῥ',
            GreekExtended::GreekDialytikaAndVaria => '῭',
            GreekExtended::GreekDialytikaAndOxia => '΅',
            GreekExtended::GreekVaria => '`',
            GreekExtended::GreekSmallLetterOmegaWithVariaAndYpogegrammeni => 'ῲ',
            GreekExtended::GreekSmallLetterOmegaWithYpogegrammeni => 'ῳ',
            GreekExtended::GreekSmallLetterOmegaWithOxiaAndYpogegrammeni => 'ῴ',
            GreekExtended::GreekSmallLetterOmegaWithPerispomeni => 'ῶ',
            GreekExtended::GreekSmallLetterOmegaWithPerispomeniAndYpogegrammeni => 'ῷ',
            GreekExtended::GreekCapitalLetterOmicronWithVaria => 'Ὸ',
            GreekExtended::GreekCapitalLetterOmicronWithOxia => 'Ό',
            GreekExtended::GreekCapitalLetterOmegaWithVaria => 'Ὼ',
            GreekExtended::GreekCapitalLetterOmegaWithOxia => 'Ώ',
            GreekExtended::GreekCapitalLetterOmegaWithProsgegrammeni => 'ῼ',
            GreekExtended::GreekOxia => '´',
            GreekExtended::GreekDasia => '῾',
        }
    }
}

impl std::convert::TryFrom<char> for GreekExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ἀ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPsili),
            'ἁ' => Ok(GreekExtended::GreekSmallLetterAlphaWithDasia),
            'ἂ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndVaria),
            'ἃ' => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndVaria),
            'ἄ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxia),
            'ἅ' => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxia),
            'ἆ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeni),
            'ἇ' => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeni),
            'Ἀ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsili),
            'Ἁ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasia),
            'Ἂ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVaria),
            'Ἃ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVaria),
            'Ἄ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxia),
            'Ἅ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxia),
            'Ἆ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeni),
            'Ἇ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeni),
            'ἐ' => Ok(GreekExtended::GreekSmallLetterEpsilonWithPsili),
            'ἑ' => Ok(GreekExtended::GreekSmallLetterEpsilonWithDasia),
            'ἒ' => Ok(GreekExtended::GreekSmallLetterEpsilonWithPsiliAndVaria),
            'ἓ' => Ok(GreekExtended::GreekSmallLetterEpsilonWithDasiaAndVaria),
            'ἔ' => Ok(GreekExtended::GreekSmallLetterEpsilonWithPsiliAndOxia),
            'ἕ' => Ok(GreekExtended::GreekSmallLetterEpsilonWithDasiaAndOxia),
            'Ἐ' => Ok(GreekExtended::GreekCapitalLetterEpsilonWithPsili),
            'Ἑ' => Ok(GreekExtended::GreekCapitalLetterEpsilonWithDasia),
            'Ἒ' => Ok(GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndVaria),
            'Ἓ' => Ok(GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndVaria),
            'Ἔ' => Ok(GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndOxia),
            'Ἕ' => Ok(GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndOxia),
            'ἠ' => Ok(GreekExtended::GreekSmallLetterEtaWithPsili),
            'ἡ' => Ok(GreekExtended::GreekSmallLetterEtaWithDasia),
            'ἢ' => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndVaria),
            'ἣ' => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndVaria),
            'ἤ' => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndOxia),
            'ἥ' => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndOxia),
            'ἦ' => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeni),
            'ἧ' => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeni),
            'Ἠ' => Ok(GreekExtended::GreekCapitalLetterEtaWithPsili),
            'Ἡ' => Ok(GreekExtended::GreekCapitalLetterEtaWithDasia),
            'Ἢ' => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndVaria),
            'Ἣ' => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndVaria),
            'Ἤ' => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxia),
            'Ἥ' => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxia),
            'Ἦ' => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeni),
            'Ἧ' => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeni),
            'ἰ' => Ok(GreekExtended::GreekSmallLetterIotaWithPsili),
            'ἱ' => Ok(GreekExtended::GreekSmallLetterIotaWithDasia),
            'ἲ' => Ok(GreekExtended::GreekSmallLetterIotaWithPsiliAndVaria),
            'ἳ' => Ok(GreekExtended::GreekSmallLetterIotaWithDasiaAndVaria),
            'ἴ' => Ok(GreekExtended::GreekSmallLetterIotaWithPsiliAndOxia),
            'ἵ' => Ok(GreekExtended::GreekSmallLetterIotaWithDasiaAndOxia),
            'ἶ' => Ok(GreekExtended::GreekSmallLetterIotaWithPsiliAndPerispomeni),
            'ἷ' => Ok(GreekExtended::GreekSmallLetterIotaWithDasiaAndPerispomeni),
            'Ἰ' => Ok(GreekExtended::GreekCapitalLetterIotaWithPsili),
            'Ἱ' => Ok(GreekExtended::GreekCapitalLetterIotaWithDasia),
            'Ἲ' => Ok(GreekExtended::GreekCapitalLetterIotaWithPsiliAndVaria),
            'Ἳ' => Ok(GreekExtended::GreekCapitalLetterIotaWithDasiaAndVaria),
            'Ἴ' => Ok(GreekExtended::GreekCapitalLetterIotaWithPsiliAndOxia),
            'Ἵ' => Ok(GreekExtended::GreekCapitalLetterIotaWithDasiaAndOxia),
            'Ἶ' => Ok(GreekExtended::GreekCapitalLetterIotaWithPsiliAndPerispomeni),
            'Ἷ' => Ok(GreekExtended::GreekCapitalLetterIotaWithDasiaAndPerispomeni),
            'ὀ' => Ok(GreekExtended::GreekSmallLetterOmicronWithPsili),
            'ὁ' => Ok(GreekExtended::GreekSmallLetterOmicronWithDasia),
            'ὂ' => Ok(GreekExtended::GreekSmallLetterOmicronWithPsiliAndVaria),
            'ὃ' => Ok(GreekExtended::GreekSmallLetterOmicronWithDasiaAndVaria),
            'ὄ' => Ok(GreekExtended::GreekSmallLetterOmicronWithPsiliAndOxia),
            'ὅ' => Ok(GreekExtended::GreekSmallLetterOmicronWithDasiaAndOxia),
            'Ὀ' => Ok(GreekExtended::GreekCapitalLetterOmicronWithPsili),
            'Ὁ' => Ok(GreekExtended::GreekCapitalLetterOmicronWithDasia),
            'Ὂ' => Ok(GreekExtended::GreekCapitalLetterOmicronWithPsiliAndVaria),
            'Ὃ' => Ok(GreekExtended::GreekCapitalLetterOmicronWithDasiaAndVaria),
            'Ὄ' => Ok(GreekExtended::GreekCapitalLetterOmicronWithPsiliAndOxia),
            'Ὅ' => Ok(GreekExtended::GreekCapitalLetterOmicronWithDasiaAndOxia),
            'ὐ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithPsili),
            'ὑ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithDasia),
            'ὒ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithPsiliAndVaria),
            'ὓ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithDasiaAndVaria),
            'ὔ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithPsiliAndOxia),
            'ὕ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithDasiaAndOxia),
            'ὖ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithPsiliAndPerispomeni),
            'ὗ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithDasiaAndPerispomeni),
            'Ὑ' => Ok(GreekExtended::GreekCapitalLetterUpsilonWithDasia),
            'Ὓ' => Ok(GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndVaria),
            'Ὕ' => Ok(GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndOxia),
            'Ὗ' => Ok(GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndPerispomeni),
            'ὠ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPsili),
            'ὡ' => Ok(GreekExtended::GreekSmallLetterOmegaWithDasia),
            'ὢ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndVaria),
            'ὣ' => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndVaria),
            'ὤ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxia),
            'ὥ' => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxia),
            'ὦ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeni),
            'ὧ' => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeni),
            'Ὠ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsili),
            'Ὡ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasia),
            'Ὢ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVaria),
            'Ὣ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVaria),
            'Ὤ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxia),
            'Ὥ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxia),
            'Ὦ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeni),
            'Ὧ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeni),
            'ὰ' => Ok(GreekExtended::GreekSmallLetterAlphaWithVaria),
            'ά' => Ok(GreekExtended::GreekSmallLetterAlphaWithOxia),
            'ὲ' => Ok(GreekExtended::GreekSmallLetterEpsilonWithVaria),
            'έ' => Ok(GreekExtended::GreekSmallLetterEpsilonWithOxia),
            'ὴ' => Ok(GreekExtended::GreekSmallLetterEtaWithVaria),
            'ή' => Ok(GreekExtended::GreekSmallLetterEtaWithOxia),
            'ὶ' => Ok(GreekExtended::GreekSmallLetterIotaWithVaria),
            'ί' => Ok(GreekExtended::GreekSmallLetterIotaWithOxia),
            'ὸ' => Ok(GreekExtended::GreekSmallLetterOmicronWithVaria),
            'ό' => Ok(GreekExtended::GreekSmallLetterOmicronWithOxia),
            'ὺ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithVaria),
            'ύ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithOxia),
            'ὼ' => Ok(GreekExtended::GreekSmallLetterOmegaWithVaria),
            'ώ' => Ok(GreekExtended::GreekSmallLetterOmegaWithOxia),
            'ᾀ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndYpogegrammeni),
            'ᾁ' => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndYpogegrammeni),
            'ᾂ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndVariaAndYpogegrammeni),
            'ᾃ' => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndVariaAndYpogegrammeni),
            'ᾄ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxiaAndYpogegrammeni),
            'ᾅ' => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxiaAndYpogegrammeni),
            'ᾆ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeniAndYpogegrammeni),
            'ᾇ' => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeniAndYpogegrammeni),
            'ᾈ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndProsgegrammeni),
            'ᾉ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndProsgegrammeni),
            'ᾊ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVariaAndProsgegrammeni),
            'ᾋ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVariaAndProsgegrammeni),
            'ᾌ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxiaAndProsgegrammeni),
            'ᾍ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxiaAndProsgegrammeni),
            'ᾎ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeniAndProsgegrammeni),
            'ᾏ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeniAndProsgegrammeni),
            'ᾐ' => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndYpogegrammeni),
            'ᾑ' => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndYpogegrammeni),
            'ᾒ' => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndVariaAndYpogegrammeni),
            'ᾓ' => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndVariaAndYpogegrammeni),
            'ᾔ' => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndOxiaAndYpogegrammeni),
            'ᾕ' => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndOxiaAndYpogegrammeni),
            'ᾖ' => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeniAndYpogegrammeni),
            'ᾗ' => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeniAndYpogegrammeni),
            'ᾘ' => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndProsgegrammeni),
            'ᾙ' => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndProsgegrammeni),
            'ᾚ' => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndVariaAndProsgegrammeni),
            'ᾛ' => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndVariaAndProsgegrammeni),
            'ᾜ' => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxiaAndProsgegrammeni),
            'ᾝ' => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxiaAndProsgegrammeni),
            'ᾞ' => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeniAndProsgegrammeni),
            'ᾟ' => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeniAndProsgegrammeni),
            'ᾠ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndYpogegrammeni),
            'ᾡ' => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndYpogegrammeni),
            'ᾢ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndVariaAndYpogegrammeni),
            'ᾣ' => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndVariaAndYpogegrammeni),
            'ᾤ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxiaAndYpogegrammeni),
            'ᾥ' => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxiaAndYpogegrammeni),
            'ᾦ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeniAndYpogegrammeni),
            'ᾧ' => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeniAndYpogegrammeni),
            'ᾨ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndProsgegrammeni),
            'ᾩ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndProsgegrammeni),
            'ᾪ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVariaAndProsgegrammeni),
            'ᾫ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVariaAndProsgegrammeni),
            'ᾬ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxiaAndProsgegrammeni),
            'ᾭ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxiaAndProsgegrammeni),
            'ᾮ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeniAndProsgegrammeni),
            'ᾯ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeniAndProsgegrammeni),
            'ᾰ' => Ok(GreekExtended::GreekSmallLetterAlphaWithVrachy),
            'ᾱ' => Ok(GreekExtended::GreekSmallLetterAlphaWithMacron),
            'ᾲ' => Ok(GreekExtended::GreekSmallLetterAlphaWithVariaAndYpogegrammeni),
            'ᾳ' => Ok(GreekExtended::GreekSmallLetterAlphaWithYpogegrammeni),
            'ᾴ' => Ok(GreekExtended::GreekSmallLetterAlphaWithOxiaAndYpogegrammeni),
            'ᾶ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPerispomeni),
            'ᾷ' => Ok(GreekExtended::GreekSmallLetterAlphaWithPerispomeniAndYpogegrammeni),
            'Ᾰ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithVrachy),
            'Ᾱ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithMacron),
            'Ὰ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithVaria),
            'Ά' => Ok(GreekExtended::GreekCapitalLetterAlphaWithOxia),
            'ᾼ' => Ok(GreekExtended::GreekCapitalLetterAlphaWithProsgegrammeni),
            '᾽' => Ok(GreekExtended::GreekKoronis),
            'ι' => Ok(GreekExtended::GreekProsgegrammeni),
            '᾿' => Ok(GreekExtended::GreekPsili),
            '῀' => Ok(GreekExtended::GreekPerispomeni),
            '῁' => Ok(GreekExtended::GreekDialytikaAndPerispomeni),
            'ῂ' => Ok(GreekExtended::GreekSmallLetterEtaWithVariaAndYpogegrammeni),
            'ῃ' => Ok(GreekExtended::GreekSmallLetterEtaWithYpogegrammeni),
            'ῄ' => Ok(GreekExtended::GreekSmallLetterEtaWithOxiaAndYpogegrammeni),
            'ῆ' => Ok(GreekExtended::GreekSmallLetterEtaWithPerispomeni),
            'ῇ' => Ok(GreekExtended::GreekSmallLetterEtaWithPerispomeniAndYpogegrammeni),
            'Ὲ' => Ok(GreekExtended::GreekCapitalLetterEpsilonWithVaria),
            'Έ' => Ok(GreekExtended::GreekCapitalLetterEpsilonWithOxia),
            'Ὴ' => Ok(GreekExtended::GreekCapitalLetterEtaWithVaria),
            'Ή' => Ok(GreekExtended::GreekCapitalLetterEtaWithOxia),
            'ῌ' => Ok(GreekExtended::GreekCapitalLetterEtaWithProsgegrammeni),
            '῍' => Ok(GreekExtended::GreekPsiliAndVaria),
            '῎' => Ok(GreekExtended::GreekPsiliAndOxia),
            '῏' => Ok(GreekExtended::GreekPsiliAndPerispomeni),
            'ῐ' => Ok(GreekExtended::GreekSmallLetterIotaWithVrachy),
            'ῑ' => Ok(GreekExtended::GreekSmallLetterIotaWithMacron),
            'ῒ' => Ok(GreekExtended::GreekSmallLetterIotaWithDialytikaAndVaria),
            'ΐ' => Ok(GreekExtended::GreekSmallLetterIotaWithDialytikaAndOxia),
            'ῖ' => Ok(GreekExtended::GreekSmallLetterIotaWithPerispomeni),
            'ῗ' => Ok(GreekExtended::GreekSmallLetterIotaWithDialytikaAndPerispomeni),
            'Ῐ' => Ok(GreekExtended::GreekCapitalLetterIotaWithVrachy),
            'Ῑ' => Ok(GreekExtended::GreekCapitalLetterIotaWithMacron),
            'Ὶ' => Ok(GreekExtended::GreekCapitalLetterIotaWithVaria),
            'Ί' => Ok(GreekExtended::GreekCapitalLetterIotaWithOxia),
            '῝' => Ok(GreekExtended::GreekDasiaAndVaria),
            '῞' => Ok(GreekExtended::GreekDasiaAndOxia),
            '῟' => Ok(GreekExtended::GreekDasiaAndPerispomeni),
            'ῠ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithVrachy),
            'ῡ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithMacron),
            'ῢ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndVaria),
            'ΰ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndOxia),
            'ῤ' => Ok(GreekExtended::GreekSmallLetterRhoWithPsili),
            'ῥ' => Ok(GreekExtended::GreekSmallLetterRhoWithDasia),
            'ῦ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithPerispomeni),
            'ῧ' => Ok(GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndPerispomeni),
            'Ῠ' => Ok(GreekExtended::GreekCapitalLetterUpsilonWithVrachy),
            'Ῡ' => Ok(GreekExtended::GreekCapitalLetterUpsilonWithMacron),
            'Ὺ' => Ok(GreekExtended::GreekCapitalLetterUpsilonWithVaria),
            'Ύ' => Ok(GreekExtended::GreekCapitalLetterUpsilonWithOxia),
            'Ῥ' => Ok(GreekExtended::GreekCapitalLetterRhoWithDasia),
            '῭' => Ok(GreekExtended::GreekDialytikaAndVaria),
            '΅' => Ok(GreekExtended::GreekDialytikaAndOxia),
            '`' => Ok(GreekExtended::GreekVaria),
            'ῲ' => Ok(GreekExtended::GreekSmallLetterOmegaWithVariaAndYpogegrammeni),
            'ῳ' => Ok(GreekExtended::GreekSmallLetterOmegaWithYpogegrammeni),
            'ῴ' => Ok(GreekExtended::GreekSmallLetterOmegaWithOxiaAndYpogegrammeni),
            'ῶ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPerispomeni),
            'ῷ' => Ok(GreekExtended::GreekSmallLetterOmegaWithPerispomeniAndYpogegrammeni),
            'Ὸ' => Ok(GreekExtended::GreekCapitalLetterOmicronWithVaria),
            'Ό' => Ok(GreekExtended::GreekCapitalLetterOmicronWithOxia),
            'Ὼ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithVaria),
            'Ώ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithOxia),
            'ῼ' => Ok(GreekExtended::GreekCapitalLetterOmegaWithProsgegrammeni),
            '´' => Ok(GreekExtended::GreekOxia),
            '῾' => Ok(GreekExtended::GreekDasia),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GreekExtended {
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

impl std::convert::TryFrom<u32> for GreekExtended {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GreekExtended {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GreekExtended {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GreekExtended::GreekSmallLetterAlphaWithPsili
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            GreekExtended::GreekSmallLetterAlphaWithPsili => "greek small letter alpha with psili",
            GreekExtended::GreekSmallLetterAlphaWithDasia => "greek small letter alpha with dasia",
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndVaria => "greek small letter alpha with psili and varia",
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndVaria => "greek small letter alpha with dasia and varia",
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxia => "greek small letter alpha with psili and oxia",
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxia => "greek small letter alpha with dasia and oxia",
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeni => "greek small letter alpha with psili and perispomeni",
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeni => "greek small letter alpha with dasia and perispomeni",
            GreekExtended::GreekCapitalLetterAlphaWithPsili => "greek capital letter alpha with psili",
            GreekExtended::GreekCapitalLetterAlphaWithDasia => "greek capital letter alpha with dasia",
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVaria => "greek capital letter alpha with psili and varia",
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVaria => "greek capital letter alpha with dasia and varia",
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxia => "greek capital letter alpha with psili and oxia",
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxia => "greek capital letter alpha with dasia and oxia",
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeni => "greek capital letter alpha with psili and perispomeni",
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeni => "greek capital letter alpha with dasia and perispomeni",
            GreekExtended::GreekSmallLetterEpsilonWithPsili => "greek small letter epsilon with psili",
            GreekExtended::GreekSmallLetterEpsilonWithDasia => "greek small letter epsilon with dasia",
            GreekExtended::GreekSmallLetterEpsilonWithPsiliAndVaria => "greek small letter epsilon with psili and varia",
            GreekExtended::GreekSmallLetterEpsilonWithDasiaAndVaria => "greek small letter epsilon with dasia and varia",
            GreekExtended::GreekSmallLetterEpsilonWithPsiliAndOxia => "greek small letter epsilon with psili and oxia",
            GreekExtended::GreekSmallLetterEpsilonWithDasiaAndOxia => "greek small letter epsilon with dasia and oxia",
            GreekExtended::GreekCapitalLetterEpsilonWithPsili => "greek capital letter epsilon with psili",
            GreekExtended::GreekCapitalLetterEpsilonWithDasia => "greek capital letter epsilon with dasia",
            GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndVaria => "greek capital letter epsilon with psili and varia",
            GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndVaria => "greek capital letter epsilon with dasia and varia",
            GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndOxia => "greek capital letter epsilon with psili and oxia",
            GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndOxia => "greek capital letter epsilon with dasia and oxia",
            GreekExtended::GreekSmallLetterEtaWithPsili => "greek small letter eta with psili",
            GreekExtended::GreekSmallLetterEtaWithDasia => "greek small letter eta with dasia",
            GreekExtended::GreekSmallLetterEtaWithPsiliAndVaria => "greek small letter eta with psili and varia",
            GreekExtended::GreekSmallLetterEtaWithDasiaAndVaria => "greek small letter eta with dasia and varia",
            GreekExtended::GreekSmallLetterEtaWithPsiliAndOxia => "greek small letter eta with psili and oxia",
            GreekExtended::GreekSmallLetterEtaWithDasiaAndOxia => "greek small letter eta with dasia and oxia",
            GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeni => "greek small letter eta with psili and perispomeni",
            GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeni => "greek small letter eta with dasia and perispomeni",
            GreekExtended::GreekCapitalLetterEtaWithPsili => "greek capital letter eta with psili",
            GreekExtended::GreekCapitalLetterEtaWithDasia => "greek capital letter eta with dasia",
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndVaria => "greek capital letter eta with psili and varia",
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndVaria => "greek capital letter eta with dasia and varia",
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxia => "greek capital letter eta with psili and oxia",
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxia => "greek capital letter eta with dasia and oxia",
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeni => "greek capital letter eta with psili and perispomeni",
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeni => "greek capital letter eta with dasia and perispomeni",
            GreekExtended::GreekSmallLetterIotaWithPsili => "greek small letter iota with psili",
            GreekExtended::GreekSmallLetterIotaWithDasia => "greek small letter iota with dasia",
            GreekExtended::GreekSmallLetterIotaWithPsiliAndVaria => "greek small letter iota with psili and varia",
            GreekExtended::GreekSmallLetterIotaWithDasiaAndVaria => "greek small letter iota with dasia and varia",
            GreekExtended::GreekSmallLetterIotaWithPsiliAndOxia => "greek small letter iota with psili and oxia",
            GreekExtended::GreekSmallLetterIotaWithDasiaAndOxia => "greek small letter iota with dasia and oxia",
            GreekExtended::GreekSmallLetterIotaWithPsiliAndPerispomeni => "greek small letter iota with psili and perispomeni",
            GreekExtended::GreekSmallLetterIotaWithDasiaAndPerispomeni => "greek small letter iota with dasia and perispomeni",
            GreekExtended::GreekCapitalLetterIotaWithPsili => "greek capital letter iota with psili",
            GreekExtended::GreekCapitalLetterIotaWithDasia => "greek capital letter iota with dasia",
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndVaria => "greek capital letter iota with psili and varia",
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndVaria => "greek capital letter iota with dasia and varia",
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndOxia => "greek capital letter iota with psili and oxia",
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndOxia => "greek capital letter iota with dasia and oxia",
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndPerispomeni => "greek capital letter iota with psili and perispomeni",
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndPerispomeni => "greek capital letter iota with dasia and perispomeni",
            GreekExtended::GreekSmallLetterOmicronWithPsili => "greek small letter omicron with psili",
            GreekExtended::GreekSmallLetterOmicronWithDasia => "greek small letter omicron with dasia",
            GreekExtended::GreekSmallLetterOmicronWithPsiliAndVaria => "greek small letter omicron with psili and varia",
            GreekExtended::GreekSmallLetterOmicronWithDasiaAndVaria => "greek small letter omicron with dasia and varia",
            GreekExtended::GreekSmallLetterOmicronWithPsiliAndOxia => "greek small letter omicron with psili and oxia",
            GreekExtended::GreekSmallLetterOmicronWithDasiaAndOxia => "greek small letter omicron with dasia and oxia",
            GreekExtended::GreekCapitalLetterOmicronWithPsili => "greek capital letter omicron with psili",
            GreekExtended::GreekCapitalLetterOmicronWithDasia => "greek capital letter omicron with dasia",
            GreekExtended::GreekCapitalLetterOmicronWithPsiliAndVaria => "greek capital letter omicron with psili and varia",
            GreekExtended::GreekCapitalLetterOmicronWithDasiaAndVaria => "greek capital letter omicron with dasia and varia",
            GreekExtended::GreekCapitalLetterOmicronWithPsiliAndOxia => "greek capital letter omicron with psili and oxia",
            GreekExtended::GreekCapitalLetterOmicronWithDasiaAndOxia => "greek capital letter omicron with dasia and oxia",
            GreekExtended::GreekSmallLetterUpsilonWithPsili => "greek small letter upsilon with psili",
            GreekExtended::GreekSmallLetterUpsilonWithDasia => "greek small letter upsilon with dasia",
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndVaria => "greek small letter upsilon with psili and varia",
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndVaria => "greek small letter upsilon with dasia and varia",
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndOxia => "greek small letter upsilon with psili and oxia",
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndOxia => "greek small letter upsilon with dasia and oxia",
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndPerispomeni => "greek small letter upsilon with psili and perispomeni",
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndPerispomeni => "greek small letter upsilon with dasia and perispomeni",
            GreekExtended::GreekCapitalLetterUpsilonWithDasia => "greek capital letter upsilon with dasia",
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndVaria => "greek capital letter upsilon with dasia and varia",
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndOxia => "greek capital letter upsilon with dasia and oxia",
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndPerispomeni => "greek capital letter upsilon with dasia and perispomeni",
            GreekExtended::GreekSmallLetterOmegaWithPsili => "greek small letter omega with psili",
            GreekExtended::GreekSmallLetterOmegaWithDasia => "greek small letter omega with dasia",
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndVaria => "greek small letter omega with psili and varia",
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndVaria => "greek small letter omega with dasia and varia",
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxia => "greek small letter omega with psili and oxia",
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxia => "greek small letter omega with dasia and oxia",
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeni => "greek small letter omega with psili and perispomeni",
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeni => "greek small letter omega with dasia and perispomeni",
            GreekExtended::GreekCapitalLetterOmegaWithPsili => "greek capital letter omega with psili",
            GreekExtended::GreekCapitalLetterOmegaWithDasia => "greek capital letter omega with dasia",
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVaria => "greek capital letter omega with psili and varia",
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVaria => "greek capital letter omega with dasia and varia",
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxia => "greek capital letter omega with psili and oxia",
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxia => "greek capital letter omega with dasia and oxia",
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeni => "greek capital letter omega with psili and perispomeni",
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeni => "greek capital letter omega with dasia and perispomeni",
            GreekExtended::GreekSmallLetterAlphaWithVaria => "greek small letter alpha with varia",
            GreekExtended::GreekSmallLetterAlphaWithOxia => "greek small letter alpha with oxia",
            GreekExtended::GreekSmallLetterEpsilonWithVaria => "greek small letter epsilon with varia",
            GreekExtended::GreekSmallLetterEpsilonWithOxia => "greek small letter epsilon with oxia",
            GreekExtended::GreekSmallLetterEtaWithVaria => "greek small letter eta with varia",
            GreekExtended::GreekSmallLetterEtaWithOxia => "greek small letter eta with oxia",
            GreekExtended::GreekSmallLetterIotaWithVaria => "greek small letter iota with varia",
            GreekExtended::GreekSmallLetterIotaWithOxia => "greek small letter iota with oxia",
            GreekExtended::GreekSmallLetterOmicronWithVaria => "greek small letter omicron with varia",
            GreekExtended::GreekSmallLetterOmicronWithOxia => "greek small letter omicron with oxia",
            GreekExtended::GreekSmallLetterUpsilonWithVaria => "greek small letter upsilon with varia",
            GreekExtended::GreekSmallLetterUpsilonWithOxia => "greek small letter upsilon with oxia",
            GreekExtended::GreekSmallLetterOmegaWithVaria => "greek small letter omega with varia",
            GreekExtended::GreekSmallLetterOmegaWithOxia => "greek small letter omega with oxia",
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndYpogegrammeni => "greek small letter alpha with psili and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndYpogegrammeni => "greek small letter alpha with dasia and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndVariaAndYpogegrammeni => "greek small letter alpha with psili and varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndVariaAndYpogegrammeni => "greek small letter alpha with dasia and varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxiaAndYpogegrammeni => "greek small letter alpha with psili and oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxiaAndYpogegrammeni => "greek small letter alpha with dasia and oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeniAndYpogegrammeni => "greek small letter alpha with psili and perispomeni and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeniAndYpogegrammeni => "greek small letter alpha with dasia and perispomeni and ypogegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndProsgegrammeni => "greek capital letter alpha with psili and prosgegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndProsgegrammeni => "greek capital letter alpha with dasia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVariaAndProsgegrammeni => "greek capital letter alpha with psili and varia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVariaAndProsgegrammeni => "greek capital letter alpha with dasia and varia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxiaAndProsgegrammeni => "greek capital letter alpha with psili and oxia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxiaAndProsgegrammeni => "greek capital letter alpha with dasia and oxia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeniAndProsgegrammeni => "greek capital letter alpha with psili and perispomeni and prosgegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeniAndProsgegrammeni => "greek capital letter alpha with dasia and perispomeni and prosgegrammeni",
            GreekExtended::GreekSmallLetterEtaWithPsiliAndYpogegrammeni => "greek small letter eta with psili and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithDasiaAndYpogegrammeni => "greek small letter eta with dasia and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithPsiliAndVariaAndYpogegrammeni => "greek small letter eta with psili and varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithDasiaAndVariaAndYpogegrammeni => "greek small letter eta with dasia and varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithPsiliAndOxiaAndYpogegrammeni => "greek small letter eta with psili and oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithDasiaAndOxiaAndYpogegrammeni => "greek small letter eta with dasia and oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeniAndYpogegrammeni => "greek small letter eta with psili and perispomeni and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeniAndYpogegrammeni => "greek small letter eta with dasia and perispomeni and ypogegrammeni",
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndProsgegrammeni => "greek capital letter eta with psili and prosgegrammeni",
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndProsgegrammeni => "greek capital letter eta with dasia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndVariaAndProsgegrammeni => "greek capital letter eta with psili and varia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndVariaAndProsgegrammeni => "greek capital letter eta with dasia and varia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxiaAndProsgegrammeni => "greek capital letter eta with psili and oxia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxiaAndProsgegrammeni => "greek capital letter eta with dasia and oxia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeniAndProsgegrammeni => "greek capital letter eta with psili and perispomeni and prosgegrammeni",
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeniAndProsgegrammeni => "greek capital letter eta with dasia and perispomeni and prosgegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndYpogegrammeni => "greek small letter omega with psili and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndYpogegrammeni => "greek small letter omega with dasia and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndVariaAndYpogegrammeni => "greek small letter omega with psili and varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndVariaAndYpogegrammeni => "greek small letter omega with dasia and varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxiaAndYpogegrammeni => "greek small letter omega with psili and oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxiaAndYpogegrammeni => "greek small letter omega with dasia and oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeniAndYpogegrammeni => "greek small letter omega with psili and perispomeni and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeniAndYpogegrammeni => "greek small letter omega with dasia and perispomeni and ypogegrammeni",
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndProsgegrammeni => "greek capital letter omega with psili and prosgegrammeni",
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndProsgegrammeni => "greek capital letter omega with dasia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVariaAndProsgegrammeni => "greek capital letter omega with psili and varia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVariaAndProsgegrammeni => "greek capital letter omega with dasia and varia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxiaAndProsgegrammeni => "greek capital letter omega with psili and oxia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxiaAndProsgegrammeni => "greek capital letter omega with dasia and oxia and prosgegrammeni",
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeniAndProsgegrammeni => "greek capital letter omega with psili and perispomeni and prosgegrammeni",
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeniAndProsgegrammeni => "greek capital letter omega with dasia and perispomeni and prosgegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithVrachy => "greek small letter alpha with vrachy",
            GreekExtended::GreekSmallLetterAlphaWithMacron => "greek small letter alpha with macron",
            GreekExtended::GreekSmallLetterAlphaWithVariaAndYpogegrammeni => "greek small letter alpha with varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithYpogegrammeni => "greek small letter alpha with ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithOxiaAndYpogegrammeni => "greek small letter alpha with oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterAlphaWithPerispomeni => "greek small letter alpha with perispomeni",
            GreekExtended::GreekSmallLetterAlphaWithPerispomeniAndYpogegrammeni => "greek small letter alpha with perispomeni and ypogegrammeni",
            GreekExtended::GreekCapitalLetterAlphaWithVrachy => "greek capital letter alpha with vrachy",
            GreekExtended::GreekCapitalLetterAlphaWithMacron => "greek capital letter alpha with macron",
            GreekExtended::GreekCapitalLetterAlphaWithVaria => "greek capital letter alpha with varia",
            GreekExtended::GreekCapitalLetterAlphaWithOxia => "greek capital letter alpha with oxia",
            GreekExtended::GreekCapitalLetterAlphaWithProsgegrammeni => "greek capital letter alpha with prosgegrammeni",
            GreekExtended::GreekKoronis => "greek koronis",
            GreekExtended::GreekProsgegrammeni => "greek prosgegrammeni",
            GreekExtended::GreekPsili => "greek psili",
            GreekExtended::GreekPerispomeni => "greek perispomeni",
            GreekExtended::GreekDialytikaAndPerispomeni => "greek dialytika and perispomeni",
            GreekExtended::GreekSmallLetterEtaWithVariaAndYpogegrammeni => "greek small letter eta with varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithYpogegrammeni => "greek small letter eta with ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithOxiaAndYpogegrammeni => "greek small letter eta with oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterEtaWithPerispomeni => "greek small letter eta with perispomeni",
            GreekExtended::GreekSmallLetterEtaWithPerispomeniAndYpogegrammeni => "greek small letter eta with perispomeni and ypogegrammeni",
            GreekExtended::GreekCapitalLetterEpsilonWithVaria => "greek capital letter epsilon with varia",
            GreekExtended::GreekCapitalLetterEpsilonWithOxia => "greek capital letter epsilon with oxia",
            GreekExtended::GreekCapitalLetterEtaWithVaria => "greek capital letter eta with varia",
            GreekExtended::GreekCapitalLetterEtaWithOxia => "greek capital letter eta with oxia",
            GreekExtended::GreekCapitalLetterEtaWithProsgegrammeni => "greek capital letter eta with prosgegrammeni",
            GreekExtended::GreekPsiliAndVaria => "greek psili and varia",
            GreekExtended::GreekPsiliAndOxia => "greek psili and oxia",
            GreekExtended::GreekPsiliAndPerispomeni => "greek psili and perispomeni",
            GreekExtended::GreekSmallLetterIotaWithVrachy => "greek small letter iota with vrachy",
            GreekExtended::GreekSmallLetterIotaWithMacron => "greek small letter iota with macron",
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndVaria => "greek small letter iota with dialytika and varia",
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndOxia => "greek small letter iota with dialytika and oxia",
            GreekExtended::GreekSmallLetterIotaWithPerispomeni => "greek small letter iota with perispomeni",
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndPerispomeni => "greek small letter iota with dialytika and perispomeni",
            GreekExtended::GreekCapitalLetterIotaWithVrachy => "greek capital letter iota with vrachy",
            GreekExtended::GreekCapitalLetterIotaWithMacron => "greek capital letter iota with macron",
            GreekExtended::GreekCapitalLetterIotaWithVaria => "greek capital letter iota with varia",
            GreekExtended::GreekCapitalLetterIotaWithOxia => "greek capital letter iota with oxia",
            GreekExtended::GreekDasiaAndVaria => "greek dasia and varia",
            GreekExtended::GreekDasiaAndOxia => "greek dasia and oxia",
            GreekExtended::GreekDasiaAndPerispomeni => "greek dasia and perispomeni",
            GreekExtended::GreekSmallLetterUpsilonWithVrachy => "greek small letter upsilon with vrachy",
            GreekExtended::GreekSmallLetterUpsilonWithMacron => "greek small letter upsilon with macron",
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndVaria => "greek small letter upsilon with dialytika and varia",
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndOxia => "greek small letter upsilon with dialytika and oxia",
            GreekExtended::GreekSmallLetterRhoWithPsili => "greek small letter rho with psili",
            GreekExtended::GreekSmallLetterRhoWithDasia => "greek small letter rho with dasia",
            GreekExtended::GreekSmallLetterUpsilonWithPerispomeni => "greek small letter upsilon with perispomeni",
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndPerispomeni => "greek small letter upsilon with dialytika and perispomeni",
            GreekExtended::GreekCapitalLetterUpsilonWithVrachy => "greek capital letter upsilon with vrachy",
            GreekExtended::GreekCapitalLetterUpsilonWithMacron => "greek capital letter upsilon with macron",
            GreekExtended::GreekCapitalLetterUpsilonWithVaria => "greek capital letter upsilon with varia",
            GreekExtended::GreekCapitalLetterUpsilonWithOxia => "greek capital letter upsilon with oxia",
            GreekExtended::GreekCapitalLetterRhoWithDasia => "greek capital letter rho with dasia",
            GreekExtended::GreekDialytikaAndVaria => "greek dialytika and varia",
            GreekExtended::GreekDialytikaAndOxia => "greek dialytika and oxia",
            GreekExtended::GreekVaria => "greek varia",
            GreekExtended::GreekSmallLetterOmegaWithVariaAndYpogegrammeni => "greek small letter omega with varia and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithYpogegrammeni => "greek small letter omega with ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithOxiaAndYpogegrammeni => "greek small letter omega with oxia and ypogegrammeni",
            GreekExtended::GreekSmallLetterOmegaWithPerispomeni => "greek small letter omega with perispomeni",
            GreekExtended::GreekSmallLetterOmegaWithPerispomeniAndYpogegrammeni => "greek small letter omega with perispomeni and ypogegrammeni",
            GreekExtended::GreekCapitalLetterOmicronWithVaria => "greek capital letter omicron with varia",
            GreekExtended::GreekCapitalLetterOmicronWithOxia => "greek capital letter omicron with oxia",
            GreekExtended::GreekCapitalLetterOmegaWithVaria => "greek capital letter omega with varia",
            GreekExtended::GreekCapitalLetterOmegaWithOxia => "greek capital letter omega with oxia",
            GreekExtended::GreekCapitalLetterOmegaWithProsgegrammeni => "greek capital letter omega with prosgegrammeni",
            GreekExtended::GreekOxia => "greek oxia",
            GreekExtended::GreekDasia => "greek dasia",
        }
    }
}
