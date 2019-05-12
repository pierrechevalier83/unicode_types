
/// An enum to represent all characters in the Cyrillic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Cyrillic {
    /// \u{400}: 'Ѐ'
    CapitalLetterIeWithGrave,
    /// \u{401}: 'Ё'
    CapitalLetterIo,
    /// \u{402}: 'Ђ'
    CapitalLetterDje,
    /// \u{403}: 'Ѓ'
    CapitalLetterGje,
    /// \u{404}: 'Є'
    CapitalLetterUkrainianIe,
    /// \u{405}: 'Ѕ'
    CapitalLetterDze,
    /// \u{406}: 'І'
    CapitalLetterByelorussianDashUkrainianI,
    /// \u{407}: 'Ї'
    CapitalLetterYi,
    /// \u{408}: 'Ј'
    CapitalLetterJe,
    /// \u{409}: 'Љ'
    CapitalLetterLje,
    /// \u{40a}: 'Њ'
    CapitalLetterNje,
    /// \u{40b}: 'Ћ'
    CapitalLetterTshe,
    /// \u{40c}: 'Ќ'
    CapitalLetterKje,
    /// \u{40d}: 'Ѝ'
    CapitalLetterIWithGrave,
    /// \u{40e}: 'Ў'
    CapitalLetterShortU,
    /// \u{40f}: 'Џ'
    CapitalLetterDzhe,
    /// \u{410}: 'А'
    CapitalLetterA,
    /// \u{411}: 'Б'
    CapitalLetterBe,
    /// \u{412}: 'В'
    CapitalLetterVe,
    /// \u{413}: 'Г'
    CapitalLetterGhe,
    /// \u{414}: 'Д'
    CapitalLetterDe,
    /// \u{415}: 'Е'
    CapitalLetterIe,
    /// \u{416}: 'Ж'
    CapitalLetterZhe,
    /// \u{417}: 'З'
    CapitalLetterZe,
    /// \u{418}: 'И'
    CapitalLetterI,
    /// \u{419}: 'Й'
    CapitalLetterShortI,
    /// \u{41a}: 'К'
    CapitalLetterKa,
    /// \u{41b}: 'Л'
    CapitalLetterEl,
    /// \u{41c}: 'М'
    CapitalLetterEm,
    /// \u{41d}: 'Н'
    CapitalLetterEn,
    /// \u{41e}: 'О'
    CapitalLetterO,
    /// \u{41f}: 'П'
    CapitalLetterPe,
    /// \u{420}: 'Р'
    CapitalLetterEr,
    /// \u{421}: 'С'
    CapitalLetterEs,
    /// \u{422}: 'Т'
    CapitalLetterTe,
    /// \u{423}: 'У'
    CapitalLetterU,
    /// \u{424}: 'Ф'
    CapitalLetterEf,
    /// \u{425}: 'Х'
    CapitalLetterHa,
    /// \u{426}: 'Ц'
    CapitalLetterTse,
    /// \u{427}: 'Ч'
    CapitalLetterChe,
    /// \u{428}: 'Ш'
    CapitalLetterSha,
    /// \u{429}: 'Щ'
    CapitalLetterShcha,
    /// \u{42a}: 'Ъ'
    CapitalLetterHardSign,
    /// \u{42b}: 'Ы'
    CapitalLetterYeru,
    /// \u{42c}: 'Ь'
    CapitalLetterSoftSign,
    /// \u{42d}: 'Э'
    CapitalLetterE,
    /// \u{42e}: 'Ю'
    CapitalLetterYu,
    /// \u{42f}: 'Я'
    CapitalLetterYa,
    /// \u{430}: 'а'
    SmallLetterA,
    /// \u{431}: 'б'
    SmallLetterBe,
    /// \u{432}: 'в'
    SmallLetterVe,
    /// \u{433}: 'г'
    SmallLetterGhe,
    /// \u{434}: 'д'
    SmallLetterDe,
    /// \u{435}: 'е'
    SmallLetterIe,
    /// \u{436}: 'ж'
    SmallLetterZhe,
    /// \u{437}: 'з'
    SmallLetterZe,
    /// \u{438}: 'и'
    SmallLetterI,
    /// \u{439}: 'й'
    SmallLetterShortI,
    /// \u{43a}: 'к'
    SmallLetterKa,
    /// \u{43b}: 'л'
    SmallLetterEl,
    /// \u{43c}: 'м'
    SmallLetterEm,
    /// \u{43d}: 'н'
    SmallLetterEn,
    /// \u{43e}: 'о'
    SmallLetterO,
    /// \u{43f}: 'п'
    SmallLetterPe,
    /// \u{440}: 'р'
    SmallLetterEr,
    /// \u{441}: 'с'
    SmallLetterEs,
    /// \u{442}: 'т'
    SmallLetterTe,
    /// \u{443}: 'у'
    SmallLetterU,
    /// \u{444}: 'ф'
    SmallLetterEf,
    /// \u{445}: 'х'
    SmallLetterHa,
    /// \u{446}: 'ц'
    SmallLetterTse,
    /// \u{447}: 'ч'
    SmallLetterChe,
    /// \u{448}: 'ш'
    SmallLetterSha,
    /// \u{449}: 'щ'
    SmallLetterShcha,
    /// \u{44a}: 'ъ'
    SmallLetterHardSign,
    /// \u{44b}: 'ы'
    SmallLetterYeru,
    /// \u{44c}: 'ь'
    SmallLetterSoftSign,
    /// \u{44d}: 'э'
    SmallLetterE,
    /// \u{44e}: 'ю'
    SmallLetterYu,
    /// \u{44f}: 'я'
    SmallLetterYa,
    /// \u{450}: 'ѐ'
    SmallLetterIeWithGrave,
    /// \u{451}: 'ё'
    SmallLetterIo,
    /// \u{452}: 'ђ'
    SmallLetterDje,
    /// \u{453}: 'ѓ'
    SmallLetterGje,
    /// \u{454}: 'є'
    SmallLetterUkrainianIe,
    /// \u{455}: 'ѕ'
    SmallLetterDze,
    /// \u{456}: 'і'
    SmallLetterByelorussianDashUkrainianI,
    /// \u{457}: 'ї'
    SmallLetterYi,
    /// \u{458}: 'ј'
    SmallLetterJe,
    /// \u{459}: 'љ'
    SmallLetterLje,
    /// \u{45a}: 'њ'
    SmallLetterNje,
    /// \u{45b}: 'ћ'
    SmallLetterTshe,
    /// \u{45c}: 'ќ'
    SmallLetterKje,
    /// \u{45d}: 'ѝ'
    SmallLetterIWithGrave,
    /// \u{45e}: 'ў'
    SmallLetterShortU,
    /// \u{45f}: 'џ'
    SmallLetterDzhe,
    /// \u{460}: 'Ѡ'
    CapitalLetterOmega,
    /// \u{461}: 'ѡ'
    SmallLetterOmega,
    /// \u{462}: 'Ѣ'
    CapitalLetterYat,
    /// \u{463}: 'ѣ'
    SmallLetterYat,
    /// \u{464}: 'Ѥ'
    CapitalLetterIotifiedE,
    /// \u{465}: 'ѥ'
    SmallLetterIotifiedE,
    /// \u{466}: 'Ѧ'
    CapitalLetterLittleYus,
    /// \u{467}: 'ѧ'
    SmallLetterLittleYus,
    /// \u{468}: 'Ѩ'
    CapitalLetterIotifiedLittleYus,
    /// \u{469}: 'ѩ'
    SmallLetterIotifiedLittleYus,
    /// \u{46a}: 'Ѫ'
    CapitalLetterBigYus,
    /// \u{46b}: 'ѫ'
    SmallLetterBigYus,
    /// \u{46c}: 'Ѭ'
    CapitalLetterIotifiedBigYus,
    /// \u{46d}: 'ѭ'
    SmallLetterIotifiedBigYus,
    /// \u{46e}: 'Ѯ'
    CapitalLetterKsi,
    /// \u{46f}: 'ѯ'
    SmallLetterKsi,
    /// \u{470}: 'Ѱ'
    CapitalLetterPsi,
    /// \u{471}: 'ѱ'
    SmallLetterPsi,
    /// \u{472}: 'Ѳ'
    CapitalLetterFita,
    /// \u{473}: 'ѳ'
    SmallLetterFita,
    /// \u{474}: 'Ѵ'
    CapitalLetterIzhitsa,
    /// \u{475}: 'ѵ'
    SmallLetterIzhitsa,
    /// \u{476}: 'Ѷ'
    CapitalLetterIzhitsaWithDoubleGraveAccent,
    /// \u{477}: 'ѷ'
    SmallLetterIzhitsaWithDoubleGraveAccent,
    /// \u{478}: 'Ѹ'
    CapitalLetterUk,
    /// \u{479}: 'ѹ'
    SmallLetterUk,
    /// \u{47a}: 'Ѻ'
    CapitalLetterRoundOmega,
    /// \u{47b}: 'ѻ'
    SmallLetterRoundOmega,
    /// \u{47c}: 'Ѽ'
    CapitalLetterOmegaWithTitlo,
    /// \u{47d}: 'ѽ'
    SmallLetterOmegaWithTitlo,
    /// \u{47e}: 'Ѿ'
    CapitalLetterOt,
    /// \u{47f}: 'ѿ'
    SmallLetterOt,
    /// \u{480}: 'Ҁ'
    CapitalLetterKoppa,
    /// \u{481}: 'ҁ'
    SmallLetterKoppa,
    /// \u{482}: '҂'
    ThousandsSign,
    /// \u{483}: '҃'
    CombiningTitlo,
    /// \u{484}: '҄'
    CombiningPalatalization,
    /// \u{485}: '҅'
    CombiningDasiaPneumata,
    /// \u{486}: '҆'
    CombiningPsiliPneumata,
    /// \u{487}: '҇'
    CombiningPokrytie,
    /// \u{488}: '҈'
    CombiningHundredThousandsSign,
    /// \u{489}: '҉'
    CombiningMillionsSign,
    /// \u{48a}: 'Ҋ'
    CapitalLetterShortIWithTail,
    /// \u{48b}: 'ҋ'
    SmallLetterShortIWithTail,
    /// \u{48c}: 'Ҍ'
    CapitalLetterSemisoftSign,
    /// \u{48d}: 'ҍ'
    SmallLetterSemisoftSign,
    /// \u{48e}: 'Ҏ'
    CapitalLetterErWithTick,
    /// \u{48f}: 'ҏ'
    SmallLetterErWithTick,
    /// \u{490}: 'Ґ'
    CapitalLetterGheWithUpturn,
    /// \u{491}: 'ґ'
    SmallLetterGheWithUpturn,
    /// \u{492}: 'Ғ'
    CapitalLetterGheWithStroke,
    /// \u{493}: 'ғ'
    SmallLetterGheWithStroke,
    /// \u{494}: 'Ҕ'
    CapitalLetterGheWithMiddleHook,
    /// \u{495}: 'ҕ'
    SmallLetterGheWithMiddleHook,
    /// \u{496}: 'Җ'
    CapitalLetterZheWithDescender,
    /// \u{497}: 'җ'
    SmallLetterZheWithDescender,
    /// \u{498}: 'Ҙ'
    CapitalLetterZeWithDescender,
    /// \u{499}: 'ҙ'
    SmallLetterZeWithDescender,
    /// \u{49a}: 'Қ'
    CapitalLetterKaWithDescender,
    /// \u{49b}: 'қ'
    SmallLetterKaWithDescender,
    /// \u{49c}: 'Ҝ'
    CapitalLetterKaWithVerticalStroke,
    /// \u{49d}: 'ҝ'
    SmallLetterKaWithVerticalStroke,
    /// \u{49e}: 'Ҟ'
    CapitalLetterKaWithStroke,
    /// \u{49f}: 'ҟ'
    SmallLetterKaWithStroke,
    /// \u{4a0}: 'Ҡ'
    CapitalLetterBashkirKa,
    /// \u{4a1}: 'ҡ'
    SmallLetterBashkirKa,
    /// \u{4a2}: 'Ң'
    CapitalLetterEnWithDescender,
    /// \u{4a3}: 'ң'
    SmallLetterEnWithDescender,
    /// \u{4a4}: 'Ҥ'
    CapitalLigatureEnGhe,
    /// \u{4a5}: 'ҥ'
    SmallLigatureEnGhe,
    /// \u{4a6}: 'Ҧ'
    CapitalLetterPeWithMiddleHook,
    /// \u{4a7}: 'ҧ'
    SmallLetterPeWithMiddleHook,
    /// \u{4a8}: 'Ҩ'
    CapitalLetterAbkhasianHa,
    /// \u{4a9}: 'ҩ'
    SmallLetterAbkhasianHa,
    /// \u{4aa}: 'Ҫ'
    CapitalLetterEsWithDescender,
    /// \u{4ab}: 'ҫ'
    SmallLetterEsWithDescender,
    /// \u{4ac}: 'Ҭ'
    CapitalLetterTeWithDescender,
    /// \u{4ad}: 'ҭ'
    SmallLetterTeWithDescender,
    /// \u{4ae}: 'Ү'
    CapitalLetterStraightU,
    /// \u{4af}: 'ү'
    SmallLetterStraightU,
    /// \u{4b0}: 'Ұ'
    CapitalLetterStraightUWithStroke,
    /// \u{4b1}: 'ұ'
    SmallLetterStraightUWithStroke,
    /// \u{4b2}: 'Ҳ'
    CapitalLetterHaWithDescender,
    /// \u{4b3}: 'ҳ'
    SmallLetterHaWithDescender,
    /// \u{4b4}: 'Ҵ'
    CapitalLigatureTeTse,
    /// \u{4b5}: 'ҵ'
    SmallLigatureTeTse,
    /// \u{4b6}: 'Ҷ'
    CapitalLetterCheWithDescender,
    /// \u{4b7}: 'ҷ'
    SmallLetterCheWithDescender,
    /// \u{4b8}: 'Ҹ'
    CapitalLetterCheWithVerticalStroke,
    /// \u{4b9}: 'ҹ'
    SmallLetterCheWithVerticalStroke,
    /// \u{4ba}: 'Һ'
    CapitalLetterShha,
    /// \u{4bb}: 'һ'
    SmallLetterShha,
    /// \u{4bc}: 'Ҽ'
    CapitalLetterAbkhasianChe,
    /// \u{4bd}: 'ҽ'
    SmallLetterAbkhasianChe,
    /// \u{4be}: 'Ҿ'
    CapitalLetterAbkhasianCheWithDescender,
    /// \u{4bf}: 'ҿ'
    SmallLetterAbkhasianCheWithDescender,
    /// \u{4c0}: 'Ӏ'
    LetterPalochka,
    /// \u{4c1}: 'Ӂ'
    CapitalLetterZheWithBreve,
    /// \u{4c2}: 'ӂ'
    SmallLetterZheWithBreve,
    /// \u{4c3}: 'Ӄ'
    CapitalLetterKaWithHook,
    /// \u{4c4}: 'ӄ'
    SmallLetterKaWithHook,
    /// \u{4c5}: 'Ӆ'
    CapitalLetterElWithTail,
    /// \u{4c6}: 'ӆ'
    SmallLetterElWithTail,
    /// \u{4c7}: 'Ӈ'
    CapitalLetterEnWithHook,
    /// \u{4c8}: 'ӈ'
    SmallLetterEnWithHook,
    /// \u{4c9}: 'Ӊ'
    CapitalLetterEnWithTail,
    /// \u{4ca}: 'ӊ'
    SmallLetterEnWithTail,
    /// \u{4cb}: 'Ӌ'
    CapitalLetterKhakassianChe,
    /// \u{4cc}: 'ӌ'
    SmallLetterKhakassianChe,
    /// \u{4cd}: 'Ӎ'
    CapitalLetterEmWithTail,
    /// \u{4ce}: 'ӎ'
    SmallLetterEmWithTail,
    /// \u{4cf}: 'ӏ'
    SmallLetterPalochka,
    /// \u{4d0}: 'Ӑ'
    CapitalLetterAWithBreve,
    /// \u{4d1}: 'ӑ'
    SmallLetterAWithBreve,
    /// \u{4d2}: 'Ӓ'
    CapitalLetterAWithDiaeresis,
    /// \u{4d3}: 'ӓ'
    SmallLetterAWithDiaeresis,
    /// \u{4d4}: 'Ӕ'
    CapitalLigatureAIe,
    /// \u{4d5}: 'ӕ'
    SmallLigatureAIe,
    /// \u{4d6}: 'Ӗ'
    CapitalLetterIeWithBreve,
    /// \u{4d7}: 'ӗ'
    SmallLetterIeWithBreve,
    /// \u{4d8}: 'Ә'
    CapitalLetterSchwa,
    /// \u{4d9}: 'ә'
    SmallLetterSchwa,
    /// \u{4da}: 'Ӛ'
    CapitalLetterSchwaWithDiaeresis,
    /// \u{4db}: 'ӛ'
    SmallLetterSchwaWithDiaeresis,
    /// \u{4dc}: 'Ӝ'
    CapitalLetterZheWithDiaeresis,
    /// \u{4dd}: 'ӝ'
    SmallLetterZheWithDiaeresis,
    /// \u{4de}: 'Ӟ'
    CapitalLetterZeWithDiaeresis,
    /// \u{4df}: 'ӟ'
    SmallLetterZeWithDiaeresis,
    /// \u{4e0}: 'Ӡ'
    CapitalLetterAbkhasianDze,
    /// \u{4e1}: 'ӡ'
    SmallLetterAbkhasianDze,
    /// \u{4e2}: 'Ӣ'
    CapitalLetterIWithMacron,
    /// \u{4e3}: 'ӣ'
    SmallLetterIWithMacron,
    /// \u{4e4}: 'Ӥ'
    CapitalLetterIWithDiaeresis,
    /// \u{4e5}: 'ӥ'
    SmallLetterIWithDiaeresis,
    /// \u{4e6}: 'Ӧ'
    CapitalLetterOWithDiaeresis,
    /// \u{4e7}: 'ӧ'
    SmallLetterOWithDiaeresis,
    /// \u{4e8}: 'Ө'
    CapitalLetterBarredO,
    /// \u{4e9}: 'ө'
    SmallLetterBarredO,
    /// \u{4ea}: 'Ӫ'
    CapitalLetterBarredOWithDiaeresis,
    /// \u{4eb}: 'ӫ'
    SmallLetterBarredOWithDiaeresis,
    /// \u{4ec}: 'Ӭ'
    CapitalLetterEWithDiaeresis,
    /// \u{4ed}: 'ӭ'
    SmallLetterEWithDiaeresis,
    /// \u{4ee}: 'Ӯ'
    CapitalLetterUWithMacron,
    /// \u{4ef}: 'ӯ'
    SmallLetterUWithMacron,
    /// \u{4f0}: 'Ӱ'
    CapitalLetterUWithDiaeresis,
    /// \u{4f1}: 'ӱ'
    SmallLetterUWithDiaeresis,
    /// \u{4f2}: 'Ӳ'
    CapitalLetterUWithDoubleAcute,
    /// \u{4f3}: 'ӳ'
    SmallLetterUWithDoubleAcute,
    /// \u{4f4}: 'Ӵ'
    CapitalLetterCheWithDiaeresis,
    /// \u{4f5}: 'ӵ'
    SmallLetterCheWithDiaeresis,
    /// \u{4f6}: 'Ӷ'
    CapitalLetterGheWithDescender,
    /// \u{4f7}: 'ӷ'
    SmallLetterGheWithDescender,
    /// \u{4f8}: 'Ӹ'
    CapitalLetterYeruWithDiaeresis,
    /// \u{4f9}: 'ӹ'
    SmallLetterYeruWithDiaeresis,
    /// \u{4fa}: 'Ӻ'
    CapitalLetterGheWithStrokeAndHook,
    /// \u{4fb}: 'ӻ'
    SmallLetterGheWithStrokeAndHook,
    /// \u{4fc}: 'Ӽ'
    CapitalLetterHaWithHook,
    /// \u{4fd}: 'ӽ'
    SmallLetterHaWithHook,
    /// \u{4fe}: 'Ӿ'
    CapitalLetterHaWithStroke,
}

impl Into<char> for Cyrillic {
    fn into(self) -> char {
        match self {
            Cyrillic::CapitalLetterIeWithGrave => 'Ѐ',
            Cyrillic::CapitalLetterIo => 'Ё',
            Cyrillic::CapitalLetterDje => 'Ђ',
            Cyrillic::CapitalLetterGje => 'Ѓ',
            Cyrillic::CapitalLetterUkrainianIe => 'Є',
            Cyrillic::CapitalLetterDze => 'Ѕ',
            Cyrillic::CapitalLetterByelorussianDashUkrainianI => 'І',
            Cyrillic::CapitalLetterYi => 'Ї',
            Cyrillic::CapitalLetterJe => 'Ј',
            Cyrillic::CapitalLetterLje => 'Љ',
            Cyrillic::CapitalLetterNje => 'Њ',
            Cyrillic::CapitalLetterTshe => 'Ћ',
            Cyrillic::CapitalLetterKje => 'Ќ',
            Cyrillic::CapitalLetterIWithGrave => 'Ѝ',
            Cyrillic::CapitalLetterShortU => 'Ў',
            Cyrillic::CapitalLetterDzhe => 'Џ',
            Cyrillic::CapitalLetterA => 'А',
            Cyrillic::CapitalLetterBe => 'Б',
            Cyrillic::CapitalLetterVe => 'В',
            Cyrillic::CapitalLetterGhe => 'Г',
            Cyrillic::CapitalLetterDe => 'Д',
            Cyrillic::CapitalLetterIe => 'Е',
            Cyrillic::CapitalLetterZhe => 'Ж',
            Cyrillic::CapitalLetterZe => 'З',
            Cyrillic::CapitalLetterI => 'И',
            Cyrillic::CapitalLetterShortI => 'Й',
            Cyrillic::CapitalLetterKa => 'К',
            Cyrillic::CapitalLetterEl => 'Л',
            Cyrillic::CapitalLetterEm => 'М',
            Cyrillic::CapitalLetterEn => 'Н',
            Cyrillic::CapitalLetterO => 'О',
            Cyrillic::CapitalLetterPe => 'П',
            Cyrillic::CapitalLetterEr => 'Р',
            Cyrillic::CapitalLetterEs => 'С',
            Cyrillic::CapitalLetterTe => 'Т',
            Cyrillic::CapitalLetterU => 'У',
            Cyrillic::CapitalLetterEf => 'Ф',
            Cyrillic::CapitalLetterHa => 'Х',
            Cyrillic::CapitalLetterTse => 'Ц',
            Cyrillic::CapitalLetterChe => 'Ч',
            Cyrillic::CapitalLetterSha => 'Ш',
            Cyrillic::CapitalLetterShcha => 'Щ',
            Cyrillic::CapitalLetterHardSign => 'Ъ',
            Cyrillic::CapitalLetterYeru => 'Ы',
            Cyrillic::CapitalLetterSoftSign => 'Ь',
            Cyrillic::CapitalLetterE => 'Э',
            Cyrillic::CapitalLetterYu => 'Ю',
            Cyrillic::CapitalLetterYa => 'Я',
            Cyrillic::SmallLetterA => 'а',
            Cyrillic::SmallLetterBe => 'б',
            Cyrillic::SmallLetterVe => 'в',
            Cyrillic::SmallLetterGhe => 'г',
            Cyrillic::SmallLetterDe => 'д',
            Cyrillic::SmallLetterIe => 'е',
            Cyrillic::SmallLetterZhe => 'ж',
            Cyrillic::SmallLetterZe => 'з',
            Cyrillic::SmallLetterI => 'и',
            Cyrillic::SmallLetterShortI => 'й',
            Cyrillic::SmallLetterKa => 'к',
            Cyrillic::SmallLetterEl => 'л',
            Cyrillic::SmallLetterEm => 'м',
            Cyrillic::SmallLetterEn => 'н',
            Cyrillic::SmallLetterO => 'о',
            Cyrillic::SmallLetterPe => 'п',
            Cyrillic::SmallLetterEr => 'р',
            Cyrillic::SmallLetterEs => 'с',
            Cyrillic::SmallLetterTe => 'т',
            Cyrillic::SmallLetterU => 'у',
            Cyrillic::SmallLetterEf => 'ф',
            Cyrillic::SmallLetterHa => 'х',
            Cyrillic::SmallLetterTse => 'ц',
            Cyrillic::SmallLetterChe => 'ч',
            Cyrillic::SmallLetterSha => 'ш',
            Cyrillic::SmallLetterShcha => 'щ',
            Cyrillic::SmallLetterHardSign => 'ъ',
            Cyrillic::SmallLetterYeru => 'ы',
            Cyrillic::SmallLetterSoftSign => 'ь',
            Cyrillic::SmallLetterE => 'э',
            Cyrillic::SmallLetterYu => 'ю',
            Cyrillic::SmallLetterYa => 'я',
            Cyrillic::SmallLetterIeWithGrave => 'ѐ',
            Cyrillic::SmallLetterIo => 'ё',
            Cyrillic::SmallLetterDje => 'ђ',
            Cyrillic::SmallLetterGje => 'ѓ',
            Cyrillic::SmallLetterUkrainianIe => 'є',
            Cyrillic::SmallLetterDze => 'ѕ',
            Cyrillic::SmallLetterByelorussianDashUkrainianI => 'і',
            Cyrillic::SmallLetterYi => 'ї',
            Cyrillic::SmallLetterJe => 'ј',
            Cyrillic::SmallLetterLje => 'љ',
            Cyrillic::SmallLetterNje => 'њ',
            Cyrillic::SmallLetterTshe => 'ћ',
            Cyrillic::SmallLetterKje => 'ќ',
            Cyrillic::SmallLetterIWithGrave => 'ѝ',
            Cyrillic::SmallLetterShortU => 'ў',
            Cyrillic::SmallLetterDzhe => 'џ',
            Cyrillic::CapitalLetterOmega => 'Ѡ',
            Cyrillic::SmallLetterOmega => 'ѡ',
            Cyrillic::CapitalLetterYat => 'Ѣ',
            Cyrillic::SmallLetterYat => 'ѣ',
            Cyrillic::CapitalLetterIotifiedE => 'Ѥ',
            Cyrillic::SmallLetterIotifiedE => 'ѥ',
            Cyrillic::CapitalLetterLittleYus => 'Ѧ',
            Cyrillic::SmallLetterLittleYus => 'ѧ',
            Cyrillic::CapitalLetterIotifiedLittleYus => 'Ѩ',
            Cyrillic::SmallLetterIotifiedLittleYus => 'ѩ',
            Cyrillic::CapitalLetterBigYus => 'Ѫ',
            Cyrillic::SmallLetterBigYus => 'ѫ',
            Cyrillic::CapitalLetterIotifiedBigYus => 'Ѭ',
            Cyrillic::SmallLetterIotifiedBigYus => 'ѭ',
            Cyrillic::CapitalLetterKsi => 'Ѯ',
            Cyrillic::SmallLetterKsi => 'ѯ',
            Cyrillic::CapitalLetterPsi => 'Ѱ',
            Cyrillic::SmallLetterPsi => 'ѱ',
            Cyrillic::CapitalLetterFita => 'Ѳ',
            Cyrillic::SmallLetterFita => 'ѳ',
            Cyrillic::CapitalLetterIzhitsa => 'Ѵ',
            Cyrillic::SmallLetterIzhitsa => 'ѵ',
            Cyrillic::CapitalLetterIzhitsaWithDoubleGraveAccent => 'Ѷ',
            Cyrillic::SmallLetterIzhitsaWithDoubleGraveAccent => 'ѷ',
            Cyrillic::CapitalLetterUk => 'Ѹ',
            Cyrillic::SmallLetterUk => 'ѹ',
            Cyrillic::CapitalLetterRoundOmega => 'Ѻ',
            Cyrillic::SmallLetterRoundOmega => 'ѻ',
            Cyrillic::CapitalLetterOmegaWithTitlo => 'Ѽ',
            Cyrillic::SmallLetterOmegaWithTitlo => 'ѽ',
            Cyrillic::CapitalLetterOt => 'Ѿ',
            Cyrillic::SmallLetterOt => 'ѿ',
            Cyrillic::CapitalLetterKoppa => 'Ҁ',
            Cyrillic::SmallLetterKoppa => 'ҁ',
            Cyrillic::ThousandsSign => '҂',
            Cyrillic::CombiningTitlo => '҃',
            Cyrillic::CombiningPalatalization => '҄',
            Cyrillic::CombiningDasiaPneumata => '҅',
            Cyrillic::CombiningPsiliPneumata => '҆',
            Cyrillic::CombiningPokrytie => '҇',
            Cyrillic::CombiningHundredThousandsSign => '҈',
            Cyrillic::CombiningMillionsSign => '҉',
            Cyrillic::CapitalLetterShortIWithTail => 'Ҋ',
            Cyrillic::SmallLetterShortIWithTail => 'ҋ',
            Cyrillic::CapitalLetterSemisoftSign => 'Ҍ',
            Cyrillic::SmallLetterSemisoftSign => 'ҍ',
            Cyrillic::CapitalLetterErWithTick => 'Ҏ',
            Cyrillic::SmallLetterErWithTick => 'ҏ',
            Cyrillic::CapitalLetterGheWithUpturn => 'Ґ',
            Cyrillic::SmallLetterGheWithUpturn => 'ґ',
            Cyrillic::CapitalLetterGheWithStroke => 'Ғ',
            Cyrillic::SmallLetterGheWithStroke => 'ғ',
            Cyrillic::CapitalLetterGheWithMiddleHook => 'Ҕ',
            Cyrillic::SmallLetterGheWithMiddleHook => 'ҕ',
            Cyrillic::CapitalLetterZheWithDescender => 'Җ',
            Cyrillic::SmallLetterZheWithDescender => 'җ',
            Cyrillic::CapitalLetterZeWithDescender => 'Ҙ',
            Cyrillic::SmallLetterZeWithDescender => 'ҙ',
            Cyrillic::CapitalLetterKaWithDescender => 'Қ',
            Cyrillic::SmallLetterKaWithDescender => 'қ',
            Cyrillic::CapitalLetterKaWithVerticalStroke => 'Ҝ',
            Cyrillic::SmallLetterKaWithVerticalStroke => 'ҝ',
            Cyrillic::CapitalLetterKaWithStroke => 'Ҟ',
            Cyrillic::SmallLetterKaWithStroke => 'ҟ',
            Cyrillic::CapitalLetterBashkirKa => 'Ҡ',
            Cyrillic::SmallLetterBashkirKa => 'ҡ',
            Cyrillic::CapitalLetterEnWithDescender => 'Ң',
            Cyrillic::SmallLetterEnWithDescender => 'ң',
            Cyrillic::CapitalLigatureEnGhe => 'Ҥ',
            Cyrillic::SmallLigatureEnGhe => 'ҥ',
            Cyrillic::CapitalLetterPeWithMiddleHook => 'Ҧ',
            Cyrillic::SmallLetterPeWithMiddleHook => 'ҧ',
            Cyrillic::CapitalLetterAbkhasianHa => 'Ҩ',
            Cyrillic::SmallLetterAbkhasianHa => 'ҩ',
            Cyrillic::CapitalLetterEsWithDescender => 'Ҫ',
            Cyrillic::SmallLetterEsWithDescender => 'ҫ',
            Cyrillic::CapitalLetterTeWithDescender => 'Ҭ',
            Cyrillic::SmallLetterTeWithDescender => 'ҭ',
            Cyrillic::CapitalLetterStraightU => 'Ү',
            Cyrillic::SmallLetterStraightU => 'ү',
            Cyrillic::CapitalLetterStraightUWithStroke => 'Ұ',
            Cyrillic::SmallLetterStraightUWithStroke => 'ұ',
            Cyrillic::CapitalLetterHaWithDescender => 'Ҳ',
            Cyrillic::SmallLetterHaWithDescender => 'ҳ',
            Cyrillic::CapitalLigatureTeTse => 'Ҵ',
            Cyrillic::SmallLigatureTeTse => 'ҵ',
            Cyrillic::CapitalLetterCheWithDescender => 'Ҷ',
            Cyrillic::SmallLetterCheWithDescender => 'ҷ',
            Cyrillic::CapitalLetterCheWithVerticalStroke => 'Ҹ',
            Cyrillic::SmallLetterCheWithVerticalStroke => 'ҹ',
            Cyrillic::CapitalLetterShha => 'Һ',
            Cyrillic::SmallLetterShha => 'һ',
            Cyrillic::CapitalLetterAbkhasianChe => 'Ҽ',
            Cyrillic::SmallLetterAbkhasianChe => 'ҽ',
            Cyrillic::CapitalLetterAbkhasianCheWithDescender => 'Ҿ',
            Cyrillic::SmallLetterAbkhasianCheWithDescender => 'ҿ',
            Cyrillic::LetterPalochka => 'Ӏ',
            Cyrillic::CapitalLetterZheWithBreve => 'Ӂ',
            Cyrillic::SmallLetterZheWithBreve => 'ӂ',
            Cyrillic::CapitalLetterKaWithHook => 'Ӄ',
            Cyrillic::SmallLetterKaWithHook => 'ӄ',
            Cyrillic::CapitalLetterElWithTail => 'Ӆ',
            Cyrillic::SmallLetterElWithTail => 'ӆ',
            Cyrillic::CapitalLetterEnWithHook => 'Ӈ',
            Cyrillic::SmallLetterEnWithHook => 'ӈ',
            Cyrillic::CapitalLetterEnWithTail => 'Ӊ',
            Cyrillic::SmallLetterEnWithTail => 'ӊ',
            Cyrillic::CapitalLetterKhakassianChe => 'Ӌ',
            Cyrillic::SmallLetterKhakassianChe => 'ӌ',
            Cyrillic::CapitalLetterEmWithTail => 'Ӎ',
            Cyrillic::SmallLetterEmWithTail => 'ӎ',
            Cyrillic::SmallLetterPalochka => 'ӏ',
            Cyrillic::CapitalLetterAWithBreve => 'Ӑ',
            Cyrillic::SmallLetterAWithBreve => 'ӑ',
            Cyrillic::CapitalLetterAWithDiaeresis => 'Ӓ',
            Cyrillic::SmallLetterAWithDiaeresis => 'ӓ',
            Cyrillic::CapitalLigatureAIe => 'Ӕ',
            Cyrillic::SmallLigatureAIe => 'ӕ',
            Cyrillic::CapitalLetterIeWithBreve => 'Ӗ',
            Cyrillic::SmallLetterIeWithBreve => 'ӗ',
            Cyrillic::CapitalLetterSchwa => 'Ә',
            Cyrillic::SmallLetterSchwa => 'ә',
            Cyrillic::CapitalLetterSchwaWithDiaeresis => 'Ӛ',
            Cyrillic::SmallLetterSchwaWithDiaeresis => 'ӛ',
            Cyrillic::CapitalLetterZheWithDiaeresis => 'Ӝ',
            Cyrillic::SmallLetterZheWithDiaeresis => 'ӝ',
            Cyrillic::CapitalLetterZeWithDiaeresis => 'Ӟ',
            Cyrillic::SmallLetterZeWithDiaeresis => 'ӟ',
            Cyrillic::CapitalLetterAbkhasianDze => 'Ӡ',
            Cyrillic::SmallLetterAbkhasianDze => 'ӡ',
            Cyrillic::CapitalLetterIWithMacron => 'Ӣ',
            Cyrillic::SmallLetterIWithMacron => 'ӣ',
            Cyrillic::CapitalLetterIWithDiaeresis => 'Ӥ',
            Cyrillic::SmallLetterIWithDiaeresis => 'ӥ',
            Cyrillic::CapitalLetterOWithDiaeresis => 'Ӧ',
            Cyrillic::SmallLetterOWithDiaeresis => 'ӧ',
            Cyrillic::CapitalLetterBarredO => 'Ө',
            Cyrillic::SmallLetterBarredO => 'ө',
            Cyrillic::CapitalLetterBarredOWithDiaeresis => 'Ӫ',
            Cyrillic::SmallLetterBarredOWithDiaeresis => 'ӫ',
            Cyrillic::CapitalLetterEWithDiaeresis => 'Ӭ',
            Cyrillic::SmallLetterEWithDiaeresis => 'ӭ',
            Cyrillic::CapitalLetterUWithMacron => 'Ӯ',
            Cyrillic::SmallLetterUWithMacron => 'ӯ',
            Cyrillic::CapitalLetterUWithDiaeresis => 'Ӱ',
            Cyrillic::SmallLetterUWithDiaeresis => 'ӱ',
            Cyrillic::CapitalLetterUWithDoubleAcute => 'Ӳ',
            Cyrillic::SmallLetterUWithDoubleAcute => 'ӳ',
            Cyrillic::CapitalLetterCheWithDiaeresis => 'Ӵ',
            Cyrillic::SmallLetterCheWithDiaeresis => 'ӵ',
            Cyrillic::CapitalLetterGheWithDescender => 'Ӷ',
            Cyrillic::SmallLetterGheWithDescender => 'ӷ',
            Cyrillic::CapitalLetterYeruWithDiaeresis => 'Ӹ',
            Cyrillic::SmallLetterYeruWithDiaeresis => 'ӹ',
            Cyrillic::CapitalLetterGheWithStrokeAndHook => 'Ӻ',
            Cyrillic::SmallLetterGheWithStrokeAndHook => 'ӻ',
            Cyrillic::CapitalLetterHaWithHook => 'Ӽ',
            Cyrillic::SmallLetterHaWithHook => 'ӽ',
            Cyrillic::CapitalLetterHaWithStroke => 'Ӿ',
        }
    }
}

impl std::convert::TryFrom<char> for Cyrillic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ѐ' => Ok(Cyrillic::CapitalLetterIeWithGrave),
            'Ё' => Ok(Cyrillic::CapitalLetterIo),
            'Ђ' => Ok(Cyrillic::CapitalLetterDje),
            'Ѓ' => Ok(Cyrillic::CapitalLetterGje),
            'Є' => Ok(Cyrillic::CapitalLetterUkrainianIe),
            'Ѕ' => Ok(Cyrillic::CapitalLetterDze),
            'І' => Ok(Cyrillic::CapitalLetterByelorussianDashUkrainianI),
            'Ї' => Ok(Cyrillic::CapitalLetterYi),
            'Ј' => Ok(Cyrillic::CapitalLetterJe),
            'Љ' => Ok(Cyrillic::CapitalLetterLje),
            'Њ' => Ok(Cyrillic::CapitalLetterNje),
            'Ћ' => Ok(Cyrillic::CapitalLetterTshe),
            'Ќ' => Ok(Cyrillic::CapitalLetterKje),
            'Ѝ' => Ok(Cyrillic::CapitalLetterIWithGrave),
            'Ў' => Ok(Cyrillic::CapitalLetterShortU),
            'Џ' => Ok(Cyrillic::CapitalLetterDzhe),
            'А' => Ok(Cyrillic::CapitalLetterA),
            'Б' => Ok(Cyrillic::CapitalLetterBe),
            'В' => Ok(Cyrillic::CapitalLetterVe),
            'Г' => Ok(Cyrillic::CapitalLetterGhe),
            'Д' => Ok(Cyrillic::CapitalLetterDe),
            'Е' => Ok(Cyrillic::CapitalLetterIe),
            'Ж' => Ok(Cyrillic::CapitalLetterZhe),
            'З' => Ok(Cyrillic::CapitalLetterZe),
            'И' => Ok(Cyrillic::CapitalLetterI),
            'Й' => Ok(Cyrillic::CapitalLetterShortI),
            'К' => Ok(Cyrillic::CapitalLetterKa),
            'Л' => Ok(Cyrillic::CapitalLetterEl),
            'М' => Ok(Cyrillic::CapitalLetterEm),
            'Н' => Ok(Cyrillic::CapitalLetterEn),
            'О' => Ok(Cyrillic::CapitalLetterO),
            'П' => Ok(Cyrillic::CapitalLetterPe),
            'Р' => Ok(Cyrillic::CapitalLetterEr),
            'С' => Ok(Cyrillic::CapitalLetterEs),
            'Т' => Ok(Cyrillic::CapitalLetterTe),
            'У' => Ok(Cyrillic::CapitalLetterU),
            'Ф' => Ok(Cyrillic::CapitalLetterEf),
            'Х' => Ok(Cyrillic::CapitalLetterHa),
            'Ц' => Ok(Cyrillic::CapitalLetterTse),
            'Ч' => Ok(Cyrillic::CapitalLetterChe),
            'Ш' => Ok(Cyrillic::CapitalLetterSha),
            'Щ' => Ok(Cyrillic::CapitalLetterShcha),
            'Ъ' => Ok(Cyrillic::CapitalLetterHardSign),
            'Ы' => Ok(Cyrillic::CapitalLetterYeru),
            'Ь' => Ok(Cyrillic::CapitalLetterSoftSign),
            'Э' => Ok(Cyrillic::CapitalLetterE),
            'Ю' => Ok(Cyrillic::CapitalLetterYu),
            'Я' => Ok(Cyrillic::CapitalLetterYa),
            'а' => Ok(Cyrillic::SmallLetterA),
            'б' => Ok(Cyrillic::SmallLetterBe),
            'в' => Ok(Cyrillic::SmallLetterVe),
            'г' => Ok(Cyrillic::SmallLetterGhe),
            'д' => Ok(Cyrillic::SmallLetterDe),
            'е' => Ok(Cyrillic::SmallLetterIe),
            'ж' => Ok(Cyrillic::SmallLetterZhe),
            'з' => Ok(Cyrillic::SmallLetterZe),
            'и' => Ok(Cyrillic::SmallLetterI),
            'й' => Ok(Cyrillic::SmallLetterShortI),
            'к' => Ok(Cyrillic::SmallLetterKa),
            'л' => Ok(Cyrillic::SmallLetterEl),
            'м' => Ok(Cyrillic::SmallLetterEm),
            'н' => Ok(Cyrillic::SmallLetterEn),
            'о' => Ok(Cyrillic::SmallLetterO),
            'п' => Ok(Cyrillic::SmallLetterPe),
            'р' => Ok(Cyrillic::SmallLetterEr),
            'с' => Ok(Cyrillic::SmallLetterEs),
            'т' => Ok(Cyrillic::SmallLetterTe),
            'у' => Ok(Cyrillic::SmallLetterU),
            'ф' => Ok(Cyrillic::SmallLetterEf),
            'х' => Ok(Cyrillic::SmallLetterHa),
            'ц' => Ok(Cyrillic::SmallLetterTse),
            'ч' => Ok(Cyrillic::SmallLetterChe),
            'ш' => Ok(Cyrillic::SmallLetterSha),
            'щ' => Ok(Cyrillic::SmallLetterShcha),
            'ъ' => Ok(Cyrillic::SmallLetterHardSign),
            'ы' => Ok(Cyrillic::SmallLetterYeru),
            'ь' => Ok(Cyrillic::SmallLetterSoftSign),
            'э' => Ok(Cyrillic::SmallLetterE),
            'ю' => Ok(Cyrillic::SmallLetterYu),
            'я' => Ok(Cyrillic::SmallLetterYa),
            'ѐ' => Ok(Cyrillic::SmallLetterIeWithGrave),
            'ё' => Ok(Cyrillic::SmallLetterIo),
            'ђ' => Ok(Cyrillic::SmallLetterDje),
            'ѓ' => Ok(Cyrillic::SmallLetterGje),
            'є' => Ok(Cyrillic::SmallLetterUkrainianIe),
            'ѕ' => Ok(Cyrillic::SmallLetterDze),
            'і' => Ok(Cyrillic::SmallLetterByelorussianDashUkrainianI),
            'ї' => Ok(Cyrillic::SmallLetterYi),
            'ј' => Ok(Cyrillic::SmallLetterJe),
            'љ' => Ok(Cyrillic::SmallLetterLje),
            'њ' => Ok(Cyrillic::SmallLetterNje),
            'ћ' => Ok(Cyrillic::SmallLetterTshe),
            'ќ' => Ok(Cyrillic::SmallLetterKje),
            'ѝ' => Ok(Cyrillic::SmallLetterIWithGrave),
            'ў' => Ok(Cyrillic::SmallLetterShortU),
            'џ' => Ok(Cyrillic::SmallLetterDzhe),
            'Ѡ' => Ok(Cyrillic::CapitalLetterOmega),
            'ѡ' => Ok(Cyrillic::SmallLetterOmega),
            'Ѣ' => Ok(Cyrillic::CapitalLetterYat),
            'ѣ' => Ok(Cyrillic::SmallLetterYat),
            'Ѥ' => Ok(Cyrillic::CapitalLetterIotifiedE),
            'ѥ' => Ok(Cyrillic::SmallLetterIotifiedE),
            'Ѧ' => Ok(Cyrillic::CapitalLetterLittleYus),
            'ѧ' => Ok(Cyrillic::SmallLetterLittleYus),
            'Ѩ' => Ok(Cyrillic::CapitalLetterIotifiedLittleYus),
            'ѩ' => Ok(Cyrillic::SmallLetterIotifiedLittleYus),
            'Ѫ' => Ok(Cyrillic::CapitalLetterBigYus),
            'ѫ' => Ok(Cyrillic::SmallLetterBigYus),
            'Ѭ' => Ok(Cyrillic::CapitalLetterIotifiedBigYus),
            'ѭ' => Ok(Cyrillic::SmallLetterIotifiedBigYus),
            'Ѯ' => Ok(Cyrillic::CapitalLetterKsi),
            'ѯ' => Ok(Cyrillic::SmallLetterKsi),
            'Ѱ' => Ok(Cyrillic::CapitalLetterPsi),
            'ѱ' => Ok(Cyrillic::SmallLetterPsi),
            'Ѳ' => Ok(Cyrillic::CapitalLetterFita),
            'ѳ' => Ok(Cyrillic::SmallLetterFita),
            'Ѵ' => Ok(Cyrillic::CapitalLetterIzhitsa),
            'ѵ' => Ok(Cyrillic::SmallLetterIzhitsa),
            'Ѷ' => Ok(Cyrillic::CapitalLetterIzhitsaWithDoubleGraveAccent),
            'ѷ' => Ok(Cyrillic::SmallLetterIzhitsaWithDoubleGraveAccent),
            'Ѹ' => Ok(Cyrillic::CapitalLetterUk),
            'ѹ' => Ok(Cyrillic::SmallLetterUk),
            'Ѻ' => Ok(Cyrillic::CapitalLetterRoundOmega),
            'ѻ' => Ok(Cyrillic::SmallLetterRoundOmega),
            'Ѽ' => Ok(Cyrillic::CapitalLetterOmegaWithTitlo),
            'ѽ' => Ok(Cyrillic::SmallLetterOmegaWithTitlo),
            'Ѿ' => Ok(Cyrillic::CapitalLetterOt),
            'ѿ' => Ok(Cyrillic::SmallLetterOt),
            'Ҁ' => Ok(Cyrillic::CapitalLetterKoppa),
            'ҁ' => Ok(Cyrillic::SmallLetterKoppa),
            '҂' => Ok(Cyrillic::ThousandsSign),
            '҃' => Ok(Cyrillic::CombiningTitlo),
            '҄' => Ok(Cyrillic::CombiningPalatalization),
            '҅' => Ok(Cyrillic::CombiningDasiaPneumata),
            '҆' => Ok(Cyrillic::CombiningPsiliPneumata),
            '҇' => Ok(Cyrillic::CombiningPokrytie),
            '҈' => Ok(Cyrillic::CombiningHundredThousandsSign),
            '҉' => Ok(Cyrillic::CombiningMillionsSign),
            'Ҋ' => Ok(Cyrillic::CapitalLetterShortIWithTail),
            'ҋ' => Ok(Cyrillic::SmallLetterShortIWithTail),
            'Ҍ' => Ok(Cyrillic::CapitalLetterSemisoftSign),
            'ҍ' => Ok(Cyrillic::SmallLetterSemisoftSign),
            'Ҏ' => Ok(Cyrillic::CapitalLetterErWithTick),
            'ҏ' => Ok(Cyrillic::SmallLetterErWithTick),
            'Ґ' => Ok(Cyrillic::CapitalLetterGheWithUpturn),
            'ґ' => Ok(Cyrillic::SmallLetterGheWithUpturn),
            'Ғ' => Ok(Cyrillic::CapitalLetterGheWithStroke),
            'ғ' => Ok(Cyrillic::SmallLetterGheWithStroke),
            'Ҕ' => Ok(Cyrillic::CapitalLetterGheWithMiddleHook),
            'ҕ' => Ok(Cyrillic::SmallLetterGheWithMiddleHook),
            'Җ' => Ok(Cyrillic::CapitalLetterZheWithDescender),
            'җ' => Ok(Cyrillic::SmallLetterZheWithDescender),
            'Ҙ' => Ok(Cyrillic::CapitalLetterZeWithDescender),
            'ҙ' => Ok(Cyrillic::SmallLetterZeWithDescender),
            'Қ' => Ok(Cyrillic::CapitalLetterKaWithDescender),
            'қ' => Ok(Cyrillic::SmallLetterKaWithDescender),
            'Ҝ' => Ok(Cyrillic::CapitalLetterKaWithVerticalStroke),
            'ҝ' => Ok(Cyrillic::SmallLetterKaWithVerticalStroke),
            'Ҟ' => Ok(Cyrillic::CapitalLetterKaWithStroke),
            'ҟ' => Ok(Cyrillic::SmallLetterKaWithStroke),
            'Ҡ' => Ok(Cyrillic::CapitalLetterBashkirKa),
            'ҡ' => Ok(Cyrillic::SmallLetterBashkirKa),
            'Ң' => Ok(Cyrillic::CapitalLetterEnWithDescender),
            'ң' => Ok(Cyrillic::SmallLetterEnWithDescender),
            'Ҥ' => Ok(Cyrillic::CapitalLigatureEnGhe),
            'ҥ' => Ok(Cyrillic::SmallLigatureEnGhe),
            'Ҧ' => Ok(Cyrillic::CapitalLetterPeWithMiddleHook),
            'ҧ' => Ok(Cyrillic::SmallLetterPeWithMiddleHook),
            'Ҩ' => Ok(Cyrillic::CapitalLetterAbkhasianHa),
            'ҩ' => Ok(Cyrillic::SmallLetterAbkhasianHa),
            'Ҫ' => Ok(Cyrillic::CapitalLetterEsWithDescender),
            'ҫ' => Ok(Cyrillic::SmallLetterEsWithDescender),
            'Ҭ' => Ok(Cyrillic::CapitalLetterTeWithDescender),
            'ҭ' => Ok(Cyrillic::SmallLetterTeWithDescender),
            'Ү' => Ok(Cyrillic::CapitalLetterStraightU),
            'ү' => Ok(Cyrillic::SmallLetterStraightU),
            'Ұ' => Ok(Cyrillic::CapitalLetterStraightUWithStroke),
            'ұ' => Ok(Cyrillic::SmallLetterStraightUWithStroke),
            'Ҳ' => Ok(Cyrillic::CapitalLetterHaWithDescender),
            'ҳ' => Ok(Cyrillic::SmallLetterHaWithDescender),
            'Ҵ' => Ok(Cyrillic::CapitalLigatureTeTse),
            'ҵ' => Ok(Cyrillic::SmallLigatureTeTse),
            'Ҷ' => Ok(Cyrillic::CapitalLetterCheWithDescender),
            'ҷ' => Ok(Cyrillic::SmallLetterCheWithDescender),
            'Ҹ' => Ok(Cyrillic::CapitalLetterCheWithVerticalStroke),
            'ҹ' => Ok(Cyrillic::SmallLetterCheWithVerticalStroke),
            'Һ' => Ok(Cyrillic::CapitalLetterShha),
            'һ' => Ok(Cyrillic::SmallLetterShha),
            'Ҽ' => Ok(Cyrillic::CapitalLetterAbkhasianChe),
            'ҽ' => Ok(Cyrillic::SmallLetterAbkhasianChe),
            'Ҿ' => Ok(Cyrillic::CapitalLetterAbkhasianCheWithDescender),
            'ҿ' => Ok(Cyrillic::SmallLetterAbkhasianCheWithDescender),
            'Ӏ' => Ok(Cyrillic::LetterPalochka),
            'Ӂ' => Ok(Cyrillic::CapitalLetterZheWithBreve),
            'ӂ' => Ok(Cyrillic::SmallLetterZheWithBreve),
            'Ӄ' => Ok(Cyrillic::CapitalLetterKaWithHook),
            'ӄ' => Ok(Cyrillic::SmallLetterKaWithHook),
            'Ӆ' => Ok(Cyrillic::CapitalLetterElWithTail),
            'ӆ' => Ok(Cyrillic::SmallLetterElWithTail),
            'Ӈ' => Ok(Cyrillic::CapitalLetterEnWithHook),
            'ӈ' => Ok(Cyrillic::SmallLetterEnWithHook),
            'Ӊ' => Ok(Cyrillic::CapitalLetterEnWithTail),
            'ӊ' => Ok(Cyrillic::SmallLetterEnWithTail),
            'Ӌ' => Ok(Cyrillic::CapitalLetterKhakassianChe),
            'ӌ' => Ok(Cyrillic::SmallLetterKhakassianChe),
            'Ӎ' => Ok(Cyrillic::CapitalLetterEmWithTail),
            'ӎ' => Ok(Cyrillic::SmallLetterEmWithTail),
            'ӏ' => Ok(Cyrillic::SmallLetterPalochka),
            'Ӑ' => Ok(Cyrillic::CapitalLetterAWithBreve),
            'ӑ' => Ok(Cyrillic::SmallLetterAWithBreve),
            'Ӓ' => Ok(Cyrillic::CapitalLetterAWithDiaeresis),
            'ӓ' => Ok(Cyrillic::SmallLetterAWithDiaeresis),
            'Ӕ' => Ok(Cyrillic::CapitalLigatureAIe),
            'ӕ' => Ok(Cyrillic::SmallLigatureAIe),
            'Ӗ' => Ok(Cyrillic::CapitalLetterIeWithBreve),
            'ӗ' => Ok(Cyrillic::SmallLetterIeWithBreve),
            'Ә' => Ok(Cyrillic::CapitalLetterSchwa),
            'ә' => Ok(Cyrillic::SmallLetterSchwa),
            'Ӛ' => Ok(Cyrillic::CapitalLetterSchwaWithDiaeresis),
            'ӛ' => Ok(Cyrillic::SmallLetterSchwaWithDiaeresis),
            'Ӝ' => Ok(Cyrillic::CapitalLetterZheWithDiaeresis),
            'ӝ' => Ok(Cyrillic::SmallLetterZheWithDiaeresis),
            'Ӟ' => Ok(Cyrillic::CapitalLetterZeWithDiaeresis),
            'ӟ' => Ok(Cyrillic::SmallLetterZeWithDiaeresis),
            'Ӡ' => Ok(Cyrillic::CapitalLetterAbkhasianDze),
            'ӡ' => Ok(Cyrillic::SmallLetterAbkhasianDze),
            'Ӣ' => Ok(Cyrillic::CapitalLetterIWithMacron),
            'ӣ' => Ok(Cyrillic::SmallLetterIWithMacron),
            'Ӥ' => Ok(Cyrillic::CapitalLetterIWithDiaeresis),
            'ӥ' => Ok(Cyrillic::SmallLetterIWithDiaeresis),
            'Ӧ' => Ok(Cyrillic::CapitalLetterOWithDiaeresis),
            'ӧ' => Ok(Cyrillic::SmallLetterOWithDiaeresis),
            'Ө' => Ok(Cyrillic::CapitalLetterBarredO),
            'ө' => Ok(Cyrillic::SmallLetterBarredO),
            'Ӫ' => Ok(Cyrillic::CapitalLetterBarredOWithDiaeresis),
            'ӫ' => Ok(Cyrillic::SmallLetterBarredOWithDiaeresis),
            'Ӭ' => Ok(Cyrillic::CapitalLetterEWithDiaeresis),
            'ӭ' => Ok(Cyrillic::SmallLetterEWithDiaeresis),
            'Ӯ' => Ok(Cyrillic::CapitalLetterUWithMacron),
            'ӯ' => Ok(Cyrillic::SmallLetterUWithMacron),
            'Ӱ' => Ok(Cyrillic::CapitalLetterUWithDiaeresis),
            'ӱ' => Ok(Cyrillic::SmallLetterUWithDiaeresis),
            'Ӳ' => Ok(Cyrillic::CapitalLetterUWithDoubleAcute),
            'ӳ' => Ok(Cyrillic::SmallLetterUWithDoubleAcute),
            'Ӵ' => Ok(Cyrillic::CapitalLetterCheWithDiaeresis),
            'ӵ' => Ok(Cyrillic::SmallLetterCheWithDiaeresis),
            'Ӷ' => Ok(Cyrillic::CapitalLetterGheWithDescender),
            'ӷ' => Ok(Cyrillic::SmallLetterGheWithDescender),
            'Ӹ' => Ok(Cyrillic::CapitalLetterYeruWithDiaeresis),
            'ӹ' => Ok(Cyrillic::SmallLetterYeruWithDiaeresis),
            'Ӻ' => Ok(Cyrillic::CapitalLetterGheWithStrokeAndHook),
            'ӻ' => Ok(Cyrillic::SmallLetterGheWithStrokeAndHook),
            'Ӽ' => Ok(Cyrillic::CapitalLetterHaWithHook),
            'ӽ' => Ok(Cyrillic::SmallLetterHaWithHook),
            'Ӿ' => Ok(Cyrillic::CapitalLetterHaWithStroke),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Cyrillic {
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

impl std::convert::TryFrom<u32> for Cyrillic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Cyrillic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Cyrillic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Cyrillic::CapitalLetterIeWithGrave
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Cyrillic{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
