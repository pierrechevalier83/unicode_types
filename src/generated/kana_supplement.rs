
/// An enum to represent all characters in the KanaSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KanaSupplement {
    /// \u{1b000}: '𛀀'
    KatakanaLetterArchaicE,
    /// \u{1b001}: '𛀁'
    HiraganaLetterArchaicYe,
    /// \u{1b002}: '𛀂'
    HentaiganaLetterADash1,
    /// \u{1b003}: '𛀃'
    HentaiganaLetterADash2,
    /// \u{1b004}: '𛀄'
    HentaiganaLetterADash3,
    /// \u{1b005}: '𛀅'
    HentaiganaLetterADashWo,
    /// \u{1b006}: '𛀆'
    HentaiganaLetterIDash1,
    /// \u{1b007}: '𛀇'
    HentaiganaLetterIDash2,
    /// \u{1b008}: '𛀈'
    HentaiganaLetterIDash3,
    /// \u{1b009}: '𛀉'
    HentaiganaLetterIDash4,
    /// \u{1b00a}: '𛀊'
    HentaiganaLetterUDash1,
    /// \u{1b00b}: '𛀋'
    HentaiganaLetterUDash2,
    /// \u{1b00c}: '𛀌'
    HentaiganaLetterUDash3,
    /// \u{1b00d}: '𛀍'
    HentaiganaLetterUDash4,
    /// \u{1b00e}: '𛀎'
    HentaiganaLetterUDash5,
    /// \u{1b00f}: '𛀏'
    HentaiganaLetterEDash2,
    /// \u{1b010}: '𛀐'
    HentaiganaLetterEDash3,
    /// \u{1b011}: '𛀑'
    HentaiganaLetterEDash4,
    /// \u{1b012}: '𛀒'
    HentaiganaLetterEDash5,
    /// \u{1b013}: '𛀓'
    HentaiganaLetterEDash6,
    /// \u{1b014}: '𛀔'
    HentaiganaLetterODash1,
    /// \u{1b015}: '𛀕'
    HentaiganaLetterODash2,
    /// \u{1b016}: '𛀖'
    HentaiganaLetterODash3,
    /// \u{1b017}: '𛀗'
    HentaiganaLetterKaDash1,
    /// \u{1b018}: '𛀘'
    HentaiganaLetterKaDash2,
    /// \u{1b019}: '𛀙'
    HentaiganaLetterKaDash3,
    /// \u{1b01a}: '𛀚'
    HentaiganaLetterKaDash4,
    /// \u{1b01b}: '𛀛'
    HentaiganaLetterKaDash5,
    /// \u{1b01c}: '𛀜'
    HentaiganaLetterKaDash6,
    /// \u{1b01d}: '𛀝'
    HentaiganaLetterKaDash7,
    /// \u{1b01e}: '𛀞'
    HentaiganaLetterKaDash8,
    /// \u{1b01f}: '𛀟'
    HentaiganaLetterKaDash9,
    /// \u{1b020}: '𛀠'
    HentaiganaLetterKaDash10,
    /// \u{1b021}: '𛀡'
    HentaiganaLetterKaDash11,
    /// \u{1b022}: '𛀢'
    HentaiganaLetterKaDashKe,
    /// \u{1b023}: '𛀣'
    HentaiganaLetterKiDash1,
    /// \u{1b024}: '𛀤'
    HentaiganaLetterKiDash2,
    /// \u{1b025}: '𛀥'
    HentaiganaLetterKiDash3,
    /// \u{1b026}: '𛀦'
    HentaiganaLetterKiDash4,
    /// \u{1b027}: '𛀧'
    HentaiganaLetterKiDash5,
    /// \u{1b028}: '𛀨'
    HentaiganaLetterKiDash6,
    /// \u{1b029}: '𛀩'
    HentaiganaLetterKiDash7,
    /// \u{1b02a}: '𛀪'
    HentaiganaLetterKiDash8,
    /// \u{1b02b}: '𛀫'
    HentaiganaLetterKuDash1,
    /// \u{1b02c}: '𛀬'
    HentaiganaLetterKuDash2,
    /// \u{1b02d}: '𛀭'
    HentaiganaLetterKuDash3,
    /// \u{1b02e}: '𛀮'
    HentaiganaLetterKuDash4,
    /// \u{1b02f}: '𛀯'
    HentaiganaLetterKuDash5,
    /// \u{1b030}: '𛀰'
    HentaiganaLetterKuDash6,
    /// \u{1b031}: '𛀱'
    HentaiganaLetterKuDash7,
    /// \u{1b032}: '𛀲'
    HentaiganaLetterKeDash1,
    /// \u{1b033}: '𛀳'
    HentaiganaLetterKeDash2,
    /// \u{1b034}: '𛀴'
    HentaiganaLetterKeDash3,
    /// \u{1b035}: '𛀵'
    HentaiganaLetterKeDash4,
    /// \u{1b036}: '𛀶'
    HentaiganaLetterKeDash5,
    /// \u{1b037}: '𛀷'
    HentaiganaLetterKeDash6,
    /// \u{1b038}: '𛀸'
    HentaiganaLetterKoDash1,
    /// \u{1b039}: '𛀹'
    HentaiganaLetterKoDash2,
    /// \u{1b03a}: '𛀺'
    HentaiganaLetterKoDash3,
    /// \u{1b03b}: '𛀻'
    HentaiganaLetterKoDashKi,
    /// \u{1b03c}: '𛀼'
    HentaiganaLetterSaDash1,
    /// \u{1b03d}: '𛀽'
    HentaiganaLetterSaDash2,
    /// \u{1b03e}: '𛀾'
    HentaiganaLetterSaDash3,
    /// \u{1b03f}: '𛀿'
    HentaiganaLetterSaDash4,
    /// \u{1b040}: '𛁀'
    HentaiganaLetterSaDash5,
    /// \u{1b041}: '𛁁'
    HentaiganaLetterSaDash6,
    /// \u{1b042}: '𛁂'
    HentaiganaLetterSaDash7,
    /// \u{1b043}: '𛁃'
    HentaiganaLetterSaDash8,
    /// \u{1b044}: '𛁄'
    HentaiganaLetterSiDash1,
    /// \u{1b045}: '𛁅'
    HentaiganaLetterSiDash2,
    /// \u{1b046}: '𛁆'
    HentaiganaLetterSiDash3,
    /// \u{1b047}: '𛁇'
    HentaiganaLetterSiDash4,
    /// \u{1b048}: '𛁈'
    HentaiganaLetterSiDash5,
    /// \u{1b049}: '𛁉'
    HentaiganaLetterSiDash6,
    /// \u{1b04a}: '𛁊'
    HentaiganaLetterSuDash1,
    /// \u{1b04b}: '𛁋'
    HentaiganaLetterSuDash2,
    /// \u{1b04c}: '𛁌'
    HentaiganaLetterSuDash3,
    /// \u{1b04d}: '𛁍'
    HentaiganaLetterSuDash4,
    /// \u{1b04e}: '𛁎'
    HentaiganaLetterSuDash5,
    /// \u{1b04f}: '𛁏'
    HentaiganaLetterSuDash6,
    /// \u{1b050}: '𛁐'
    HentaiganaLetterSuDash7,
    /// \u{1b051}: '𛁑'
    HentaiganaLetterSuDash8,
    /// \u{1b052}: '𛁒'
    HentaiganaLetterSeDash1,
    /// \u{1b053}: '𛁓'
    HentaiganaLetterSeDash2,
    /// \u{1b054}: '𛁔'
    HentaiganaLetterSeDash3,
    /// \u{1b055}: '𛁕'
    HentaiganaLetterSeDash4,
    /// \u{1b056}: '𛁖'
    HentaiganaLetterSeDash5,
    /// \u{1b057}: '𛁗'
    HentaiganaLetterSoDash1,
    /// \u{1b058}: '𛁘'
    HentaiganaLetterSoDash2,
    /// \u{1b059}: '𛁙'
    HentaiganaLetterSoDash3,
    /// \u{1b05a}: '𛁚'
    HentaiganaLetterSoDash4,
    /// \u{1b05b}: '𛁛'
    HentaiganaLetterSoDash5,
    /// \u{1b05c}: '𛁜'
    HentaiganaLetterSoDash6,
    /// \u{1b05d}: '𛁝'
    HentaiganaLetterSoDash7,
    /// \u{1b05e}: '𛁞'
    HentaiganaLetterTaDash1,
    /// \u{1b05f}: '𛁟'
    HentaiganaLetterTaDash2,
    /// \u{1b060}: '𛁠'
    HentaiganaLetterTaDash3,
    /// \u{1b061}: '𛁡'
    HentaiganaLetterTaDash4,
    /// \u{1b062}: '𛁢'
    HentaiganaLetterTiDash1,
    /// \u{1b063}: '𛁣'
    HentaiganaLetterTiDash2,
    /// \u{1b064}: '𛁤'
    HentaiganaLetterTiDash3,
    /// \u{1b065}: '𛁥'
    HentaiganaLetterTiDash4,
    /// \u{1b066}: '𛁦'
    HentaiganaLetterTiDash5,
    /// \u{1b067}: '𛁧'
    HentaiganaLetterTiDash6,
    /// \u{1b068}: '𛁨'
    HentaiganaLetterTiDash7,
    /// \u{1b069}: '𛁩'
    HentaiganaLetterTuDash1,
    /// \u{1b06a}: '𛁪'
    HentaiganaLetterTuDash2,
    /// \u{1b06b}: '𛁫'
    HentaiganaLetterTuDash3,
    /// \u{1b06c}: '𛁬'
    HentaiganaLetterTuDash4,
    /// \u{1b06d}: '𛁭'
    HentaiganaLetterTuDashTo,
    /// \u{1b06e}: '𛁮'
    HentaiganaLetterTeDash1,
    /// \u{1b06f}: '𛁯'
    HentaiganaLetterTeDash2,
    /// \u{1b070}: '𛁰'
    HentaiganaLetterTeDash3,
    /// \u{1b071}: '𛁱'
    HentaiganaLetterTeDash4,
    /// \u{1b072}: '𛁲'
    HentaiganaLetterTeDash5,
    /// \u{1b073}: '𛁳'
    HentaiganaLetterTeDash6,
    /// \u{1b074}: '𛁴'
    HentaiganaLetterTeDash7,
    /// \u{1b075}: '𛁵'
    HentaiganaLetterTeDash8,
    /// \u{1b076}: '𛁶'
    HentaiganaLetterTeDash9,
    /// \u{1b077}: '𛁷'
    HentaiganaLetterToDash1,
    /// \u{1b078}: '𛁸'
    HentaiganaLetterToDash2,
    /// \u{1b079}: '𛁹'
    HentaiganaLetterToDash3,
    /// \u{1b07a}: '𛁺'
    HentaiganaLetterToDash4,
    /// \u{1b07b}: '𛁻'
    HentaiganaLetterToDash5,
    /// \u{1b07c}: '𛁼'
    HentaiganaLetterToDash6,
    /// \u{1b07d}: '𛁽'
    HentaiganaLetterToDashRa,
    /// \u{1b07e}: '𛁾'
    HentaiganaLetterNaDash1,
    /// \u{1b07f}: '𛁿'
    HentaiganaLetterNaDash2,
    /// \u{1b080}: '𛂀'
    HentaiganaLetterNaDash3,
    /// \u{1b081}: '𛂁'
    HentaiganaLetterNaDash4,
    /// \u{1b082}: '𛂂'
    HentaiganaLetterNaDash5,
    /// \u{1b083}: '𛂃'
    HentaiganaLetterNaDash6,
    /// \u{1b084}: '𛂄'
    HentaiganaLetterNaDash7,
    /// \u{1b085}: '𛂅'
    HentaiganaLetterNaDash8,
    /// \u{1b086}: '𛂆'
    HentaiganaLetterNaDash9,
    /// \u{1b087}: '𛂇'
    HentaiganaLetterNiDash1,
    /// \u{1b088}: '𛂈'
    HentaiganaLetterNiDash2,
    /// \u{1b089}: '𛂉'
    HentaiganaLetterNiDash3,
    /// \u{1b08a}: '𛂊'
    HentaiganaLetterNiDash4,
    /// \u{1b08b}: '𛂋'
    HentaiganaLetterNiDash5,
    /// \u{1b08c}: '𛂌'
    HentaiganaLetterNiDash6,
    /// \u{1b08d}: '𛂍'
    HentaiganaLetterNiDash7,
    /// \u{1b08e}: '𛂎'
    HentaiganaLetterNiDashTe,
    /// \u{1b08f}: '𛂏'
    HentaiganaLetterNuDash1,
    /// \u{1b090}: '𛂐'
    HentaiganaLetterNuDash2,
    /// \u{1b091}: '𛂑'
    HentaiganaLetterNuDash3,
    /// \u{1b092}: '𛂒'
    HentaiganaLetterNeDash1,
    /// \u{1b093}: '𛂓'
    HentaiganaLetterNeDash2,
    /// \u{1b094}: '𛂔'
    HentaiganaLetterNeDash3,
    /// \u{1b095}: '𛂕'
    HentaiganaLetterNeDash4,
    /// \u{1b096}: '𛂖'
    HentaiganaLetterNeDash5,
    /// \u{1b097}: '𛂗'
    HentaiganaLetterNeDash6,
    /// \u{1b098}: '𛂘'
    HentaiganaLetterNeDashKo,
    /// \u{1b099}: '𛂙'
    HentaiganaLetterNoDash1,
    /// \u{1b09a}: '𛂚'
    HentaiganaLetterNoDash2,
    /// \u{1b09b}: '𛂛'
    HentaiganaLetterNoDash3,
    /// \u{1b09c}: '𛂜'
    HentaiganaLetterNoDash4,
    /// \u{1b09d}: '𛂝'
    HentaiganaLetterNoDash5,
    /// \u{1b09e}: '𛂞'
    HentaiganaLetterHaDash1,
    /// \u{1b09f}: '𛂟'
    HentaiganaLetterHaDash2,
    /// \u{1b0a0}: '𛂠'
    HentaiganaLetterHaDash3,
    /// \u{1b0a1}: '𛂡'
    HentaiganaLetterHaDash4,
    /// \u{1b0a2}: '𛂢'
    HentaiganaLetterHaDash5,
    /// \u{1b0a3}: '𛂣'
    HentaiganaLetterHaDash6,
    /// \u{1b0a4}: '𛂤'
    HentaiganaLetterHaDash7,
    /// \u{1b0a5}: '𛂥'
    HentaiganaLetterHaDash8,
    /// \u{1b0a6}: '𛂦'
    HentaiganaLetterHaDash9,
    /// \u{1b0a7}: '𛂧'
    HentaiganaLetterHaDash10,
    /// \u{1b0a8}: '𛂨'
    HentaiganaLetterHaDash11,
    /// \u{1b0a9}: '𛂩'
    HentaiganaLetterHiDash1,
    /// \u{1b0aa}: '𛂪'
    HentaiganaLetterHiDash2,
    /// \u{1b0ab}: '𛂫'
    HentaiganaLetterHiDash3,
    /// \u{1b0ac}: '𛂬'
    HentaiganaLetterHiDash4,
    /// \u{1b0ad}: '𛂭'
    HentaiganaLetterHiDash5,
    /// \u{1b0ae}: '𛂮'
    HentaiganaLetterHiDash6,
    /// \u{1b0af}: '𛂯'
    HentaiganaLetterHiDash7,
    /// \u{1b0b0}: '𛂰'
    HentaiganaLetterHuDash1,
    /// \u{1b0b1}: '𛂱'
    HentaiganaLetterHuDash2,
    /// \u{1b0b2}: '𛂲'
    HentaiganaLetterHuDash3,
    /// \u{1b0b3}: '𛂳'
    HentaiganaLetterHeDash1,
    /// \u{1b0b4}: '𛂴'
    HentaiganaLetterHeDash2,
    /// \u{1b0b5}: '𛂵'
    HentaiganaLetterHeDash3,
    /// \u{1b0b6}: '𛂶'
    HentaiganaLetterHeDash4,
    /// \u{1b0b7}: '𛂷'
    HentaiganaLetterHeDash5,
    /// \u{1b0b8}: '𛂸'
    HentaiganaLetterHeDash6,
    /// \u{1b0b9}: '𛂹'
    HentaiganaLetterHeDash7,
    /// \u{1b0ba}: '𛂺'
    HentaiganaLetterHoDash1,
    /// \u{1b0bb}: '𛂻'
    HentaiganaLetterHoDash2,
    /// \u{1b0bc}: '𛂼'
    HentaiganaLetterHoDash3,
    /// \u{1b0bd}: '𛂽'
    HentaiganaLetterHoDash4,
    /// \u{1b0be}: '𛂾'
    HentaiganaLetterHoDash5,
    /// \u{1b0bf}: '𛂿'
    HentaiganaLetterHoDash6,
    /// \u{1b0c0}: '𛃀'
    HentaiganaLetterHoDash7,
    /// \u{1b0c1}: '𛃁'
    HentaiganaLetterHoDash8,
    /// \u{1b0c2}: '𛃂'
    HentaiganaLetterMaDash1,
    /// \u{1b0c3}: '𛃃'
    HentaiganaLetterMaDash2,
    /// \u{1b0c4}: '𛃄'
    HentaiganaLetterMaDash3,
    /// \u{1b0c5}: '𛃅'
    HentaiganaLetterMaDash4,
    /// \u{1b0c6}: '𛃆'
    HentaiganaLetterMaDash5,
    /// \u{1b0c7}: '𛃇'
    HentaiganaLetterMaDash6,
    /// \u{1b0c8}: '𛃈'
    HentaiganaLetterMaDash7,
    /// \u{1b0c9}: '𛃉'
    HentaiganaLetterMiDash1,
    /// \u{1b0ca}: '𛃊'
    HentaiganaLetterMiDash2,
    /// \u{1b0cb}: '𛃋'
    HentaiganaLetterMiDash3,
    /// \u{1b0cc}: '𛃌'
    HentaiganaLetterMiDash4,
    /// \u{1b0cd}: '𛃍'
    HentaiganaLetterMiDash5,
    /// \u{1b0ce}: '𛃎'
    HentaiganaLetterMiDash6,
    /// \u{1b0cf}: '𛃏'
    HentaiganaLetterMiDash7,
    /// \u{1b0d0}: '𛃐'
    HentaiganaLetterMuDash1,
    /// \u{1b0d1}: '𛃑'
    HentaiganaLetterMuDash2,
    /// \u{1b0d2}: '𛃒'
    HentaiganaLetterMuDash3,
    /// \u{1b0d3}: '𛃓'
    HentaiganaLetterMuDash4,
    /// \u{1b0d4}: '𛃔'
    HentaiganaLetterMeDash1,
    /// \u{1b0d5}: '𛃕'
    HentaiganaLetterMeDash2,
    /// \u{1b0d6}: '𛃖'
    HentaiganaLetterMeDashMa,
    /// \u{1b0d7}: '𛃗'
    HentaiganaLetterMoDash1,
    /// \u{1b0d8}: '𛃘'
    HentaiganaLetterMoDash2,
    /// \u{1b0d9}: '𛃙'
    HentaiganaLetterMoDash3,
    /// \u{1b0da}: '𛃚'
    HentaiganaLetterMoDash4,
    /// \u{1b0db}: '𛃛'
    HentaiganaLetterMoDash5,
    /// \u{1b0dc}: '𛃜'
    HentaiganaLetterMoDash6,
    /// \u{1b0dd}: '𛃝'
    HentaiganaLetterYaDash1,
    /// \u{1b0de}: '𛃞'
    HentaiganaLetterYaDash2,
    /// \u{1b0df}: '𛃟'
    HentaiganaLetterYaDash3,
    /// \u{1b0e0}: '𛃠'
    HentaiganaLetterYaDash4,
    /// \u{1b0e1}: '𛃡'
    HentaiganaLetterYaDash5,
    /// \u{1b0e2}: '𛃢'
    HentaiganaLetterYaDashYo,
    /// \u{1b0e3}: '𛃣'
    HentaiganaLetterYuDash1,
    /// \u{1b0e4}: '𛃤'
    HentaiganaLetterYuDash2,
    /// \u{1b0e5}: '𛃥'
    HentaiganaLetterYuDash3,
    /// \u{1b0e6}: '𛃦'
    HentaiganaLetterYuDash4,
    /// \u{1b0e7}: '𛃧'
    HentaiganaLetterYoDash1,
    /// \u{1b0e8}: '𛃨'
    HentaiganaLetterYoDash2,
    /// \u{1b0e9}: '𛃩'
    HentaiganaLetterYoDash3,
    /// \u{1b0ea}: '𛃪'
    HentaiganaLetterYoDash4,
    /// \u{1b0eb}: '𛃫'
    HentaiganaLetterYoDash5,
    /// \u{1b0ec}: '𛃬'
    HentaiganaLetterYoDash6,
    /// \u{1b0ed}: '𛃭'
    HentaiganaLetterRaDash1,
    /// \u{1b0ee}: '𛃮'
    HentaiganaLetterRaDash2,
    /// \u{1b0ef}: '𛃯'
    HentaiganaLetterRaDash3,
    /// \u{1b0f0}: '𛃰'
    HentaiganaLetterRaDash4,
    /// \u{1b0f1}: '𛃱'
    HentaiganaLetterRiDash1,
    /// \u{1b0f2}: '𛃲'
    HentaiganaLetterRiDash2,
    /// \u{1b0f3}: '𛃳'
    HentaiganaLetterRiDash3,
    /// \u{1b0f4}: '𛃴'
    HentaiganaLetterRiDash4,
    /// \u{1b0f5}: '𛃵'
    HentaiganaLetterRiDash5,
    /// \u{1b0f6}: '𛃶'
    HentaiganaLetterRiDash6,
    /// \u{1b0f7}: '𛃷'
    HentaiganaLetterRiDash7,
    /// \u{1b0f8}: '𛃸'
    HentaiganaLetterRuDash1,
    /// \u{1b0f9}: '𛃹'
    HentaiganaLetterRuDash2,
    /// \u{1b0fa}: '𛃺'
    HentaiganaLetterRuDash3,
    /// \u{1b0fb}: '𛃻'
    HentaiganaLetterRuDash4,
    /// \u{1b0fc}: '𛃼'
    HentaiganaLetterRuDash5,
    /// \u{1b0fd}: '𛃽'
    HentaiganaLetterRuDash6,
    /// \u{1b0fe}: '𛃾'
    HentaiganaLetterReDash1,
}

impl Into<char> for KanaSupplement {
    fn into(self) -> char {
        match self {
            KanaSupplement::KatakanaLetterArchaicE => '𛀀',
            KanaSupplement::HiraganaLetterArchaicYe => '𛀁',
            KanaSupplement::HentaiganaLetterADash1 => '𛀂',
            KanaSupplement::HentaiganaLetterADash2 => '𛀃',
            KanaSupplement::HentaiganaLetterADash3 => '𛀄',
            KanaSupplement::HentaiganaLetterADashWo => '𛀅',
            KanaSupplement::HentaiganaLetterIDash1 => '𛀆',
            KanaSupplement::HentaiganaLetterIDash2 => '𛀇',
            KanaSupplement::HentaiganaLetterIDash3 => '𛀈',
            KanaSupplement::HentaiganaLetterIDash4 => '𛀉',
            KanaSupplement::HentaiganaLetterUDash1 => '𛀊',
            KanaSupplement::HentaiganaLetterUDash2 => '𛀋',
            KanaSupplement::HentaiganaLetterUDash3 => '𛀌',
            KanaSupplement::HentaiganaLetterUDash4 => '𛀍',
            KanaSupplement::HentaiganaLetterUDash5 => '𛀎',
            KanaSupplement::HentaiganaLetterEDash2 => '𛀏',
            KanaSupplement::HentaiganaLetterEDash3 => '𛀐',
            KanaSupplement::HentaiganaLetterEDash4 => '𛀑',
            KanaSupplement::HentaiganaLetterEDash5 => '𛀒',
            KanaSupplement::HentaiganaLetterEDash6 => '𛀓',
            KanaSupplement::HentaiganaLetterODash1 => '𛀔',
            KanaSupplement::HentaiganaLetterODash2 => '𛀕',
            KanaSupplement::HentaiganaLetterODash3 => '𛀖',
            KanaSupplement::HentaiganaLetterKaDash1 => '𛀗',
            KanaSupplement::HentaiganaLetterKaDash2 => '𛀘',
            KanaSupplement::HentaiganaLetterKaDash3 => '𛀙',
            KanaSupplement::HentaiganaLetterKaDash4 => '𛀚',
            KanaSupplement::HentaiganaLetterKaDash5 => '𛀛',
            KanaSupplement::HentaiganaLetterKaDash6 => '𛀜',
            KanaSupplement::HentaiganaLetterKaDash7 => '𛀝',
            KanaSupplement::HentaiganaLetterKaDash8 => '𛀞',
            KanaSupplement::HentaiganaLetterKaDash9 => '𛀟',
            KanaSupplement::HentaiganaLetterKaDash10 => '𛀠',
            KanaSupplement::HentaiganaLetterKaDash11 => '𛀡',
            KanaSupplement::HentaiganaLetterKaDashKe => '𛀢',
            KanaSupplement::HentaiganaLetterKiDash1 => '𛀣',
            KanaSupplement::HentaiganaLetterKiDash2 => '𛀤',
            KanaSupplement::HentaiganaLetterKiDash3 => '𛀥',
            KanaSupplement::HentaiganaLetterKiDash4 => '𛀦',
            KanaSupplement::HentaiganaLetterKiDash5 => '𛀧',
            KanaSupplement::HentaiganaLetterKiDash6 => '𛀨',
            KanaSupplement::HentaiganaLetterKiDash7 => '𛀩',
            KanaSupplement::HentaiganaLetterKiDash8 => '𛀪',
            KanaSupplement::HentaiganaLetterKuDash1 => '𛀫',
            KanaSupplement::HentaiganaLetterKuDash2 => '𛀬',
            KanaSupplement::HentaiganaLetterKuDash3 => '𛀭',
            KanaSupplement::HentaiganaLetterKuDash4 => '𛀮',
            KanaSupplement::HentaiganaLetterKuDash5 => '𛀯',
            KanaSupplement::HentaiganaLetterKuDash6 => '𛀰',
            KanaSupplement::HentaiganaLetterKuDash7 => '𛀱',
            KanaSupplement::HentaiganaLetterKeDash1 => '𛀲',
            KanaSupplement::HentaiganaLetterKeDash2 => '𛀳',
            KanaSupplement::HentaiganaLetterKeDash3 => '𛀴',
            KanaSupplement::HentaiganaLetterKeDash4 => '𛀵',
            KanaSupplement::HentaiganaLetterKeDash5 => '𛀶',
            KanaSupplement::HentaiganaLetterKeDash6 => '𛀷',
            KanaSupplement::HentaiganaLetterKoDash1 => '𛀸',
            KanaSupplement::HentaiganaLetterKoDash2 => '𛀹',
            KanaSupplement::HentaiganaLetterKoDash3 => '𛀺',
            KanaSupplement::HentaiganaLetterKoDashKi => '𛀻',
            KanaSupplement::HentaiganaLetterSaDash1 => '𛀼',
            KanaSupplement::HentaiganaLetterSaDash2 => '𛀽',
            KanaSupplement::HentaiganaLetterSaDash3 => '𛀾',
            KanaSupplement::HentaiganaLetterSaDash4 => '𛀿',
            KanaSupplement::HentaiganaLetterSaDash5 => '𛁀',
            KanaSupplement::HentaiganaLetterSaDash6 => '𛁁',
            KanaSupplement::HentaiganaLetterSaDash7 => '𛁂',
            KanaSupplement::HentaiganaLetterSaDash8 => '𛁃',
            KanaSupplement::HentaiganaLetterSiDash1 => '𛁄',
            KanaSupplement::HentaiganaLetterSiDash2 => '𛁅',
            KanaSupplement::HentaiganaLetterSiDash3 => '𛁆',
            KanaSupplement::HentaiganaLetterSiDash4 => '𛁇',
            KanaSupplement::HentaiganaLetterSiDash5 => '𛁈',
            KanaSupplement::HentaiganaLetterSiDash6 => '𛁉',
            KanaSupplement::HentaiganaLetterSuDash1 => '𛁊',
            KanaSupplement::HentaiganaLetterSuDash2 => '𛁋',
            KanaSupplement::HentaiganaLetterSuDash3 => '𛁌',
            KanaSupplement::HentaiganaLetterSuDash4 => '𛁍',
            KanaSupplement::HentaiganaLetterSuDash5 => '𛁎',
            KanaSupplement::HentaiganaLetterSuDash6 => '𛁏',
            KanaSupplement::HentaiganaLetterSuDash7 => '𛁐',
            KanaSupplement::HentaiganaLetterSuDash8 => '𛁑',
            KanaSupplement::HentaiganaLetterSeDash1 => '𛁒',
            KanaSupplement::HentaiganaLetterSeDash2 => '𛁓',
            KanaSupplement::HentaiganaLetterSeDash3 => '𛁔',
            KanaSupplement::HentaiganaLetterSeDash4 => '𛁕',
            KanaSupplement::HentaiganaLetterSeDash5 => '𛁖',
            KanaSupplement::HentaiganaLetterSoDash1 => '𛁗',
            KanaSupplement::HentaiganaLetterSoDash2 => '𛁘',
            KanaSupplement::HentaiganaLetterSoDash3 => '𛁙',
            KanaSupplement::HentaiganaLetterSoDash4 => '𛁚',
            KanaSupplement::HentaiganaLetterSoDash5 => '𛁛',
            KanaSupplement::HentaiganaLetterSoDash6 => '𛁜',
            KanaSupplement::HentaiganaLetterSoDash7 => '𛁝',
            KanaSupplement::HentaiganaLetterTaDash1 => '𛁞',
            KanaSupplement::HentaiganaLetterTaDash2 => '𛁟',
            KanaSupplement::HentaiganaLetterTaDash3 => '𛁠',
            KanaSupplement::HentaiganaLetterTaDash4 => '𛁡',
            KanaSupplement::HentaiganaLetterTiDash1 => '𛁢',
            KanaSupplement::HentaiganaLetterTiDash2 => '𛁣',
            KanaSupplement::HentaiganaLetterTiDash3 => '𛁤',
            KanaSupplement::HentaiganaLetterTiDash4 => '𛁥',
            KanaSupplement::HentaiganaLetterTiDash5 => '𛁦',
            KanaSupplement::HentaiganaLetterTiDash6 => '𛁧',
            KanaSupplement::HentaiganaLetterTiDash7 => '𛁨',
            KanaSupplement::HentaiganaLetterTuDash1 => '𛁩',
            KanaSupplement::HentaiganaLetterTuDash2 => '𛁪',
            KanaSupplement::HentaiganaLetterTuDash3 => '𛁫',
            KanaSupplement::HentaiganaLetterTuDash4 => '𛁬',
            KanaSupplement::HentaiganaLetterTuDashTo => '𛁭',
            KanaSupplement::HentaiganaLetterTeDash1 => '𛁮',
            KanaSupplement::HentaiganaLetterTeDash2 => '𛁯',
            KanaSupplement::HentaiganaLetterTeDash3 => '𛁰',
            KanaSupplement::HentaiganaLetterTeDash4 => '𛁱',
            KanaSupplement::HentaiganaLetterTeDash5 => '𛁲',
            KanaSupplement::HentaiganaLetterTeDash6 => '𛁳',
            KanaSupplement::HentaiganaLetterTeDash7 => '𛁴',
            KanaSupplement::HentaiganaLetterTeDash8 => '𛁵',
            KanaSupplement::HentaiganaLetterTeDash9 => '𛁶',
            KanaSupplement::HentaiganaLetterToDash1 => '𛁷',
            KanaSupplement::HentaiganaLetterToDash2 => '𛁸',
            KanaSupplement::HentaiganaLetterToDash3 => '𛁹',
            KanaSupplement::HentaiganaLetterToDash4 => '𛁺',
            KanaSupplement::HentaiganaLetterToDash5 => '𛁻',
            KanaSupplement::HentaiganaLetterToDash6 => '𛁼',
            KanaSupplement::HentaiganaLetterToDashRa => '𛁽',
            KanaSupplement::HentaiganaLetterNaDash1 => '𛁾',
            KanaSupplement::HentaiganaLetterNaDash2 => '𛁿',
            KanaSupplement::HentaiganaLetterNaDash3 => '𛂀',
            KanaSupplement::HentaiganaLetterNaDash4 => '𛂁',
            KanaSupplement::HentaiganaLetterNaDash5 => '𛂂',
            KanaSupplement::HentaiganaLetterNaDash6 => '𛂃',
            KanaSupplement::HentaiganaLetterNaDash7 => '𛂄',
            KanaSupplement::HentaiganaLetterNaDash8 => '𛂅',
            KanaSupplement::HentaiganaLetterNaDash9 => '𛂆',
            KanaSupplement::HentaiganaLetterNiDash1 => '𛂇',
            KanaSupplement::HentaiganaLetterNiDash2 => '𛂈',
            KanaSupplement::HentaiganaLetterNiDash3 => '𛂉',
            KanaSupplement::HentaiganaLetterNiDash4 => '𛂊',
            KanaSupplement::HentaiganaLetterNiDash5 => '𛂋',
            KanaSupplement::HentaiganaLetterNiDash6 => '𛂌',
            KanaSupplement::HentaiganaLetterNiDash7 => '𛂍',
            KanaSupplement::HentaiganaLetterNiDashTe => '𛂎',
            KanaSupplement::HentaiganaLetterNuDash1 => '𛂏',
            KanaSupplement::HentaiganaLetterNuDash2 => '𛂐',
            KanaSupplement::HentaiganaLetterNuDash3 => '𛂑',
            KanaSupplement::HentaiganaLetterNeDash1 => '𛂒',
            KanaSupplement::HentaiganaLetterNeDash2 => '𛂓',
            KanaSupplement::HentaiganaLetterNeDash3 => '𛂔',
            KanaSupplement::HentaiganaLetterNeDash4 => '𛂕',
            KanaSupplement::HentaiganaLetterNeDash5 => '𛂖',
            KanaSupplement::HentaiganaLetterNeDash6 => '𛂗',
            KanaSupplement::HentaiganaLetterNeDashKo => '𛂘',
            KanaSupplement::HentaiganaLetterNoDash1 => '𛂙',
            KanaSupplement::HentaiganaLetterNoDash2 => '𛂚',
            KanaSupplement::HentaiganaLetterNoDash3 => '𛂛',
            KanaSupplement::HentaiganaLetterNoDash4 => '𛂜',
            KanaSupplement::HentaiganaLetterNoDash5 => '𛂝',
            KanaSupplement::HentaiganaLetterHaDash1 => '𛂞',
            KanaSupplement::HentaiganaLetterHaDash2 => '𛂟',
            KanaSupplement::HentaiganaLetterHaDash3 => '𛂠',
            KanaSupplement::HentaiganaLetterHaDash4 => '𛂡',
            KanaSupplement::HentaiganaLetterHaDash5 => '𛂢',
            KanaSupplement::HentaiganaLetterHaDash6 => '𛂣',
            KanaSupplement::HentaiganaLetterHaDash7 => '𛂤',
            KanaSupplement::HentaiganaLetterHaDash8 => '𛂥',
            KanaSupplement::HentaiganaLetterHaDash9 => '𛂦',
            KanaSupplement::HentaiganaLetterHaDash10 => '𛂧',
            KanaSupplement::HentaiganaLetterHaDash11 => '𛂨',
            KanaSupplement::HentaiganaLetterHiDash1 => '𛂩',
            KanaSupplement::HentaiganaLetterHiDash2 => '𛂪',
            KanaSupplement::HentaiganaLetterHiDash3 => '𛂫',
            KanaSupplement::HentaiganaLetterHiDash4 => '𛂬',
            KanaSupplement::HentaiganaLetterHiDash5 => '𛂭',
            KanaSupplement::HentaiganaLetterHiDash6 => '𛂮',
            KanaSupplement::HentaiganaLetterHiDash7 => '𛂯',
            KanaSupplement::HentaiganaLetterHuDash1 => '𛂰',
            KanaSupplement::HentaiganaLetterHuDash2 => '𛂱',
            KanaSupplement::HentaiganaLetterHuDash3 => '𛂲',
            KanaSupplement::HentaiganaLetterHeDash1 => '𛂳',
            KanaSupplement::HentaiganaLetterHeDash2 => '𛂴',
            KanaSupplement::HentaiganaLetterHeDash3 => '𛂵',
            KanaSupplement::HentaiganaLetterHeDash4 => '𛂶',
            KanaSupplement::HentaiganaLetterHeDash5 => '𛂷',
            KanaSupplement::HentaiganaLetterHeDash6 => '𛂸',
            KanaSupplement::HentaiganaLetterHeDash7 => '𛂹',
            KanaSupplement::HentaiganaLetterHoDash1 => '𛂺',
            KanaSupplement::HentaiganaLetterHoDash2 => '𛂻',
            KanaSupplement::HentaiganaLetterHoDash3 => '𛂼',
            KanaSupplement::HentaiganaLetterHoDash4 => '𛂽',
            KanaSupplement::HentaiganaLetterHoDash5 => '𛂾',
            KanaSupplement::HentaiganaLetterHoDash6 => '𛂿',
            KanaSupplement::HentaiganaLetterHoDash7 => '𛃀',
            KanaSupplement::HentaiganaLetterHoDash8 => '𛃁',
            KanaSupplement::HentaiganaLetterMaDash1 => '𛃂',
            KanaSupplement::HentaiganaLetterMaDash2 => '𛃃',
            KanaSupplement::HentaiganaLetterMaDash3 => '𛃄',
            KanaSupplement::HentaiganaLetterMaDash4 => '𛃅',
            KanaSupplement::HentaiganaLetterMaDash5 => '𛃆',
            KanaSupplement::HentaiganaLetterMaDash6 => '𛃇',
            KanaSupplement::HentaiganaLetterMaDash7 => '𛃈',
            KanaSupplement::HentaiganaLetterMiDash1 => '𛃉',
            KanaSupplement::HentaiganaLetterMiDash2 => '𛃊',
            KanaSupplement::HentaiganaLetterMiDash3 => '𛃋',
            KanaSupplement::HentaiganaLetterMiDash4 => '𛃌',
            KanaSupplement::HentaiganaLetterMiDash5 => '𛃍',
            KanaSupplement::HentaiganaLetterMiDash6 => '𛃎',
            KanaSupplement::HentaiganaLetterMiDash7 => '𛃏',
            KanaSupplement::HentaiganaLetterMuDash1 => '𛃐',
            KanaSupplement::HentaiganaLetterMuDash2 => '𛃑',
            KanaSupplement::HentaiganaLetterMuDash3 => '𛃒',
            KanaSupplement::HentaiganaLetterMuDash4 => '𛃓',
            KanaSupplement::HentaiganaLetterMeDash1 => '𛃔',
            KanaSupplement::HentaiganaLetterMeDash2 => '𛃕',
            KanaSupplement::HentaiganaLetterMeDashMa => '𛃖',
            KanaSupplement::HentaiganaLetterMoDash1 => '𛃗',
            KanaSupplement::HentaiganaLetterMoDash2 => '𛃘',
            KanaSupplement::HentaiganaLetterMoDash3 => '𛃙',
            KanaSupplement::HentaiganaLetterMoDash4 => '𛃚',
            KanaSupplement::HentaiganaLetterMoDash5 => '𛃛',
            KanaSupplement::HentaiganaLetterMoDash6 => '𛃜',
            KanaSupplement::HentaiganaLetterYaDash1 => '𛃝',
            KanaSupplement::HentaiganaLetterYaDash2 => '𛃞',
            KanaSupplement::HentaiganaLetterYaDash3 => '𛃟',
            KanaSupplement::HentaiganaLetterYaDash4 => '𛃠',
            KanaSupplement::HentaiganaLetterYaDash5 => '𛃡',
            KanaSupplement::HentaiganaLetterYaDashYo => '𛃢',
            KanaSupplement::HentaiganaLetterYuDash1 => '𛃣',
            KanaSupplement::HentaiganaLetterYuDash2 => '𛃤',
            KanaSupplement::HentaiganaLetterYuDash3 => '𛃥',
            KanaSupplement::HentaiganaLetterYuDash4 => '𛃦',
            KanaSupplement::HentaiganaLetterYoDash1 => '𛃧',
            KanaSupplement::HentaiganaLetterYoDash2 => '𛃨',
            KanaSupplement::HentaiganaLetterYoDash3 => '𛃩',
            KanaSupplement::HentaiganaLetterYoDash4 => '𛃪',
            KanaSupplement::HentaiganaLetterYoDash5 => '𛃫',
            KanaSupplement::HentaiganaLetterYoDash6 => '𛃬',
            KanaSupplement::HentaiganaLetterRaDash1 => '𛃭',
            KanaSupplement::HentaiganaLetterRaDash2 => '𛃮',
            KanaSupplement::HentaiganaLetterRaDash3 => '𛃯',
            KanaSupplement::HentaiganaLetterRaDash4 => '𛃰',
            KanaSupplement::HentaiganaLetterRiDash1 => '𛃱',
            KanaSupplement::HentaiganaLetterRiDash2 => '𛃲',
            KanaSupplement::HentaiganaLetterRiDash3 => '𛃳',
            KanaSupplement::HentaiganaLetterRiDash4 => '𛃴',
            KanaSupplement::HentaiganaLetterRiDash5 => '𛃵',
            KanaSupplement::HentaiganaLetterRiDash6 => '𛃶',
            KanaSupplement::HentaiganaLetterRiDash7 => '𛃷',
            KanaSupplement::HentaiganaLetterRuDash1 => '𛃸',
            KanaSupplement::HentaiganaLetterRuDash2 => '𛃹',
            KanaSupplement::HentaiganaLetterRuDash3 => '𛃺',
            KanaSupplement::HentaiganaLetterRuDash4 => '𛃻',
            KanaSupplement::HentaiganaLetterRuDash5 => '𛃼',
            KanaSupplement::HentaiganaLetterRuDash6 => '𛃽',
            KanaSupplement::HentaiganaLetterReDash1 => '𛃾',
        }
    }
}

impl std::convert::TryFrom<char> for KanaSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𛀀' => Ok(KanaSupplement::KatakanaLetterArchaicE),
            '𛀁' => Ok(KanaSupplement::HiraganaLetterArchaicYe),
            '𛀂' => Ok(KanaSupplement::HentaiganaLetterADash1),
            '𛀃' => Ok(KanaSupplement::HentaiganaLetterADash2),
            '𛀄' => Ok(KanaSupplement::HentaiganaLetterADash3),
            '𛀅' => Ok(KanaSupplement::HentaiganaLetterADashWo),
            '𛀆' => Ok(KanaSupplement::HentaiganaLetterIDash1),
            '𛀇' => Ok(KanaSupplement::HentaiganaLetterIDash2),
            '𛀈' => Ok(KanaSupplement::HentaiganaLetterIDash3),
            '𛀉' => Ok(KanaSupplement::HentaiganaLetterIDash4),
            '𛀊' => Ok(KanaSupplement::HentaiganaLetterUDash1),
            '𛀋' => Ok(KanaSupplement::HentaiganaLetterUDash2),
            '𛀌' => Ok(KanaSupplement::HentaiganaLetterUDash3),
            '𛀍' => Ok(KanaSupplement::HentaiganaLetterUDash4),
            '𛀎' => Ok(KanaSupplement::HentaiganaLetterUDash5),
            '𛀏' => Ok(KanaSupplement::HentaiganaLetterEDash2),
            '𛀐' => Ok(KanaSupplement::HentaiganaLetterEDash3),
            '𛀑' => Ok(KanaSupplement::HentaiganaLetterEDash4),
            '𛀒' => Ok(KanaSupplement::HentaiganaLetterEDash5),
            '𛀓' => Ok(KanaSupplement::HentaiganaLetterEDash6),
            '𛀔' => Ok(KanaSupplement::HentaiganaLetterODash1),
            '𛀕' => Ok(KanaSupplement::HentaiganaLetterODash2),
            '𛀖' => Ok(KanaSupplement::HentaiganaLetterODash3),
            '𛀗' => Ok(KanaSupplement::HentaiganaLetterKaDash1),
            '𛀘' => Ok(KanaSupplement::HentaiganaLetterKaDash2),
            '𛀙' => Ok(KanaSupplement::HentaiganaLetterKaDash3),
            '𛀚' => Ok(KanaSupplement::HentaiganaLetterKaDash4),
            '𛀛' => Ok(KanaSupplement::HentaiganaLetterKaDash5),
            '𛀜' => Ok(KanaSupplement::HentaiganaLetterKaDash6),
            '𛀝' => Ok(KanaSupplement::HentaiganaLetterKaDash7),
            '𛀞' => Ok(KanaSupplement::HentaiganaLetterKaDash8),
            '𛀟' => Ok(KanaSupplement::HentaiganaLetterKaDash9),
            '𛀠' => Ok(KanaSupplement::HentaiganaLetterKaDash10),
            '𛀡' => Ok(KanaSupplement::HentaiganaLetterKaDash11),
            '𛀢' => Ok(KanaSupplement::HentaiganaLetterKaDashKe),
            '𛀣' => Ok(KanaSupplement::HentaiganaLetterKiDash1),
            '𛀤' => Ok(KanaSupplement::HentaiganaLetterKiDash2),
            '𛀥' => Ok(KanaSupplement::HentaiganaLetterKiDash3),
            '𛀦' => Ok(KanaSupplement::HentaiganaLetterKiDash4),
            '𛀧' => Ok(KanaSupplement::HentaiganaLetterKiDash5),
            '𛀨' => Ok(KanaSupplement::HentaiganaLetterKiDash6),
            '𛀩' => Ok(KanaSupplement::HentaiganaLetterKiDash7),
            '𛀪' => Ok(KanaSupplement::HentaiganaLetterKiDash8),
            '𛀫' => Ok(KanaSupplement::HentaiganaLetterKuDash1),
            '𛀬' => Ok(KanaSupplement::HentaiganaLetterKuDash2),
            '𛀭' => Ok(KanaSupplement::HentaiganaLetterKuDash3),
            '𛀮' => Ok(KanaSupplement::HentaiganaLetterKuDash4),
            '𛀯' => Ok(KanaSupplement::HentaiganaLetterKuDash5),
            '𛀰' => Ok(KanaSupplement::HentaiganaLetterKuDash6),
            '𛀱' => Ok(KanaSupplement::HentaiganaLetterKuDash7),
            '𛀲' => Ok(KanaSupplement::HentaiganaLetterKeDash1),
            '𛀳' => Ok(KanaSupplement::HentaiganaLetterKeDash2),
            '𛀴' => Ok(KanaSupplement::HentaiganaLetterKeDash3),
            '𛀵' => Ok(KanaSupplement::HentaiganaLetterKeDash4),
            '𛀶' => Ok(KanaSupplement::HentaiganaLetterKeDash5),
            '𛀷' => Ok(KanaSupplement::HentaiganaLetterKeDash6),
            '𛀸' => Ok(KanaSupplement::HentaiganaLetterKoDash1),
            '𛀹' => Ok(KanaSupplement::HentaiganaLetterKoDash2),
            '𛀺' => Ok(KanaSupplement::HentaiganaLetterKoDash3),
            '𛀻' => Ok(KanaSupplement::HentaiganaLetterKoDashKi),
            '𛀼' => Ok(KanaSupplement::HentaiganaLetterSaDash1),
            '𛀽' => Ok(KanaSupplement::HentaiganaLetterSaDash2),
            '𛀾' => Ok(KanaSupplement::HentaiganaLetterSaDash3),
            '𛀿' => Ok(KanaSupplement::HentaiganaLetterSaDash4),
            '𛁀' => Ok(KanaSupplement::HentaiganaLetterSaDash5),
            '𛁁' => Ok(KanaSupplement::HentaiganaLetterSaDash6),
            '𛁂' => Ok(KanaSupplement::HentaiganaLetterSaDash7),
            '𛁃' => Ok(KanaSupplement::HentaiganaLetterSaDash8),
            '𛁄' => Ok(KanaSupplement::HentaiganaLetterSiDash1),
            '𛁅' => Ok(KanaSupplement::HentaiganaLetterSiDash2),
            '𛁆' => Ok(KanaSupplement::HentaiganaLetterSiDash3),
            '𛁇' => Ok(KanaSupplement::HentaiganaLetterSiDash4),
            '𛁈' => Ok(KanaSupplement::HentaiganaLetterSiDash5),
            '𛁉' => Ok(KanaSupplement::HentaiganaLetterSiDash6),
            '𛁊' => Ok(KanaSupplement::HentaiganaLetterSuDash1),
            '𛁋' => Ok(KanaSupplement::HentaiganaLetterSuDash2),
            '𛁌' => Ok(KanaSupplement::HentaiganaLetterSuDash3),
            '𛁍' => Ok(KanaSupplement::HentaiganaLetterSuDash4),
            '𛁎' => Ok(KanaSupplement::HentaiganaLetterSuDash5),
            '𛁏' => Ok(KanaSupplement::HentaiganaLetterSuDash6),
            '𛁐' => Ok(KanaSupplement::HentaiganaLetterSuDash7),
            '𛁑' => Ok(KanaSupplement::HentaiganaLetterSuDash8),
            '𛁒' => Ok(KanaSupplement::HentaiganaLetterSeDash1),
            '𛁓' => Ok(KanaSupplement::HentaiganaLetterSeDash2),
            '𛁔' => Ok(KanaSupplement::HentaiganaLetterSeDash3),
            '𛁕' => Ok(KanaSupplement::HentaiganaLetterSeDash4),
            '𛁖' => Ok(KanaSupplement::HentaiganaLetterSeDash5),
            '𛁗' => Ok(KanaSupplement::HentaiganaLetterSoDash1),
            '𛁘' => Ok(KanaSupplement::HentaiganaLetterSoDash2),
            '𛁙' => Ok(KanaSupplement::HentaiganaLetterSoDash3),
            '𛁚' => Ok(KanaSupplement::HentaiganaLetterSoDash4),
            '𛁛' => Ok(KanaSupplement::HentaiganaLetterSoDash5),
            '𛁜' => Ok(KanaSupplement::HentaiganaLetterSoDash6),
            '𛁝' => Ok(KanaSupplement::HentaiganaLetterSoDash7),
            '𛁞' => Ok(KanaSupplement::HentaiganaLetterTaDash1),
            '𛁟' => Ok(KanaSupplement::HentaiganaLetterTaDash2),
            '𛁠' => Ok(KanaSupplement::HentaiganaLetterTaDash3),
            '𛁡' => Ok(KanaSupplement::HentaiganaLetterTaDash4),
            '𛁢' => Ok(KanaSupplement::HentaiganaLetterTiDash1),
            '𛁣' => Ok(KanaSupplement::HentaiganaLetterTiDash2),
            '𛁤' => Ok(KanaSupplement::HentaiganaLetterTiDash3),
            '𛁥' => Ok(KanaSupplement::HentaiganaLetterTiDash4),
            '𛁦' => Ok(KanaSupplement::HentaiganaLetterTiDash5),
            '𛁧' => Ok(KanaSupplement::HentaiganaLetterTiDash6),
            '𛁨' => Ok(KanaSupplement::HentaiganaLetterTiDash7),
            '𛁩' => Ok(KanaSupplement::HentaiganaLetterTuDash1),
            '𛁪' => Ok(KanaSupplement::HentaiganaLetterTuDash2),
            '𛁫' => Ok(KanaSupplement::HentaiganaLetterTuDash3),
            '𛁬' => Ok(KanaSupplement::HentaiganaLetterTuDash4),
            '𛁭' => Ok(KanaSupplement::HentaiganaLetterTuDashTo),
            '𛁮' => Ok(KanaSupplement::HentaiganaLetterTeDash1),
            '𛁯' => Ok(KanaSupplement::HentaiganaLetterTeDash2),
            '𛁰' => Ok(KanaSupplement::HentaiganaLetterTeDash3),
            '𛁱' => Ok(KanaSupplement::HentaiganaLetterTeDash4),
            '𛁲' => Ok(KanaSupplement::HentaiganaLetterTeDash5),
            '𛁳' => Ok(KanaSupplement::HentaiganaLetterTeDash6),
            '𛁴' => Ok(KanaSupplement::HentaiganaLetterTeDash7),
            '𛁵' => Ok(KanaSupplement::HentaiganaLetterTeDash8),
            '𛁶' => Ok(KanaSupplement::HentaiganaLetterTeDash9),
            '𛁷' => Ok(KanaSupplement::HentaiganaLetterToDash1),
            '𛁸' => Ok(KanaSupplement::HentaiganaLetterToDash2),
            '𛁹' => Ok(KanaSupplement::HentaiganaLetterToDash3),
            '𛁺' => Ok(KanaSupplement::HentaiganaLetterToDash4),
            '𛁻' => Ok(KanaSupplement::HentaiganaLetterToDash5),
            '𛁼' => Ok(KanaSupplement::HentaiganaLetterToDash6),
            '𛁽' => Ok(KanaSupplement::HentaiganaLetterToDashRa),
            '𛁾' => Ok(KanaSupplement::HentaiganaLetterNaDash1),
            '𛁿' => Ok(KanaSupplement::HentaiganaLetterNaDash2),
            '𛂀' => Ok(KanaSupplement::HentaiganaLetterNaDash3),
            '𛂁' => Ok(KanaSupplement::HentaiganaLetterNaDash4),
            '𛂂' => Ok(KanaSupplement::HentaiganaLetterNaDash5),
            '𛂃' => Ok(KanaSupplement::HentaiganaLetterNaDash6),
            '𛂄' => Ok(KanaSupplement::HentaiganaLetterNaDash7),
            '𛂅' => Ok(KanaSupplement::HentaiganaLetterNaDash8),
            '𛂆' => Ok(KanaSupplement::HentaiganaLetterNaDash9),
            '𛂇' => Ok(KanaSupplement::HentaiganaLetterNiDash1),
            '𛂈' => Ok(KanaSupplement::HentaiganaLetterNiDash2),
            '𛂉' => Ok(KanaSupplement::HentaiganaLetterNiDash3),
            '𛂊' => Ok(KanaSupplement::HentaiganaLetterNiDash4),
            '𛂋' => Ok(KanaSupplement::HentaiganaLetterNiDash5),
            '𛂌' => Ok(KanaSupplement::HentaiganaLetterNiDash6),
            '𛂍' => Ok(KanaSupplement::HentaiganaLetterNiDash7),
            '𛂎' => Ok(KanaSupplement::HentaiganaLetterNiDashTe),
            '𛂏' => Ok(KanaSupplement::HentaiganaLetterNuDash1),
            '𛂐' => Ok(KanaSupplement::HentaiganaLetterNuDash2),
            '𛂑' => Ok(KanaSupplement::HentaiganaLetterNuDash3),
            '𛂒' => Ok(KanaSupplement::HentaiganaLetterNeDash1),
            '𛂓' => Ok(KanaSupplement::HentaiganaLetterNeDash2),
            '𛂔' => Ok(KanaSupplement::HentaiganaLetterNeDash3),
            '𛂕' => Ok(KanaSupplement::HentaiganaLetterNeDash4),
            '𛂖' => Ok(KanaSupplement::HentaiganaLetterNeDash5),
            '𛂗' => Ok(KanaSupplement::HentaiganaLetterNeDash6),
            '𛂘' => Ok(KanaSupplement::HentaiganaLetterNeDashKo),
            '𛂙' => Ok(KanaSupplement::HentaiganaLetterNoDash1),
            '𛂚' => Ok(KanaSupplement::HentaiganaLetterNoDash2),
            '𛂛' => Ok(KanaSupplement::HentaiganaLetterNoDash3),
            '𛂜' => Ok(KanaSupplement::HentaiganaLetterNoDash4),
            '𛂝' => Ok(KanaSupplement::HentaiganaLetterNoDash5),
            '𛂞' => Ok(KanaSupplement::HentaiganaLetterHaDash1),
            '𛂟' => Ok(KanaSupplement::HentaiganaLetterHaDash2),
            '𛂠' => Ok(KanaSupplement::HentaiganaLetterHaDash3),
            '𛂡' => Ok(KanaSupplement::HentaiganaLetterHaDash4),
            '𛂢' => Ok(KanaSupplement::HentaiganaLetterHaDash5),
            '𛂣' => Ok(KanaSupplement::HentaiganaLetterHaDash6),
            '𛂤' => Ok(KanaSupplement::HentaiganaLetterHaDash7),
            '𛂥' => Ok(KanaSupplement::HentaiganaLetterHaDash8),
            '𛂦' => Ok(KanaSupplement::HentaiganaLetterHaDash9),
            '𛂧' => Ok(KanaSupplement::HentaiganaLetterHaDash10),
            '𛂨' => Ok(KanaSupplement::HentaiganaLetterHaDash11),
            '𛂩' => Ok(KanaSupplement::HentaiganaLetterHiDash1),
            '𛂪' => Ok(KanaSupplement::HentaiganaLetterHiDash2),
            '𛂫' => Ok(KanaSupplement::HentaiganaLetterHiDash3),
            '𛂬' => Ok(KanaSupplement::HentaiganaLetterHiDash4),
            '𛂭' => Ok(KanaSupplement::HentaiganaLetterHiDash5),
            '𛂮' => Ok(KanaSupplement::HentaiganaLetterHiDash6),
            '𛂯' => Ok(KanaSupplement::HentaiganaLetterHiDash7),
            '𛂰' => Ok(KanaSupplement::HentaiganaLetterHuDash1),
            '𛂱' => Ok(KanaSupplement::HentaiganaLetterHuDash2),
            '𛂲' => Ok(KanaSupplement::HentaiganaLetterHuDash3),
            '𛂳' => Ok(KanaSupplement::HentaiganaLetterHeDash1),
            '𛂴' => Ok(KanaSupplement::HentaiganaLetterHeDash2),
            '𛂵' => Ok(KanaSupplement::HentaiganaLetterHeDash3),
            '𛂶' => Ok(KanaSupplement::HentaiganaLetterHeDash4),
            '𛂷' => Ok(KanaSupplement::HentaiganaLetterHeDash5),
            '𛂸' => Ok(KanaSupplement::HentaiganaLetterHeDash6),
            '𛂹' => Ok(KanaSupplement::HentaiganaLetterHeDash7),
            '𛂺' => Ok(KanaSupplement::HentaiganaLetterHoDash1),
            '𛂻' => Ok(KanaSupplement::HentaiganaLetterHoDash2),
            '𛂼' => Ok(KanaSupplement::HentaiganaLetterHoDash3),
            '𛂽' => Ok(KanaSupplement::HentaiganaLetterHoDash4),
            '𛂾' => Ok(KanaSupplement::HentaiganaLetterHoDash5),
            '𛂿' => Ok(KanaSupplement::HentaiganaLetterHoDash6),
            '𛃀' => Ok(KanaSupplement::HentaiganaLetterHoDash7),
            '𛃁' => Ok(KanaSupplement::HentaiganaLetterHoDash8),
            '𛃂' => Ok(KanaSupplement::HentaiganaLetterMaDash1),
            '𛃃' => Ok(KanaSupplement::HentaiganaLetterMaDash2),
            '𛃄' => Ok(KanaSupplement::HentaiganaLetterMaDash3),
            '𛃅' => Ok(KanaSupplement::HentaiganaLetterMaDash4),
            '𛃆' => Ok(KanaSupplement::HentaiganaLetterMaDash5),
            '𛃇' => Ok(KanaSupplement::HentaiganaLetterMaDash6),
            '𛃈' => Ok(KanaSupplement::HentaiganaLetterMaDash7),
            '𛃉' => Ok(KanaSupplement::HentaiganaLetterMiDash1),
            '𛃊' => Ok(KanaSupplement::HentaiganaLetterMiDash2),
            '𛃋' => Ok(KanaSupplement::HentaiganaLetterMiDash3),
            '𛃌' => Ok(KanaSupplement::HentaiganaLetterMiDash4),
            '𛃍' => Ok(KanaSupplement::HentaiganaLetterMiDash5),
            '𛃎' => Ok(KanaSupplement::HentaiganaLetterMiDash6),
            '𛃏' => Ok(KanaSupplement::HentaiganaLetterMiDash7),
            '𛃐' => Ok(KanaSupplement::HentaiganaLetterMuDash1),
            '𛃑' => Ok(KanaSupplement::HentaiganaLetterMuDash2),
            '𛃒' => Ok(KanaSupplement::HentaiganaLetterMuDash3),
            '𛃓' => Ok(KanaSupplement::HentaiganaLetterMuDash4),
            '𛃔' => Ok(KanaSupplement::HentaiganaLetterMeDash1),
            '𛃕' => Ok(KanaSupplement::HentaiganaLetterMeDash2),
            '𛃖' => Ok(KanaSupplement::HentaiganaLetterMeDashMa),
            '𛃗' => Ok(KanaSupplement::HentaiganaLetterMoDash1),
            '𛃘' => Ok(KanaSupplement::HentaiganaLetterMoDash2),
            '𛃙' => Ok(KanaSupplement::HentaiganaLetterMoDash3),
            '𛃚' => Ok(KanaSupplement::HentaiganaLetterMoDash4),
            '𛃛' => Ok(KanaSupplement::HentaiganaLetterMoDash5),
            '𛃜' => Ok(KanaSupplement::HentaiganaLetterMoDash6),
            '𛃝' => Ok(KanaSupplement::HentaiganaLetterYaDash1),
            '𛃞' => Ok(KanaSupplement::HentaiganaLetterYaDash2),
            '𛃟' => Ok(KanaSupplement::HentaiganaLetterYaDash3),
            '𛃠' => Ok(KanaSupplement::HentaiganaLetterYaDash4),
            '𛃡' => Ok(KanaSupplement::HentaiganaLetterYaDash5),
            '𛃢' => Ok(KanaSupplement::HentaiganaLetterYaDashYo),
            '𛃣' => Ok(KanaSupplement::HentaiganaLetterYuDash1),
            '𛃤' => Ok(KanaSupplement::HentaiganaLetterYuDash2),
            '𛃥' => Ok(KanaSupplement::HentaiganaLetterYuDash3),
            '𛃦' => Ok(KanaSupplement::HentaiganaLetterYuDash4),
            '𛃧' => Ok(KanaSupplement::HentaiganaLetterYoDash1),
            '𛃨' => Ok(KanaSupplement::HentaiganaLetterYoDash2),
            '𛃩' => Ok(KanaSupplement::HentaiganaLetterYoDash3),
            '𛃪' => Ok(KanaSupplement::HentaiganaLetterYoDash4),
            '𛃫' => Ok(KanaSupplement::HentaiganaLetterYoDash5),
            '𛃬' => Ok(KanaSupplement::HentaiganaLetterYoDash6),
            '𛃭' => Ok(KanaSupplement::HentaiganaLetterRaDash1),
            '𛃮' => Ok(KanaSupplement::HentaiganaLetterRaDash2),
            '𛃯' => Ok(KanaSupplement::HentaiganaLetterRaDash3),
            '𛃰' => Ok(KanaSupplement::HentaiganaLetterRaDash4),
            '𛃱' => Ok(KanaSupplement::HentaiganaLetterRiDash1),
            '𛃲' => Ok(KanaSupplement::HentaiganaLetterRiDash2),
            '𛃳' => Ok(KanaSupplement::HentaiganaLetterRiDash3),
            '𛃴' => Ok(KanaSupplement::HentaiganaLetterRiDash4),
            '𛃵' => Ok(KanaSupplement::HentaiganaLetterRiDash5),
            '𛃶' => Ok(KanaSupplement::HentaiganaLetterRiDash6),
            '𛃷' => Ok(KanaSupplement::HentaiganaLetterRiDash7),
            '𛃸' => Ok(KanaSupplement::HentaiganaLetterRuDash1),
            '𛃹' => Ok(KanaSupplement::HentaiganaLetterRuDash2),
            '𛃺' => Ok(KanaSupplement::HentaiganaLetterRuDash3),
            '𛃻' => Ok(KanaSupplement::HentaiganaLetterRuDash4),
            '𛃼' => Ok(KanaSupplement::HentaiganaLetterRuDash5),
            '𛃽' => Ok(KanaSupplement::HentaiganaLetterRuDash6),
            '𛃾' => Ok(KanaSupplement::HentaiganaLetterReDash1),
            _ => Err(()),
        }
    }
}

impl Into<u32> for KanaSupplement {
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

impl std::convert::TryFrom<u32> for KanaSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for KanaSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl KanaSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        KanaSupplement::KatakanaLetterArchaicE
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("KanaSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
