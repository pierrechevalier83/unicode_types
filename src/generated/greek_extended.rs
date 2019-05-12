/// \u{1f00} → \u{1fff}
///
/// ἀ ἁ ἂ ἃ ἄ ἅ ἆ ἇ Ἀ Ἁ Ἂ Ἃ Ἄ Ἅ Ἆ Ἇ\
/// ἐ ἑ ἒ ἓ ἔ ἕ Ἐ Ἑ Ἒ Ἓ Ἔ Ἕ ἠ ἡ ἢ ἣ\
/// ἤ ἥ ἦ ἧ Ἠ Ἡ Ἢ Ἣ Ἤ Ἥ Ἦ Ἧ ἰ ἱ ἲ ἳ\
/// ἴ ἵ ἶ ἷ Ἰ Ἱ Ἲ Ἳ Ἴ Ἵ Ἶ Ἷ ὀ ὁ ὂ ὃ\
/// ὄ ὅ Ὀ Ὁ Ὂ Ὃ Ὄ Ὅ ὐ ὑ ὒ ὓ ὔ ὕ ὖ ὗ\
/// Ὑ Ὓ Ὕ Ὗ ὠ ὡ ὢ ὣ ὤ ὥ ὦ ὧ Ὠ Ὡ Ὢ Ὣ\
/// Ὤ Ὥ Ὦ Ὧ ὰ ά ὲ έ ὴ ή ὶ ί ὸ ό ὺ ύ\
/// ὼ ώ ᾀ ᾁ ᾂ ᾃ ᾄ ᾅ ᾆ ᾇ ᾈ ᾉ ᾊ ᾋ ᾌ ᾍ\
/// ᾎ ᾏ ᾐ ᾑ ᾒ ᾓ ᾔ ᾕ ᾖ ᾗ ᾘ ᾙ ᾚ ᾛ ᾜ ᾝ\
/// ᾞ ᾟ ᾠ ᾡ ᾢ ᾣ ᾤ ᾥ ᾦ ᾧ ᾨ ᾩ ᾪ ᾫ ᾬ ᾭ\
/// ᾮ ᾯ ᾰ ᾱ ᾲ ᾳ ᾴ ᾶ ᾷ Ᾰ Ᾱ Ὰ Ά ᾼ ᾽ ι\
/// ᾿ ῀ ῁ ῂ ῃ ῄ ῆ ῇ Ὲ Έ Ὴ Ή ῌ ῍ ῎ ῏\
/// ῐ ῑ ῒ ΐ ῖ ῗ Ῐ Ῑ Ὶ Ί ῝ ῞ ῟ ῠ ῡ ῢ\
/// ΰ ῤ ῥ ῦ ῧ Ῠ Ῡ Ὺ Ύ Ῥ ῭ ΅ ` ῲ ῳ ῴ\
/// ῶ ῷ Ὸ Ό Ὼ Ώ ῼ ´ ῾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1f00}: 'ἀ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PSILI: char = 'ἀ';
    /// \u{1f01}: 'ἁ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_DASIA: char = 'ἁ';
    /// \u{1f02}: 'ἂ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_VARIA: char = 'ἂ';
    /// \u{1f03}: 'ἃ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_VARIA: char = 'ἃ';
    /// \u{1f04}: 'ἄ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_OXIA: char = 'ἄ';
    /// \u{1f05}: 'ἅ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_OXIA: char = 'ἅ';
    /// \u{1f06}: 'ἆ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI: char = 'ἆ';
    /// \u{1f07}: 'ἇ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI: char = 'ἇ';
    /// \u{1f08}: 'Ἀ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI: char = 'Ἀ';
    /// \u{1f09}: 'Ἁ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA: char = 'Ἁ';
    /// \u{1f0a}: 'Ἂ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_VARIA: char = 'Ἂ';
    /// \u{1f0b}: 'Ἃ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_VARIA: char = 'Ἃ';
    /// \u{1f0c}: 'Ἄ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_OXIA: char = 'Ἄ';
    /// \u{1f0d}: 'Ἅ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_OXIA: char = 'Ἅ';
    /// \u{1f0e}: 'Ἆ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI: char = 'Ἆ';
    /// \u{1f0f}: 'Ἇ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI: char = 'Ἇ';
    /// \u{1f10}: 'ἐ'
    pub const GREEK_SMALL_LETTER_EPSILON_WITH_PSILI: char = 'ἐ';
    /// \u{1f11}: 'ἑ'
    pub const GREEK_SMALL_LETTER_EPSILON_WITH_DASIA: char = 'ἑ';
    /// \u{1f12}: 'ἒ'
    pub const GREEK_SMALL_LETTER_EPSILON_WITH_PSILI_AND_VARIA: char = 'ἒ';
    /// \u{1f13}: 'ἓ'
    pub const GREEK_SMALL_LETTER_EPSILON_WITH_DASIA_AND_VARIA: char = 'ἓ';
    /// \u{1f14}: 'ἔ'
    pub const GREEK_SMALL_LETTER_EPSILON_WITH_PSILI_AND_OXIA: char = 'ἔ';
    /// \u{1f15}: 'ἕ'
    pub const GREEK_SMALL_LETTER_EPSILON_WITH_DASIA_AND_OXIA: char = 'ἕ';
    /// \u{1f18}: 'Ἐ'
    pub const GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI: char = 'Ἐ';
    /// \u{1f19}: 'Ἑ'
    pub const GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA: char = 'Ἑ';
    /// \u{1f1a}: 'Ἒ'
    pub const GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI_AND_VARIA: char = 'Ἒ';
    /// \u{1f1b}: 'Ἓ'
    pub const GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA_AND_VARIA: char = 'Ἓ';
    /// \u{1f1c}: 'Ἔ'
    pub const GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI_AND_OXIA: char = 'Ἔ';
    /// \u{1f1d}: 'Ἕ'
    pub const GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA_AND_OXIA: char = 'Ἕ';
    /// \u{1f20}: 'ἠ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PSILI: char = 'ἠ';
    /// \u{1f21}: 'ἡ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_DASIA: char = 'ἡ';
    /// \u{1f22}: 'ἢ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_VARIA: char = 'ἢ';
    /// \u{1f23}: 'ἣ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_VARIA: char = 'ἣ';
    /// \u{1f24}: 'ἤ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_OXIA: char = 'ἤ';
    /// \u{1f25}: 'ἥ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_OXIA: char = 'ἥ';
    /// \u{1f26}: 'ἦ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI: char = 'ἦ';
    /// \u{1f27}: 'ἧ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI: char = 'ἧ';
    /// \u{1f28}: 'Ἠ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PSILI: char = 'Ἠ';
    /// \u{1f29}: 'Ἡ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_DASIA: char = 'Ἡ';
    /// \u{1f2a}: 'Ἢ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_VARIA: char = 'Ἢ';
    /// \u{1f2b}: 'Ἣ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_VARIA: char = 'Ἣ';
    /// \u{1f2c}: 'Ἤ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_OXIA: char = 'Ἤ';
    /// \u{1f2d}: 'Ἥ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_OXIA: char = 'Ἥ';
    /// \u{1f2e}: 'Ἦ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI: char = 'Ἦ';
    /// \u{1f2f}: 'Ἧ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI: char = 'Ἧ';
    /// \u{1f30}: 'ἰ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_PSILI: char = 'ἰ';
    /// \u{1f31}: 'ἱ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_DASIA: char = 'ἱ';
    /// \u{1f32}: 'ἲ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_VARIA: char = 'ἲ';
    /// \u{1f33}: 'ἳ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_VARIA: char = 'ἳ';
    /// \u{1f34}: 'ἴ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_OXIA: char = 'ἴ';
    /// \u{1f35}: 'ἵ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_OXIA: char = 'ἵ';
    /// \u{1f36}: 'ἶ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_PERISPOMENI: char = 'ἶ';
    /// \u{1f37}: 'ἷ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_PERISPOMENI: char = 'ἷ';
    /// \u{1f38}: 'Ἰ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI: char = 'Ἰ';
    /// \u{1f39}: 'Ἱ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA: char = 'Ἱ';
    /// \u{1f3a}: 'Ἲ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_VARIA: char = 'Ἲ';
    /// \u{1f3b}: 'Ἳ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_VARIA: char = 'Ἳ';
    /// \u{1f3c}: 'Ἴ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_OXIA: char = 'Ἴ';
    /// \u{1f3d}: 'Ἵ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_OXIA: char = 'Ἵ';
    /// \u{1f3e}: 'Ἶ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_PERISPOMENI: char = 'Ἶ';
    /// \u{1f3f}: 'Ἷ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_PERISPOMENI: char = 'Ἷ';
    /// \u{1f40}: 'ὀ'
    pub const GREEK_SMALL_LETTER_OMICRON_WITH_PSILI: char = 'ὀ';
    /// \u{1f41}: 'ὁ'
    pub const GREEK_SMALL_LETTER_OMICRON_WITH_DASIA: char = 'ὁ';
    /// \u{1f42}: 'ὂ'
    pub const GREEK_SMALL_LETTER_OMICRON_WITH_PSILI_AND_VARIA: char = 'ὂ';
    /// \u{1f43}: 'ὃ'
    pub const GREEK_SMALL_LETTER_OMICRON_WITH_DASIA_AND_VARIA: char = 'ὃ';
    /// \u{1f44}: 'ὄ'
    pub const GREEK_SMALL_LETTER_OMICRON_WITH_PSILI_AND_OXIA: char = 'ὄ';
    /// \u{1f45}: 'ὅ'
    pub const GREEK_SMALL_LETTER_OMICRON_WITH_DASIA_AND_OXIA: char = 'ὅ';
    /// \u{1f48}: 'Ὀ'
    pub const GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI: char = 'Ὀ';
    /// \u{1f49}: 'Ὁ'
    pub const GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA: char = 'Ὁ';
    /// \u{1f4a}: 'Ὂ'
    pub const GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI_AND_VARIA: char = 'Ὂ';
    /// \u{1f4b}: 'Ὃ'
    pub const GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA_AND_VARIA: char = 'Ὃ';
    /// \u{1f4c}: 'Ὄ'
    pub const GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI_AND_OXIA: char = 'Ὄ';
    /// \u{1f4d}: 'Ὅ'
    pub const GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA_AND_OXIA: char = 'Ὅ';
    /// \u{1f50}: 'ὐ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_PSILI: char = 'ὐ';
    /// \u{1f51}: 'ὑ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_DASIA: char = 'ὑ';
    /// \u{1f52}: 'ὒ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_VARIA: char = 'ὒ';
    /// \u{1f53}: 'ὓ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_VARIA: char = 'ὓ';
    /// \u{1f54}: 'ὔ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_OXIA: char = 'ὔ';
    /// \u{1f55}: 'ὕ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_OXIA: char = 'ὕ';
    /// \u{1f56}: 'ὖ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_PERISPOMENI: char = 'ὖ';
    /// \u{1f57}: 'ὗ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_PERISPOMENI: char = 'ὗ';
    /// \u{1f59}: 'Ὑ'
    pub const GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA: char = 'Ὑ';
    /// \u{1f5b}: 'Ὓ'
    pub const GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_VARIA: char = 'Ὓ';
    /// \u{1f5d}: 'Ὕ'
    pub const GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_OXIA: char = 'Ὕ';
    /// \u{1f5f}: 'Ὗ'
    pub const GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_PERISPOMENI: char = 'Ὗ';
    /// \u{1f60}: 'ὠ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PSILI: char = 'ὠ';
    /// \u{1f61}: 'ὡ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_DASIA: char = 'ὡ';
    /// \u{1f62}: 'ὢ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_VARIA: char = 'ὢ';
    /// \u{1f63}: 'ὣ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_VARIA: char = 'ὣ';
    /// \u{1f64}: 'ὤ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_OXIA: char = 'ὤ';
    /// \u{1f65}: 'ὥ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_OXIA: char = 'ὥ';
    /// \u{1f66}: 'ὦ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI: char = 'ὦ';
    /// \u{1f67}: 'ὧ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI: char = 'ὧ';
    /// \u{1f68}: 'Ὠ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI: char = 'Ὠ';
    /// \u{1f69}: 'Ὡ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA: char = 'Ὡ';
    /// \u{1f6a}: 'Ὢ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_VARIA: char = 'Ὢ';
    /// \u{1f6b}: 'Ὣ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_VARIA: char = 'Ὣ';
    /// \u{1f6c}: 'Ὤ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_OXIA: char = 'Ὤ';
    /// \u{1f6d}: 'Ὥ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_OXIA: char = 'Ὥ';
    /// \u{1f6e}: 'Ὦ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI: char = 'Ὦ';
    /// \u{1f6f}: 'Ὧ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI: char = 'Ὧ';
    /// \u{1f70}: 'ὰ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_VARIA: char = 'ὰ';
    /// \u{1f71}: 'ά'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_OXIA: char = 'ά';
    /// \u{1f72}: 'ὲ'
    pub const GREEK_SMALL_LETTER_EPSILON_WITH_VARIA: char = 'ὲ';
    /// \u{1f73}: 'έ'
    pub const GREEK_SMALL_LETTER_EPSILON_WITH_OXIA: char = 'έ';
    /// \u{1f74}: 'ὴ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_VARIA: char = 'ὴ';
    /// \u{1f75}: 'ή'
    pub const GREEK_SMALL_LETTER_ETA_WITH_OXIA: char = 'ή';
    /// \u{1f76}: 'ὶ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_VARIA: char = 'ὶ';
    /// \u{1f77}: 'ί'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_OXIA: char = 'ί';
    /// \u{1f78}: 'ὸ'
    pub const GREEK_SMALL_LETTER_OMICRON_WITH_VARIA: char = 'ὸ';
    /// \u{1f79}: 'ό'
    pub const GREEK_SMALL_LETTER_OMICRON_WITH_OXIA: char = 'ό';
    /// \u{1f7a}: 'ὺ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_VARIA: char = 'ὺ';
    /// \u{1f7b}: 'ύ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_OXIA: char = 'ύ';
    /// \u{1f7c}: 'ὼ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_VARIA: char = 'ὼ';
    /// \u{1f7d}: 'ώ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_OXIA: char = 'ώ';
    /// \u{1f80}: 'ᾀ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_YPOGEGRAMMENI: char = 'ᾀ';
    /// \u{1f81}: 'ᾁ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_YPOGEGRAMMENI: char = 'ᾁ';
    /// \u{1f82}: 'ᾂ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI: char = 'ᾂ';
    /// \u{1f83}: 'ᾃ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI: char = 'ᾃ';
    /// \u{1f84}: 'ᾄ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI: char = 'ᾄ';
    /// \u{1f85}: 'ᾅ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI: char = 'ᾅ';
    /// \u{1f86}: 'ᾆ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ᾆ';
    /// \u{1f87}: 'ᾇ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ᾇ';
    /// \u{1f88}: 'ᾈ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PROSGEGRAMMENI: char = 'ᾈ';
    /// \u{1f89}: 'ᾉ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PROSGEGRAMMENI: char = 'ᾉ';
    /// \u{1f8a}: 'ᾊ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI: char = 'ᾊ';
    /// \u{1f8b}: 'ᾋ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI: char = 'ᾋ';
    /// \u{1f8c}: 'ᾌ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI: char = 'ᾌ';
    /// \u{1f8d}: 'ᾍ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI: char = 'ᾍ';
    /// \u{1f8e}: 'ᾎ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI: char = 'ᾎ';
    /// \u{1f8f}: 'ᾏ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI: char = 'ᾏ';
    /// \u{1f90}: 'ᾐ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_YPOGEGRAMMENI: char = 'ᾐ';
    /// \u{1f91}: 'ᾑ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_YPOGEGRAMMENI: char = 'ᾑ';
    /// \u{1f92}: 'ᾒ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI: char = 'ᾒ';
    /// \u{1f93}: 'ᾓ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI: char = 'ᾓ';
    /// \u{1f94}: 'ᾔ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI: char = 'ᾔ';
    /// \u{1f95}: 'ᾕ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI: char = 'ᾕ';
    /// \u{1f96}: 'ᾖ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ᾖ';
    /// \u{1f97}: 'ᾗ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ᾗ';
    /// \u{1f98}: 'ᾘ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PROSGEGRAMMENI: char = 'ᾘ';
    /// \u{1f99}: 'ᾙ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PROSGEGRAMMENI: char = 'ᾙ';
    /// \u{1f9a}: 'ᾚ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI: char = 'ᾚ';
    /// \u{1f9b}: 'ᾛ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI: char = 'ᾛ';
    /// \u{1f9c}: 'ᾜ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI: char = 'ᾜ';
    /// \u{1f9d}: 'ᾝ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI: char = 'ᾝ';
    /// \u{1f9e}: 'ᾞ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI: char = 'ᾞ';
    /// \u{1f9f}: 'ᾟ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI: char = 'ᾟ';
    /// \u{1fa0}: 'ᾠ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_YPOGEGRAMMENI: char = 'ᾠ';
    /// \u{1fa1}: 'ᾡ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_YPOGEGRAMMENI: char = 'ᾡ';
    /// \u{1fa2}: 'ᾢ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI: char = 'ᾢ';
    /// \u{1fa3}: 'ᾣ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI: char = 'ᾣ';
    /// \u{1fa4}: 'ᾤ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI: char = 'ᾤ';
    /// \u{1fa5}: 'ᾥ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI: char = 'ᾥ';
    /// \u{1fa6}: 'ᾦ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ᾦ';
    /// \u{1fa7}: 'ᾧ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ᾧ';
    /// \u{1fa8}: 'ᾨ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PROSGEGRAMMENI: char = 'ᾨ';
    /// \u{1fa9}: 'ᾩ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PROSGEGRAMMENI: char = 'ᾩ';
    /// \u{1faa}: 'ᾪ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI: char = 'ᾪ';
    /// \u{1fab}: 'ᾫ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI: char = 'ᾫ';
    /// \u{1fac}: 'ᾬ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI: char = 'ᾬ';
    /// \u{1fad}: 'ᾭ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI: char = 'ᾭ';
    /// \u{1fae}: 'ᾮ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI: char = 'ᾮ';
    /// \u{1faf}: 'ᾯ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI: char = 'ᾯ';
    /// \u{1fb0}: 'ᾰ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_VRACHY: char = 'ᾰ';
    /// \u{1fb1}: 'ᾱ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_MACRON: char = 'ᾱ';
    /// \u{1fb2}: 'ᾲ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_VARIA_AND_YPOGEGRAMMENI: char = 'ᾲ';
    /// \u{1fb3}: 'ᾳ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_YPOGEGRAMMENI: char = 'ᾳ';
    /// \u{1fb4}: 'ᾴ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_OXIA_AND_YPOGEGRAMMENI: char = 'ᾴ';
    /// \u{1fb6}: 'ᾶ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PERISPOMENI: char = 'ᾶ';
    /// \u{1fb7}: 'ᾷ'
    pub const GREEK_SMALL_LETTER_ALPHA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ᾷ';
    /// \u{1fb8}: 'Ᾰ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_VRACHY: char = 'Ᾰ';
    /// \u{1fb9}: 'Ᾱ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_MACRON: char = 'Ᾱ';
    /// \u{1fba}: 'Ὰ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_VARIA: char = 'Ὰ';
    /// \u{1fbb}: 'Ά'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_OXIA: char = 'Ά';
    /// \u{1fbc}: 'ᾼ'
    pub const GREEK_CAPITAL_LETTER_ALPHA_WITH_PROSGEGRAMMENI: char = 'ᾼ';
    /// \u{1fbd}: '᾽'
    pub const GREEK_KORONIS: char = '᾽';
    /// \u{1fbe}: 'ι'
    pub const GREEK_PROSGEGRAMMENI: char = 'ι';
    /// \u{1fbf}: '᾿'
    pub const GREEK_PSILI: char = '᾿';
    /// \u{1fc0}: '῀'
    pub const GREEK_PERISPOMENI: char = '῀';
    /// \u{1fc1}: '῁'
    pub const GREEK_DIALYTIKA_AND_PERISPOMENI: char = '῁';
    /// \u{1fc2}: 'ῂ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_VARIA_AND_YPOGEGRAMMENI: char = 'ῂ';
    /// \u{1fc3}: 'ῃ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_YPOGEGRAMMENI: char = 'ῃ';
    /// \u{1fc4}: 'ῄ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_OXIA_AND_YPOGEGRAMMENI: char = 'ῄ';
    /// \u{1fc6}: 'ῆ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PERISPOMENI: char = 'ῆ';
    /// \u{1fc7}: 'ῇ'
    pub const GREEK_SMALL_LETTER_ETA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ῇ';
    /// \u{1fc8}: 'Ὲ'
    pub const GREEK_CAPITAL_LETTER_EPSILON_WITH_VARIA: char = 'Ὲ';
    /// \u{1fc9}: 'Έ'
    pub const GREEK_CAPITAL_LETTER_EPSILON_WITH_OXIA: char = 'Έ';
    /// \u{1fca}: 'Ὴ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_VARIA: char = 'Ὴ';
    /// \u{1fcb}: 'Ή'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_OXIA: char = 'Ή';
    /// \u{1fcc}: 'ῌ'
    pub const GREEK_CAPITAL_LETTER_ETA_WITH_PROSGEGRAMMENI: char = 'ῌ';
    /// \u{1fcd}: '῍'
    pub const GREEK_PSILI_AND_VARIA: char = '῍';
    /// \u{1fce}: '῎'
    pub const GREEK_PSILI_AND_OXIA: char = '῎';
    /// \u{1fcf}: '῏'
    pub const GREEK_PSILI_AND_PERISPOMENI: char = '῏';
    /// \u{1fd0}: 'ῐ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_VRACHY: char = 'ῐ';
    /// \u{1fd1}: 'ῑ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_MACRON: char = 'ῑ';
    /// \u{1fd2}: 'ῒ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_VARIA: char = 'ῒ';
    /// \u{1fd3}: 'ΐ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_OXIA: char = 'ΐ';
    /// \u{1fd6}: 'ῖ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_PERISPOMENI: char = 'ῖ';
    /// \u{1fd7}: 'ῗ'
    pub const GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_PERISPOMENI: char = 'ῗ';
    /// \u{1fd8}: 'Ῐ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_VRACHY: char = 'Ῐ';
    /// \u{1fd9}: 'Ῑ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_MACRON: char = 'Ῑ';
    /// \u{1fda}: 'Ὶ'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_VARIA: char = 'Ὶ';
    /// \u{1fdb}: 'Ί'
    pub const GREEK_CAPITAL_LETTER_IOTA_WITH_OXIA: char = 'Ί';
    /// \u{1fdd}: '῝'
    pub const GREEK_DASIA_AND_VARIA: char = '῝';
    /// \u{1fde}: '῞'
    pub const GREEK_DASIA_AND_OXIA: char = '῞';
    /// \u{1fdf}: '῟'
    pub const GREEK_DASIA_AND_PERISPOMENI: char = '῟';
    /// \u{1fe0}: 'ῠ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_VRACHY: char = 'ῠ';
    /// \u{1fe1}: 'ῡ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_MACRON: char = 'ῡ';
    /// \u{1fe2}: 'ῢ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_VARIA: char = 'ῢ';
    /// \u{1fe3}: 'ΰ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_OXIA: char = 'ΰ';
    /// \u{1fe4}: 'ῤ'
    pub const GREEK_SMALL_LETTER_RHO_WITH_PSILI: char = 'ῤ';
    /// \u{1fe5}: 'ῥ'
    pub const GREEK_SMALL_LETTER_RHO_WITH_DASIA: char = 'ῥ';
    /// \u{1fe6}: 'ῦ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_PERISPOMENI: char = 'ῦ';
    /// \u{1fe7}: 'ῧ'
    pub const GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_PERISPOMENI: char = 'ῧ';
    /// \u{1fe8}: 'Ῠ'
    pub const GREEK_CAPITAL_LETTER_UPSILON_WITH_VRACHY: char = 'Ῠ';
    /// \u{1fe9}: 'Ῡ'
    pub const GREEK_CAPITAL_LETTER_UPSILON_WITH_MACRON: char = 'Ῡ';
    /// \u{1fea}: 'Ὺ'
    pub const GREEK_CAPITAL_LETTER_UPSILON_WITH_VARIA: char = 'Ὺ';
    /// \u{1feb}: 'Ύ'
    pub const GREEK_CAPITAL_LETTER_UPSILON_WITH_OXIA: char = 'Ύ';
    /// \u{1fec}: 'Ῥ'
    pub const GREEK_CAPITAL_LETTER_RHO_WITH_DASIA: char = 'Ῥ';
    /// \u{1fed}: '῭'
    pub const GREEK_DIALYTIKA_AND_VARIA: char = '῭';
    /// \u{1fee}: '΅'
    pub const GREEK_DIALYTIKA_AND_OXIA: char = '΅';
    /// \u{1fef}: '`'
    pub const GREEK_VARIA: char = '`';
    /// \u{1ff2}: 'ῲ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_VARIA_AND_YPOGEGRAMMENI: char = 'ῲ';
    /// \u{1ff3}: 'ῳ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_YPOGEGRAMMENI: char = 'ῳ';
    /// \u{1ff4}: 'ῴ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_OXIA_AND_YPOGEGRAMMENI: char = 'ῴ';
    /// \u{1ff6}: 'ῶ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PERISPOMENI: char = 'ῶ';
    /// \u{1ff7}: 'ῷ'
    pub const GREEK_SMALL_LETTER_OMEGA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI: char = 'ῷ';
    /// \u{1ff8}: 'Ὸ'
    pub const GREEK_CAPITAL_LETTER_OMICRON_WITH_VARIA: char = 'Ὸ';
    /// \u{1ff9}: 'Ό'
    pub const GREEK_CAPITAL_LETTER_OMICRON_WITH_OXIA: char = 'Ό';
    /// \u{1ffa}: 'Ὼ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_VARIA: char = 'Ὼ';
    /// \u{1ffb}: 'Ώ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_OXIA: char = 'Ώ';
    /// \u{1ffc}: 'ῼ'
    pub const GREEK_CAPITAL_LETTER_OMEGA_WITH_PROSGEGRAMMENI: char = 'ῼ';
    /// \u{1ffd}: '´'
    pub const GREEK_OXIA: char = '´';
    /// \u{1ffe}: '῾'
    pub const GREEK_DASIA: char = '῾';
}

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
        use constants::*;
        match self {
            GreekExtended::GreekSmallLetterAlphaWithPsili => GREEK_SMALL_LETTER_ALPHA_WITH_PSILI,
            GreekExtended::GreekSmallLetterAlphaWithDasia => GREEK_SMALL_LETTER_ALPHA_WITH_DASIA,
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndVaria => GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndVaria => GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxia => GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxia => GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeni => GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeni => GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterAlphaWithPsili => GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI,
            GreekExtended::GreekCapitalLetterAlphaWithDasia => GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA,
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVaria => GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVaria => GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxia => GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxia => GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterEpsilonWithPsili => GREEK_SMALL_LETTER_EPSILON_WITH_PSILI,
            GreekExtended::GreekSmallLetterEpsilonWithDasia => GREEK_SMALL_LETTER_EPSILON_WITH_DASIA,
            GreekExtended::GreekSmallLetterEpsilonWithPsiliAndVaria => GREEK_SMALL_LETTER_EPSILON_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekSmallLetterEpsilonWithDasiaAndVaria => GREEK_SMALL_LETTER_EPSILON_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekSmallLetterEpsilonWithPsiliAndOxia => GREEK_SMALL_LETTER_EPSILON_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekSmallLetterEpsilonWithDasiaAndOxia => GREEK_SMALL_LETTER_EPSILON_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekCapitalLetterEpsilonWithPsili => GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI,
            GreekExtended::GreekCapitalLetterEpsilonWithDasia => GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA,
            GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndVaria => GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndVaria => GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndOxia => GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndOxia => GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekSmallLetterEtaWithPsili => GREEK_SMALL_LETTER_ETA_WITH_PSILI,
            GreekExtended::GreekSmallLetterEtaWithDasia => GREEK_SMALL_LETTER_ETA_WITH_DASIA,
            GreekExtended::GreekSmallLetterEtaWithPsiliAndVaria => GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekSmallLetterEtaWithDasiaAndVaria => GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekSmallLetterEtaWithPsiliAndOxia => GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekSmallLetterEtaWithDasiaAndOxia => GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeni => GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeni => GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterEtaWithPsili => GREEK_CAPITAL_LETTER_ETA_WITH_PSILI,
            GreekExtended::GreekCapitalLetterEtaWithDasia => GREEK_CAPITAL_LETTER_ETA_WITH_DASIA,
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndVaria => GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndVaria => GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxia => GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxia => GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeni => GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeni => GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterIotaWithPsili => GREEK_SMALL_LETTER_IOTA_WITH_PSILI,
            GreekExtended::GreekSmallLetterIotaWithDasia => GREEK_SMALL_LETTER_IOTA_WITH_DASIA,
            GreekExtended::GreekSmallLetterIotaWithPsiliAndVaria => GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekSmallLetterIotaWithDasiaAndVaria => GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekSmallLetterIotaWithPsiliAndOxia => GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekSmallLetterIotaWithDasiaAndOxia => GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekSmallLetterIotaWithPsiliAndPerispomeni => GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterIotaWithDasiaAndPerispomeni => GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterIotaWithPsili => GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI,
            GreekExtended::GreekCapitalLetterIotaWithDasia => GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA,
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndVaria => GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndVaria => GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndOxia => GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndOxia => GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekCapitalLetterIotaWithPsiliAndPerispomeni => GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterIotaWithDasiaAndPerispomeni => GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterOmicronWithPsili => GREEK_SMALL_LETTER_OMICRON_WITH_PSILI,
            GreekExtended::GreekSmallLetterOmicronWithDasia => GREEK_SMALL_LETTER_OMICRON_WITH_DASIA,
            GreekExtended::GreekSmallLetterOmicronWithPsiliAndVaria => GREEK_SMALL_LETTER_OMICRON_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekSmallLetterOmicronWithDasiaAndVaria => GREEK_SMALL_LETTER_OMICRON_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekSmallLetterOmicronWithPsiliAndOxia => GREEK_SMALL_LETTER_OMICRON_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekSmallLetterOmicronWithDasiaAndOxia => GREEK_SMALL_LETTER_OMICRON_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekCapitalLetterOmicronWithPsili => GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI,
            GreekExtended::GreekCapitalLetterOmicronWithDasia => GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA,
            GreekExtended::GreekCapitalLetterOmicronWithPsiliAndVaria => GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekCapitalLetterOmicronWithDasiaAndVaria => GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekCapitalLetterOmicronWithPsiliAndOxia => GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekCapitalLetterOmicronWithDasiaAndOxia => GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekSmallLetterUpsilonWithPsili => GREEK_SMALL_LETTER_UPSILON_WITH_PSILI,
            GreekExtended::GreekSmallLetterUpsilonWithDasia => GREEK_SMALL_LETTER_UPSILON_WITH_DASIA,
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndVaria => GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndVaria => GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndOxia => GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndOxia => GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekSmallLetterUpsilonWithPsiliAndPerispomeni => GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterUpsilonWithDasiaAndPerispomeni => GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterUpsilonWithDasia => GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA,
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndVaria => GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndOxia => GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndPerispomeni => GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterOmegaWithPsili => GREEK_SMALL_LETTER_OMEGA_WITH_PSILI,
            GreekExtended::GreekSmallLetterOmegaWithDasia => GREEK_SMALL_LETTER_OMEGA_WITH_DASIA,
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndVaria => GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndVaria => GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxia => GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxia => GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeni => GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeni => GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterOmegaWithPsili => GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI,
            GreekExtended::GreekCapitalLetterOmegaWithDasia => GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA,
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVaria => GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_VARIA,
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVaria => GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_VARIA,
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxia => GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_OXIA,
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxia => GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_OXIA,
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterAlphaWithVaria => GREEK_SMALL_LETTER_ALPHA_WITH_VARIA,
            GreekExtended::GreekSmallLetterAlphaWithOxia => GREEK_SMALL_LETTER_ALPHA_WITH_OXIA,
            GreekExtended::GreekSmallLetterEpsilonWithVaria => GREEK_SMALL_LETTER_EPSILON_WITH_VARIA,
            GreekExtended::GreekSmallLetterEpsilonWithOxia => GREEK_SMALL_LETTER_EPSILON_WITH_OXIA,
            GreekExtended::GreekSmallLetterEtaWithVaria => GREEK_SMALL_LETTER_ETA_WITH_VARIA,
            GreekExtended::GreekSmallLetterEtaWithOxia => GREEK_SMALL_LETTER_ETA_WITH_OXIA,
            GreekExtended::GreekSmallLetterIotaWithVaria => GREEK_SMALL_LETTER_IOTA_WITH_VARIA,
            GreekExtended::GreekSmallLetterIotaWithOxia => GREEK_SMALL_LETTER_IOTA_WITH_OXIA,
            GreekExtended::GreekSmallLetterOmicronWithVaria => GREEK_SMALL_LETTER_OMICRON_WITH_VARIA,
            GreekExtended::GreekSmallLetterOmicronWithOxia => GREEK_SMALL_LETTER_OMICRON_WITH_OXIA,
            GreekExtended::GreekSmallLetterUpsilonWithVaria => GREEK_SMALL_LETTER_UPSILON_WITH_VARIA,
            GreekExtended::GreekSmallLetterUpsilonWithOxia => GREEK_SMALL_LETTER_UPSILON_WITH_OXIA,
            GreekExtended::GreekSmallLetterOmegaWithVaria => GREEK_SMALL_LETTER_OMEGA_WITH_VARIA,
            GreekExtended::GreekSmallLetterOmegaWithOxia => GREEK_SMALL_LETTER_OMEGA_WITH_OXIA,
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndVariaAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndVariaAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVariaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVariaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeniAndProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeniAndProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithPsiliAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithDasiaAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithPsiliAndVariaAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithDasiaAndVariaAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithPsiliAndOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithDasiaAndOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndVariaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndVariaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeniAndProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeniAndProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndVariaAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndVariaAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVariaAndProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVariaAndProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxiaAndProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeniAndProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeniAndProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithVrachy => GREEK_SMALL_LETTER_ALPHA_WITH_VRACHY,
            GreekExtended::GreekSmallLetterAlphaWithMacron => GREEK_SMALL_LETTER_ALPHA_WITH_MACRON,
            GreekExtended::GreekSmallLetterAlphaWithVariaAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterAlphaWithPerispomeni => GREEK_SMALL_LETTER_ALPHA_WITH_PERISPOMENI,
            GreekExtended::GreekSmallLetterAlphaWithPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_ALPHA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekCapitalLetterAlphaWithVrachy => GREEK_CAPITAL_LETTER_ALPHA_WITH_VRACHY,
            GreekExtended::GreekCapitalLetterAlphaWithMacron => GREEK_CAPITAL_LETTER_ALPHA_WITH_MACRON,
            GreekExtended::GreekCapitalLetterAlphaWithVaria => GREEK_CAPITAL_LETTER_ALPHA_WITH_VARIA,
            GreekExtended::GreekCapitalLetterAlphaWithOxia => GREEK_CAPITAL_LETTER_ALPHA_WITH_OXIA,
            GreekExtended::GreekCapitalLetterAlphaWithProsgegrammeni => GREEK_CAPITAL_LETTER_ALPHA_WITH_PROSGEGRAMMENI,
            GreekExtended::GreekKoronis => GREEK_KORONIS,
            GreekExtended::GreekProsgegrammeni => GREEK_PROSGEGRAMMENI,
            GreekExtended::GreekPsili => GREEK_PSILI,
            GreekExtended::GreekPerispomeni => GREEK_PERISPOMENI,
            GreekExtended::GreekDialytikaAndPerispomeni => GREEK_DIALYTIKA_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterEtaWithVariaAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterEtaWithPerispomeni => GREEK_SMALL_LETTER_ETA_WITH_PERISPOMENI,
            GreekExtended::GreekSmallLetterEtaWithPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_ETA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekCapitalLetterEpsilonWithVaria => GREEK_CAPITAL_LETTER_EPSILON_WITH_VARIA,
            GreekExtended::GreekCapitalLetterEpsilonWithOxia => GREEK_CAPITAL_LETTER_EPSILON_WITH_OXIA,
            GreekExtended::GreekCapitalLetterEtaWithVaria => GREEK_CAPITAL_LETTER_ETA_WITH_VARIA,
            GreekExtended::GreekCapitalLetterEtaWithOxia => GREEK_CAPITAL_LETTER_ETA_WITH_OXIA,
            GreekExtended::GreekCapitalLetterEtaWithProsgegrammeni => GREEK_CAPITAL_LETTER_ETA_WITH_PROSGEGRAMMENI,
            GreekExtended::GreekPsiliAndVaria => GREEK_PSILI_AND_VARIA,
            GreekExtended::GreekPsiliAndOxia => GREEK_PSILI_AND_OXIA,
            GreekExtended::GreekPsiliAndPerispomeni => GREEK_PSILI_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterIotaWithVrachy => GREEK_SMALL_LETTER_IOTA_WITH_VRACHY,
            GreekExtended::GreekSmallLetterIotaWithMacron => GREEK_SMALL_LETTER_IOTA_WITH_MACRON,
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndVaria => GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_VARIA,
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndOxia => GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_OXIA,
            GreekExtended::GreekSmallLetterIotaWithPerispomeni => GREEK_SMALL_LETTER_IOTA_WITH_PERISPOMENI,
            GreekExtended::GreekSmallLetterIotaWithDialytikaAndPerispomeni => GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterIotaWithVrachy => GREEK_CAPITAL_LETTER_IOTA_WITH_VRACHY,
            GreekExtended::GreekCapitalLetterIotaWithMacron => GREEK_CAPITAL_LETTER_IOTA_WITH_MACRON,
            GreekExtended::GreekCapitalLetterIotaWithVaria => GREEK_CAPITAL_LETTER_IOTA_WITH_VARIA,
            GreekExtended::GreekCapitalLetterIotaWithOxia => GREEK_CAPITAL_LETTER_IOTA_WITH_OXIA,
            GreekExtended::GreekDasiaAndVaria => GREEK_DASIA_AND_VARIA,
            GreekExtended::GreekDasiaAndOxia => GREEK_DASIA_AND_OXIA,
            GreekExtended::GreekDasiaAndPerispomeni => GREEK_DASIA_AND_PERISPOMENI,
            GreekExtended::GreekSmallLetterUpsilonWithVrachy => GREEK_SMALL_LETTER_UPSILON_WITH_VRACHY,
            GreekExtended::GreekSmallLetterUpsilonWithMacron => GREEK_SMALL_LETTER_UPSILON_WITH_MACRON,
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndVaria => GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_VARIA,
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndOxia => GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_OXIA,
            GreekExtended::GreekSmallLetterRhoWithPsili => GREEK_SMALL_LETTER_RHO_WITH_PSILI,
            GreekExtended::GreekSmallLetterRhoWithDasia => GREEK_SMALL_LETTER_RHO_WITH_DASIA,
            GreekExtended::GreekSmallLetterUpsilonWithPerispomeni => GREEK_SMALL_LETTER_UPSILON_WITH_PERISPOMENI,
            GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndPerispomeni => GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_PERISPOMENI,
            GreekExtended::GreekCapitalLetterUpsilonWithVrachy => GREEK_CAPITAL_LETTER_UPSILON_WITH_VRACHY,
            GreekExtended::GreekCapitalLetterUpsilonWithMacron => GREEK_CAPITAL_LETTER_UPSILON_WITH_MACRON,
            GreekExtended::GreekCapitalLetterUpsilonWithVaria => GREEK_CAPITAL_LETTER_UPSILON_WITH_VARIA,
            GreekExtended::GreekCapitalLetterUpsilonWithOxia => GREEK_CAPITAL_LETTER_UPSILON_WITH_OXIA,
            GreekExtended::GreekCapitalLetterRhoWithDasia => GREEK_CAPITAL_LETTER_RHO_WITH_DASIA,
            GreekExtended::GreekDialytikaAndVaria => GREEK_DIALYTIKA_AND_VARIA,
            GreekExtended::GreekDialytikaAndOxia => GREEK_DIALYTIKA_AND_OXIA,
            GreekExtended::GreekVaria => GREEK_VARIA,
            GreekExtended::GreekSmallLetterOmegaWithVariaAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_VARIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithOxiaAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_OXIA_AND_YPOGEGRAMMENI,
            GreekExtended::GreekSmallLetterOmegaWithPerispomeni => GREEK_SMALL_LETTER_OMEGA_WITH_PERISPOMENI,
            GreekExtended::GreekSmallLetterOmegaWithPerispomeniAndYpogegrammeni => GREEK_SMALL_LETTER_OMEGA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI,
            GreekExtended::GreekCapitalLetterOmicronWithVaria => GREEK_CAPITAL_LETTER_OMICRON_WITH_VARIA,
            GreekExtended::GreekCapitalLetterOmicronWithOxia => GREEK_CAPITAL_LETTER_OMICRON_WITH_OXIA,
            GreekExtended::GreekCapitalLetterOmegaWithVaria => GREEK_CAPITAL_LETTER_OMEGA_WITH_VARIA,
            GreekExtended::GreekCapitalLetterOmegaWithOxia => GREEK_CAPITAL_LETTER_OMEGA_WITH_OXIA,
            GreekExtended::GreekCapitalLetterOmegaWithProsgegrammeni => GREEK_CAPITAL_LETTER_OMEGA_WITH_PROSGEGRAMMENI,
            GreekExtended::GreekOxia => GREEK_OXIA,
            GreekExtended::GreekDasia => GREEK_DASIA,
        }
    }
}

impl std::convert::TryFrom<char> for GreekExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            GREEK_SMALL_LETTER_ALPHA_WITH_PSILI => Ok(GreekExtended::GreekSmallLetterAlphaWithPsili),
            GREEK_SMALL_LETTER_ALPHA_WITH_DASIA => Ok(GreekExtended::GreekSmallLetterAlphaWithDasia),
            GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndVaria),
            GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndVaria),
            GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxia),
            GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxia),
            GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsili),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasia),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVaria),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVaria),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxia),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxia),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeni),
            GREEK_SMALL_LETTER_EPSILON_WITH_PSILI => Ok(GreekExtended::GreekSmallLetterEpsilonWithPsili),
            GREEK_SMALL_LETTER_EPSILON_WITH_DASIA => Ok(GreekExtended::GreekSmallLetterEpsilonWithDasia),
            GREEK_SMALL_LETTER_EPSILON_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekSmallLetterEpsilonWithPsiliAndVaria),
            GREEK_SMALL_LETTER_EPSILON_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterEpsilonWithDasiaAndVaria),
            GREEK_SMALL_LETTER_EPSILON_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekSmallLetterEpsilonWithPsiliAndOxia),
            GREEK_SMALL_LETTER_EPSILON_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterEpsilonWithDasiaAndOxia),
            GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI => Ok(GreekExtended::GreekCapitalLetterEpsilonWithPsili),
            GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA => Ok(GreekExtended::GreekCapitalLetterEpsilonWithDasia),
            GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndVaria),
            GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndVaria),
            GREEK_CAPITAL_LETTER_EPSILON_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterEpsilonWithPsiliAndOxia),
            GREEK_CAPITAL_LETTER_EPSILON_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterEpsilonWithDasiaAndOxia),
            GREEK_SMALL_LETTER_ETA_WITH_PSILI => Ok(GreekExtended::GreekSmallLetterEtaWithPsili),
            GREEK_SMALL_LETTER_ETA_WITH_DASIA => Ok(GreekExtended::GreekSmallLetterEtaWithDasia),
            GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndVaria),
            GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndVaria),
            GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndOxia),
            GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndOxia),
            GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeni),
            GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_PSILI => Ok(GreekExtended::GreekCapitalLetterEtaWithPsili),
            GREEK_CAPITAL_LETTER_ETA_WITH_DASIA => Ok(GreekExtended::GreekCapitalLetterEtaWithDasia),
            GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndVaria),
            GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndVaria),
            GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxia),
            GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxia),
            GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeni),
            GREEK_SMALL_LETTER_IOTA_WITH_PSILI => Ok(GreekExtended::GreekSmallLetterIotaWithPsili),
            GREEK_SMALL_LETTER_IOTA_WITH_DASIA => Ok(GreekExtended::GreekSmallLetterIotaWithDasia),
            GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekSmallLetterIotaWithPsiliAndVaria),
            GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterIotaWithDasiaAndVaria),
            GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekSmallLetterIotaWithPsiliAndOxia),
            GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterIotaWithDasiaAndOxia),
            GREEK_SMALL_LETTER_IOTA_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterIotaWithPsiliAndPerispomeni),
            GREEK_SMALL_LETTER_IOTA_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterIotaWithDasiaAndPerispomeni),
            GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI => Ok(GreekExtended::GreekCapitalLetterIotaWithPsili),
            GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA => Ok(GreekExtended::GreekCapitalLetterIotaWithDasia),
            GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterIotaWithPsiliAndVaria),
            GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterIotaWithDasiaAndVaria),
            GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterIotaWithPsiliAndOxia),
            GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterIotaWithDasiaAndOxia),
            GREEK_CAPITAL_LETTER_IOTA_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterIotaWithPsiliAndPerispomeni),
            GREEK_CAPITAL_LETTER_IOTA_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterIotaWithDasiaAndPerispomeni),
            GREEK_SMALL_LETTER_OMICRON_WITH_PSILI => Ok(GreekExtended::GreekSmallLetterOmicronWithPsili),
            GREEK_SMALL_LETTER_OMICRON_WITH_DASIA => Ok(GreekExtended::GreekSmallLetterOmicronWithDasia),
            GREEK_SMALL_LETTER_OMICRON_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekSmallLetterOmicronWithPsiliAndVaria),
            GREEK_SMALL_LETTER_OMICRON_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterOmicronWithDasiaAndVaria),
            GREEK_SMALL_LETTER_OMICRON_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekSmallLetterOmicronWithPsiliAndOxia),
            GREEK_SMALL_LETTER_OMICRON_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterOmicronWithDasiaAndOxia),
            GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI => Ok(GreekExtended::GreekCapitalLetterOmicronWithPsili),
            GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA => Ok(GreekExtended::GreekCapitalLetterOmicronWithDasia),
            GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterOmicronWithPsiliAndVaria),
            GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterOmicronWithDasiaAndVaria),
            GREEK_CAPITAL_LETTER_OMICRON_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterOmicronWithPsiliAndOxia),
            GREEK_CAPITAL_LETTER_OMICRON_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterOmicronWithDasiaAndOxia),
            GREEK_SMALL_LETTER_UPSILON_WITH_PSILI => Ok(GreekExtended::GreekSmallLetterUpsilonWithPsili),
            GREEK_SMALL_LETTER_UPSILON_WITH_DASIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithDasia),
            GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithPsiliAndVaria),
            GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithDasiaAndVaria),
            GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithPsiliAndOxia),
            GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithDasiaAndOxia),
            GREEK_SMALL_LETTER_UPSILON_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterUpsilonWithPsiliAndPerispomeni),
            GREEK_SMALL_LETTER_UPSILON_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterUpsilonWithDasiaAndPerispomeni),
            GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA => Ok(GreekExtended::GreekCapitalLetterUpsilonWithDasia),
            GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndVaria),
            GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndOxia),
            GREEK_CAPITAL_LETTER_UPSILON_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterUpsilonWithDasiaAndPerispomeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_PSILI => Ok(GreekExtended::GreekSmallLetterOmegaWithPsili),
            GREEK_SMALL_LETTER_OMEGA_WITH_DASIA => Ok(GreekExtended::GreekSmallLetterOmegaWithDasia),
            GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndVaria),
            GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndVaria),
            GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxia),
            GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxia),
            GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsili),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasia),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVaria),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_VARIA => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVaria),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxia),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_OXIA => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxia),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_VARIA => Ok(GreekExtended::GreekSmallLetterAlphaWithVaria),
            GREEK_SMALL_LETTER_ALPHA_WITH_OXIA => Ok(GreekExtended::GreekSmallLetterAlphaWithOxia),
            GREEK_SMALL_LETTER_EPSILON_WITH_VARIA => Ok(GreekExtended::GreekSmallLetterEpsilonWithVaria),
            GREEK_SMALL_LETTER_EPSILON_WITH_OXIA => Ok(GreekExtended::GreekSmallLetterEpsilonWithOxia),
            GREEK_SMALL_LETTER_ETA_WITH_VARIA => Ok(GreekExtended::GreekSmallLetterEtaWithVaria),
            GREEK_SMALL_LETTER_ETA_WITH_OXIA => Ok(GreekExtended::GreekSmallLetterEtaWithOxia),
            GREEK_SMALL_LETTER_IOTA_WITH_VARIA => Ok(GreekExtended::GreekSmallLetterIotaWithVaria),
            GREEK_SMALL_LETTER_IOTA_WITH_OXIA => Ok(GreekExtended::GreekSmallLetterIotaWithOxia),
            GREEK_SMALL_LETTER_OMICRON_WITH_VARIA => Ok(GreekExtended::GreekSmallLetterOmicronWithVaria),
            GREEK_SMALL_LETTER_OMICRON_WITH_OXIA => Ok(GreekExtended::GreekSmallLetterOmicronWithOxia),
            GREEK_SMALL_LETTER_UPSILON_WITH_VARIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithVaria),
            GREEK_SMALL_LETTER_UPSILON_WITH_OXIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithOxia),
            GREEK_SMALL_LETTER_OMEGA_WITH_VARIA => Ok(GreekExtended::GreekSmallLetterOmegaWithVaria),
            GREEK_SMALL_LETTER_OMEGA_WITH_OXIA => Ok(GreekExtended::GreekSmallLetterOmegaWithOxia),
            GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithPsiliAndPerispomeniAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithDasiaAndPerispomeniAndYpogegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndVariaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndVariaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndOxiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndOxiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithPsiliAndPerispomeniAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithDasiaAndPerispomeniAndProsgegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithPsiliAndPerispomeniAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithDasiaAndPerispomeniAndYpogegrammeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndVariaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndVariaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndOxiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndOxiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithPsiliAndPerispomeniAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_ETA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithDasiaAndPerispomeniAndProsgegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithPsiliAndPerispomeniAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithDasiaAndPerispomeniAndYpogegrammeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndVariaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndVariaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndOxiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndOxiaAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithPsiliAndPerispomeniAndProsgegrammeni),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithDasiaAndPerispomeniAndProsgegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_VRACHY => Ok(GreekExtended::GreekSmallLetterAlphaWithVrachy),
            GREEK_SMALL_LETTER_ALPHA_WITH_MACRON => Ok(GreekExtended::GreekSmallLetterAlphaWithMacron),
            GREEK_SMALL_LETTER_ALPHA_WITH_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithPerispomeni),
            GREEK_SMALL_LETTER_ALPHA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterAlphaWithPerispomeniAndYpogegrammeni),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_VRACHY => Ok(GreekExtended::GreekCapitalLetterAlphaWithVrachy),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_MACRON => Ok(GreekExtended::GreekCapitalLetterAlphaWithMacron),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_VARIA => Ok(GreekExtended::GreekCapitalLetterAlphaWithVaria),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_OXIA => Ok(GreekExtended::GreekCapitalLetterAlphaWithOxia),
            GREEK_CAPITAL_LETTER_ALPHA_WITH_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterAlphaWithProsgegrammeni),
            GREEK_KORONIS => Ok(GreekExtended::GreekKoronis),
            GREEK_PROSGEGRAMMENI => Ok(GreekExtended::GreekProsgegrammeni),
            GREEK_PSILI => Ok(GreekExtended::GreekPsili),
            GREEK_PERISPOMENI => Ok(GreekExtended::GreekPerispomeni),
            GREEK_DIALYTIKA_AND_PERISPOMENI => Ok(GreekExtended::GreekDialytikaAndPerispomeni),
            GREEK_SMALL_LETTER_ETA_WITH_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_ETA_WITH_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterEtaWithPerispomeni),
            GREEK_SMALL_LETTER_ETA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterEtaWithPerispomeniAndYpogegrammeni),
            GREEK_CAPITAL_LETTER_EPSILON_WITH_VARIA => Ok(GreekExtended::GreekCapitalLetterEpsilonWithVaria),
            GREEK_CAPITAL_LETTER_EPSILON_WITH_OXIA => Ok(GreekExtended::GreekCapitalLetterEpsilonWithOxia),
            GREEK_CAPITAL_LETTER_ETA_WITH_VARIA => Ok(GreekExtended::GreekCapitalLetterEtaWithVaria),
            GREEK_CAPITAL_LETTER_ETA_WITH_OXIA => Ok(GreekExtended::GreekCapitalLetterEtaWithOxia),
            GREEK_CAPITAL_LETTER_ETA_WITH_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterEtaWithProsgegrammeni),
            GREEK_PSILI_AND_VARIA => Ok(GreekExtended::GreekPsiliAndVaria),
            GREEK_PSILI_AND_OXIA => Ok(GreekExtended::GreekPsiliAndOxia),
            GREEK_PSILI_AND_PERISPOMENI => Ok(GreekExtended::GreekPsiliAndPerispomeni),
            GREEK_SMALL_LETTER_IOTA_WITH_VRACHY => Ok(GreekExtended::GreekSmallLetterIotaWithVrachy),
            GREEK_SMALL_LETTER_IOTA_WITH_MACRON => Ok(GreekExtended::GreekSmallLetterIotaWithMacron),
            GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterIotaWithDialytikaAndVaria),
            GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterIotaWithDialytikaAndOxia),
            GREEK_SMALL_LETTER_IOTA_WITH_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterIotaWithPerispomeni),
            GREEK_SMALL_LETTER_IOTA_WITH_DIALYTIKA_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterIotaWithDialytikaAndPerispomeni),
            GREEK_CAPITAL_LETTER_IOTA_WITH_VRACHY => Ok(GreekExtended::GreekCapitalLetterIotaWithVrachy),
            GREEK_CAPITAL_LETTER_IOTA_WITH_MACRON => Ok(GreekExtended::GreekCapitalLetterIotaWithMacron),
            GREEK_CAPITAL_LETTER_IOTA_WITH_VARIA => Ok(GreekExtended::GreekCapitalLetterIotaWithVaria),
            GREEK_CAPITAL_LETTER_IOTA_WITH_OXIA => Ok(GreekExtended::GreekCapitalLetterIotaWithOxia),
            GREEK_DASIA_AND_VARIA => Ok(GreekExtended::GreekDasiaAndVaria),
            GREEK_DASIA_AND_OXIA => Ok(GreekExtended::GreekDasiaAndOxia),
            GREEK_DASIA_AND_PERISPOMENI => Ok(GreekExtended::GreekDasiaAndPerispomeni),
            GREEK_SMALL_LETTER_UPSILON_WITH_VRACHY => Ok(GreekExtended::GreekSmallLetterUpsilonWithVrachy),
            GREEK_SMALL_LETTER_UPSILON_WITH_MACRON => Ok(GreekExtended::GreekSmallLetterUpsilonWithMacron),
            GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_VARIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndVaria),
            GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_OXIA => Ok(GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndOxia),
            GREEK_SMALL_LETTER_RHO_WITH_PSILI => Ok(GreekExtended::GreekSmallLetterRhoWithPsili),
            GREEK_SMALL_LETTER_RHO_WITH_DASIA => Ok(GreekExtended::GreekSmallLetterRhoWithDasia),
            GREEK_SMALL_LETTER_UPSILON_WITH_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterUpsilonWithPerispomeni),
            GREEK_SMALL_LETTER_UPSILON_WITH_DIALYTIKA_AND_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterUpsilonWithDialytikaAndPerispomeni),
            GREEK_CAPITAL_LETTER_UPSILON_WITH_VRACHY => Ok(GreekExtended::GreekCapitalLetterUpsilonWithVrachy),
            GREEK_CAPITAL_LETTER_UPSILON_WITH_MACRON => Ok(GreekExtended::GreekCapitalLetterUpsilonWithMacron),
            GREEK_CAPITAL_LETTER_UPSILON_WITH_VARIA => Ok(GreekExtended::GreekCapitalLetterUpsilonWithVaria),
            GREEK_CAPITAL_LETTER_UPSILON_WITH_OXIA => Ok(GreekExtended::GreekCapitalLetterUpsilonWithOxia),
            GREEK_CAPITAL_LETTER_RHO_WITH_DASIA => Ok(GreekExtended::GreekCapitalLetterRhoWithDasia),
            GREEK_DIALYTIKA_AND_VARIA => Ok(GreekExtended::GreekDialytikaAndVaria),
            GREEK_DIALYTIKA_AND_OXIA => Ok(GreekExtended::GreekDialytikaAndOxia),
            GREEK_VARIA => Ok(GreekExtended::GreekVaria),
            GREEK_SMALL_LETTER_OMEGA_WITH_VARIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithVariaAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_OXIA_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithOxiaAndYpogegrammeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_PERISPOMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithPerispomeni),
            GREEK_SMALL_LETTER_OMEGA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI => Ok(GreekExtended::GreekSmallLetterOmegaWithPerispomeniAndYpogegrammeni),
            GREEK_CAPITAL_LETTER_OMICRON_WITH_VARIA => Ok(GreekExtended::GreekCapitalLetterOmicronWithVaria),
            GREEK_CAPITAL_LETTER_OMICRON_WITH_OXIA => Ok(GreekExtended::GreekCapitalLetterOmicronWithOxia),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_VARIA => Ok(GreekExtended::GreekCapitalLetterOmegaWithVaria),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_OXIA => Ok(GreekExtended::GreekCapitalLetterOmegaWithOxia),
            GREEK_CAPITAL_LETTER_OMEGA_WITH_PROSGEGRAMMENI => Ok(GreekExtended::GreekCapitalLetterOmegaWithProsgegrammeni),
            GREEK_OXIA => Ok(GreekExtended::GreekOxia),
            GREEK_DASIA => Ok(GreekExtended::GreekDasia),
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
