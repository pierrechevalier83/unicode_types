
/// An enum to represent all characters in the Vai block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Vai {
    /// \u{a500}: 'ꔀ'
    SyllableEe,
    /// \u{a501}: 'ꔁ'
    SyllableEen,
    /// \u{a502}: 'ꔂ'
    SyllableHee,
    /// \u{a503}: 'ꔃ'
    SyllableWee,
    /// \u{a504}: 'ꔄ'
    SyllableWeen,
    /// \u{a505}: 'ꔅ'
    SyllablePee,
    /// \u{a506}: 'ꔆ'
    SyllableBhee,
    /// \u{a507}: 'ꔇ'
    SyllableBee,
    /// \u{a508}: 'ꔈ'
    SyllableMbee,
    /// \u{a509}: 'ꔉ'
    SyllableKpee,
    /// \u{a50a}: 'ꔊ'
    SyllableMgbee,
    /// \u{a50b}: 'ꔋ'
    SyllableGbee,
    /// \u{a50c}: 'ꔌ'
    SyllableFee,
    /// \u{a50d}: 'ꔍ'
    SyllableVee,
    /// \u{a50e}: 'ꔎ'
    SyllableTee,
    /// \u{a50f}: 'ꔏ'
    SyllableThee,
    /// \u{a510}: 'ꔐ'
    SyllableDhee,
    /// \u{a511}: 'ꔑ'
    SyllableDhhee,
    /// \u{a512}: 'ꔒ'
    SyllableLee,
    /// \u{a513}: 'ꔓ'
    SyllableRee,
    /// \u{a514}: 'ꔔ'
    SyllableDee,
    /// \u{a515}: 'ꔕ'
    SyllableNdee,
    /// \u{a516}: 'ꔖ'
    SyllableSee,
    /// \u{a517}: 'ꔗ'
    SyllableShee,
    /// \u{a518}: 'ꔘ'
    SyllableZee,
    /// \u{a519}: 'ꔙ'
    SyllableZhee,
    /// \u{a51a}: 'ꔚ'
    SyllableCee,
    /// \u{a51b}: 'ꔛ'
    SyllableJee,
    /// \u{a51c}: 'ꔜ'
    SyllableNjee,
    /// \u{a51d}: 'ꔝ'
    SyllableYee,
    /// \u{a51e}: 'ꔞ'
    SyllableKee,
    /// \u{a51f}: 'ꔟ'
    SyllableNggee,
    /// \u{a520}: 'ꔠ'
    SyllableGee,
    /// \u{a521}: 'ꔡ'
    SyllableMee,
    /// \u{a522}: 'ꔢ'
    SyllableNee,
    /// \u{a523}: 'ꔣ'
    SyllableNyee,
    /// \u{a524}: 'ꔤ'
    SyllableI,
    /// \u{a525}: 'ꔥ'
    SyllableIn,
    /// \u{a526}: 'ꔦ'
    SyllableHi,
    /// \u{a527}: 'ꔧ'
    SyllableHin,
    /// \u{a528}: 'ꔨ'
    SyllableWi,
    /// \u{a529}: 'ꔩ'
    SyllableWin,
    /// \u{a52a}: 'ꔪ'
    SyllablePi,
    /// \u{a52b}: 'ꔫ'
    SyllableBhi,
    /// \u{a52c}: 'ꔬ'
    SyllableBi,
    /// \u{a52d}: 'ꔭ'
    SyllableMbi,
    /// \u{a52e}: 'ꔮ'
    SyllableKpi,
    /// \u{a52f}: 'ꔯ'
    SyllableMgbi,
    /// \u{a530}: 'ꔰ'
    SyllableGbi,
    /// \u{a531}: 'ꔱ'
    SyllableFi,
    /// \u{a532}: 'ꔲ'
    SyllableVi,
    /// \u{a533}: 'ꔳ'
    SyllableTi,
    /// \u{a534}: 'ꔴ'
    SyllableThi,
    /// \u{a535}: 'ꔵ'
    SyllableDhi,
    /// \u{a536}: 'ꔶ'
    SyllableDhhi,
    /// \u{a537}: 'ꔷ'
    SyllableLi,
    /// \u{a538}: 'ꔸ'
    SyllableRi,
    /// \u{a539}: 'ꔹ'
    SyllableDi,
    /// \u{a53a}: 'ꔺ'
    SyllableNdi,
    /// \u{a53b}: 'ꔻ'
    SyllableSi,
    /// \u{a53c}: 'ꔼ'
    SyllableShi,
    /// \u{a53d}: 'ꔽ'
    SyllableZi,
    /// \u{a53e}: 'ꔾ'
    SyllableZhi,
    /// \u{a53f}: 'ꔿ'
    SyllableCi,
    /// \u{a540}: 'ꕀ'
    SyllableJi,
    /// \u{a541}: 'ꕁ'
    SyllableNji,
    /// \u{a542}: 'ꕂ'
    SyllableYi,
    /// \u{a543}: 'ꕃ'
    SyllableKi,
    /// \u{a544}: 'ꕄ'
    SyllableNggi,
    /// \u{a545}: 'ꕅ'
    SyllableGi,
    /// \u{a546}: 'ꕆ'
    SyllableMi,
    /// \u{a547}: 'ꕇ'
    SyllableNi,
    /// \u{a548}: 'ꕈ'
    SyllableNyi,
    /// \u{a549}: 'ꕉ'
    SyllableA,
    /// \u{a54a}: 'ꕊ'
    SyllableAn,
    /// \u{a54b}: 'ꕋ'
    SyllableNgan,
    /// \u{a54c}: 'ꕌ'
    SyllableHa,
    /// \u{a54d}: 'ꕍ'
    SyllableHan,
    /// \u{a54e}: 'ꕎ'
    SyllableWa,
    /// \u{a54f}: 'ꕏ'
    SyllableWan,
    /// \u{a550}: 'ꕐ'
    SyllablePa,
    /// \u{a551}: 'ꕑ'
    SyllableBha,
    /// \u{a552}: 'ꕒ'
    SyllableBa,
    /// \u{a553}: 'ꕓ'
    SyllableMba,
    /// \u{a554}: 'ꕔ'
    SyllableKpa,
    /// \u{a555}: 'ꕕ'
    SyllableKpan,
    /// \u{a556}: 'ꕖ'
    SyllableMgba,
    /// \u{a557}: 'ꕗ'
    SyllableGba,
    /// \u{a558}: 'ꕘ'
    SyllableFa,
    /// \u{a559}: 'ꕙ'
    SyllableVa,
    /// \u{a55a}: 'ꕚ'
    SyllableTa,
    /// \u{a55b}: 'ꕛ'
    SyllableTha,
    /// \u{a55c}: 'ꕜ'
    SyllableDha,
    /// \u{a55d}: 'ꕝ'
    SyllableDhha,
    /// \u{a55e}: 'ꕞ'
    SyllableLa,
    /// \u{a55f}: 'ꕟ'
    SyllableRa,
    /// \u{a560}: 'ꕠ'
    SyllableDa,
    /// \u{a561}: 'ꕡ'
    SyllableNda,
    /// \u{a562}: 'ꕢ'
    SyllableSa,
    /// \u{a563}: 'ꕣ'
    SyllableSha,
    /// \u{a564}: 'ꕤ'
    SyllableZa,
    /// \u{a565}: 'ꕥ'
    SyllableZha,
    /// \u{a566}: 'ꕦ'
    SyllableCa,
    /// \u{a567}: 'ꕧ'
    SyllableJa,
    /// \u{a568}: 'ꕨ'
    SyllableNja,
    /// \u{a569}: 'ꕩ'
    SyllableYa,
    /// \u{a56a}: 'ꕪ'
    SyllableKa,
    /// \u{a56b}: 'ꕫ'
    SyllableKan,
    /// \u{a56c}: 'ꕬ'
    SyllableNgga,
    /// \u{a56d}: 'ꕭ'
    SyllableGa,
    /// \u{a56e}: 'ꕮ'
    SyllableMa,
    /// \u{a56f}: 'ꕯ'
    SyllableNa,
    /// \u{a570}: 'ꕰ'
    SyllableNya,
    /// \u{a571}: 'ꕱ'
    SyllableOo,
    /// \u{a572}: 'ꕲ'
    SyllableOon,
    /// \u{a573}: 'ꕳ'
    SyllableHoo,
    /// \u{a574}: 'ꕴ'
    SyllableWoo,
    /// \u{a575}: 'ꕵ'
    SyllableWoon,
    /// \u{a576}: 'ꕶ'
    SyllablePoo,
    /// \u{a577}: 'ꕷ'
    SyllableBhoo,
    /// \u{a578}: 'ꕸ'
    SyllableBoo,
    /// \u{a579}: 'ꕹ'
    SyllableMboo,
    /// \u{a57a}: 'ꕺ'
    SyllableKpoo,
    /// \u{a57b}: 'ꕻ'
    SyllableMgboo,
    /// \u{a57c}: 'ꕼ'
    SyllableGboo,
    /// \u{a57d}: 'ꕽ'
    SyllableFoo,
    /// \u{a57e}: 'ꕾ'
    SyllableVoo,
    /// \u{a57f}: 'ꕿ'
    SyllableToo,
    /// \u{a580}: 'ꖀ'
    SyllableThoo,
    /// \u{a581}: 'ꖁ'
    SyllableDhoo,
    /// \u{a582}: 'ꖂ'
    SyllableDhhoo,
    /// \u{a583}: 'ꖃ'
    SyllableLoo,
    /// \u{a584}: 'ꖄ'
    SyllableRoo,
    /// \u{a585}: 'ꖅ'
    SyllableDoo,
    /// \u{a586}: 'ꖆ'
    SyllableNdoo,
    /// \u{a587}: 'ꖇ'
    SyllableSoo,
    /// \u{a588}: 'ꖈ'
    SyllableShoo,
    /// \u{a589}: 'ꖉ'
    SyllableZoo,
    /// \u{a58a}: 'ꖊ'
    SyllableZhoo,
    /// \u{a58b}: 'ꖋ'
    SyllableCoo,
    /// \u{a58c}: 'ꖌ'
    SyllableJoo,
    /// \u{a58d}: 'ꖍ'
    SyllableNjoo,
    /// \u{a58e}: 'ꖎ'
    SyllableYoo,
    /// \u{a58f}: 'ꖏ'
    SyllableKoo,
    /// \u{a590}: 'ꖐ'
    SyllableNggoo,
    /// \u{a591}: 'ꖑ'
    SyllableGoo,
    /// \u{a592}: 'ꖒ'
    SyllableMoo,
    /// \u{a593}: 'ꖓ'
    SyllableNoo,
    /// \u{a594}: 'ꖔ'
    SyllableNyoo,
    /// \u{a595}: 'ꖕ'
    SyllableU,
    /// \u{a596}: 'ꖖ'
    SyllableUn,
    /// \u{a597}: 'ꖗ'
    SyllableHu,
    /// \u{a598}: 'ꖘ'
    SyllableHun,
    /// \u{a599}: 'ꖙ'
    SyllableWu,
    /// \u{a59a}: 'ꖚ'
    SyllableWun,
    /// \u{a59b}: 'ꖛ'
    SyllablePu,
    /// \u{a59c}: 'ꖜ'
    SyllableBhu,
    /// \u{a59d}: 'ꖝ'
    SyllableBu,
    /// \u{a59e}: 'ꖞ'
    SyllableMbu,
    /// \u{a59f}: 'ꖟ'
    SyllableKpu,
    /// \u{a5a0}: 'ꖠ'
    SyllableMgbu,
    /// \u{a5a1}: 'ꖡ'
    SyllableGbu,
    /// \u{a5a2}: 'ꖢ'
    SyllableFu,
    /// \u{a5a3}: 'ꖣ'
    SyllableVu,
    /// \u{a5a4}: 'ꖤ'
    SyllableTu,
    /// \u{a5a5}: 'ꖥ'
    SyllableThu,
    /// \u{a5a6}: 'ꖦ'
    SyllableDhu,
    /// \u{a5a7}: 'ꖧ'
    SyllableDhhu,
    /// \u{a5a8}: 'ꖨ'
    SyllableLu,
    /// \u{a5a9}: 'ꖩ'
    SyllableRu,
    /// \u{a5aa}: 'ꖪ'
    SyllableDu,
    /// \u{a5ab}: 'ꖫ'
    SyllableNdu,
    /// \u{a5ac}: 'ꖬ'
    SyllableSu,
    /// \u{a5ad}: 'ꖭ'
    SyllableShu,
    /// \u{a5ae}: 'ꖮ'
    SyllableZu,
    /// \u{a5af}: 'ꖯ'
    SyllableZhu,
    /// \u{a5b0}: 'ꖰ'
    SyllableCu,
    /// \u{a5b1}: 'ꖱ'
    SyllableJu,
    /// \u{a5b2}: 'ꖲ'
    SyllableNju,
    /// \u{a5b3}: 'ꖳ'
    SyllableYu,
    /// \u{a5b4}: 'ꖴ'
    SyllableKu,
    /// \u{a5b5}: 'ꖵ'
    SyllableNggu,
    /// \u{a5b6}: 'ꖶ'
    SyllableGu,
    /// \u{a5b7}: 'ꖷ'
    SyllableMu,
    /// \u{a5b8}: 'ꖸ'
    SyllableNu,
    /// \u{a5b9}: 'ꖹ'
    SyllableNyu,
    /// \u{a5ba}: 'ꖺ'
    SyllableO,
    /// \u{a5bb}: 'ꖻ'
    SyllableOn,
    /// \u{a5bc}: 'ꖼ'
    SyllableNgon,
    /// \u{a5bd}: 'ꖽ'
    SyllableHo,
    /// \u{a5be}: 'ꖾ'
    SyllableHon,
    /// \u{a5bf}: 'ꖿ'
    SyllableWo,
    /// \u{a5c0}: 'ꗀ'
    SyllableWon,
    /// \u{a5c1}: 'ꗁ'
    SyllablePo,
    /// \u{a5c2}: 'ꗂ'
    SyllableBho,
    /// \u{a5c3}: 'ꗃ'
    SyllableBo,
    /// \u{a5c4}: 'ꗄ'
    SyllableMbo,
    /// \u{a5c5}: 'ꗅ'
    SyllableKpo,
    /// \u{a5c6}: 'ꗆ'
    SyllableMgbo,
    /// \u{a5c7}: 'ꗇ'
    SyllableGbo,
    /// \u{a5c8}: 'ꗈ'
    SyllableGbon,
    /// \u{a5c9}: 'ꗉ'
    SyllableFo,
    /// \u{a5ca}: 'ꗊ'
    SyllableVo,
    /// \u{a5cb}: 'ꗋ'
    SyllableTo,
    /// \u{a5cc}: 'ꗌ'
    SyllableTho,
    /// \u{a5cd}: 'ꗍ'
    SyllableDho,
    /// \u{a5ce}: 'ꗎ'
    SyllableDhho,
    /// \u{a5cf}: 'ꗏ'
    SyllableLo,
    /// \u{a5d0}: 'ꗐ'
    SyllableRo,
    /// \u{a5d1}: 'ꗑ'
    SyllableDo,
    /// \u{a5d2}: 'ꗒ'
    SyllableNdo,
    /// \u{a5d3}: 'ꗓ'
    SyllableSo,
    /// \u{a5d4}: 'ꗔ'
    SyllableSho,
    /// \u{a5d5}: 'ꗕ'
    SyllableZo,
    /// \u{a5d6}: 'ꗖ'
    SyllableZho,
    /// \u{a5d7}: 'ꗗ'
    SyllableCo,
    /// \u{a5d8}: 'ꗘ'
    SyllableJo,
    /// \u{a5d9}: 'ꗙ'
    SyllableNjo,
    /// \u{a5da}: 'ꗚ'
    SyllableYo,
    /// \u{a5db}: 'ꗛ'
    SyllableKo,
    /// \u{a5dc}: 'ꗜ'
    SyllableNggo,
    /// \u{a5dd}: 'ꗝ'
    SyllableGo,
    /// \u{a5de}: 'ꗞ'
    SyllableMo,
    /// \u{a5df}: 'ꗟ'
    SyllableNo,
    /// \u{a5e0}: 'ꗠ'
    SyllableNyo,
    /// \u{a5e1}: 'ꗡ'
    SyllableE,
    /// \u{a5e2}: 'ꗢ'
    SyllableEn,
    /// \u{a5e3}: 'ꗣ'
    SyllableNgen,
    /// \u{a5e4}: 'ꗤ'
    SyllableHe,
    /// \u{a5e5}: 'ꗥ'
    SyllableHen,
    /// \u{a5e6}: 'ꗦ'
    SyllableWe,
    /// \u{a5e7}: 'ꗧ'
    SyllableWen,
    /// \u{a5e8}: 'ꗨ'
    SyllablePe,
    /// \u{a5e9}: 'ꗩ'
    SyllableBhe,
    /// \u{a5ea}: 'ꗪ'
    SyllableBe,
    /// \u{a5eb}: 'ꗫ'
    SyllableMbe,
    /// \u{a5ec}: 'ꗬ'
    SyllableKpe,
    /// \u{a5ed}: 'ꗭ'
    SyllableKpen,
    /// \u{a5ee}: 'ꗮ'
    SyllableMgbe,
    /// \u{a5ef}: 'ꗯ'
    SyllableGbe,
    /// \u{a5f0}: 'ꗰ'
    SyllableGben,
    /// \u{a5f1}: 'ꗱ'
    SyllableFe,
    /// \u{a5f2}: 'ꗲ'
    SyllableVe,
    /// \u{a5f3}: 'ꗳ'
    SyllableTe,
    /// \u{a5f4}: 'ꗴ'
    SyllableThe,
    /// \u{a5f5}: 'ꗵ'
    SyllableDhe,
    /// \u{a5f6}: 'ꗶ'
    SyllableDhhe,
    /// \u{a5f7}: 'ꗷ'
    SyllableLe,
    /// \u{a5f8}: 'ꗸ'
    SyllableRe,
    /// \u{a5f9}: 'ꗹ'
    SyllableDe,
    /// \u{a5fa}: 'ꗺ'
    SyllableNde,
    /// \u{a5fb}: 'ꗻ'
    SyllableSe,
    /// \u{a5fc}: 'ꗼ'
    SyllableShe,
    /// \u{a5fd}: 'ꗽ'
    SyllableZe,
    /// \u{a5fe}: 'ꗾ'
    SyllableZhe,
    /// \u{a5ff}: 'ꗿ'
    SyllableCe,
    /// \u{a600}: 'ꘀ'
    SyllableJe,
    /// \u{a601}: 'ꘁ'
    SyllableNje,
    /// \u{a602}: 'ꘂ'
    SyllableYe,
    /// \u{a603}: 'ꘃ'
    SyllableKe,
    /// \u{a604}: 'ꘄ'
    SyllableNgge,
    /// \u{a605}: 'ꘅ'
    SyllableNggen,
    /// \u{a606}: 'ꘆ'
    SyllableGe,
    /// \u{a607}: 'ꘇ'
    SyllableGen,
    /// \u{a608}: 'ꘈ'
    SyllableMe,
    /// \u{a609}: 'ꘉ'
    SyllableNe,
    /// \u{a60a}: 'ꘊ'
    SyllableNye,
    /// \u{a60b}: 'ꘋ'
    SyllableNg,
    /// \u{a60c}: 'ꘌ'
    SyllableLengthener,
    /// \u{a60d}: '꘍'
    Comma,
    /// \u{a60e}: '꘎'
    FullStop,
    /// \u{a60f}: '꘏'
    QuestionMark,
    /// \u{a610}: 'ꘐ'
    SyllableNdoleFa,
    /// \u{a611}: 'ꘑ'
    SyllableNdoleKa,
    /// \u{a612}: 'ꘒ'
    SyllableNdoleSoo,
    /// \u{a613}: 'ꘓ'
    SymbolFeeng,
    /// \u{a614}: 'ꘔ'
    SymbolKeeng,
    /// \u{a615}: 'ꘕ'
    SymbolTing,
    /// \u{a616}: 'ꘖ'
    SymbolNii,
    /// \u{a617}: 'ꘗ'
    SymbolBang,
    /// \u{a618}: 'ꘘ'
    SymbolFaa,
    /// \u{a619}: 'ꘙ'
    SymbolTaa,
    /// \u{a61a}: 'ꘚ'
    SymbolDang,
    /// \u{a61b}: 'ꘛ'
    SymbolDoong,
    /// \u{a61c}: 'ꘜ'
    SymbolKung,
    /// \u{a61d}: 'ꘝ'
    SymbolTong,
    /// \u{a61e}: 'ꘞ'
    SymbolDoDashO,
    /// \u{a61f}: 'ꘟ'
    SymbolJong,
    /// \u{a620}: '꘠'
    DigitZero,
    /// \u{a621}: '꘡'
    DigitOne,
    /// \u{a622}: '꘢'
    DigitTwo,
    /// \u{a623}: '꘣'
    DigitThree,
    /// \u{a624}: '꘤'
    DigitFour,
    /// \u{a625}: '꘥'
    DigitFive,
    /// \u{a626}: '꘦'
    DigitSix,
    /// \u{a627}: '꘧'
    DigitSeven,
    /// \u{a628}: '꘨'
    DigitEight,
    /// \u{a629}: '꘩'
    DigitNine,
    /// \u{a62a}: 'ꘪ'
    SyllableNdoleMa,
    /// \u{a62b}: 'ꘫ'
    SyllableNdoleDo,
}

impl Into<char> for Vai {
    fn into(self) -> char {
        match self {
            Vai::SyllableEe => 'ꔀ',
            Vai::SyllableEen => 'ꔁ',
            Vai::SyllableHee => 'ꔂ',
            Vai::SyllableWee => 'ꔃ',
            Vai::SyllableWeen => 'ꔄ',
            Vai::SyllablePee => 'ꔅ',
            Vai::SyllableBhee => 'ꔆ',
            Vai::SyllableBee => 'ꔇ',
            Vai::SyllableMbee => 'ꔈ',
            Vai::SyllableKpee => 'ꔉ',
            Vai::SyllableMgbee => 'ꔊ',
            Vai::SyllableGbee => 'ꔋ',
            Vai::SyllableFee => 'ꔌ',
            Vai::SyllableVee => 'ꔍ',
            Vai::SyllableTee => 'ꔎ',
            Vai::SyllableThee => 'ꔏ',
            Vai::SyllableDhee => 'ꔐ',
            Vai::SyllableDhhee => 'ꔑ',
            Vai::SyllableLee => 'ꔒ',
            Vai::SyllableRee => 'ꔓ',
            Vai::SyllableDee => 'ꔔ',
            Vai::SyllableNdee => 'ꔕ',
            Vai::SyllableSee => 'ꔖ',
            Vai::SyllableShee => 'ꔗ',
            Vai::SyllableZee => 'ꔘ',
            Vai::SyllableZhee => 'ꔙ',
            Vai::SyllableCee => 'ꔚ',
            Vai::SyllableJee => 'ꔛ',
            Vai::SyllableNjee => 'ꔜ',
            Vai::SyllableYee => 'ꔝ',
            Vai::SyllableKee => 'ꔞ',
            Vai::SyllableNggee => 'ꔟ',
            Vai::SyllableGee => 'ꔠ',
            Vai::SyllableMee => 'ꔡ',
            Vai::SyllableNee => 'ꔢ',
            Vai::SyllableNyee => 'ꔣ',
            Vai::SyllableI => 'ꔤ',
            Vai::SyllableIn => 'ꔥ',
            Vai::SyllableHi => 'ꔦ',
            Vai::SyllableHin => 'ꔧ',
            Vai::SyllableWi => 'ꔨ',
            Vai::SyllableWin => 'ꔩ',
            Vai::SyllablePi => 'ꔪ',
            Vai::SyllableBhi => 'ꔫ',
            Vai::SyllableBi => 'ꔬ',
            Vai::SyllableMbi => 'ꔭ',
            Vai::SyllableKpi => 'ꔮ',
            Vai::SyllableMgbi => 'ꔯ',
            Vai::SyllableGbi => 'ꔰ',
            Vai::SyllableFi => 'ꔱ',
            Vai::SyllableVi => 'ꔲ',
            Vai::SyllableTi => 'ꔳ',
            Vai::SyllableThi => 'ꔴ',
            Vai::SyllableDhi => 'ꔵ',
            Vai::SyllableDhhi => 'ꔶ',
            Vai::SyllableLi => 'ꔷ',
            Vai::SyllableRi => 'ꔸ',
            Vai::SyllableDi => 'ꔹ',
            Vai::SyllableNdi => 'ꔺ',
            Vai::SyllableSi => 'ꔻ',
            Vai::SyllableShi => 'ꔼ',
            Vai::SyllableZi => 'ꔽ',
            Vai::SyllableZhi => 'ꔾ',
            Vai::SyllableCi => 'ꔿ',
            Vai::SyllableJi => 'ꕀ',
            Vai::SyllableNji => 'ꕁ',
            Vai::SyllableYi => 'ꕂ',
            Vai::SyllableKi => 'ꕃ',
            Vai::SyllableNggi => 'ꕄ',
            Vai::SyllableGi => 'ꕅ',
            Vai::SyllableMi => 'ꕆ',
            Vai::SyllableNi => 'ꕇ',
            Vai::SyllableNyi => 'ꕈ',
            Vai::SyllableA => 'ꕉ',
            Vai::SyllableAn => 'ꕊ',
            Vai::SyllableNgan => 'ꕋ',
            Vai::SyllableHa => 'ꕌ',
            Vai::SyllableHan => 'ꕍ',
            Vai::SyllableWa => 'ꕎ',
            Vai::SyllableWan => 'ꕏ',
            Vai::SyllablePa => 'ꕐ',
            Vai::SyllableBha => 'ꕑ',
            Vai::SyllableBa => 'ꕒ',
            Vai::SyllableMba => 'ꕓ',
            Vai::SyllableKpa => 'ꕔ',
            Vai::SyllableKpan => 'ꕕ',
            Vai::SyllableMgba => 'ꕖ',
            Vai::SyllableGba => 'ꕗ',
            Vai::SyllableFa => 'ꕘ',
            Vai::SyllableVa => 'ꕙ',
            Vai::SyllableTa => 'ꕚ',
            Vai::SyllableTha => 'ꕛ',
            Vai::SyllableDha => 'ꕜ',
            Vai::SyllableDhha => 'ꕝ',
            Vai::SyllableLa => 'ꕞ',
            Vai::SyllableRa => 'ꕟ',
            Vai::SyllableDa => 'ꕠ',
            Vai::SyllableNda => 'ꕡ',
            Vai::SyllableSa => 'ꕢ',
            Vai::SyllableSha => 'ꕣ',
            Vai::SyllableZa => 'ꕤ',
            Vai::SyllableZha => 'ꕥ',
            Vai::SyllableCa => 'ꕦ',
            Vai::SyllableJa => 'ꕧ',
            Vai::SyllableNja => 'ꕨ',
            Vai::SyllableYa => 'ꕩ',
            Vai::SyllableKa => 'ꕪ',
            Vai::SyllableKan => 'ꕫ',
            Vai::SyllableNgga => 'ꕬ',
            Vai::SyllableGa => 'ꕭ',
            Vai::SyllableMa => 'ꕮ',
            Vai::SyllableNa => 'ꕯ',
            Vai::SyllableNya => 'ꕰ',
            Vai::SyllableOo => 'ꕱ',
            Vai::SyllableOon => 'ꕲ',
            Vai::SyllableHoo => 'ꕳ',
            Vai::SyllableWoo => 'ꕴ',
            Vai::SyllableWoon => 'ꕵ',
            Vai::SyllablePoo => 'ꕶ',
            Vai::SyllableBhoo => 'ꕷ',
            Vai::SyllableBoo => 'ꕸ',
            Vai::SyllableMboo => 'ꕹ',
            Vai::SyllableKpoo => 'ꕺ',
            Vai::SyllableMgboo => 'ꕻ',
            Vai::SyllableGboo => 'ꕼ',
            Vai::SyllableFoo => 'ꕽ',
            Vai::SyllableVoo => 'ꕾ',
            Vai::SyllableToo => 'ꕿ',
            Vai::SyllableThoo => 'ꖀ',
            Vai::SyllableDhoo => 'ꖁ',
            Vai::SyllableDhhoo => 'ꖂ',
            Vai::SyllableLoo => 'ꖃ',
            Vai::SyllableRoo => 'ꖄ',
            Vai::SyllableDoo => 'ꖅ',
            Vai::SyllableNdoo => 'ꖆ',
            Vai::SyllableSoo => 'ꖇ',
            Vai::SyllableShoo => 'ꖈ',
            Vai::SyllableZoo => 'ꖉ',
            Vai::SyllableZhoo => 'ꖊ',
            Vai::SyllableCoo => 'ꖋ',
            Vai::SyllableJoo => 'ꖌ',
            Vai::SyllableNjoo => 'ꖍ',
            Vai::SyllableYoo => 'ꖎ',
            Vai::SyllableKoo => 'ꖏ',
            Vai::SyllableNggoo => 'ꖐ',
            Vai::SyllableGoo => 'ꖑ',
            Vai::SyllableMoo => 'ꖒ',
            Vai::SyllableNoo => 'ꖓ',
            Vai::SyllableNyoo => 'ꖔ',
            Vai::SyllableU => 'ꖕ',
            Vai::SyllableUn => 'ꖖ',
            Vai::SyllableHu => 'ꖗ',
            Vai::SyllableHun => 'ꖘ',
            Vai::SyllableWu => 'ꖙ',
            Vai::SyllableWun => 'ꖚ',
            Vai::SyllablePu => 'ꖛ',
            Vai::SyllableBhu => 'ꖜ',
            Vai::SyllableBu => 'ꖝ',
            Vai::SyllableMbu => 'ꖞ',
            Vai::SyllableKpu => 'ꖟ',
            Vai::SyllableMgbu => 'ꖠ',
            Vai::SyllableGbu => 'ꖡ',
            Vai::SyllableFu => 'ꖢ',
            Vai::SyllableVu => 'ꖣ',
            Vai::SyllableTu => 'ꖤ',
            Vai::SyllableThu => 'ꖥ',
            Vai::SyllableDhu => 'ꖦ',
            Vai::SyllableDhhu => 'ꖧ',
            Vai::SyllableLu => 'ꖨ',
            Vai::SyllableRu => 'ꖩ',
            Vai::SyllableDu => 'ꖪ',
            Vai::SyllableNdu => 'ꖫ',
            Vai::SyllableSu => 'ꖬ',
            Vai::SyllableShu => 'ꖭ',
            Vai::SyllableZu => 'ꖮ',
            Vai::SyllableZhu => 'ꖯ',
            Vai::SyllableCu => 'ꖰ',
            Vai::SyllableJu => 'ꖱ',
            Vai::SyllableNju => 'ꖲ',
            Vai::SyllableYu => 'ꖳ',
            Vai::SyllableKu => 'ꖴ',
            Vai::SyllableNggu => 'ꖵ',
            Vai::SyllableGu => 'ꖶ',
            Vai::SyllableMu => 'ꖷ',
            Vai::SyllableNu => 'ꖸ',
            Vai::SyllableNyu => 'ꖹ',
            Vai::SyllableO => 'ꖺ',
            Vai::SyllableOn => 'ꖻ',
            Vai::SyllableNgon => 'ꖼ',
            Vai::SyllableHo => 'ꖽ',
            Vai::SyllableHon => 'ꖾ',
            Vai::SyllableWo => 'ꖿ',
            Vai::SyllableWon => 'ꗀ',
            Vai::SyllablePo => 'ꗁ',
            Vai::SyllableBho => 'ꗂ',
            Vai::SyllableBo => 'ꗃ',
            Vai::SyllableMbo => 'ꗄ',
            Vai::SyllableKpo => 'ꗅ',
            Vai::SyllableMgbo => 'ꗆ',
            Vai::SyllableGbo => 'ꗇ',
            Vai::SyllableGbon => 'ꗈ',
            Vai::SyllableFo => 'ꗉ',
            Vai::SyllableVo => 'ꗊ',
            Vai::SyllableTo => 'ꗋ',
            Vai::SyllableTho => 'ꗌ',
            Vai::SyllableDho => 'ꗍ',
            Vai::SyllableDhho => 'ꗎ',
            Vai::SyllableLo => 'ꗏ',
            Vai::SyllableRo => 'ꗐ',
            Vai::SyllableDo => 'ꗑ',
            Vai::SyllableNdo => 'ꗒ',
            Vai::SyllableSo => 'ꗓ',
            Vai::SyllableSho => 'ꗔ',
            Vai::SyllableZo => 'ꗕ',
            Vai::SyllableZho => 'ꗖ',
            Vai::SyllableCo => 'ꗗ',
            Vai::SyllableJo => 'ꗘ',
            Vai::SyllableNjo => 'ꗙ',
            Vai::SyllableYo => 'ꗚ',
            Vai::SyllableKo => 'ꗛ',
            Vai::SyllableNggo => 'ꗜ',
            Vai::SyllableGo => 'ꗝ',
            Vai::SyllableMo => 'ꗞ',
            Vai::SyllableNo => 'ꗟ',
            Vai::SyllableNyo => 'ꗠ',
            Vai::SyllableE => 'ꗡ',
            Vai::SyllableEn => 'ꗢ',
            Vai::SyllableNgen => 'ꗣ',
            Vai::SyllableHe => 'ꗤ',
            Vai::SyllableHen => 'ꗥ',
            Vai::SyllableWe => 'ꗦ',
            Vai::SyllableWen => 'ꗧ',
            Vai::SyllablePe => 'ꗨ',
            Vai::SyllableBhe => 'ꗩ',
            Vai::SyllableBe => 'ꗪ',
            Vai::SyllableMbe => 'ꗫ',
            Vai::SyllableKpe => 'ꗬ',
            Vai::SyllableKpen => 'ꗭ',
            Vai::SyllableMgbe => 'ꗮ',
            Vai::SyllableGbe => 'ꗯ',
            Vai::SyllableGben => 'ꗰ',
            Vai::SyllableFe => 'ꗱ',
            Vai::SyllableVe => 'ꗲ',
            Vai::SyllableTe => 'ꗳ',
            Vai::SyllableThe => 'ꗴ',
            Vai::SyllableDhe => 'ꗵ',
            Vai::SyllableDhhe => 'ꗶ',
            Vai::SyllableLe => 'ꗷ',
            Vai::SyllableRe => 'ꗸ',
            Vai::SyllableDe => 'ꗹ',
            Vai::SyllableNde => 'ꗺ',
            Vai::SyllableSe => 'ꗻ',
            Vai::SyllableShe => 'ꗼ',
            Vai::SyllableZe => 'ꗽ',
            Vai::SyllableZhe => 'ꗾ',
            Vai::SyllableCe => 'ꗿ',
            Vai::SyllableJe => 'ꘀ',
            Vai::SyllableNje => 'ꘁ',
            Vai::SyllableYe => 'ꘂ',
            Vai::SyllableKe => 'ꘃ',
            Vai::SyllableNgge => 'ꘄ',
            Vai::SyllableNggen => 'ꘅ',
            Vai::SyllableGe => 'ꘆ',
            Vai::SyllableGen => 'ꘇ',
            Vai::SyllableMe => 'ꘈ',
            Vai::SyllableNe => 'ꘉ',
            Vai::SyllableNye => 'ꘊ',
            Vai::SyllableNg => 'ꘋ',
            Vai::SyllableLengthener => 'ꘌ',
            Vai::Comma => '꘍',
            Vai::FullStop => '꘎',
            Vai::QuestionMark => '꘏',
            Vai::SyllableNdoleFa => 'ꘐ',
            Vai::SyllableNdoleKa => 'ꘑ',
            Vai::SyllableNdoleSoo => 'ꘒ',
            Vai::SymbolFeeng => 'ꘓ',
            Vai::SymbolKeeng => 'ꘔ',
            Vai::SymbolTing => 'ꘕ',
            Vai::SymbolNii => 'ꘖ',
            Vai::SymbolBang => 'ꘗ',
            Vai::SymbolFaa => 'ꘘ',
            Vai::SymbolTaa => 'ꘙ',
            Vai::SymbolDang => 'ꘚ',
            Vai::SymbolDoong => 'ꘛ',
            Vai::SymbolKung => 'ꘜ',
            Vai::SymbolTong => 'ꘝ',
            Vai::SymbolDoDashO => 'ꘞ',
            Vai::SymbolJong => 'ꘟ',
            Vai::DigitZero => '꘠',
            Vai::DigitOne => '꘡',
            Vai::DigitTwo => '꘢',
            Vai::DigitThree => '꘣',
            Vai::DigitFour => '꘤',
            Vai::DigitFive => '꘥',
            Vai::DigitSix => '꘦',
            Vai::DigitSeven => '꘧',
            Vai::DigitEight => '꘨',
            Vai::DigitNine => '꘩',
            Vai::SyllableNdoleMa => 'ꘪ',
            Vai::SyllableNdoleDo => 'ꘫ',
        }
    }
}

impl std::convert::TryFrom<char> for Vai {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꔀ' => Ok(Vai::SyllableEe),
            'ꔁ' => Ok(Vai::SyllableEen),
            'ꔂ' => Ok(Vai::SyllableHee),
            'ꔃ' => Ok(Vai::SyllableWee),
            'ꔄ' => Ok(Vai::SyllableWeen),
            'ꔅ' => Ok(Vai::SyllablePee),
            'ꔆ' => Ok(Vai::SyllableBhee),
            'ꔇ' => Ok(Vai::SyllableBee),
            'ꔈ' => Ok(Vai::SyllableMbee),
            'ꔉ' => Ok(Vai::SyllableKpee),
            'ꔊ' => Ok(Vai::SyllableMgbee),
            'ꔋ' => Ok(Vai::SyllableGbee),
            'ꔌ' => Ok(Vai::SyllableFee),
            'ꔍ' => Ok(Vai::SyllableVee),
            'ꔎ' => Ok(Vai::SyllableTee),
            'ꔏ' => Ok(Vai::SyllableThee),
            'ꔐ' => Ok(Vai::SyllableDhee),
            'ꔑ' => Ok(Vai::SyllableDhhee),
            'ꔒ' => Ok(Vai::SyllableLee),
            'ꔓ' => Ok(Vai::SyllableRee),
            'ꔔ' => Ok(Vai::SyllableDee),
            'ꔕ' => Ok(Vai::SyllableNdee),
            'ꔖ' => Ok(Vai::SyllableSee),
            'ꔗ' => Ok(Vai::SyllableShee),
            'ꔘ' => Ok(Vai::SyllableZee),
            'ꔙ' => Ok(Vai::SyllableZhee),
            'ꔚ' => Ok(Vai::SyllableCee),
            'ꔛ' => Ok(Vai::SyllableJee),
            'ꔜ' => Ok(Vai::SyllableNjee),
            'ꔝ' => Ok(Vai::SyllableYee),
            'ꔞ' => Ok(Vai::SyllableKee),
            'ꔟ' => Ok(Vai::SyllableNggee),
            'ꔠ' => Ok(Vai::SyllableGee),
            'ꔡ' => Ok(Vai::SyllableMee),
            'ꔢ' => Ok(Vai::SyllableNee),
            'ꔣ' => Ok(Vai::SyllableNyee),
            'ꔤ' => Ok(Vai::SyllableI),
            'ꔥ' => Ok(Vai::SyllableIn),
            'ꔦ' => Ok(Vai::SyllableHi),
            'ꔧ' => Ok(Vai::SyllableHin),
            'ꔨ' => Ok(Vai::SyllableWi),
            'ꔩ' => Ok(Vai::SyllableWin),
            'ꔪ' => Ok(Vai::SyllablePi),
            'ꔫ' => Ok(Vai::SyllableBhi),
            'ꔬ' => Ok(Vai::SyllableBi),
            'ꔭ' => Ok(Vai::SyllableMbi),
            'ꔮ' => Ok(Vai::SyllableKpi),
            'ꔯ' => Ok(Vai::SyllableMgbi),
            'ꔰ' => Ok(Vai::SyllableGbi),
            'ꔱ' => Ok(Vai::SyllableFi),
            'ꔲ' => Ok(Vai::SyllableVi),
            'ꔳ' => Ok(Vai::SyllableTi),
            'ꔴ' => Ok(Vai::SyllableThi),
            'ꔵ' => Ok(Vai::SyllableDhi),
            'ꔶ' => Ok(Vai::SyllableDhhi),
            'ꔷ' => Ok(Vai::SyllableLi),
            'ꔸ' => Ok(Vai::SyllableRi),
            'ꔹ' => Ok(Vai::SyllableDi),
            'ꔺ' => Ok(Vai::SyllableNdi),
            'ꔻ' => Ok(Vai::SyllableSi),
            'ꔼ' => Ok(Vai::SyllableShi),
            'ꔽ' => Ok(Vai::SyllableZi),
            'ꔾ' => Ok(Vai::SyllableZhi),
            'ꔿ' => Ok(Vai::SyllableCi),
            'ꕀ' => Ok(Vai::SyllableJi),
            'ꕁ' => Ok(Vai::SyllableNji),
            'ꕂ' => Ok(Vai::SyllableYi),
            'ꕃ' => Ok(Vai::SyllableKi),
            'ꕄ' => Ok(Vai::SyllableNggi),
            'ꕅ' => Ok(Vai::SyllableGi),
            'ꕆ' => Ok(Vai::SyllableMi),
            'ꕇ' => Ok(Vai::SyllableNi),
            'ꕈ' => Ok(Vai::SyllableNyi),
            'ꕉ' => Ok(Vai::SyllableA),
            'ꕊ' => Ok(Vai::SyllableAn),
            'ꕋ' => Ok(Vai::SyllableNgan),
            'ꕌ' => Ok(Vai::SyllableHa),
            'ꕍ' => Ok(Vai::SyllableHan),
            'ꕎ' => Ok(Vai::SyllableWa),
            'ꕏ' => Ok(Vai::SyllableWan),
            'ꕐ' => Ok(Vai::SyllablePa),
            'ꕑ' => Ok(Vai::SyllableBha),
            'ꕒ' => Ok(Vai::SyllableBa),
            'ꕓ' => Ok(Vai::SyllableMba),
            'ꕔ' => Ok(Vai::SyllableKpa),
            'ꕕ' => Ok(Vai::SyllableKpan),
            'ꕖ' => Ok(Vai::SyllableMgba),
            'ꕗ' => Ok(Vai::SyllableGba),
            'ꕘ' => Ok(Vai::SyllableFa),
            'ꕙ' => Ok(Vai::SyllableVa),
            'ꕚ' => Ok(Vai::SyllableTa),
            'ꕛ' => Ok(Vai::SyllableTha),
            'ꕜ' => Ok(Vai::SyllableDha),
            'ꕝ' => Ok(Vai::SyllableDhha),
            'ꕞ' => Ok(Vai::SyllableLa),
            'ꕟ' => Ok(Vai::SyllableRa),
            'ꕠ' => Ok(Vai::SyllableDa),
            'ꕡ' => Ok(Vai::SyllableNda),
            'ꕢ' => Ok(Vai::SyllableSa),
            'ꕣ' => Ok(Vai::SyllableSha),
            'ꕤ' => Ok(Vai::SyllableZa),
            'ꕥ' => Ok(Vai::SyllableZha),
            'ꕦ' => Ok(Vai::SyllableCa),
            'ꕧ' => Ok(Vai::SyllableJa),
            'ꕨ' => Ok(Vai::SyllableNja),
            'ꕩ' => Ok(Vai::SyllableYa),
            'ꕪ' => Ok(Vai::SyllableKa),
            'ꕫ' => Ok(Vai::SyllableKan),
            'ꕬ' => Ok(Vai::SyllableNgga),
            'ꕭ' => Ok(Vai::SyllableGa),
            'ꕮ' => Ok(Vai::SyllableMa),
            'ꕯ' => Ok(Vai::SyllableNa),
            'ꕰ' => Ok(Vai::SyllableNya),
            'ꕱ' => Ok(Vai::SyllableOo),
            'ꕲ' => Ok(Vai::SyllableOon),
            'ꕳ' => Ok(Vai::SyllableHoo),
            'ꕴ' => Ok(Vai::SyllableWoo),
            'ꕵ' => Ok(Vai::SyllableWoon),
            'ꕶ' => Ok(Vai::SyllablePoo),
            'ꕷ' => Ok(Vai::SyllableBhoo),
            'ꕸ' => Ok(Vai::SyllableBoo),
            'ꕹ' => Ok(Vai::SyllableMboo),
            'ꕺ' => Ok(Vai::SyllableKpoo),
            'ꕻ' => Ok(Vai::SyllableMgboo),
            'ꕼ' => Ok(Vai::SyllableGboo),
            'ꕽ' => Ok(Vai::SyllableFoo),
            'ꕾ' => Ok(Vai::SyllableVoo),
            'ꕿ' => Ok(Vai::SyllableToo),
            'ꖀ' => Ok(Vai::SyllableThoo),
            'ꖁ' => Ok(Vai::SyllableDhoo),
            'ꖂ' => Ok(Vai::SyllableDhhoo),
            'ꖃ' => Ok(Vai::SyllableLoo),
            'ꖄ' => Ok(Vai::SyllableRoo),
            'ꖅ' => Ok(Vai::SyllableDoo),
            'ꖆ' => Ok(Vai::SyllableNdoo),
            'ꖇ' => Ok(Vai::SyllableSoo),
            'ꖈ' => Ok(Vai::SyllableShoo),
            'ꖉ' => Ok(Vai::SyllableZoo),
            'ꖊ' => Ok(Vai::SyllableZhoo),
            'ꖋ' => Ok(Vai::SyllableCoo),
            'ꖌ' => Ok(Vai::SyllableJoo),
            'ꖍ' => Ok(Vai::SyllableNjoo),
            'ꖎ' => Ok(Vai::SyllableYoo),
            'ꖏ' => Ok(Vai::SyllableKoo),
            'ꖐ' => Ok(Vai::SyllableNggoo),
            'ꖑ' => Ok(Vai::SyllableGoo),
            'ꖒ' => Ok(Vai::SyllableMoo),
            'ꖓ' => Ok(Vai::SyllableNoo),
            'ꖔ' => Ok(Vai::SyllableNyoo),
            'ꖕ' => Ok(Vai::SyllableU),
            'ꖖ' => Ok(Vai::SyllableUn),
            'ꖗ' => Ok(Vai::SyllableHu),
            'ꖘ' => Ok(Vai::SyllableHun),
            'ꖙ' => Ok(Vai::SyllableWu),
            'ꖚ' => Ok(Vai::SyllableWun),
            'ꖛ' => Ok(Vai::SyllablePu),
            'ꖜ' => Ok(Vai::SyllableBhu),
            'ꖝ' => Ok(Vai::SyllableBu),
            'ꖞ' => Ok(Vai::SyllableMbu),
            'ꖟ' => Ok(Vai::SyllableKpu),
            'ꖠ' => Ok(Vai::SyllableMgbu),
            'ꖡ' => Ok(Vai::SyllableGbu),
            'ꖢ' => Ok(Vai::SyllableFu),
            'ꖣ' => Ok(Vai::SyllableVu),
            'ꖤ' => Ok(Vai::SyllableTu),
            'ꖥ' => Ok(Vai::SyllableThu),
            'ꖦ' => Ok(Vai::SyllableDhu),
            'ꖧ' => Ok(Vai::SyllableDhhu),
            'ꖨ' => Ok(Vai::SyllableLu),
            'ꖩ' => Ok(Vai::SyllableRu),
            'ꖪ' => Ok(Vai::SyllableDu),
            'ꖫ' => Ok(Vai::SyllableNdu),
            'ꖬ' => Ok(Vai::SyllableSu),
            'ꖭ' => Ok(Vai::SyllableShu),
            'ꖮ' => Ok(Vai::SyllableZu),
            'ꖯ' => Ok(Vai::SyllableZhu),
            'ꖰ' => Ok(Vai::SyllableCu),
            'ꖱ' => Ok(Vai::SyllableJu),
            'ꖲ' => Ok(Vai::SyllableNju),
            'ꖳ' => Ok(Vai::SyllableYu),
            'ꖴ' => Ok(Vai::SyllableKu),
            'ꖵ' => Ok(Vai::SyllableNggu),
            'ꖶ' => Ok(Vai::SyllableGu),
            'ꖷ' => Ok(Vai::SyllableMu),
            'ꖸ' => Ok(Vai::SyllableNu),
            'ꖹ' => Ok(Vai::SyllableNyu),
            'ꖺ' => Ok(Vai::SyllableO),
            'ꖻ' => Ok(Vai::SyllableOn),
            'ꖼ' => Ok(Vai::SyllableNgon),
            'ꖽ' => Ok(Vai::SyllableHo),
            'ꖾ' => Ok(Vai::SyllableHon),
            'ꖿ' => Ok(Vai::SyllableWo),
            'ꗀ' => Ok(Vai::SyllableWon),
            'ꗁ' => Ok(Vai::SyllablePo),
            'ꗂ' => Ok(Vai::SyllableBho),
            'ꗃ' => Ok(Vai::SyllableBo),
            'ꗄ' => Ok(Vai::SyllableMbo),
            'ꗅ' => Ok(Vai::SyllableKpo),
            'ꗆ' => Ok(Vai::SyllableMgbo),
            'ꗇ' => Ok(Vai::SyllableGbo),
            'ꗈ' => Ok(Vai::SyllableGbon),
            'ꗉ' => Ok(Vai::SyllableFo),
            'ꗊ' => Ok(Vai::SyllableVo),
            'ꗋ' => Ok(Vai::SyllableTo),
            'ꗌ' => Ok(Vai::SyllableTho),
            'ꗍ' => Ok(Vai::SyllableDho),
            'ꗎ' => Ok(Vai::SyllableDhho),
            'ꗏ' => Ok(Vai::SyllableLo),
            'ꗐ' => Ok(Vai::SyllableRo),
            'ꗑ' => Ok(Vai::SyllableDo),
            'ꗒ' => Ok(Vai::SyllableNdo),
            'ꗓ' => Ok(Vai::SyllableSo),
            'ꗔ' => Ok(Vai::SyllableSho),
            'ꗕ' => Ok(Vai::SyllableZo),
            'ꗖ' => Ok(Vai::SyllableZho),
            'ꗗ' => Ok(Vai::SyllableCo),
            'ꗘ' => Ok(Vai::SyllableJo),
            'ꗙ' => Ok(Vai::SyllableNjo),
            'ꗚ' => Ok(Vai::SyllableYo),
            'ꗛ' => Ok(Vai::SyllableKo),
            'ꗜ' => Ok(Vai::SyllableNggo),
            'ꗝ' => Ok(Vai::SyllableGo),
            'ꗞ' => Ok(Vai::SyllableMo),
            'ꗟ' => Ok(Vai::SyllableNo),
            'ꗠ' => Ok(Vai::SyllableNyo),
            'ꗡ' => Ok(Vai::SyllableE),
            'ꗢ' => Ok(Vai::SyllableEn),
            'ꗣ' => Ok(Vai::SyllableNgen),
            'ꗤ' => Ok(Vai::SyllableHe),
            'ꗥ' => Ok(Vai::SyllableHen),
            'ꗦ' => Ok(Vai::SyllableWe),
            'ꗧ' => Ok(Vai::SyllableWen),
            'ꗨ' => Ok(Vai::SyllablePe),
            'ꗩ' => Ok(Vai::SyllableBhe),
            'ꗪ' => Ok(Vai::SyllableBe),
            'ꗫ' => Ok(Vai::SyllableMbe),
            'ꗬ' => Ok(Vai::SyllableKpe),
            'ꗭ' => Ok(Vai::SyllableKpen),
            'ꗮ' => Ok(Vai::SyllableMgbe),
            'ꗯ' => Ok(Vai::SyllableGbe),
            'ꗰ' => Ok(Vai::SyllableGben),
            'ꗱ' => Ok(Vai::SyllableFe),
            'ꗲ' => Ok(Vai::SyllableVe),
            'ꗳ' => Ok(Vai::SyllableTe),
            'ꗴ' => Ok(Vai::SyllableThe),
            'ꗵ' => Ok(Vai::SyllableDhe),
            'ꗶ' => Ok(Vai::SyllableDhhe),
            'ꗷ' => Ok(Vai::SyllableLe),
            'ꗸ' => Ok(Vai::SyllableRe),
            'ꗹ' => Ok(Vai::SyllableDe),
            'ꗺ' => Ok(Vai::SyllableNde),
            'ꗻ' => Ok(Vai::SyllableSe),
            'ꗼ' => Ok(Vai::SyllableShe),
            'ꗽ' => Ok(Vai::SyllableZe),
            'ꗾ' => Ok(Vai::SyllableZhe),
            'ꗿ' => Ok(Vai::SyllableCe),
            'ꘀ' => Ok(Vai::SyllableJe),
            'ꘁ' => Ok(Vai::SyllableNje),
            'ꘂ' => Ok(Vai::SyllableYe),
            'ꘃ' => Ok(Vai::SyllableKe),
            'ꘄ' => Ok(Vai::SyllableNgge),
            'ꘅ' => Ok(Vai::SyllableNggen),
            'ꘆ' => Ok(Vai::SyllableGe),
            'ꘇ' => Ok(Vai::SyllableGen),
            'ꘈ' => Ok(Vai::SyllableMe),
            'ꘉ' => Ok(Vai::SyllableNe),
            'ꘊ' => Ok(Vai::SyllableNye),
            'ꘋ' => Ok(Vai::SyllableNg),
            'ꘌ' => Ok(Vai::SyllableLengthener),
            '꘍' => Ok(Vai::Comma),
            '꘎' => Ok(Vai::FullStop),
            '꘏' => Ok(Vai::QuestionMark),
            'ꘐ' => Ok(Vai::SyllableNdoleFa),
            'ꘑ' => Ok(Vai::SyllableNdoleKa),
            'ꘒ' => Ok(Vai::SyllableNdoleSoo),
            'ꘓ' => Ok(Vai::SymbolFeeng),
            'ꘔ' => Ok(Vai::SymbolKeeng),
            'ꘕ' => Ok(Vai::SymbolTing),
            'ꘖ' => Ok(Vai::SymbolNii),
            'ꘗ' => Ok(Vai::SymbolBang),
            'ꘘ' => Ok(Vai::SymbolFaa),
            'ꘙ' => Ok(Vai::SymbolTaa),
            'ꘚ' => Ok(Vai::SymbolDang),
            'ꘛ' => Ok(Vai::SymbolDoong),
            'ꘜ' => Ok(Vai::SymbolKung),
            'ꘝ' => Ok(Vai::SymbolTong),
            'ꘞ' => Ok(Vai::SymbolDoDashO),
            'ꘟ' => Ok(Vai::SymbolJong),
            '꘠' => Ok(Vai::DigitZero),
            '꘡' => Ok(Vai::DigitOne),
            '꘢' => Ok(Vai::DigitTwo),
            '꘣' => Ok(Vai::DigitThree),
            '꘤' => Ok(Vai::DigitFour),
            '꘥' => Ok(Vai::DigitFive),
            '꘦' => Ok(Vai::DigitSix),
            '꘧' => Ok(Vai::DigitSeven),
            '꘨' => Ok(Vai::DigitEight),
            '꘩' => Ok(Vai::DigitNine),
            'ꘪ' => Ok(Vai::SyllableNdoleMa),
            'ꘫ' => Ok(Vai::SyllableNdoleDo),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Vai {
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

impl std::convert::TryFrom<u32> for Vai {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Vai {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Vai {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Vai::SyllableEe
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Vai{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
