
/// An enum to represent all characters in the YiSyllables block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum YiSyllables {
    /// \u{a000}: 'ꀀ'
    YiSyllableIt,
    /// \u{a001}: 'ꀁ'
    YiSyllableIx,
    /// \u{a002}: 'ꀂ'
    YiSyllableI,
    /// \u{a003}: 'ꀃ'
    YiSyllableIp,
    /// \u{a004}: 'ꀄ'
    YiSyllableIet,
    /// \u{a005}: 'ꀅ'
    YiSyllableIex,
    /// \u{a006}: 'ꀆ'
    YiSyllableIe,
    /// \u{a007}: 'ꀇ'
    YiSyllableIep,
    /// \u{a008}: 'ꀈ'
    YiSyllableAt,
    /// \u{a009}: 'ꀉ'
    YiSyllableAx,
    /// \u{a00a}: 'ꀊ'
    YiSyllableA,
    /// \u{a00b}: 'ꀋ'
    YiSyllableAp,
    /// \u{a00c}: 'ꀌ'
    YiSyllableUox,
    /// \u{a00d}: 'ꀍ'
    YiSyllableUo,
    /// \u{a00e}: 'ꀎ'
    YiSyllableUop,
    /// \u{a00f}: 'ꀏ'
    YiSyllableOt,
    /// \u{a010}: 'ꀐ'
    YiSyllableOx,
    /// \u{a011}: 'ꀑ'
    YiSyllableO,
    /// \u{a012}: 'ꀒ'
    YiSyllableOp,
    /// \u{a013}: 'ꀓ'
    YiSyllableEx,
    /// \u{a014}: 'ꀔ'
    YiSyllableE,
    /// \u{a015}: 'ꀕ'
    YiSyllableWu,
    /// \u{a016}: 'ꀖ'
    YiSyllableBit,
    /// \u{a017}: 'ꀗ'
    YiSyllableBix,
    /// \u{a018}: 'ꀘ'
    YiSyllableBi,
    /// \u{a019}: 'ꀙ'
    YiSyllableBip,
    /// \u{a01a}: 'ꀚ'
    YiSyllableBiet,
    /// \u{a01b}: 'ꀛ'
    YiSyllableBiex,
    /// \u{a01c}: 'ꀜ'
    YiSyllableBie,
    /// \u{a01d}: 'ꀝ'
    YiSyllableBiep,
    /// \u{a01e}: 'ꀞ'
    YiSyllableBat,
    /// \u{a01f}: 'ꀟ'
    YiSyllableBax,
    /// \u{a020}: 'ꀠ'
    YiSyllableBa,
    /// \u{a021}: 'ꀡ'
    YiSyllableBap,
    /// \u{a022}: 'ꀢ'
    YiSyllableBuox,
    /// \u{a023}: 'ꀣ'
    YiSyllableBuo,
    /// \u{a024}: 'ꀤ'
    YiSyllableBuop,
    /// \u{a025}: 'ꀥ'
    YiSyllableBot,
    /// \u{a026}: 'ꀦ'
    YiSyllableBox,
    /// \u{a027}: 'ꀧ'
    YiSyllableBo,
    /// \u{a028}: 'ꀨ'
    YiSyllableBop,
    /// \u{a029}: 'ꀩ'
    YiSyllableBex,
    /// \u{a02a}: 'ꀪ'
    YiSyllableBe,
    /// \u{a02b}: 'ꀫ'
    YiSyllableBep,
    /// \u{a02c}: 'ꀬ'
    YiSyllableBut,
    /// \u{a02d}: 'ꀭ'
    YiSyllableBux,
    /// \u{a02e}: 'ꀮ'
    YiSyllableBu,
    /// \u{a02f}: 'ꀯ'
    YiSyllableBup,
    /// \u{a030}: 'ꀰ'
    YiSyllableBurx,
    /// \u{a031}: 'ꀱ'
    YiSyllableBur,
    /// \u{a032}: 'ꀲ'
    YiSyllableByt,
    /// \u{a033}: 'ꀳ'
    YiSyllableByx,
    /// \u{a034}: 'ꀴ'
    YiSyllableBy,
    /// \u{a035}: 'ꀵ'
    YiSyllableByp,
    /// \u{a036}: 'ꀶ'
    YiSyllableByrx,
    /// \u{a037}: 'ꀷ'
    YiSyllableByr,
    /// \u{a038}: 'ꀸ'
    YiSyllablePit,
    /// \u{a039}: 'ꀹ'
    YiSyllablePix,
    /// \u{a03a}: 'ꀺ'
    YiSyllablePi,
    /// \u{a03b}: 'ꀻ'
    YiSyllablePip,
    /// \u{a03c}: 'ꀼ'
    YiSyllablePiex,
    /// \u{a03d}: 'ꀽ'
    YiSyllablePie,
    /// \u{a03e}: 'ꀾ'
    YiSyllablePiep,
    /// \u{a03f}: 'ꀿ'
    YiSyllablePat,
    /// \u{a040}: 'ꁀ'
    YiSyllablePax,
    /// \u{a041}: 'ꁁ'
    YiSyllablePa,
    /// \u{a042}: 'ꁂ'
    YiSyllablePap,
    /// \u{a043}: 'ꁃ'
    YiSyllablePuox,
    /// \u{a044}: 'ꁄ'
    YiSyllablePuo,
    /// \u{a045}: 'ꁅ'
    YiSyllablePuop,
    /// \u{a046}: 'ꁆ'
    YiSyllablePot,
    /// \u{a047}: 'ꁇ'
    YiSyllablePox,
    /// \u{a048}: 'ꁈ'
    YiSyllablePo,
    /// \u{a049}: 'ꁉ'
    YiSyllablePop,
    /// \u{a04a}: 'ꁊ'
    YiSyllablePut,
    /// \u{a04b}: 'ꁋ'
    YiSyllablePux,
    /// \u{a04c}: 'ꁌ'
    YiSyllablePu,
    /// \u{a04d}: 'ꁍ'
    YiSyllablePup,
    /// \u{a04e}: 'ꁎ'
    YiSyllablePurx,
    /// \u{a04f}: 'ꁏ'
    YiSyllablePur,
    /// \u{a050}: 'ꁐ'
    YiSyllablePyt,
    /// \u{a051}: 'ꁑ'
    YiSyllablePyx,
    /// \u{a052}: 'ꁒ'
    YiSyllablePy,
    /// \u{a053}: 'ꁓ'
    YiSyllablePyp,
    /// \u{a054}: 'ꁔ'
    YiSyllablePyrx,
    /// \u{a055}: 'ꁕ'
    YiSyllablePyr,
    /// \u{a056}: 'ꁖ'
    YiSyllableBbit,
    /// \u{a057}: 'ꁗ'
    YiSyllableBbix,
    /// \u{a058}: 'ꁘ'
    YiSyllableBbi,
    /// \u{a059}: 'ꁙ'
    YiSyllableBbip,
    /// \u{a05a}: 'ꁚ'
    YiSyllableBbiet,
    /// \u{a05b}: 'ꁛ'
    YiSyllableBbiex,
    /// \u{a05c}: 'ꁜ'
    YiSyllableBbie,
    /// \u{a05d}: 'ꁝ'
    YiSyllableBbiep,
    /// \u{a05e}: 'ꁞ'
    YiSyllableBbat,
    /// \u{a05f}: 'ꁟ'
    YiSyllableBbax,
    /// \u{a060}: 'ꁠ'
    YiSyllableBba,
    /// \u{a061}: 'ꁡ'
    YiSyllableBbap,
    /// \u{a062}: 'ꁢ'
    YiSyllableBbuox,
    /// \u{a063}: 'ꁣ'
    YiSyllableBbuo,
    /// \u{a064}: 'ꁤ'
    YiSyllableBbuop,
    /// \u{a065}: 'ꁥ'
    YiSyllableBbot,
    /// \u{a066}: 'ꁦ'
    YiSyllableBbox,
    /// \u{a067}: 'ꁧ'
    YiSyllableBbo,
    /// \u{a068}: 'ꁨ'
    YiSyllableBbop,
    /// \u{a069}: 'ꁩ'
    YiSyllableBbex,
    /// \u{a06a}: 'ꁪ'
    YiSyllableBbe,
    /// \u{a06b}: 'ꁫ'
    YiSyllableBbep,
    /// \u{a06c}: 'ꁬ'
    YiSyllableBbut,
    /// \u{a06d}: 'ꁭ'
    YiSyllableBbux,
    /// \u{a06e}: 'ꁮ'
    YiSyllableBbu,
    /// \u{a06f}: 'ꁯ'
    YiSyllableBbup,
    /// \u{a070}: 'ꁰ'
    YiSyllableBburx,
    /// \u{a071}: 'ꁱ'
    YiSyllableBbur,
    /// \u{a072}: 'ꁲ'
    YiSyllableBbyt,
    /// \u{a073}: 'ꁳ'
    YiSyllableBbyx,
    /// \u{a074}: 'ꁴ'
    YiSyllableBby,
    /// \u{a075}: 'ꁵ'
    YiSyllableBbyp,
    /// \u{a076}: 'ꁶ'
    YiSyllableNbit,
    /// \u{a077}: 'ꁷ'
    YiSyllableNbix,
    /// \u{a078}: 'ꁸ'
    YiSyllableNbi,
    /// \u{a079}: 'ꁹ'
    YiSyllableNbip,
    /// \u{a07a}: 'ꁺ'
    YiSyllableNbiex,
    /// \u{a07b}: 'ꁻ'
    YiSyllableNbie,
    /// \u{a07c}: 'ꁼ'
    YiSyllableNbiep,
    /// \u{a07d}: 'ꁽ'
    YiSyllableNbat,
    /// \u{a07e}: 'ꁾ'
    YiSyllableNbax,
    /// \u{a07f}: 'ꁿ'
    YiSyllableNba,
    /// \u{a080}: 'ꂀ'
    YiSyllableNbap,
    /// \u{a081}: 'ꂁ'
    YiSyllableNbot,
    /// \u{a082}: 'ꂂ'
    YiSyllableNbox,
    /// \u{a083}: 'ꂃ'
    YiSyllableNbo,
    /// \u{a084}: 'ꂄ'
    YiSyllableNbop,
    /// \u{a085}: 'ꂅ'
    YiSyllableNbut,
    /// \u{a086}: 'ꂆ'
    YiSyllableNbux,
    /// \u{a087}: 'ꂇ'
    YiSyllableNbu,
    /// \u{a088}: 'ꂈ'
    YiSyllableNbup,
    /// \u{a089}: 'ꂉ'
    YiSyllableNburx,
    /// \u{a08a}: 'ꂊ'
    YiSyllableNbur,
    /// \u{a08b}: 'ꂋ'
    YiSyllableNbyt,
    /// \u{a08c}: 'ꂌ'
    YiSyllableNbyx,
    /// \u{a08d}: 'ꂍ'
    YiSyllableNby,
    /// \u{a08e}: 'ꂎ'
    YiSyllableNbyp,
    /// \u{a08f}: 'ꂏ'
    YiSyllableNbyrx,
    /// \u{a090}: 'ꂐ'
    YiSyllableNbyr,
    /// \u{a091}: 'ꂑ'
    YiSyllableHmit,
    /// \u{a092}: 'ꂒ'
    YiSyllableHmix,
    /// \u{a093}: 'ꂓ'
    YiSyllableHmi,
    /// \u{a094}: 'ꂔ'
    YiSyllableHmip,
    /// \u{a095}: 'ꂕ'
    YiSyllableHmiex,
    /// \u{a096}: 'ꂖ'
    YiSyllableHmie,
    /// \u{a097}: 'ꂗ'
    YiSyllableHmiep,
    /// \u{a098}: 'ꂘ'
    YiSyllableHmat,
    /// \u{a099}: 'ꂙ'
    YiSyllableHmax,
    /// \u{a09a}: 'ꂚ'
    YiSyllableHma,
    /// \u{a09b}: 'ꂛ'
    YiSyllableHmap,
    /// \u{a09c}: 'ꂜ'
    YiSyllableHmuox,
    /// \u{a09d}: 'ꂝ'
    YiSyllableHmuo,
    /// \u{a09e}: 'ꂞ'
    YiSyllableHmuop,
    /// \u{a09f}: 'ꂟ'
    YiSyllableHmot,
    /// \u{a0a0}: 'ꂠ'
    YiSyllableHmox,
    /// \u{a0a1}: 'ꂡ'
    YiSyllableHmo,
    /// \u{a0a2}: 'ꂢ'
    YiSyllableHmop,
    /// \u{a0a3}: 'ꂣ'
    YiSyllableHmut,
    /// \u{a0a4}: 'ꂤ'
    YiSyllableHmux,
    /// \u{a0a5}: 'ꂥ'
    YiSyllableHmu,
    /// \u{a0a6}: 'ꂦ'
    YiSyllableHmup,
    /// \u{a0a7}: 'ꂧ'
    YiSyllableHmurx,
    /// \u{a0a8}: 'ꂨ'
    YiSyllableHmur,
    /// \u{a0a9}: 'ꂩ'
    YiSyllableHmyx,
    /// \u{a0aa}: 'ꂪ'
    YiSyllableHmy,
    /// \u{a0ab}: 'ꂫ'
    YiSyllableHmyp,
    /// \u{a0ac}: 'ꂬ'
    YiSyllableHmyrx,
    /// \u{a0ad}: 'ꂭ'
    YiSyllableHmyr,
    /// \u{a0ae}: 'ꂮ'
    YiSyllableMit,
    /// \u{a0af}: 'ꂯ'
    YiSyllableMix,
    /// \u{a0b0}: 'ꂰ'
    YiSyllableMi,
    /// \u{a0b1}: 'ꂱ'
    YiSyllableMip,
    /// \u{a0b2}: 'ꂲ'
    YiSyllableMiex,
    /// \u{a0b3}: 'ꂳ'
    YiSyllableMie,
    /// \u{a0b4}: 'ꂴ'
    YiSyllableMiep,
    /// \u{a0b5}: 'ꂵ'
    YiSyllableMat,
    /// \u{a0b6}: 'ꂶ'
    YiSyllableMax,
    /// \u{a0b7}: 'ꂷ'
    YiSyllableMa,
    /// \u{a0b8}: 'ꂸ'
    YiSyllableMap,
    /// \u{a0b9}: 'ꂹ'
    YiSyllableMuot,
    /// \u{a0ba}: 'ꂺ'
    YiSyllableMuox,
    /// \u{a0bb}: 'ꂻ'
    YiSyllableMuo,
    /// \u{a0bc}: 'ꂼ'
    YiSyllableMuop,
    /// \u{a0bd}: 'ꂽ'
    YiSyllableMot,
    /// \u{a0be}: 'ꂾ'
    YiSyllableMox,
    /// \u{a0bf}: 'ꂿ'
    YiSyllableMo,
    /// \u{a0c0}: 'ꃀ'
    YiSyllableMop,
    /// \u{a0c1}: 'ꃁ'
    YiSyllableMex,
    /// \u{a0c2}: 'ꃂ'
    YiSyllableMe,
    /// \u{a0c3}: 'ꃃ'
    YiSyllableMut,
    /// \u{a0c4}: 'ꃄ'
    YiSyllableMux,
    /// \u{a0c5}: 'ꃅ'
    YiSyllableMu,
    /// \u{a0c6}: 'ꃆ'
    YiSyllableMup,
    /// \u{a0c7}: 'ꃇ'
    YiSyllableMurx,
    /// \u{a0c8}: 'ꃈ'
    YiSyllableMur,
    /// \u{a0c9}: 'ꃉ'
    YiSyllableMyt,
    /// \u{a0ca}: 'ꃊ'
    YiSyllableMyx,
    /// \u{a0cb}: 'ꃋ'
    YiSyllableMy,
    /// \u{a0cc}: 'ꃌ'
    YiSyllableMyp,
    /// \u{a0cd}: 'ꃍ'
    YiSyllableFit,
    /// \u{a0ce}: 'ꃎ'
    YiSyllableFix,
    /// \u{a0cf}: 'ꃏ'
    YiSyllableFi,
    /// \u{a0d0}: 'ꃐ'
    YiSyllableFip,
    /// \u{a0d1}: 'ꃑ'
    YiSyllableFat,
    /// \u{a0d2}: 'ꃒ'
    YiSyllableFax,
    /// \u{a0d3}: 'ꃓ'
    YiSyllableFa,
    /// \u{a0d4}: 'ꃔ'
    YiSyllableFap,
    /// \u{a0d5}: 'ꃕ'
    YiSyllableFox,
    /// \u{a0d6}: 'ꃖ'
    YiSyllableFo,
    /// \u{a0d7}: 'ꃗ'
    YiSyllableFop,
    /// \u{a0d8}: 'ꃘ'
    YiSyllableFut,
    /// \u{a0d9}: 'ꃙ'
    YiSyllableFux,
    /// \u{a0da}: 'ꃚ'
    YiSyllableFu,
    /// \u{a0db}: 'ꃛ'
    YiSyllableFup,
    /// \u{a0dc}: 'ꃜ'
    YiSyllableFurx,
    /// \u{a0dd}: 'ꃝ'
    YiSyllableFur,
    /// \u{a0de}: 'ꃞ'
    YiSyllableFyt,
    /// \u{a0df}: 'ꃟ'
    YiSyllableFyx,
    /// \u{a0e0}: 'ꃠ'
    YiSyllableFy,
    /// \u{a0e1}: 'ꃡ'
    YiSyllableFyp,
    /// \u{a0e2}: 'ꃢ'
    YiSyllableVit,
    /// \u{a0e3}: 'ꃣ'
    YiSyllableVix,
    /// \u{a0e4}: 'ꃤ'
    YiSyllableVi,
    /// \u{a0e5}: 'ꃥ'
    YiSyllableVip,
    /// \u{a0e6}: 'ꃦ'
    YiSyllableViet,
    /// \u{a0e7}: 'ꃧ'
    YiSyllableViex,
    /// \u{a0e8}: 'ꃨ'
    YiSyllableVie,
    /// \u{a0e9}: 'ꃩ'
    YiSyllableViep,
    /// \u{a0ea}: 'ꃪ'
    YiSyllableVat,
    /// \u{a0eb}: 'ꃫ'
    YiSyllableVax,
    /// \u{a0ec}: 'ꃬ'
    YiSyllableVa,
    /// \u{a0ed}: 'ꃭ'
    YiSyllableVap,
    /// \u{a0ee}: 'ꃮ'
    YiSyllableVot,
    /// \u{a0ef}: 'ꃯ'
    YiSyllableVox,
    /// \u{a0f0}: 'ꃰ'
    YiSyllableVo,
    /// \u{a0f1}: 'ꃱ'
    YiSyllableVop,
    /// \u{a0f2}: 'ꃲ'
    YiSyllableVex,
    /// \u{a0f3}: 'ꃳ'
    YiSyllableVep,
    /// \u{a0f4}: 'ꃴ'
    YiSyllableVut,
    /// \u{a0f5}: 'ꃵ'
    YiSyllableVux,
    /// \u{a0f6}: 'ꃶ'
    YiSyllableVu,
    /// \u{a0f7}: 'ꃷ'
    YiSyllableVup,
    /// \u{a0f8}: 'ꃸ'
    YiSyllableVurx,
    /// \u{a0f9}: 'ꃹ'
    YiSyllableVur,
    /// \u{a0fa}: 'ꃺ'
    YiSyllableVyt,
    /// \u{a0fb}: 'ꃻ'
    YiSyllableVyx,
    /// \u{a0fc}: 'ꃼ'
    YiSyllableVy,
    /// \u{a0fd}: 'ꃽ'
    YiSyllableVyp,
    /// \u{a0fe}: 'ꃾ'
    YiSyllableVyrx,
    /// \u{a0ff}: 'ꃿ'
    YiSyllableVyr,
    /// \u{a100}: 'ꄀ'
    YiSyllableDit,
    /// \u{a101}: 'ꄁ'
    YiSyllableDix,
    /// \u{a102}: 'ꄂ'
    YiSyllableDi,
    /// \u{a103}: 'ꄃ'
    YiSyllableDip,
    /// \u{a104}: 'ꄄ'
    YiSyllableDiex,
    /// \u{a105}: 'ꄅ'
    YiSyllableDie,
    /// \u{a106}: 'ꄆ'
    YiSyllableDiep,
    /// \u{a107}: 'ꄇ'
    YiSyllableDat,
    /// \u{a108}: 'ꄈ'
    YiSyllableDax,
    /// \u{a109}: 'ꄉ'
    YiSyllableDa,
    /// \u{a10a}: 'ꄊ'
    YiSyllableDap,
    /// \u{a10b}: 'ꄋ'
    YiSyllableDuox,
    /// \u{a10c}: 'ꄌ'
    YiSyllableDuo,
    /// \u{a10d}: 'ꄍ'
    YiSyllableDot,
    /// \u{a10e}: 'ꄎ'
    YiSyllableDox,
    /// \u{a10f}: 'ꄏ'
    YiSyllableDo,
    /// \u{a110}: 'ꄐ'
    YiSyllableDop,
    /// \u{a111}: 'ꄑ'
    YiSyllableDex,
    /// \u{a112}: 'ꄒ'
    YiSyllableDe,
    /// \u{a113}: 'ꄓ'
    YiSyllableDep,
    /// \u{a114}: 'ꄔ'
    YiSyllableDut,
    /// \u{a115}: 'ꄕ'
    YiSyllableDux,
    /// \u{a116}: 'ꄖ'
    YiSyllableDu,
    /// \u{a117}: 'ꄗ'
    YiSyllableDup,
    /// \u{a118}: 'ꄘ'
    YiSyllableDurx,
    /// \u{a119}: 'ꄙ'
    YiSyllableDur,
    /// \u{a11a}: 'ꄚ'
    YiSyllableTit,
    /// \u{a11b}: 'ꄛ'
    YiSyllableTix,
    /// \u{a11c}: 'ꄜ'
    YiSyllableTi,
    /// \u{a11d}: 'ꄝ'
    YiSyllableTip,
    /// \u{a11e}: 'ꄞ'
    YiSyllableTiex,
    /// \u{a11f}: 'ꄟ'
    YiSyllableTie,
    /// \u{a120}: 'ꄠ'
    YiSyllableTiep,
    /// \u{a121}: 'ꄡ'
    YiSyllableTat,
    /// \u{a122}: 'ꄢ'
    YiSyllableTax,
    /// \u{a123}: 'ꄣ'
    YiSyllableTa,
    /// \u{a124}: 'ꄤ'
    YiSyllableTap,
    /// \u{a125}: 'ꄥ'
    YiSyllableTuot,
    /// \u{a126}: 'ꄦ'
    YiSyllableTuox,
    /// \u{a127}: 'ꄧ'
    YiSyllableTuo,
    /// \u{a128}: 'ꄨ'
    YiSyllableTuop,
    /// \u{a129}: 'ꄩ'
    YiSyllableTot,
    /// \u{a12a}: 'ꄪ'
    YiSyllableTox,
    /// \u{a12b}: 'ꄫ'
    YiSyllableTo,
    /// \u{a12c}: 'ꄬ'
    YiSyllableTop,
    /// \u{a12d}: 'ꄭ'
    YiSyllableTex,
    /// \u{a12e}: 'ꄮ'
    YiSyllableTe,
    /// \u{a12f}: 'ꄯ'
    YiSyllableTep,
    /// \u{a130}: 'ꄰ'
    YiSyllableTut,
    /// \u{a131}: 'ꄱ'
    YiSyllableTux,
    /// \u{a132}: 'ꄲ'
    YiSyllableTu,
    /// \u{a133}: 'ꄳ'
    YiSyllableTup,
    /// \u{a134}: 'ꄴ'
    YiSyllableTurx,
    /// \u{a135}: 'ꄵ'
    YiSyllableTur,
    /// \u{a136}: 'ꄶ'
    YiSyllableDdit,
    /// \u{a137}: 'ꄷ'
    YiSyllableDdix,
    /// \u{a138}: 'ꄸ'
    YiSyllableDdi,
    /// \u{a139}: 'ꄹ'
    YiSyllableDdip,
    /// \u{a13a}: 'ꄺ'
    YiSyllableDdiex,
    /// \u{a13b}: 'ꄻ'
    YiSyllableDdie,
    /// \u{a13c}: 'ꄼ'
    YiSyllableDdiep,
    /// \u{a13d}: 'ꄽ'
    YiSyllableDdat,
    /// \u{a13e}: 'ꄾ'
    YiSyllableDdax,
    /// \u{a13f}: 'ꄿ'
    YiSyllableDda,
    /// \u{a140}: 'ꅀ'
    YiSyllableDdap,
    /// \u{a141}: 'ꅁ'
    YiSyllableDduox,
    /// \u{a142}: 'ꅂ'
    YiSyllableDduo,
    /// \u{a143}: 'ꅃ'
    YiSyllableDduop,
    /// \u{a144}: 'ꅄ'
    YiSyllableDdot,
    /// \u{a145}: 'ꅅ'
    YiSyllableDdox,
    /// \u{a146}: 'ꅆ'
    YiSyllableDdo,
    /// \u{a147}: 'ꅇ'
    YiSyllableDdop,
    /// \u{a148}: 'ꅈ'
    YiSyllableDdex,
    /// \u{a149}: 'ꅉ'
    YiSyllableDde,
    /// \u{a14a}: 'ꅊ'
    YiSyllableDdep,
    /// \u{a14b}: 'ꅋ'
    YiSyllableDdut,
    /// \u{a14c}: 'ꅌ'
    YiSyllableDdux,
    /// \u{a14d}: 'ꅍ'
    YiSyllableDdu,
    /// \u{a14e}: 'ꅎ'
    YiSyllableDdup,
    /// \u{a14f}: 'ꅏ'
    YiSyllableDdurx,
    /// \u{a150}: 'ꅐ'
    YiSyllableDdur,
    /// \u{a151}: 'ꅑ'
    YiSyllableNdit,
    /// \u{a152}: 'ꅒ'
    YiSyllableNdix,
    /// \u{a153}: 'ꅓ'
    YiSyllableNdi,
    /// \u{a154}: 'ꅔ'
    YiSyllableNdip,
    /// \u{a155}: 'ꅕ'
    YiSyllableNdiex,
    /// \u{a156}: 'ꅖ'
    YiSyllableNdie,
    /// \u{a157}: 'ꅗ'
    YiSyllableNdat,
    /// \u{a158}: 'ꅘ'
    YiSyllableNdax,
    /// \u{a159}: 'ꅙ'
    YiSyllableNda,
    /// \u{a15a}: 'ꅚ'
    YiSyllableNdap,
    /// \u{a15b}: 'ꅛ'
    YiSyllableNdot,
    /// \u{a15c}: 'ꅜ'
    YiSyllableNdox,
    /// \u{a15d}: 'ꅝ'
    YiSyllableNdo,
    /// \u{a15e}: 'ꅞ'
    YiSyllableNdop,
    /// \u{a15f}: 'ꅟ'
    YiSyllableNdex,
    /// \u{a160}: 'ꅠ'
    YiSyllableNde,
    /// \u{a161}: 'ꅡ'
    YiSyllableNdep,
    /// \u{a162}: 'ꅢ'
    YiSyllableNdut,
    /// \u{a163}: 'ꅣ'
    YiSyllableNdux,
    /// \u{a164}: 'ꅤ'
    YiSyllableNdu,
    /// \u{a165}: 'ꅥ'
    YiSyllableNdup,
    /// \u{a166}: 'ꅦ'
    YiSyllableNdurx,
    /// \u{a167}: 'ꅧ'
    YiSyllableNdur,
    /// \u{a168}: 'ꅨ'
    YiSyllableHnit,
    /// \u{a169}: 'ꅩ'
    YiSyllableHnix,
    /// \u{a16a}: 'ꅪ'
    YiSyllableHni,
    /// \u{a16b}: 'ꅫ'
    YiSyllableHnip,
    /// \u{a16c}: 'ꅬ'
    YiSyllableHniet,
    /// \u{a16d}: 'ꅭ'
    YiSyllableHniex,
    /// \u{a16e}: 'ꅮ'
    YiSyllableHnie,
    /// \u{a16f}: 'ꅯ'
    YiSyllableHniep,
    /// \u{a170}: 'ꅰ'
    YiSyllableHnat,
    /// \u{a171}: 'ꅱ'
    YiSyllableHnax,
    /// \u{a172}: 'ꅲ'
    YiSyllableHna,
    /// \u{a173}: 'ꅳ'
    YiSyllableHnap,
    /// \u{a174}: 'ꅴ'
    YiSyllableHnuox,
    /// \u{a175}: 'ꅵ'
    YiSyllableHnuo,
    /// \u{a176}: 'ꅶ'
    YiSyllableHnot,
    /// \u{a177}: 'ꅷ'
    YiSyllableHnox,
    /// \u{a178}: 'ꅸ'
    YiSyllableHnop,
    /// \u{a179}: 'ꅹ'
    YiSyllableHnex,
    /// \u{a17a}: 'ꅺ'
    YiSyllableHne,
    /// \u{a17b}: 'ꅻ'
    YiSyllableHnep,
    /// \u{a17c}: 'ꅼ'
    YiSyllableHnut,
    /// \u{a17d}: 'ꅽ'
    YiSyllableNit,
    /// \u{a17e}: 'ꅾ'
    YiSyllableNix,
    /// \u{a17f}: 'ꅿ'
    YiSyllableNi,
    /// \u{a180}: 'ꆀ'
    YiSyllableNip,
    /// \u{a181}: 'ꆁ'
    YiSyllableNiex,
    /// \u{a182}: 'ꆂ'
    YiSyllableNie,
    /// \u{a183}: 'ꆃ'
    YiSyllableNiep,
    /// \u{a184}: 'ꆄ'
    YiSyllableNax,
    /// \u{a185}: 'ꆅ'
    YiSyllableNa,
    /// \u{a186}: 'ꆆ'
    YiSyllableNap,
    /// \u{a187}: 'ꆇ'
    YiSyllableNuox,
    /// \u{a188}: 'ꆈ'
    YiSyllableNuo,
    /// \u{a189}: 'ꆉ'
    YiSyllableNuop,
    /// \u{a18a}: 'ꆊ'
    YiSyllableNot,
    /// \u{a18b}: 'ꆋ'
    YiSyllableNox,
    /// \u{a18c}: 'ꆌ'
    YiSyllableNo,
    /// \u{a18d}: 'ꆍ'
    YiSyllableNop,
    /// \u{a18e}: 'ꆎ'
    YiSyllableNex,
    /// \u{a18f}: 'ꆏ'
    YiSyllableNe,
    /// \u{a190}: 'ꆐ'
    YiSyllableNep,
    /// \u{a191}: 'ꆑ'
    YiSyllableNut,
    /// \u{a192}: 'ꆒ'
    YiSyllableNux,
    /// \u{a193}: 'ꆓ'
    YiSyllableNu,
    /// \u{a194}: 'ꆔ'
    YiSyllableNup,
    /// \u{a195}: 'ꆕ'
    YiSyllableNurx,
    /// \u{a196}: 'ꆖ'
    YiSyllableNur,
    /// \u{a197}: 'ꆗ'
    YiSyllableHlit,
    /// \u{a198}: 'ꆘ'
    YiSyllableHlix,
    /// \u{a199}: 'ꆙ'
    YiSyllableHli,
    /// \u{a19a}: 'ꆚ'
    YiSyllableHlip,
    /// \u{a19b}: 'ꆛ'
    YiSyllableHliex,
    /// \u{a19c}: 'ꆜ'
    YiSyllableHlie,
    /// \u{a19d}: 'ꆝ'
    YiSyllableHliep,
    /// \u{a19e}: 'ꆞ'
    YiSyllableHlat,
    /// \u{a19f}: 'ꆟ'
    YiSyllableHlax,
    /// \u{a1a0}: 'ꆠ'
    YiSyllableHla,
    /// \u{a1a1}: 'ꆡ'
    YiSyllableHlap,
    /// \u{a1a2}: 'ꆢ'
    YiSyllableHluox,
    /// \u{a1a3}: 'ꆣ'
    YiSyllableHluo,
    /// \u{a1a4}: 'ꆤ'
    YiSyllableHluop,
    /// \u{a1a5}: 'ꆥ'
    YiSyllableHlox,
    /// \u{a1a6}: 'ꆦ'
    YiSyllableHlo,
    /// \u{a1a7}: 'ꆧ'
    YiSyllableHlop,
    /// \u{a1a8}: 'ꆨ'
    YiSyllableHlex,
    /// \u{a1a9}: 'ꆩ'
    YiSyllableHle,
    /// \u{a1aa}: 'ꆪ'
    YiSyllableHlep,
    /// \u{a1ab}: 'ꆫ'
    YiSyllableHlut,
    /// \u{a1ac}: 'ꆬ'
    YiSyllableHlux,
    /// \u{a1ad}: 'ꆭ'
    YiSyllableHlu,
    /// \u{a1ae}: 'ꆮ'
    YiSyllableHlup,
    /// \u{a1af}: 'ꆯ'
    YiSyllableHlurx,
    /// \u{a1b0}: 'ꆰ'
    YiSyllableHlur,
    /// \u{a1b1}: 'ꆱ'
    YiSyllableHlyt,
    /// \u{a1b2}: 'ꆲ'
    YiSyllableHlyx,
    /// \u{a1b3}: 'ꆳ'
    YiSyllableHly,
    /// \u{a1b4}: 'ꆴ'
    YiSyllableHlyp,
    /// \u{a1b5}: 'ꆵ'
    YiSyllableHlyrx,
    /// \u{a1b6}: 'ꆶ'
    YiSyllableHlyr,
    /// \u{a1b7}: 'ꆷ'
    YiSyllableLit,
    /// \u{a1b8}: 'ꆸ'
    YiSyllableLix,
    /// \u{a1b9}: 'ꆹ'
    YiSyllableLi,
    /// \u{a1ba}: 'ꆺ'
    YiSyllableLip,
    /// \u{a1bb}: 'ꆻ'
    YiSyllableLiet,
    /// \u{a1bc}: 'ꆼ'
    YiSyllableLiex,
    /// \u{a1bd}: 'ꆽ'
    YiSyllableLie,
    /// \u{a1be}: 'ꆾ'
    YiSyllableLiep,
    /// \u{a1bf}: 'ꆿ'
    YiSyllableLat,
    /// \u{a1c0}: 'ꇀ'
    YiSyllableLax,
    /// \u{a1c1}: 'ꇁ'
    YiSyllableLa,
    /// \u{a1c2}: 'ꇂ'
    YiSyllableLap,
    /// \u{a1c3}: 'ꇃ'
    YiSyllableLuot,
    /// \u{a1c4}: 'ꇄ'
    YiSyllableLuox,
    /// \u{a1c5}: 'ꇅ'
    YiSyllableLuo,
    /// \u{a1c6}: 'ꇆ'
    YiSyllableLuop,
    /// \u{a1c7}: 'ꇇ'
    YiSyllableLot,
    /// \u{a1c8}: 'ꇈ'
    YiSyllableLox,
    /// \u{a1c9}: 'ꇉ'
    YiSyllableLo,
    /// \u{a1ca}: 'ꇊ'
    YiSyllableLop,
    /// \u{a1cb}: 'ꇋ'
    YiSyllableLex,
    /// \u{a1cc}: 'ꇌ'
    YiSyllableLe,
    /// \u{a1cd}: 'ꇍ'
    YiSyllableLep,
    /// \u{a1ce}: 'ꇎ'
    YiSyllableLut,
    /// \u{a1cf}: 'ꇏ'
    YiSyllableLux,
    /// \u{a1d0}: 'ꇐ'
    YiSyllableLu,
    /// \u{a1d1}: 'ꇑ'
    YiSyllableLup,
    /// \u{a1d2}: 'ꇒ'
    YiSyllableLurx,
    /// \u{a1d3}: 'ꇓ'
    YiSyllableLur,
    /// \u{a1d4}: 'ꇔ'
    YiSyllableLyt,
    /// \u{a1d5}: 'ꇕ'
    YiSyllableLyx,
    /// \u{a1d6}: 'ꇖ'
    YiSyllableLy,
    /// \u{a1d7}: 'ꇗ'
    YiSyllableLyp,
    /// \u{a1d8}: 'ꇘ'
    YiSyllableLyrx,
    /// \u{a1d9}: 'ꇙ'
    YiSyllableLyr,
    /// \u{a1da}: 'ꇚ'
    YiSyllableGit,
    /// \u{a1db}: 'ꇛ'
    YiSyllableGix,
    /// \u{a1dc}: 'ꇜ'
    YiSyllableGi,
    /// \u{a1dd}: 'ꇝ'
    YiSyllableGip,
    /// \u{a1de}: 'ꇞ'
    YiSyllableGiet,
    /// \u{a1df}: 'ꇟ'
    YiSyllableGiex,
    /// \u{a1e0}: 'ꇠ'
    YiSyllableGie,
    /// \u{a1e1}: 'ꇡ'
    YiSyllableGiep,
    /// \u{a1e2}: 'ꇢ'
    YiSyllableGat,
    /// \u{a1e3}: 'ꇣ'
    YiSyllableGax,
    /// \u{a1e4}: 'ꇤ'
    YiSyllableGa,
    /// \u{a1e5}: 'ꇥ'
    YiSyllableGap,
    /// \u{a1e6}: 'ꇦ'
    YiSyllableGuot,
    /// \u{a1e7}: 'ꇧ'
    YiSyllableGuox,
    /// \u{a1e8}: 'ꇨ'
    YiSyllableGuo,
    /// \u{a1e9}: 'ꇩ'
    YiSyllableGuop,
    /// \u{a1ea}: 'ꇪ'
    YiSyllableGot,
    /// \u{a1eb}: 'ꇫ'
    YiSyllableGox,
    /// \u{a1ec}: 'ꇬ'
    YiSyllableGo,
    /// \u{a1ed}: 'ꇭ'
    YiSyllableGop,
    /// \u{a1ee}: 'ꇮ'
    YiSyllableGet,
    /// \u{a1ef}: 'ꇯ'
    YiSyllableGex,
    /// \u{a1f0}: 'ꇰ'
    YiSyllableGe,
    /// \u{a1f1}: 'ꇱ'
    YiSyllableGep,
    /// \u{a1f2}: 'ꇲ'
    YiSyllableGut,
    /// \u{a1f3}: 'ꇳ'
    YiSyllableGux,
    /// \u{a1f4}: 'ꇴ'
    YiSyllableGu,
    /// \u{a1f5}: 'ꇵ'
    YiSyllableGup,
    /// \u{a1f6}: 'ꇶ'
    YiSyllableGurx,
    /// \u{a1f7}: 'ꇷ'
    YiSyllableGur,
    /// \u{a1f8}: 'ꇸ'
    YiSyllableKit,
    /// \u{a1f9}: 'ꇹ'
    YiSyllableKix,
    /// \u{a1fa}: 'ꇺ'
    YiSyllableKi,
    /// \u{a1fb}: 'ꇻ'
    YiSyllableKip,
    /// \u{a1fc}: 'ꇼ'
    YiSyllableKiex,
    /// \u{a1fd}: 'ꇽ'
    YiSyllableKie,
    /// \u{a1fe}: 'ꇾ'
    YiSyllableKiep,
    /// \u{a1ff}: 'ꇿ'
    YiSyllableKat,
    /// \u{a200}: 'ꈀ'
    YiSyllableKax,
    /// \u{a201}: 'ꈁ'
    YiSyllableKa,
    /// \u{a202}: 'ꈂ'
    YiSyllableKap,
    /// \u{a203}: 'ꈃ'
    YiSyllableKuox,
    /// \u{a204}: 'ꈄ'
    YiSyllableKuo,
    /// \u{a205}: 'ꈅ'
    YiSyllableKuop,
    /// \u{a206}: 'ꈆ'
    YiSyllableKot,
    /// \u{a207}: 'ꈇ'
    YiSyllableKox,
    /// \u{a208}: 'ꈈ'
    YiSyllableKo,
    /// \u{a209}: 'ꈉ'
    YiSyllableKop,
    /// \u{a20a}: 'ꈊ'
    YiSyllableKet,
    /// \u{a20b}: 'ꈋ'
    YiSyllableKex,
    /// \u{a20c}: 'ꈌ'
    YiSyllableKe,
    /// \u{a20d}: 'ꈍ'
    YiSyllableKep,
    /// \u{a20e}: 'ꈎ'
    YiSyllableKut,
    /// \u{a20f}: 'ꈏ'
    YiSyllableKux,
    /// \u{a210}: 'ꈐ'
    YiSyllableKu,
    /// \u{a211}: 'ꈑ'
    YiSyllableKup,
    /// \u{a212}: 'ꈒ'
    YiSyllableKurx,
    /// \u{a213}: 'ꈓ'
    YiSyllableKur,
    /// \u{a214}: 'ꈔ'
    YiSyllableGgit,
    /// \u{a215}: 'ꈕ'
    YiSyllableGgix,
    /// \u{a216}: 'ꈖ'
    YiSyllableGgi,
    /// \u{a217}: 'ꈗ'
    YiSyllableGgiex,
    /// \u{a218}: 'ꈘ'
    YiSyllableGgie,
    /// \u{a219}: 'ꈙ'
    YiSyllableGgiep,
    /// \u{a21a}: 'ꈚ'
    YiSyllableGgat,
    /// \u{a21b}: 'ꈛ'
    YiSyllableGgax,
    /// \u{a21c}: 'ꈜ'
    YiSyllableGga,
    /// \u{a21d}: 'ꈝ'
    YiSyllableGgap,
    /// \u{a21e}: 'ꈞ'
    YiSyllableGguot,
    /// \u{a21f}: 'ꈟ'
    YiSyllableGguox,
    /// \u{a220}: 'ꈠ'
    YiSyllableGguo,
    /// \u{a221}: 'ꈡ'
    YiSyllableGguop,
    /// \u{a222}: 'ꈢ'
    YiSyllableGgot,
    /// \u{a223}: 'ꈣ'
    YiSyllableGgox,
    /// \u{a224}: 'ꈤ'
    YiSyllableGgo,
    /// \u{a225}: 'ꈥ'
    YiSyllableGgop,
    /// \u{a226}: 'ꈦ'
    YiSyllableGget,
    /// \u{a227}: 'ꈧ'
    YiSyllableGgex,
    /// \u{a228}: 'ꈨ'
    YiSyllableGge,
    /// \u{a229}: 'ꈩ'
    YiSyllableGgep,
    /// \u{a22a}: 'ꈪ'
    YiSyllableGgut,
    /// \u{a22b}: 'ꈫ'
    YiSyllableGgux,
    /// \u{a22c}: 'ꈬ'
    YiSyllableGgu,
    /// \u{a22d}: 'ꈭ'
    YiSyllableGgup,
    /// \u{a22e}: 'ꈮ'
    YiSyllableGgurx,
    /// \u{a22f}: 'ꈯ'
    YiSyllableGgur,
    /// \u{a230}: 'ꈰ'
    YiSyllableMgiex,
    /// \u{a231}: 'ꈱ'
    YiSyllableMgie,
    /// \u{a232}: 'ꈲ'
    YiSyllableMgat,
    /// \u{a233}: 'ꈳ'
    YiSyllableMgax,
    /// \u{a234}: 'ꈴ'
    YiSyllableMga,
    /// \u{a235}: 'ꈵ'
    YiSyllableMgap,
    /// \u{a236}: 'ꈶ'
    YiSyllableMguox,
    /// \u{a237}: 'ꈷ'
    YiSyllableMguo,
    /// \u{a238}: 'ꈸ'
    YiSyllableMguop,
    /// \u{a239}: 'ꈹ'
    YiSyllableMgot,
    /// \u{a23a}: 'ꈺ'
    YiSyllableMgox,
    /// \u{a23b}: 'ꈻ'
    YiSyllableMgo,
    /// \u{a23c}: 'ꈼ'
    YiSyllableMgop,
    /// \u{a23d}: 'ꈽ'
    YiSyllableMgex,
    /// \u{a23e}: 'ꈾ'
    YiSyllableMge,
    /// \u{a23f}: 'ꈿ'
    YiSyllableMgep,
    /// \u{a240}: 'ꉀ'
    YiSyllableMgut,
    /// \u{a241}: 'ꉁ'
    YiSyllableMgux,
    /// \u{a242}: 'ꉂ'
    YiSyllableMgu,
    /// \u{a243}: 'ꉃ'
    YiSyllableMgup,
    /// \u{a244}: 'ꉄ'
    YiSyllableMgurx,
    /// \u{a245}: 'ꉅ'
    YiSyllableMgur,
    /// \u{a246}: 'ꉆ'
    YiSyllableHxit,
    /// \u{a247}: 'ꉇ'
    YiSyllableHxix,
    /// \u{a248}: 'ꉈ'
    YiSyllableHxi,
    /// \u{a249}: 'ꉉ'
    YiSyllableHxip,
    /// \u{a24a}: 'ꉊ'
    YiSyllableHxiet,
    /// \u{a24b}: 'ꉋ'
    YiSyllableHxiex,
    /// \u{a24c}: 'ꉌ'
    YiSyllableHxie,
    /// \u{a24d}: 'ꉍ'
    YiSyllableHxiep,
    /// \u{a24e}: 'ꉎ'
    YiSyllableHxat,
    /// \u{a24f}: 'ꉏ'
    YiSyllableHxax,
    /// \u{a250}: 'ꉐ'
    YiSyllableHxa,
    /// \u{a251}: 'ꉑ'
    YiSyllableHxap,
    /// \u{a252}: 'ꉒ'
    YiSyllableHxuot,
    /// \u{a253}: 'ꉓ'
    YiSyllableHxuox,
    /// \u{a254}: 'ꉔ'
    YiSyllableHxuo,
    /// \u{a255}: 'ꉕ'
    YiSyllableHxuop,
    /// \u{a256}: 'ꉖ'
    YiSyllableHxot,
    /// \u{a257}: 'ꉗ'
    YiSyllableHxox,
    /// \u{a258}: 'ꉘ'
    YiSyllableHxo,
    /// \u{a259}: 'ꉙ'
    YiSyllableHxop,
    /// \u{a25a}: 'ꉚ'
    YiSyllableHxex,
    /// \u{a25b}: 'ꉛ'
    YiSyllableHxe,
    /// \u{a25c}: 'ꉜ'
    YiSyllableHxep,
    /// \u{a25d}: 'ꉝ'
    YiSyllableNgiex,
    /// \u{a25e}: 'ꉞ'
    YiSyllableNgie,
    /// \u{a25f}: 'ꉟ'
    YiSyllableNgiep,
    /// \u{a260}: 'ꉠ'
    YiSyllableNgat,
    /// \u{a261}: 'ꉡ'
    YiSyllableNgax,
    /// \u{a262}: 'ꉢ'
    YiSyllableNga,
    /// \u{a263}: 'ꉣ'
    YiSyllableNgap,
    /// \u{a264}: 'ꉤ'
    YiSyllableNguot,
    /// \u{a265}: 'ꉥ'
    YiSyllableNguox,
    /// \u{a266}: 'ꉦ'
    YiSyllableNguo,
    /// \u{a267}: 'ꉧ'
    YiSyllableNgot,
    /// \u{a268}: 'ꉨ'
    YiSyllableNgox,
    /// \u{a269}: 'ꉩ'
    YiSyllableNgo,
    /// \u{a26a}: 'ꉪ'
    YiSyllableNgop,
    /// \u{a26b}: 'ꉫ'
    YiSyllableNgex,
    /// \u{a26c}: 'ꉬ'
    YiSyllableNge,
    /// \u{a26d}: 'ꉭ'
    YiSyllableNgep,
    /// \u{a26e}: 'ꉮ'
    YiSyllableHit,
    /// \u{a26f}: 'ꉯ'
    YiSyllableHiex,
    /// \u{a270}: 'ꉰ'
    YiSyllableHie,
    /// \u{a271}: 'ꉱ'
    YiSyllableHat,
    /// \u{a272}: 'ꉲ'
    YiSyllableHax,
    /// \u{a273}: 'ꉳ'
    YiSyllableHa,
    /// \u{a274}: 'ꉴ'
    YiSyllableHap,
    /// \u{a275}: 'ꉵ'
    YiSyllableHuot,
    /// \u{a276}: 'ꉶ'
    YiSyllableHuox,
    /// \u{a277}: 'ꉷ'
    YiSyllableHuo,
    /// \u{a278}: 'ꉸ'
    YiSyllableHuop,
    /// \u{a279}: 'ꉹ'
    YiSyllableHot,
    /// \u{a27a}: 'ꉺ'
    YiSyllableHox,
    /// \u{a27b}: 'ꉻ'
    YiSyllableHo,
    /// \u{a27c}: 'ꉼ'
    YiSyllableHop,
    /// \u{a27d}: 'ꉽ'
    YiSyllableHex,
    /// \u{a27e}: 'ꉾ'
    YiSyllableHe,
    /// \u{a27f}: 'ꉿ'
    YiSyllableHep,
    /// \u{a280}: 'ꊀ'
    YiSyllableWat,
    /// \u{a281}: 'ꊁ'
    YiSyllableWax,
    /// \u{a282}: 'ꊂ'
    YiSyllableWa,
    /// \u{a283}: 'ꊃ'
    YiSyllableWap,
    /// \u{a284}: 'ꊄ'
    YiSyllableWuox,
    /// \u{a285}: 'ꊅ'
    YiSyllableWuo,
    /// \u{a286}: 'ꊆ'
    YiSyllableWuop,
    /// \u{a287}: 'ꊇ'
    YiSyllableWox,
    /// \u{a288}: 'ꊈ'
    YiSyllableWo,
    /// \u{a289}: 'ꊉ'
    YiSyllableWop,
    /// \u{a28a}: 'ꊊ'
    YiSyllableWex,
    /// \u{a28b}: 'ꊋ'
    YiSyllableWe,
    /// \u{a28c}: 'ꊌ'
    YiSyllableWep,
    /// \u{a28d}: 'ꊍ'
    YiSyllableZit,
    /// \u{a28e}: 'ꊎ'
    YiSyllableZix,
    /// \u{a28f}: 'ꊏ'
    YiSyllableZi,
    /// \u{a290}: 'ꊐ'
    YiSyllableZip,
    /// \u{a291}: 'ꊑ'
    YiSyllableZiex,
    /// \u{a292}: 'ꊒ'
    YiSyllableZie,
    /// \u{a293}: 'ꊓ'
    YiSyllableZiep,
    /// \u{a294}: 'ꊔ'
    YiSyllableZat,
    /// \u{a295}: 'ꊕ'
    YiSyllableZax,
    /// \u{a296}: 'ꊖ'
    YiSyllableZa,
    /// \u{a297}: 'ꊗ'
    YiSyllableZap,
    /// \u{a298}: 'ꊘ'
    YiSyllableZuox,
    /// \u{a299}: 'ꊙ'
    YiSyllableZuo,
    /// \u{a29a}: 'ꊚ'
    YiSyllableZuop,
    /// \u{a29b}: 'ꊛ'
    YiSyllableZot,
    /// \u{a29c}: 'ꊜ'
    YiSyllableZox,
    /// \u{a29d}: 'ꊝ'
    YiSyllableZo,
    /// \u{a29e}: 'ꊞ'
    YiSyllableZop,
    /// \u{a29f}: 'ꊟ'
    YiSyllableZex,
    /// \u{a2a0}: 'ꊠ'
    YiSyllableZe,
    /// \u{a2a1}: 'ꊡ'
    YiSyllableZep,
    /// \u{a2a2}: 'ꊢ'
    YiSyllableZut,
    /// \u{a2a3}: 'ꊣ'
    YiSyllableZux,
    /// \u{a2a4}: 'ꊤ'
    YiSyllableZu,
    /// \u{a2a5}: 'ꊥ'
    YiSyllableZup,
    /// \u{a2a6}: 'ꊦ'
    YiSyllableZurx,
    /// \u{a2a7}: 'ꊧ'
    YiSyllableZur,
    /// \u{a2a8}: 'ꊨ'
    YiSyllableZyt,
    /// \u{a2a9}: 'ꊩ'
    YiSyllableZyx,
    /// \u{a2aa}: 'ꊪ'
    YiSyllableZy,
    /// \u{a2ab}: 'ꊫ'
    YiSyllableZyp,
    /// \u{a2ac}: 'ꊬ'
    YiSyllableZyrx,
    /// \u{a2ad}: 'ꊭ'
    YiSyllableZyr,
    /// \u{a2ae}: 'ꊮ'
    YiSyllableCit,
    /// \u{a2af}: 'ꊯ'
    YiSyllableCix,
    /// \u{a2b0}: 'ꊰ'
    YiSyllableCi,
    /// \u{a2b1}: 'ꊱ'
    YiSyllableCip,
    /// \u{a2b2}: 'ꊲ'
    YiSyllableCiet,
    /// \u{a2b3}: 'ꊳ'
    YiSyllableCiex,
    /// \u{a2b4}: 'ꊴ'
    YiSyllableCie,
    /// \u{a2b5}: 'ꊵ'
    YiSyllableCiep,
    /// \u{a2b6}: 'ꊶ'
    YiSyllableCat,
    /// \u{a2b7}: 'ꊷ'
    YiSyllableCax,
    /// \u{a2b8}: 'ꊸ'
    YiSyllableCa,
    /// \u{a2b9}: 'ꊹ'
    YiSyllableCap,
    /// \u{a2ba}: 'ꊺ'
    YiSyllableCuox,
    /// \u{a2bb}: 'ꊻ'
    YiSyllableCuo,
    /// \u{a2bc}: 'ꊼ'
    YiSyllableCuop,
    /// \u{a2bd}: 'ꊽ'
    YiSyllableCot,
    /// \u{a2be}: 'ꊾ'
    YiSyllableCox,
    /// \u{a2bf}: 'ꊿ'
    YiSyllableCo,
    /// \u{a2c0}: 'ꋀ'
    YiSyllableCop,
    /// \u{a2c1}: 'ꋁ'
    YiSyllableCex,
    /// \u{a2c2}: 'ꋂ'
    YiSyllableCe,
    /// \u{a2c3}: 'ꋃ'
    YiSyllableCep,
    /// \u{a2c4}: 'ꋄ'
    YiSyllableCut,
    /// \u{a2c5}: 'ꋅ'
    YiSyllableCux,
    /// \u{a2c6}: 'ꋆ'
    YiSyllableCu,
    /// \u{a2c7}: 'ꋇ'
    YiSyllableCup,
    /// \u{a2c8}: 'ꋈ'
    YiSyllableCurx,
    /// \u{a2c9}: 'ꋉ'
    YiSyllableCur,
    /// \u{a2ca}: 'ꋊ'
    YiSyllableCyt,
    /// \u{a2cb}: 'ꋋ'
    YiSyllableCyx,
    /// \u{a2cc}: 'ꋌ'
    YiSyllableCy,
    /// \u{a2cd}: 'ꋍ'
    YiSyllableCyp,
    /// \u{a2ce}: 'ꋎ'
    YiSyllableCyrx,
    /// \u{a2cf}: 'ꋏ'
    YiSyllableCyr,
    /// \u{a2d0}: 'ꋐ'
    YiSyllableZzit,
    /// \u{a2d1}: 'ꋑ'
    YiSyllableZzix,
    /// \u{a2d2}: 'ꋒ'
    YiSyllableZzi,
    /// \u{a2d3}: 'ꋓ'
    YiSyllableZzip,
    /// \u{a2d4}: 'ꋔ'
    YiSyllableZziet,
    /// \u{a2d5}: 'ꋕ'
    YiSyllableZziex,
    /// \u{a2d6}: 'ꋖ'
    YiSyllableZzie,
    /// \u{a2d7}: 'ꋗ'
    YiSyllableZziep,
    /// \u{a2d8}: 'ꋘ'
    YiSyllableZzat,
    /// \u{a2d9}: 'ꋙ'
    YiSyllableZzax,
    /// \u{a2da}: 'ꋚ'
    YiSyllableZza,
    /// \u{a2db}: 'ꋛ'
    YiSyllableZzap,
    /// \u{a2dc}: 'ꋜ'
    YiSyllableZzox,
    /// \u{a2dd}: 'ꋝ'
    YiSyllableZzo,
    /// \u{a2de}: 'ꋞ'
    YiSyllableZzop,
    /// \u{a2df}: 'ꋟ'
    YiSyllableZzex,
    /// \u{a2e0}: 'ꋠ'
    YiSyllableZze,
    /// \u{a2e1}: 'ꋡ'
    YiSyllableZzep,
    /// \u{a2e2}: 'ꋢ'
    YiSyllableZzux,
    /// \u{a2e3}: 'ꋣ'
    YiSyllableZzu,
    /// \u{a2e4}: 'ꋤ'
    YiSyllableZzup,
    /// \u{a2e5}: 'ꋥ'
    YiSyllableZzurx,
    /// \u{a2e6}: 'ꋦ'
    YiSyllableZzur,
    /// \u{a2e7}: 'ꋧ'
    YiSyllableZzyt,
    /// \u{a2e8}: 'ꋨ'
    YiSyllableZzyx,
    /// \u{a2e9}: 'ꋩ'
    YiSyllableZzy,
    /// \u{a2ea}: 'ꋪ'
    YiSyllableZzyp,
    /// \u{a2eb}: 'ꋫ'
    YiSyllableZzyrx,
    /// \u{a2ec}: 'ꋬ'
    YiSyllableZzyr,
    /// \u{a2ed}: 'ꋭ'
    YiSyllableNzit,
    /// \u{a2ee}: 'ꋮ'
    YiSyllableNzix,
    /// \u{a2ef}: 'ꋯ'
    YiSyllableNzi,
    /// \u{a2f0}: 'ꋰ'
    YiSyllableNzip,
    /// \u{a2f1}: 'ꋱ'
    YiSyllableNziex,
    /// \u{a2f2}: 'ꋲ'
    YiSyllableNzie,
    /// \u{a2f3}: 'ꋳ'
    YiSyllableNziep,
    /// \u{a2f4}: 'ꋴ'
    YiSyllableNzat,
    /// \u{a2f5}: 'ꋵ'
    YiSyllableNzax,
    /// \u{a2f6}: 'ꋶ'
    YiSyllableNza,
    /// \u{a2f7}: 'ꋷ'
    YiSyllableNzap,
    /// \u{a2f8}: 'ꋸ'
    YiSyllableNzuox,
    /// \u{a2f9}: 'ꋹ'
    YiSyllableNzuo,
    /// \u{a2fa}: 'ꋺ'
    YiSyllableNzox,
    /// \u{a2fb}: 'ꋻ'
    YiSyllableNzop,
    /// \u{a2fc}: 'ꋼ'
    YiSyllableNzex,
    /// \u{a2fd}: 'ꋽ'
    YiSyllableNze,
    /// \u{a2fe}: 'ꋾ'
    YiSyllableNzux,
    /// \u{a2ff}: 'ꋿ'
    YiSyllableNzu,
    /// \u{a300}: 'ꌀ'
    YiSyllableNzup,
    /// \u{a301}: 'ꌁ'
    YiSyllableNzurx,
    /// \u{a302}: 'ꌂ'
    YiSyllableNzur,
    /// \u{a303}: 'ꌃ'
    YiSyllableNzyt,
    /// \u{a304}: 'ꌄ'
    YiSyllableNzyx,
    /// \u{a305}: 'ꌅ'
    YiSyllableNzy,
    /// \u{a306}: 'ꌆ'
    YiSyllableNzyp,
    /// \u{a307}: 'ꌇ'
    YiSyllableNzyrx,
    /// \u{a308}: 'ꌈ'
    YiSyllableNzyr,
    /// \u{a309}: 'ꌉ'
    YiSyllableSit,
    /// \u{a30a}: 'ꌊ'
    YiSyllableSix,
    /// \u{a30b}: 'ꌋ'
    YiSyllableSi,
    /// \u{a30c}: 'ꌌ'
    YiSyllableSip,
    /// \u{a30d}: 'ꌍ'
    YiSyllableSiex,
    /// \u{a30e}: 'ꌎ'
    YiSyllableSie,
    /// \u{a30f}: 'ꌏ'
    YiSyllableSiep,
    /// \u{a310}: 'ꌐ'
    YiSyllableSat,
    /// \u{a311}: 'ꌑ'
    YiSyllableSax,
    /// \u{a312}: 'ꌒ'
    YiSyllableSa,
    /// \u{a313}: 'ꌓ'
    YiSyllableSap,
    /// \u{a314}: 'ꌔ'
    YiSyllableSuox,
    /// \u{a315}: 'ꌕ'
    YiSyllableSuo,
    /// \u{a316}: 'ꌖ'
    YiSyllableSuop,
    /// \u{a317}: 'ꌗ'
    YiSyllableSot,
    /// \u{a318}: 'ꌘ'
    YiSyllableSox,
    /// \u{a319}: 'ꌙ'
    YiSyllableSo,
    /// \u{a31a}: 'ꌚ'
    YiSyllableSop,
    /// \u{a31b}: 'ꌛ'
    YiSyllableSex,
    /// \u{a31c}: 'ꌜ'
    YiSyllableSe,
    /// \u{a31d}: 'ꌝ'
    YiSyllableSep,
    /// \u{a31e}: 'ꌞ'
    YiSyllableSut,
    /// \u{a31f}: 'ꌟ'
    YiSyllableSux,
    /// \u{a320}: 'ꌠ'
    YiSyllableSu,
    /// \u{a321}: 'ꌡ'
    YiSyllableSup,
    /// \u{a322}: 'ꌢ'
    YiSyllableSurx,
    /// \u{a323}: 'ꌣ'
    YiSyllableSur,
    /// \u{a324}: 'ꌤ'
    YiSyllableSyt,
    /// \u{a325}: 'ꌥ'
    YiSyllableSyx,
    /// \u{a326}: 'ꌦ'
    YiSyllableSy,
    /// \u{a327}: 'ꌧ'
    YiSyllableSyp,
    /// \u{a328}: 'ꌨ'
    YiSyllableSyrx,
    /// \u{a329}: 'ꌩ'
    YiSyllableSyr,
    /// \u{a32a}: 'ꌪ'
    YiSyllableSsit,
    /// \u{a32b}: 'ꌫ'
    YiSyllableSsix,
    /// \u{a32c}: 'ꌬ'
    YiSyllableSsi,
    /// \u{a32d}: 'ꌭ'
    YiSyllableSsip,
    /// \u{a32e}: 'ꌮ'
    YiSyllableSsiex,
    /// \u{a32f}: 'ꌯ'
    YiSyllableSsie,
    /// \u{a330}: 'ꌰ'
    YiSyllableSsiep,
    /// \u{a331}: 'ꌱ'
    YiSyllableSsat,
    /// \u{a332}: 'ꌲ'
    YiSyllableSsax,
    /// \u{a333}: 'ꌳ'
    YiSyllableSsa,
    /// \u{a334}: 'ꌴ'
    YiSyllableSsap,
    /// \u{a335}: 'ꌵ'
    YiSyllableSsot,
    /// \u{a336}: 'ꌶ'
    YiSyllableSsox,
    /// \u{a337}: 'ꌷ'
    YiSyllableSso,
    /// \u{a338}: 'ꌸ'
    YiSyllableSsop,
    /// \u{a339}: 'ꌹ'
    YiSyllableSsex,
    /// \u{a33a}: 'ꌺ'
    YiSyllableSse,
    /// \u{a33b}: 'ꌻ'
    YiSyllableSsep,
    /// \u{a33c}: 'ꌼ'
    YiSyllableSsut,
    /// \u{a33d}: 'ꌽ'
    YiSyllableSsux,
    /// \u{a33e}: 'ꌾ'
    YiSyllableSsu,
    /// \u{a33f}: 'ꌿ'
    YiSyllableSsup,
    /// \u{a340}: 'ꍀ'
    YiSyllableSsyt,
    /// \u{a341}: 'ꍁ'
    YiSyllableSsyx,
    /// \u{a342}: 'ꍂ'
    YiSyllableSsy,
    /// \u{a343}: 'ꍃ'
    YiSyllableSsyp,
    /// \u{a344}: 'ꍄ'
    YiSyllableSsyrx,
    /// \u{a345}: 'ꍅ'
    YiSyllableSsyr,
    /// \u{a346}: 'ꍆ'
    YiSyllableZhat,
    /// \u{a347}: 'ꍇ'
    YiSyllableZhax,
    /// \u{a348}: 'ꍈ'
    YiSyllableZha,
    /// \u{a349}: 'ꍉ'
    YiSyllableZhap,
    /// \u{a34a}: 'ꍊ'
    YiSyllableZhuox,
    /// \u{a34b}: 'ꍋ'
    YiSyllableZhuo,
    /// \u{a34c}: 'ꍌ'
    YiSyllableZhuop,
    /// \u{a34d}: 'ꍍ'
    YiSyllableZhot,
    /// \u{a34e}: 'ꍎ'
    YiSyllableZhox,
    /// \u{a34f}: 'ꍏ'
    YiSyllableZho,
    /// \u{a350}: 'ꍐ'
    YiSyllableZhop,
    /// \u{a351}: 'ꍑ'
    YiSyllableZhet,
    /// \u{a352}: 'ꍒ'
    YiSyllableZhex,
    /// \u{a353}: 'ꍓ'
    YiSyllableZhe,
    /// \u{a354}: 'ꍔ'
    YiSyllableZhep,
    /// \u{a355}: 'ꍕ'
    YiSyllableZhut,
    /// \u{a356}: 'ꍖ'
    YiSyllableZhux,
    /// \u{a357}: 'ꍗ'
    YiSyllableZhu,
    /// \u{a358}: 'ꍘ'
    YiSyllableZhup,
    /// \u{a359}: 'ꍙ'
    YiSyllableZhurx,
    /// \u{a35a}: 'ꍚ'
    YiSyllableZhur,
    /// \u{a35b}: 'ꍛ'
    YiSyllableZhyt,
    /// \u{a35c}: 'ꍜ'
    YiSyllableZhyx,
    /// \u{a35d}: 'ꍝ'
    YiSyllableZhy,
    /// \u{a35e}: 'ꍞ'
    YiSyllableZhyp,
    /// \u{a35f}: 'ꍟ'
    YiSyllableZhyrx,
    /// \u{a360}: 'ꍠ'
    YiSyllableZhyr,
    /// \u{a361}: 'ꍡ'
    YiSyllableChat,
    /// \u{a362}: 'ꍢ'
    YiSyllableChax,
    /// \u{a363}: 'ꍣ'
    YiSyllableCha,
    /// \u{a364}: 'ꍤ'
    YiSyllableChap,
    /// \u{a365}: 'ꍥ'
    YiSyllableChuot,
    /// \u{a366}: 'ꍦ'
    YiSyllableChuox,
    /// \u{a367}: 'ꍧ'
    YiSyllableChuo,
    /// \u{a368}: 'ꍨ'
    YiSyllableChuop,
    /// \u{a369}: 'ꍩ'
    YiSyllableChot,
    /// \u{a36a}: 'ꍪ'
    YiSyllableChox,
    /// \u{a36b}: 'ꍫ'
    YiSyllableCho,
    /// \u{a36c}: 'ꍬ'
    YiSyllableChop,
    /// \u{a36d}: 'ꍭ'
    YiSyllableChet,
    /// \u{a36e}: 'ꍮ'
    YiSyllableChex,
    /// \u{a36f}: 'ꍯ'
    YiSyllableChe,
    /// \u{a370}: 'ꍰ'
    YiSyllableChep,
    /// \u{a371}: 'ꍱ'
    YiSyllableChux,
    /// \u{a372}: 'ꍲ'
    YiSyllableChu,
    /// \u{a373}: 'ꍳ'
    YiSyllableChup,
    /// \u{a374}: 'ꍴ'
    YiSyllableChurx,
    /// \u{a375}: 'ꍵ'
    YiSyllableChur,
    /// \u{a376}: 'ꍶ'
    YiSyllableChyt,
    /// \u{a377}: 'ꍷ'
    YiSyllableChyx,
    /// \u{a378}: 'ꍸ'
    YiSyllableChy,
    /// \u{a379}: 'ꍹ'
    YiSyllableChyp,
    /// \u{a37a}: 'ꍺ'
    YiSyllableChyrx,
    /// \u{a37b}: 'ꍻ'
    YiSyllableChyr,
    /// \u{a37c}: 'ꍼ'
    YiSyllableRrax,
    /// \u{a37d}: 'ꍽ'
    YiSyllableRra,
    /// \u{a37e}: 'ꍾ'
    YiSyllableRruox,
    /// \u{a37f}: 'ꍿ'
    YiSyllableRruo,
    /// \u{a380}: 'ꎀ'
    YiSyllableRrot,
    /// \u{a381}: 'ꎁ'
    YiSyllableRrox,
    /// \u{a382}: 'ꎂ'
    YiSyllableRro,
    /// \u{a383}: 'ꎃ'
    YiSyllableRrop,
    /// \u{a384}: 'ꎄ'
    YiSyllableRret,
    /// \u{a385}: 'ꎅ'
    YiSyllableRrex,
    /// \u{a386}: 'ꎆ'
    YiSyllableRre,
    /// \u{a387}: 'ꎇ'
    YiSyllableRrep,
    /// \u{a388}: 'ꎈ'
    YiSyllableRrut,
    /// \u{a389}: 'ꎉ'
    YiSyllableRrux,
    /// \u{a38a}: 'ꎊ'
    YiSyllableRru,
    /// \u{a38b}: 'ꎋ'
    YiSyllableRrup,
    /// \u{a38c}: 'ꎌ'
    YiSyllableRrurx,
    /// \u{a38d}: 'ꎍ'
    YiSyllableRrur,
    /// \u{a38e}: 'ꎎ'
    YiSyllableRryt,
    /// \u{a38f}: 'ꎏ'
    YiSyllableRryx,
    /// \u{a390}: 'ꎐ'
    YiSyllableRry,
    /// \u{a391}: 'ꎑ'
    YiSyllableRryp,
    /// \u{a392}: 'ꎒ'
    YiSyllableRryrx,
    /// \u{a393}: 'ꎓ'
    YiSyllableRryr,
    /// \u{a394}: 'ꎔ'
    YiSyllableNrat,
    /// \u{a395}: 'ꎕ'
    YiSyllableNrax,
    /// \u{a396}: 'ꎖ'
    YiSyllableNra,
    /// \u{a397}: 'ꎗ'
    YiSyllableNrap,
    /// \u{a398}: 'ꎘ'
    YiSyllableNrox,
    /// \u{a399}: 'ꎙ'
    YiSyllableNro,
    /// \u{a39a}: 'ꎚ'
    YiSyllableNrop,
    /// \u{a39b}: 'ꎛ'
    YiSyllableNret,
    /// \u{a39c}: 'ꎜ'
    YiSyllableNrex,
    /// \u{a39d}: 'ꎝ'
    YiSyllableNre,
    /// \u{a39e}: 'ꎞ'
    YiSyllableNrep,
    /// \u{a39f}: 'ꎟ'
    YiSyllableNrut,
    /// \u{a3a0}: 'ꎠ'
    YiSyllableNrux,
    /// \u{a3a1}: 'ꎡ'
    YiSyllableNru,
    /// \u{a3a2}: 'ꎢ'
    YiSyllableNrup,
    /// \u{a3a3}: 'ꎣ'
    YiSyllableNrurx,
    /// \u{a3a4}: 'ꎤ'
    YiSyllableNrur,
    /// \u{a3a5}: 'ꎥ'
    YiSyllableNryt,
    /// \u{a3a6}: 'ꎦ'
    YiSyllableNryx,
    /// \u{a3a7}: 'ꎧ'
    YiSyllableNry,
    /// \u{a3a8}: 'ꎨ'
    YiSyllableNryp,
    /// \u{a3a9}: 'ꎩ'
    YiSyllableNryrx,
    /// \u{a3aa}: 'ꎪ'
    YiSyllableNryr,
    /// \u{a3ab}: 'ꎫ'
    YiSyllableShat,
    /// \u{a3ac}: 'ꎬ'
    YiSyllableShax,
    /// \u{a3ad}: 'ꎭ'
    YiSyllableSha,
    /// \u{a3ae}: 'ꎮ'
    YiSyllableShap,
    /// \u{a3af}: 'ꎯ'
    YiSyllableShuox,
    /// \u{a3b0}: 'ꎰ'
    YiSyllableShuo,
    /// \u{a3b1}: 'ꎱ'
    YiSyllableShuop,
    /// \u{a3b2}: 'ꎲ'
    YiSyllableShot,
    /// \u{a3b3}: 'ꎳ'
    YiSyllableShox,
    /// \u{a3b4}: 'ꎴ'
    YiSyllableSho,
    /// \u{a3b5}: 'ꎵ'
    YiSyllableShop,
    /// \u{a3b6}: 'ꎶ'
    YiSyllableShet,
    /// \u{a3b7}: 'ꎷ'
    YiSyllableShex,
    /// \u{a3b8}: 'ꎸ'
    YiSyllableShe,
    /// \u{a3b9}: 'ꎹ'
    YiSyllableShep,
    /// \u{a3ba}: 'ꎺ'
    YiSyllableShut,
    /// \u{a3bb}: 'ꎻ'
    YiSyllableShux,
    /// \u{a3bc}: 'ꎼ'
    YiSyllableShu,
    /// \u{a3bd}: 'ꎽ'
    YiSyllableShup,
    /// \u{a3be}: 'ꎾ'
    YiSyllableShurx,
    /// \u{a3bf}: 'ꎿ'
    YiSyllableShur,
    /// \u{a3c0}: 'ꏀ'
    YiSyllableShyt,
    /// \u{a3c1}: 'ꏁ'
    YiSyllableShyx,
    /// \u{a3c2}: 'ꏂ'
    YiSyllableShy,
    /// \u{a3c3}: 'ꏃ'
    YiSyllableShyp,
    /// \u{a3c4}: 'ꏄ'
    YiSyllableShyrx,
    /// \u{a3c5}: 'ꏅ'
    YiSyllableShyr,
    /// \u{a3c6}: 'ꏆ'
    YiSyllableRat,
    /// \u{a3c7}: 'ꏇ'
    YiSyllableRax,
    /// \u{a3c8}: 'ꏈ'
    YiSyllableRa,
    /// \u{a3c9}: 'ꏉ'
    YiSyllableRap,
    /// \u{a3ca}: 'ꏊ'
    YiSyllableRuox,
    /// \u{a3cb}: 'ꏋ'
    YiSyllableRuo,
    /// \u{a3cc}: 'ꏌ'
    YiSyllableRuop,
    /// \u{a3cd}: 'ꏍ'
    YiSyllableRot,
    /// \u{a3ce}: 'ꏎ'
    YiSyllableRox,
    /// \u{a3cf}: 'ꏏ'
    YiSyllableRo,
    /// \u{a3d0}: 'ꏐ'
    YiSyllableRop,
    /// \u{a3d1}: 'ꏑ'
    YiSyllableRex,
    /// \u{a3d2}: 'ꏒ'
    YiSyllableRe,
    /// \u{a3d3}: 'ꏓ'
    YiSyllableRep,
    /// \u{a3d4}: 'ꏔ'
    YiSyllableRut,
    /// \u{a3d5}: 'ꏕ'
    YiSyllableRux,
    /// \u{a3d6}: 'ꏖ'
    YiSyllableRu,
    /// \u{a3d7}: 'ꏗ'
    YiSyllableRup,
    /// \u{a3d8}: 'ꏘ'
    YiSyllableRurx,
    /// \u{a3d9}: 'ꏙ'
    YiSyllableRur,
    /// \u{a3da}: 'ꏚ'
    YiSyllableRyt,
    /// \u{a3db}: 'ꏛ'
    YiSyllableRyx,
    /// \u{a3dc}: 'ꏜ'
    YiSyllableRy,
    /// \u{a3dd}: 'ꏝ'
    YiSyllableRyp,
    /// \u{a3de}: 'ꏞ'
    YiSyllableRyrx,
    /// \u{a3df}: 'ꏟ'
    YiSyllableRyr,
    /// \u{a3e0}: 'ꏠ'
    YiSyllableJit,
    /// \u{a3e1}: 'ꏡ'
    YiSyllableJix,
    /// \u{a3e2}: 'ꏢ'
    YiSyllableJi,
    /// \u{a3e3}: 'ꏣ'
    YiSyllableJip,
    /// \u{a3e4}: 'ꏤ'
    YiSyllableJiet,
    /// \u{a3e5}: 'ꏥ'
    YiSyllableJiex,
    /// \u{a3e6}: 'ꏦ'
    YiSyllableJie,
    /// \u{a3e7}: 'ꏧ'
    YiSyllableJiep,
    /// \u{a3e8}: 'ꏨ'
    YiSyllableJuot,
    /// \u{a3e9}: 'ꏩ'
    YiSyllableJuox,
    /// \u{a3ea}: 'ꏪ'
    YiSyllableJuo,
    /// \u{a3eb}: 'ꏫ'
    YiSyllableJuop,
    /// \u{a3ec}: 'ꏬ'
    YiSyllableJot,
    /// \u{a3ed}: 'ꏭ'
    YiSyllableJox,
    /// \u{a3ee}: 'ꏮ'
    YiSyllableJo,
    /// \u{a3ef}: 'ꏯ'
    YiSyllableJop,
    /// \u{a3f0}: 'ꏰ'
    YiSyllableJut,
    /// \u{a3f1}: 'ꏱ'
    YiSyllableJux,
    /// \u{a3f2}: 'ꏲ'
    YiSyllableJu,
    /// \u{a3f3}: 'ꏳ'
    YiSyllableJup,
    /// \u{a3f4}: 'ꏴ'
    YiSyllableJurx,
    /// \u{a3f5}: 'ꏵ'
    YiSyllableJur,
    /// \u{a3f6}: 'ꏶ'
    YiSyllableJyt,
    /// \u{a3f7}: 'ꏷ'
    YiSyllableJyx,
    /// \u{a3f8}: 'ꏸ'
    YiSyllableJy,
    /// \u{a3f9}: 'ꏹ'
    YiSyllableJyp,
    /// \u{a3fa}: 'ꏺ'
    YiSyllableJyrx,
    /// \u{a3fb}: 'ꏻ'
    YiSyllableJyr,
    /// \u{a3fc}: 'ꏼ'
    YiSyllableQit,
    /// \u{a3fd}: 'ꏽ'
    YiSyllableQix,
    /// \u{a3fe}: 'ꏾ'
    YiSyllableQi,
    /// \u{a3ff}: 'ꏿ'
    YiSyllableQip,
    /// \u{a400}: 'ꐀ'
    YiSyllableQiet,
    /// \u{a401}: 'ꐁ'
    YiSyllableQiex,
    /// \u{a402}: 'ꐂ'
    YiSyllableQie,
    /// \u{a403}: 'ꐃ'
    YiSyllableQiep,
    /// \u{a404}: 'ꐄ'
    YiSyllableQuot,
    /// \u{a405}: 'ꐅ'
    YiSyllableQuox,
    /// \u{a406}: 'ꐆ'
    YiSyllableQuo,
    /// \u{a407}: 'ꐇ'
    YiSyllableQuop,
    /// \u{a408}: 'ꐈ'
    YiSyllableQot,
    /// \u{a409}: 'ꐉ'
    YiSyllableQox,
    /// \u{a40a}: 'ꐊ'
    YiSyllableQo,
    /// \u{a40b}: 'ꐋ'
    YiSyllableQop,
    /// \u{a40c}: 'ꐌ'
    YiSyllableQut,
    /// \u{a40d}: 'ꐍ'
    YiSyllableQux,
    /// \u{a40e}: 'ꐎ'
    YiSyllableQu,
    /// \u{a40f}: 'ꐏ'
    YiSyllableQup,
    /// \u{a410}: 'ꐐ'
    YiSyllableQurx,
    /// \u{a411}: 'ꐑ'
    YiSyllableQur,
    /// \u{a412}: 'ꐒ'
    YiSyllableQyt,
    /// \u{a413}: 'ꐓ'
    YiSyllableQyx,
    /// \u{a414}: 'ꐔ'
    YiSyllableQy,
    /// \u{a415}: 'ꐕ'
    YiSyllableQyp,
    /// \u{a416}: 'ꐖ'
    YiSyllableQyrx,
    /// \u{a417}: 'ꐗ'
    YiSyllableQyr,
    /// \u{a418}: 'ꐘ'
    YiSyllableJjit,
    /// \u{a419}: 'ꐙ'
    YiSyllableJjix,
    /// \u{a41a}: 'ꐚ'
    YiSyllableJji,
    /// \u{a41b}: 'ꐛ'
    YiSyllableJjip,
    /// \u{a41c}: 'ꐜ'
    YiSyllableJjiet,
    /// \u{a41d}: 'ꐝ'
    YiSyllableJjiex,
    /// \u{a41e}: 'ꐞ'
    YiSyllableJjie,
    /// \u{a41f}: 'ꐟ'
    YiSyllableJjiep,
    /// \u{a420}: 'ꐠ'
    YiSyllableJjuox,
    /// \u{a421}: 'ꐡ'
    YiSyllableJjuo,
    /// \u{a422}: 'ꐢ'
    YiSyllableJjuop,
    /// \u{a423}: 'ꐣ'
    YiSyllableJjot,
    /// \u{a424}: 'ꐤ'
    YiSyllableJjox,
    /// \u{a425}: 'ꐥ'
    YiSyllableJjo,
    /// \u{a426}: 'ꐦ'
    YiSyllableJjop,
    /// \u{a427}: 'ꐧ'
    YiSyllableJjut,
    /// \u{a428}: 'ꐨ'
    YiSyllableJjux,
    /// \u{a429}: 'ꐩ'
    YiSyllableJju,
    /// \u{a42a}: 'ꐪ'
    YiSyllableJjup,
    /// \u{a42b}: 'ꐫ'
    YiSyllableJjurx,
    /// \u{a42c}: 'ꐬ'
    YiSyllableJjur,
    /// \u{a42d}: 'ꐭ'
    YiSyllableJjyt,
    /// \u{a42e}: 'ꐮ'
    YiSyllableJjyx,
    /// \u{a42f}: 'ꐯ'
    YiSyllableJjy,
    /// \u{a430}: 'ꐰ'
    YiSyllableJjyp,
    /// \u{a431}: 'ꐱ'
    YiSyllableNjit,
    /// \u{a432}: 'ꐲ'
    YiSyllableNjix,
    /// \u{a433}: 'ꐳ'
    YiSyllableNji,
    /// \u{a434}: 'ꐴ'
    YiSyllableNjip,
    /// \u{a435}: 'ꐵ'
    YiSyllableNjiet,
    /// \u{a436}: 'ꐶ'
    YiSyllableNjiex,
    /// \u{a437}: 'ꐷ'
    YiSyllableNjie,
    /// \u{a438}: 'ꐸ'
    YiSyllableNjiep,
    /// \u{a439}: 'ꐹ'
    YiSyllableNjuox,
    /// \u{a43a}: 'ꐺ'
    YiSyllableNjuo,
    /// \u{a43b}: 'ꐻ'
    YiSyllableNjot,
    /// \u{a43c}: 'ꐼ'
    YiSyllableNjox,
    /// \u{a43d}: 'ꐽ'
    YiSyllableNjo,
    /// \u{a43e}: 'ꐾ'
    YiSyllableNjop,
    /// \u{a43f}: 'ꐿ'
    YiSyllableNjux,
    /// \u{a440}: 'ꑀ'
    YiSyllableNju,
    /// \u{a441}: 'ꑁ'
    YiSyllableNjup,
    /// \u{a442}: 'ꑂ'
    YiSyllableNjurx,
    /// \u{a443}: 'ꑃ'
    YiSyllableNjur,
    /// \u{a444}: 'ꑄ'
    YiSyllableNjyt,
    /// \u{a445}: 'ꑅ'
    YiSyllableNjyx,
    /// \u{a446}: 'ꑆ'
    YiSyllableNjy,
    /// \u{a447}: 'ꑇ'
    YiSyllableNjyp,
    /// \u{a448}: 'ꑈ'
    YiSyllableNjyrx,
    /// \u{a449}: 'ꑉ'
    YiSyllableNjyr,
    /// \u{a44a}: 'ꑊ'
    YiSyllableNyit,
    /// \u{a44b}: 'ꑋ'
    YiSyllableNyix,
    /// \u{a44c}: 'ꑌ'
    YiSyllableNyi,
    /// \u{a44d}: 'ꑍ'
    YiSyllableNyip,
    /// \u{a44e}: 'ꑎ'
    YiSyllableNyiet,
    /// \u{a44f}: 'ꑏ'
    YiSyllableNyiex,
    /// \u{a450}: 'ꑐ'
    YiSyllableNyie,
    /// \u{a451}: 'ꑑ'
    YiSyllableNyiep,
    /// \u{a452}: 'ꑒ'
    YiSyllableNyuox,
    /// \u{a453}: 'ꑓ'
    YiSyllableNyuo,
    /// \u{a454}: 'ꑔ'
    YiSyllableNyuop,
    /// \u{a455}: 'ꑕ'
    YiSyllableNyot,
    /// \u{a456}: 'ꑖ'
    YiSyllableNyox,
    /// \u{a457}: 'ꑗ'
    YiSyllableNyo,
    /// \u{a458}: 'ꑘ'
    YiSyllableNyop,
    /// \u{a459}: 'ꑙ'
    YiSyllableNyut,
    /// \u{a45a}: 'ꑚ'
    YiSyllableNyux,
    /// \u{a45b}: 'ꑛ'
    YiSyllableNyu,
    /// \u{a45c}: 'ꑜ'
    YiSyllableNyup,
    /// \u{a45d}: 'ꑝ'
    YiSyllableXit,
    /// \u{a45e}: 'ꑞ'
    YiSyllableXix,
    /// \u{a45f}: 'ꑟ'
    YiSyllableXi,
    /// \u{a460}: 'ꑠ'
    YiSyllableXip,
    /// \u{a461}: 'ꑡ'
    YiSyllableXiet,
    /// \u{a462}: 'ꑢ'
    YiSyllableXiex,
    /// \u{a463}: 'ꑣ'
    YiSyllableXie,
    /// \u{a464}: 'ꑤ'
    YiSyllableXiep,
    /// \u{a465}: 'ꑥ'
    YiSyllableXuox,
    /// \u{a466}: 'ꑦ'
    YiSyllableXuo,
    /// \u{a467}: 'ꑧ'
    YiSyllableXot,
    /// \u{a468}: 'ꑨ'
    YiSyllableXox,
    /// \u{a469}: 'ꑩ'
    YiSyllableXo,
    /// \u{a46a}: 'ꑪ'
    YiSyllableXop,
    /// \u{a46b}: 'ꑫ'
    YiSyllableXyt,
    /// \u{a46c}: 'ꑬ'
    YiSyllableXyx,
    /// \u{a46d}: 'ꑭ'
    YiSyllableXy,
    /// \u{a46e}: 'ꑮ'
    YiSyllableXyp,
    /// \u{a46f}: 'ꑯ'
    YiSyllableXyrx,
    /// \u{a470}: 'ꑰ'
    YiSyllableXyr,
    /// \u{a471}: 'ꑱ'
    YiSyllableYit,
    /// \u{a472}: 'ꑲ'
    YiSyllableYix,
    /// \u{a473}: 'ꑳ'
    YiSyllableYi,
    /// \u{a474}: 'ꑴ'
    YiSyllableYip,
    /// \u{a475}: 'ꑵ'
    YiSyllableYiet,
    /// \u{a476}: 'ꑶ'
    YiSyllableYiex,
    /// \u{a477}: 'ꑷ'
    YiSyllableYie,
    /// \u{a478}: 'ꑸ'
    YiSyllableYiep,
    /// \u{a479}: 'ꑹ'
    YiSyllableYuot,
    /// \u{a47a}: 'ꑺ'
    YiSyllableYuox,
    /// \u{a47b}: 'ꑻ'
    YiSyllableYuo,
    /// \u{a47c}: 'ꑼ'
    YiSyllableYuop,
    /// \u{a47d}: 'ꑽ'
    YiSyllableYot,
    /// \u{a47e}: 'ꑾ'
    YiSyllableYox,
    /// \u{a47f}: 'ꑿ'
    YiSyllableYo,
    /// \u{a480}: 'ꒀ'
    YiSyllableYop,
    /// \u{a481}: 'ꒁ'
    YiSyllableYut,
    /// \u{a482}: 'ꒂ'
    YiSyllableYux,
    /// \u{a483}: 'ꒃ'
    YiSyllableYu,
    /// \u{a484}: 'ꒄ'
    YiSyllableYup,
    /// \u{a485}: 'ꒅ'
    YiSyllableYurx,
    /// \u{a486}: 'ꒆ'
    YiSyllableYur,
    /// \u{a487}: 'ꒇ'
    YiSyllableYyt,
    /// \u{a488}: 'ꒈ'
    YiSyllableYyx,
    /// \u{a489}: 'ꒉ'
    YiSyllableYy,
    /// \u{a48a}: 'ꒊ'
    YiSyllableYyp,
    /// \u{a48b}: 'ꒋ'
    YiSyllableYyrx,
    /// \u{a48c}: 'ꒌ'
    YiSyllableYyr,
}

impl Into<char> for YiSyllables {
    fn into(self) -> char {
        match self {
            YiSyllables::YiSyllableIt => 'ꀀ',
            YiSyllables::YiSyllableIx => 'ꀁ',
            YiSyllables::YiSyllableI => 'ꀂ',
            YiSyllables::YiSyllableIp => 'ꀃ',
            YiSyllables::YiSyllableIet => 'ꀄ',
            YiSyllables::YiSyllableIex => 'ꀅ',
            YiSyllables::YiSyllableIe => 'ꀆ',
            YiSyllables::YiSyllableIep => 'ꀇ',
            YiSyllables::YiSyllableAt => 'ꀈ',
            YiSyllables::YiSyllableAx => 'ꀉ',
            YiSyllables::YiSyllableA => 'ꀊ',
            YiSyllables::YiSyllableAp => 'ꀋ',
            YiSyllables::YiSyllableUox => 'ꀌ',
            YiSyllables::YiSyllableUo => 'ꀍ',
            YiSyllables::YiSyllableUop => 'ꀎ',
            YiSyllables::YiSyllableOt => 'ꀏ',
            YiSyllables::YiSyllableOx => 'ꀐ',
            YiSyllables::YiSyllableO => 'ꀑ',
            YiSyllables::YiSyllableOp => 'ꀒ',
            YiSyllables::YiSyllableEx => 'ꀓ',
            YiSyllables::YiSyllableE => 'ꀔ',
            YiSyllables::YiSyllableWu => 'ꀕ',
            YiSyllables::YiSyllableBit => 'ꀖ',
            YiSyllables::YiSyllableBix => 'ꀗ',
            YiSyllables::YiSyllableBi => 'ꀘ',
            YiSyllables::YiSyllableBip => 'ꀙ',
            YiSyllables::YiSyllableBiet => 'ꀚ',
            YiSyllables::YiSyllableBiex => 'ꀛ',
            YiSyllables::YiSyllableBie => 'ꀜ',
            YiSyllables::YiSyllableBiep => 'ꀝ',
            YiSyllables::YiSyllableBat => 'ꀞ',
            YiSyllables::YiSyllableBax => 'ꀟ',
            YiSyllables::YiSyllableBa => 'ꀠ',
            YiSyllables::YiSyllableBap => 'ꀡ',
            YiSyllables::YiSyllableBuox => 'ꀢ',
            YiSyllables::YiSyllableBuo => 'ꀣ',
            YiSyllables::YiSyllableBuop => 'ꀤ',
            YiSyllables::YiSyllableBot => 'ꀥ',
            YiSyllables::YiSyllableBox => 'ꀦ',
            YiSyllables::YiSyllableBo => 'ꀧ',
            YiSyllables::YiSyllableBop => 'ꀨ',
            YiSyllables::YiSyllableBex => 'ꀩ',
            YiSyllables::YiSyllableBe => 'ꀪ',
            YiSyllables::YiSyllableBep => 'ꀫ',
            YiSyllables::YiSyllableBut => 'ꀬ',
            YiSyllables::YiSyllableBux => 'ꀭ',
            YiSyllables::YiSyllableBu => 'ꀮ',
            YiSyllables::YiSyllableBup => 'ꀯ',
            YiSyllables::YiSyllableBurx => 'ꀰ',
            YiSyllables::YiSyllableBur => 'ꀱ',
            YiSyllables::YiSyllableByt => 'ꀲ',
            YiSyllables::YiSyllableByx => 'ꀳ',
            YiSyllables::YiSyllableBy => 'ꀴ',
            YiSyllables::YiSyllableByp => 'ꀵ',
            YiSyllables::YiSyllableByrx => 'ꀶ',
            YiSyllables::YiSyllableByr => 'ꀷ',
            YiSyllables::YiSyllablePit => 'ꀸ',
            YiSyllables::YiSyllablePix => 'ꀹ',
            YiSyllables::YiSyllablePi => 'ꀺ',
            YiSyllables::YiSyllablePip => 'ꀻ',
            YiSyllables::YiSyllablePiex => 'ꀼ',
            YiSyllables::YiSyllablePie => 'ꀽ',
            YiSyllables::YiSyllablePiep => 'ꀾ',
            YiSyllables::YiSyllablePat => 'ꀿ',
            YiSyllables::YiSyllablePax => 'ꁀ',
            YiSyllables::YiSyllablePa => 'ꁁ',
            YiSyllables::YiSyllablePap => 'ꁂ',
            YiSyllables::YiSyllablePuox => 'ꁃ',
            YiSyllables::YiSyllablePuo => 'ꁄ',
            YiSyllables::YiSyllablePuop => 'ꁅ',
            YiSyllables::YiSyllablePot => 'ꁆ',
            YiSyllables::YiSyllablePox => 'ꁇ',
            YiSyllables::YiSyllablePo => 'ꁈ',
            YiSyllables::YiSyllablePop => 'ꁉ',
            YiSyllables::YiSyllablePut => 'ꁊ',
            YiSyllables::YiSyllablePux => 'ꁋ',
            YiSyllables::YiSyllablePu => 'ꁌ',
            YiSyllables::YiSyllablePup => 'ꁍ',
            YiSyllables::YiSyllablePurx => 'ꁎ',
            YiSyllables::YiSyllablePur => 'ꁏ',
            YiSyllables::YiSyllablePyt => 'ꁐ',
            YiSyllables::YiSyllablePyx => 'ꁑ',
            YiSyllables::YiSyllablePy => 'ꁒ',
            YiSyllables::YiSyllablePyp => 'ꁓ',
            YiSyllables::YiSyllablePyrx => 'ꁔ',
            YiSyllables::YiSyllablePyr => 'ꁕ',
            YiSyllables::YiSyllableBbit => 'ꁖ',
            YiSyllables::YiSyllableBbix => 'ꁗ',
            YiSyllables::YiSyllableBbi => 'ꁘ',
            YiSyllables::YiSyllableBbip => 'ꁙ',
            YiSyllables::YiSyllableBbiet => 'ꁚ',
            YiSyllables::YiSyllableBbiex => 'ꁛ',
            YiSyllables::YiSyllableBbie => 'ꁜ',
            YiSyllables::YiSyllableBbiep => 'ꁝ',
            YiSyllables::YiSyllableBbat => 'ꁞ',
            YiSyllables::YiSyllableBbax => 'ꁟ',
            YiSyllables::YiSyllableBba => 'ꁠ',
            YiSyllables::YiSyllableBbap => 'ꁡ',
            YiSyllables::YiSyllableBbuox => 'ꁢ',
            YiSyllables::YiSyllableBbuo => 'ꁣ',
            YiSyllables::YiSyllableBbuop => 'ꁤ',
            YiSyllables::YiSyllableBbot => 'ꁥ',
            YiSyllables::YiSyllableBbox => 'ꁦ',
            YiSyllables::YiSyllableBbo => 'ꁧ',
            YiSyllables::YiSyllableBbop => 'ꁨ',
            YiSyllables::YiSyllableBbex => 'ꁩ',
            YiSyllables::YiSyllableBbe => 'ꁪ',
            YiSyllables::YiSyllableBbep => 'ꁫ',
            YiSyllables::YiSyllableBbut => 'ꁬ',
            YiSyllables::YiSyllableBbux => 'ꁭ',
            YiSyllables::YiSyllableBbu => 'ꁮ',
            YiSyllables::YiSyllableBbup => 'ꁯ',
            YiSyllables::YiSyllableBburx => 'ꁰ',
            YiSyllables::YiSyllableBbur => 'ꁱ',
            YiSyllables::YiSyllableBbyt => 'ꁲ',
            YiSyllables::YiSyllableBbyx => 'ꁳ',
            YiSyllables::YiSyllableBby => 'ꁴ',
            YiSyllables::YiSyllableBbyp => 'ꁵ',
            YiSyllables::YiSyllableNbit => 'ꁶ',
            YiSyllables::YiSyllableNbix => 'ꁷ',
            YiSyllables::YiSyllableNbi => 'ꁸ',
            YiSyllables::YiSyllableNbip => 'ꁹ',
            YiSyllables::YiSyllableNbiex => 'ꁺ',
            YiSyllables::YiSyllableNbie => 'ꁻ',
            YiSyllables::YiSyllableNbiep => 'ꁼ',
            YiSyllables::YiSyllableNbat => 'ꁽ',
            YiSyllables::YiSyllableNbax => 'ꁾ',
            YiSyllables::YiSyllableNba => 'ꁿ',
            YiSyllables::YiSyllableNbap => 'ꂀ',
            YiSyllables::YiSyllableNbot => 'ꂁ',
            YiSyllables::YiSyllableNbox => 'ꂂ',
            YiSyllables::YiSyllableNbo => 'ꂃ',
            YiSyllables::YiSyllableNbop => 'ꂄ',
            YiSyllables::YiSyllableNbut => 'ꂅ',
            YiSyllables::YiSyllableNbux => 'ꂆ',
            YiSyllables::YiSyllableNbu => 'ꂇ',
            YiSyllables::YiSyllableNbup => 'ꂈ',
            YiSyllables::YiSyllableNburx => 'ꂉ',
            YiSyllables::YiSyllableNbur => 'ꂊ',
            YiSyllables::YiSyllableNbyt => 'ꂋ',
            YiSyllables::YiSyllableNbyx => 'ꂌ',
            YiSyllables::YiSyllableNby => 'ꂍ',
            YiSyllables::YiSyllableNbyp => 'ꂎ',
            YiSyllables::YiSyllableNbyrx => 'ꂏ',
            YiSyllables::YiSyllableNbyr => 'ꂐ',
            YiSyllables::YiSyllableHmit => 'ꂑ',
            YiSyllables::YiSyllableHmix => 'ꂒ',
            YiSyllables::YiSyllableHmi => 'ꂓ',
            YiSyllables::YiSyllableHmip => 'ꂔ',
            YiSyllables::YiSyllableHmiex => 'ꂕ',
            YiSyllables::YiSyllableHmie => 'ꂖ',
            YiSyllables::YiSyllableHmiep => 'ꂗ',
            YiSyllables::YiSyllableHmat => 'ꂘ',
            YiSyllables::YiSyllableHmax => 'ꂙ',
            YiSyllables::YiSyllableHma => 'ꂚ',
            YiSyllables::YiSyllableHmap => 'ꂛ',
            YiSyllables::YiSyllableHmuox => 'ꂜ',
            YiSyllables::YiSyllableHmuo => 'ꂝ',
            YiSyllables::YiSyllableHmuop => 'ꂞ',
            YiSyllables::YiSyllableHmot => 'ꂟ',
            YiSyllables::YiSyllableHmox => 'ꂠ',
            YiSyllables::YiSyllableHmo => 'ꂡ',
            YiSyllables::YiSyllableHmop => 'ꂢ',
            YiSyllables::YiSyllableHmut => 'ꂣ',
            YiSyllables::YiSyllableHmux => 'ꂤ',
            YiSyllables::YiSyllableHmu => 'ꂥ',
            YiSyllables::YiSyllableHmup => 'ꂦ',
            YiSyllables::YiSyllableHmurx => 'ꂧ',
            YiSyllables::YiSyllableHmur => 'ꂨ',
            YiSyllables::YiSyllableHmyx => 'ꂩ',
            YiSyllables::YiSyllableHmy => 'ꂪ',
            YiSyllables::YiSyllableHmyp => 'ꂫ',
            YiSyllables::YiSyllableHmyrx => 'ꂬ',
            YiSyllables::YiSyllableHmyr => 'ꂭ',
            YiSyllables::YiSyllableMit => 'ꂮ',
            YiSyllables::YiSyllableMix => 'ꂯ',
            YiSyllables::YiSyllableMi => 'ꂰ',
            YiSyllables::YiSyllableMip => 'ꂱ',
            YiSyllables::YiSyllableMiex => 'ꂲ',
            YiSyllables::YiSyllableMie => 'ꂳ',
            YiSyllables::YiSyllableMiep => 'ꂴ',
            YiSyllables::YiSyllableMat => 'ꂵ',
            YiSyllables::YiSyllableMax => 'ꂶ',
            YiSyllables::YiSyllableMa => 'ꂷ',
            YiSyllables::YiSyllableMap => 'ꂸ',
            YiSyllables::YiSyllableMuot => 'ꂹ',
            YiSyllables::YiSyllableMuox => 'ꂺ',
            YiSyllables::YiSyllableMuo => 'ꂻ',
            YiSyllables::YiSyllableMuop => 'ꂼ',
            YiSyllables::YiSyllableMot => 'ꂽ',
            YiSyllables::YiSyllableMox => 'ꂾ',
            YiSyllables::YiSyllableMo => 'ꂿ',
            YiSyllables::YiSyllableMop => 'ꃀ',
            YiSyllables::YiSyllableMex => 'ꃁ',
            YiSyllables::YiSyllableMe => 'ꃂ',
            YiSyllables::YiSyllableMut => 'ꃃ',
            YiSyllables::YiSyllableMux => 'ꃄ',
            YiSyllables::YiSyllableMu => 'ꃅ',
            YiSyllables::YiSyllableMup => 'ꃆ',
            YiSyllables::YiSyllableMurx => 'ꃇ',
            YiSyllables::YiSyllableMur => 'ꃈ',
            YiSyllables::YiSyllableMyt => 'ꃉ',
            YiSyllables::YiSyllableMyx => 'ꃊ',
            YiSyllables::YiSyllableMy => 'ꃋ',
            YiSyllables::YiSyllableMyp => 'ꃌ',
            YiSyllables::YiSyllableFit => 'ꃍ',
            YiSyllables::YiSyllableFix => 'ꃎ',
            YiSyllables::YiSyllableFi => 'ꃏ',
            YiSyllables::YiSyllableFip => 'ꃐ',
            YiSyllables::YiSyllableFat => 'ꃑ',
            YiSyllables::YiSyllableFax => 'ꃒ',
            YiSyllables::YiSyllableFa => 'ꃓ',
            YiSyllables::YiSyllableFap => 'ꃔ',
            YiSyllables::YiSyllableFox => 'ꃕ',
            YiSyllables::YiSyllableFo => 'ꃖ',
            YiSyllables::YiSyllableFop => 'ꃗ',
            YiSyllables::YiSyllableFut => 'ꃘ',
            YiSyllables::YiSyllableFux => 'ꃙ',
            YiSyllables::YiSyllableFu => 'ꃚ',
            YiSyllables::YiSyllableFup => 'ꃛ',
            YiSyllables::YiSyllableFurx => 'ꃜ',
            YiSyllables::YiSyllableFur => 'ꃝ',
            YiSyllables::YiSyllableFyt => 'ꃞ',
            YiSyllables::YiSyllableFyx => 'ꃟ',
            YiSyllables::YiSyllableFy => 'ꃠ',
            YiSyllables::YiSyllableFyp => 'ꃡ',
            YiSyllables::YiSyllableVit => 'ꃢ',
            YiSyllables::YiSyllableVix => 'ꃣ',
            YiSyllables::YiSyllableVi => 'ꃤ',
            YiSyllables::YiSyllableVip => 'ꃥ',
            YiSyllables::YiSyllableViet => 'ꃦ',
            YiSyllables::YiSyllableViex => 'ꃧ',
            YiSyllables::YiSyllableVie => 'ꃨ',
            YiSyllables::YiSyllableViep => 'ꃩ',
            YiSyllables::YiSyllableVat => 'ꃪ',
            YiSyllables::YiSyllableVax => 'ꃫ',
            YiSyllables::YiSyllableVa => 'ꃬ',
            YiSyllables::YiSyllableVap => 'ꃭ',
            YiSyllables::YiSyllableVot => 'ꃮ',
            YiSyllables::YiSyllableVox => 'ꃯ',
            YiSyllables::YiSyllableVo => 'ꃰ',
            YiSyllables::YiSyllableVop => 'ꃱ',
            YiSyllables::YiSyllableVex => 'ꃲ',
            YiSyllables::YiSyllableVep => 'ꃳ',
            YiSyllables::YiSyllableVut => 'ꃴ',
            YiSyllables::YiSyllableVux => 'ꃵ',
            YiSyllables::YiSyllableVu => 'ꃶ',
            YiSyllables::YiSyllableVup => 'ꃷ',
            YiSyllables::YiSyllableVurx => 'ꃸ',
            YiSyllables::YiSyllableVur => 'ꃹ',
            YiSyllables::YiSyllableVyt => 'ꃺ',
            YiSyllables::YiSyllableVyx => 'ꃻ',
            YiSyllables::YiSyllableVy => 'ꃼ',
            YiSyllables::YiSyllableVyp => 'ꃽ',
            YiSyllables::YiSyllableVyrx => 'ꃾ',
            YiSyllables::YiSyllableVyr => 'ꃿ',
            YiSyllables::YiSyllableDit => 'ꄀ',
            YiSyllables::YiSyllableDix => 'ꄁ',
            YiSyllables::YiSyllableDi => 'ꄂ',
            YiSyllables::YiSyllableDip => 'ꄃ',
            YiSyllables::YiSyllableDiex => 'ꄄ',
            YiSyllables::YiSyllableDie => 'ꄅ',
            YiSyllables::YiSyllableDiep => 'ꄆ',
            YiSyllables::YiSyllableDat => 'ꄇ',
            YiSyllables::YiSyllableDax => 'ꄈ',
            YiSyllables::YiSyllableDa => 'ꄉ',
            YiSyllables::YiSyllableDap => 'ꄊ',
            YiSyllables::YiSyllableDuox => 'ꄋ',
            YiSyllables::YiSyllableDuo => 'ꄌ',
            YiSyllables::YiSyllableDot => 'ꄍ',
            YiSyllables::YiSyllableDox => 'ꄎ',
            YiSyllables::YiSyllableDo => 'ꄏ',
            YiSyllables::YiSyllableDop => 'ꄐ',
            YiSyllables::YiSyllableDex => 'ꄑ',
            YiSyllables::YiSyllableDe => 'ꄒ',
            YiSyllables::YiSyllableDep => 'ꄓ',
            YiSyllables::YiSyllableDut => 'ꄔ',
            YiSyllables::YiSyllableDux => 'ꄕ',
            YiSyllables::YiSyllableDu => 'ꄖ',
            YiSyllables::YiSyllableDup => 'ꄗ',
            YiSyllables::YiSyllableDurx => 'ꄘ',
            YiSyllables::YiSyllableDur => 'ꄙ',
            YiSyllables::YiSyllableTit => 'ꄚ',
            YiSyllables::YiSyllableTix => 'ꄛ',
            YiSyllables::YiSyllableTi => 'ꄜ',
            YiSyllables::YiSyllableTip => 'ꄝ',
            YiSyllables::YiSyllableTiex => 'ꄞ',
            YiSyllables::YiSyllableTie => 'ꄟ',
            YiSyllables::YiSyllableTiep => 'ꄠ',
            YiSyllables::YiSyllableTat => 'ꄡ',
            YiSyllables::YiSyllableTax => 'ꄢ',
            YiSyllables::YiSyllableTa => 'ꄣ',
            YiSyllables::YiSyllableTap => 'ꄤ',
            YiSyllables::YiSyllableTuot => 'ꄥ',
            YiSyllables::YiSyllableTuox => 'ꄦ',
            YiSyllables::YiSyllableTuo => 'ꄧ',
            YiSyllables::YiSyllableTuop => 'ꄨ',
            YiSyllables::YiSyllableTot => 'ꄩ',
            YiSyllables::YiSyllableTox => 'ꄪ',
            YiSyllables::YiSyllableTo => 'ꄫ',
            YiSyllables::YiSyllableTop => 'ꄬ',
            YiSyllables::YiSyllableTex => 'ꄭ',
            YiSyllables::YiSyllableTe => 'ꄮ',
            YiSyllables::YiSyllableTep => 'ꄯ',
            YiSyllables::YiSyllableTut => 'ꄰ',
            YiSyllables::YiSyllableTux => 'ꄱ',
            YiSyllables::YiSyllableTu => 'ꄲ',
            YiSyllables::YiSyllableTup => 'ꄳ',
            YiSyllables::YiSyllableTurx => 'ꄴ',
            YiSyllables::YiSyllableTur => 'ꄵ',
            YiSyllables::YiSyllableDdit => 'ꄶ',
            YiSyllables::YiSyllableDdix => 'ꄷ',
            YiSyllables::YiSyllableDdi => 'ꄸ',
            YiSyllables::YiSyllableDdip => 'ꄹ',
            YiSyllables::YiSyllableDdiex => 'ꄺ',
            YiSyllables::YiSyllableDdie => 'ꄻ',
            YiSyllables::YiSyllableDdiep => 'ꄼ',
            YiSyllables::YiSyllableDdat => 'ꄽ',
            YiSyllables::YiSyllableDdax => 'ꄾ',
            YiSyllables::YiSyllableDda => 'ꄿ',
            YiSyllables::YiSyllableDdap => 'ꅀ',
            YiSyllables::YiSyllableDduox => 'ꅁ',
            YiSyllables::YiSyllableDduo => 'ꅂ',
            YiSyllables::YiSyllableDduop => 'ꅃ',
            YiSyllables::YiSyllableDdot => 'ꅄ',
            YiSyllables::YiSyllableDdox => 'ꅅ',
            YiSyllables::YiSyllableDdo => 'ꅆ',
            YiSyllables::YiSyllableDdop => 'ꅇ',
            YiSyllables::YiSyllableDdex => 'ꅈ',
            YiSyllables::YiSyllableDde => 'ꅉ',
            YiSyllables::YiSyllableDdep => 'ꅊ',
            YiSyllables::YiSyllableDdut => 'ꅋ',
            YiSyllables::YiSyllableDdux => 'ꅌ',
            YiSyllables::YiSyllableDdu => 'ꅍ',
            YiSyllables::YiSyllableDdup => 'ꅎ',
            YiSyllables::YiSyllableDdurx => 'ꅏ',
            YiSyllables::YiSyllableDdur => 'ꅐ',
            YiSyllables::YiSyllableNdit => 'ꅑ',
            YiSyllables::YiSyllableNdix => 'ꅒ',
            YiSyllables::YiSyllableNdi => 'ꅓ',
            YiSyllables::YiSyllableNdip => 'ꅔ',
            YiSyllables::YiSyllableNdiex => 'ꅕ',
            YiSyllables::YiSyllableNdie => 'ꅖ',
            YiSyllables::YiSyllableNdat => 'ꅗ',
            YiSyllables::YiSyllableNdax => 'ꅘ',
            YiSyllables::YiSyllableNda => 'ꅙ',
            YiSyllables::YiSyllableNdap => 'ꅚ',
            YiSyllables::YiSyllableNdot => 'ꅛ',
            YiSyllables::YiSyllableNdox => 'ꅜ',
            YiSyllables::YiSyllableNdo => 'ꅝ',
            YiSyllables::YiSyllableNdop => 'ꅞ',
            YiSyllables::YiSyllableNdex => 'ꅟ',
            YiSyllables::YiSyllableNde => 'ꅠ',
            YiSyllables::YiSyllableNdep => 'ꅡ',
            YiSyllables::YiSyllableNdut => 'ꅢ',
            YiSyllables::YiSyllableNdux => 'ꅣ',
            YiSyllables::YiSyllableNdu => 'ꅤ',
            YiSyllables::YiSyllableNdup => 'ꅥ',
            YiSyllables::YiSyllableNdurx => 'ꅦ',
            YiSyllables::YiSyllableNdur => 'ꅧ',
            YiSyllables::YiSyllableHnit => 'ꅨ',
            YiSyllables::YiSyllableHnix => 'ꅩ',
            YiSyllables::YiSyllableHni => 'ꅪ',
            YiSyllables::YiSyllableHnip => 'ꅫ',
            YiSyllables::YiSyllableHniet => 'ꅬ',
            YiSyllables::YiSyllableHniex => 'ꅭ',
            YiSyllables::YiSyllableHnie => 'ꅮ',
            YiSyllables::YiSyllableHniep => 'ꅯ',
            YiSyllables::YiSyllableHnat => 'ꅰ',
            YiSyllables::YiSyllableHnax => 'ꅱ',
            YiSyllables::YiSyllableHna => 'ꅲ',
            YiSyllables::YiSyllableHnap => 'ꅳ',
            YiSyllables::YiSyllableHnuox => 'ꅴ',
            YiSyllables::YiSyllableHnuo => 'ꅵ',
            YiSyllables::YiSyllableHnot => 'ꅶ',
            YiSyllables::YiSyllableHnox => 'ꅷ',
            YiSyllables::YiSyllableHnop => 'ꅸ',
            YiSyllables::YiSyllableHnex => 'ꅹ',
            YiSyllables::YiSyllableHne => 'ꅺ',
            YiSyllables::YiSyllableHnep => 'ꅻ',
            YiSyllables::YiSyllableHnut => 'ꅼ',
            YiSyllables::YiSyllableNit => 'ꅽ',
            YiSyllables::YiSyllableNix => 'ꅾ',
            YiSyllables::YiSyllableNi => 'ꅿ',
            YiSyllables::YiSyllableNip => 'ꆀ',
            YiSyllables::YiSyllableNiex => 'ꆁ',
            YiSyllables::YiSyllableNie => 'ꆂ',
            YiSyllables::YiSyllableNiep => 'ꆃ',
            YiSyllables::YiSyllableNax => 'ꆄ',
            YiSyllables::YiSyllableNa => 'ꆅ',
            YiSyllables::YiSyllableNap => 'ꆆ',
            YiSyllables::YiSyllableNuox => 'ꆇ',
            YiSyllables::YiSyllableNuo => 'ꆈ',
            YiSyllables::YiSyllableNuop => 'ꆉ',
            YiSyllables::YiSyllableNot => 'ꆊ',
            YiSyllables::YiSyllableNox => 'ꆋ',
            YiSyllables::YiSyllableNo => 'ꆌ',
            YiSyllables::YiSyllableNop => 'ꆍ',
            YiSyllables::YiSyllableNex => 'ꆎ',
            YiSyllables::YiSyllableNe => 'ꆏ',
            YiSyllables::YiSyllableNep => 'ꆐ',
            YiSyllables::YiSyllableNut => 'ꆑ',
            YiSyllables::YiSyllableNux => 'ꆒ',
            YiSyllables::YiSyllableNu => 'ꆓ',
            YiSyllables::YiSyllableNup => 'ꆔ',
            YiSyllables::YiSyllableNurx => 'ꆕ',
            YiSyllables::YiSyllableNur => 'ꆖ',
            YiSyllables::YiSyllableHlit => 'ꆗ',
            YiSyllables::YiSyllableHlix => 'ꆘ',
            YiSyllables::YiSyllableHli => 'ꆙ',
            YiSyllables::YiSyllableHlip => 'ꆚ',
            YiSyllables::YiSyllableHliex => 'ꆛ',
            YiSyllables::YiSyllableHlie => 'ꆜ',
            YiSyllables::YiSyllableHliep => 'ꆝ',
            YiSyllables::YiSyllableHlat => 'ꆞ',
            YiSyllables::YiSyllableHlax => 'ꆟ',
            YiSyllables::YiSyllableHla => 'ꆠ',
            YiSyllables::YiSyllableHlap => 'ꆡ',
            YiSyllables::YiSyllableHluox => 'ꆢ',
            YiSyllables::YiSyllableHluo => 'ꆣ',
            YiSyllables::YiSyllableHluop => 'ꆤ',
            YiSyllables::YiSyllableHlox => 'ꆥ',
            YiSyllables::YiSyllableHlo => 'ꆦ',
            YiSyllables::YiSyllableHlop => 'ꆧ',
            YiSyllables::YiSyllableHlex => 'ꆨ',
            YiSyllables::YiSyllableHle => 'ꆩ',
            YiSyllables::YiSyllableHlep => 'ꆪ',
            YiSyllables::YiSyllableHlut => 'ꆫ',
            YiSyllables::YiSyllableHlux => 'ꆬ',
            YiSyllables::YiSyllableHlu => 'ꆭ',
            YiSyllables::YiSyllableHlup => 'ꆮ',
            YiSyllables::YiSyllableHlurx => 'ꆯ',
            YiSyllables::YiSyllableHlur => 'ꆰ',
            YiSyllables::YiSyllableHlyt => 'ꆱ',
            YiSyllables::YiSyllableHlyx => 'ꆲ',
            YiSyllables::YiSyllableHly => 'ꆳ',
            YiSyllables::YiSyllableHlyp => 'ꆴ',
            YiSyllables::YiSyllableHlyrx => 'ꆵ',
            YiSyllables::YiSyllableHlyr => 'ꆶ',
            YiSyllables::YiSyllableLit => 'ꆷ',
            YiSyllables::YiSyllableLix => 'ꆸ',
            YiSyllables::YiSyllableLi => 'ꆹ',
            YiSyllables::YiSyllableLip => 'ꆺ',
            YiSyllables::YiSyllableLiet => 'ꆻ',
            YiSyllables::YiSyllableLiex => 'ꆼ',
            YiSyllables::YiSyllableLie => 'ꆽ',
            YiSyllables::YiSyllableLiep => 'ꆾ',
            YiSyllables::YiSyllableLat => 'ꆿ',
            YiSyllables::YiSyllableLax => 'ꇀ',
            YiSyllables::YiSyllableLa => 'ꇁ',
            YiSyllables::YiSyllableLap => 'ꇂ',
            YiSyllables::YiSyllableLuot => 'ꇃ',
            YiSyllables::YiSyllableLuox => 'ꇄ',
            YiSyllables::YiSyllableLuo => 'ꇅ',
            YiSyllables::YiSyllableLuop => 'ꇆ',
            YiSyllables::YiSyllableLot => 'ꇇ',
            YiSyllables::YiSyllableLox => 'ꇈ',
            YiSyllables::YiSyllableLo => 'ꇉ',
            YiSyllables::YiSyllableLop => 'ꇊ',
            YiSyllables::YiSyllableLex => 'ꇋ',
            YiSyllables::YiSyllableLe => 'ꇌ',
            YiSyllables::YiSyllableLep => 'ꇍ',
            YiSyllables::YiSyllableLut => 'ꇎ',
            YiSyllables::YiSyllableLux => 'ꇏ',
            YiSyllables::YiSyllableLu => 'ꇐ',
            YiSyllables::YiSyllableLup => 'ꇑ',
            YiSyllables::YiSyllableLurx => 'ꇒ',
            YiSyllables::YiSyllableLur => 'ꇓ',
            YiSyllables::YiSyllableLyt => 'ꇔ',
            YiSyllables::YiSyllableLyx => 'ꇕ',
            YiSyllables::YiSyllableLy => 'ꇖ',
            YiSyllables::YiSyllableLyp => 'ꇗ',
            YiSyllables::YiSyllableLyrx => 'ꇘ',
            YiSyllables::YiSyllableLyr => 'ꇙ',
            YiSyllables::YiSyllableGit => 'ꇚ',
            YiSyllables::YiSyllableGix => 'ꇛ',
            YiSyllables::YiSyllableGi => 'ꇜ',
            YiSyllables::YiSyllableGip => 'ꇝ',
            YiSyllables::YiSyllableGiet => 'ꇞ',
            YiSyllables::YiSyllableGiex => 'ꇟ',
            YiSyllables::YiSyllableGie => 'ꇠ',
            YiSyllables::YiSyllableGiep => 'ꇡ',
            YiSyllables::YiSyllableGat => 'ꇢ',
            YiSyllables::YiSyllableGax => 'ꇣ',
            YiSyllables::YiSyllableGa => 'ꇤ',
            YiSyllables::YiSyllableGap => 'ꇥ',
            YiSyllables::YiSyllableGuot => 'ꇦ',
            YiSyllables::YiSyllableGuox => 'ꇧ',
            YiSyllables::YiSyllableGuo => 'ꇨ',
            YiSyllables::YiSyllableGuop => 'ꇩ',
            YiSyllables::YiSyllableGot => 'ꇪ',
            YiSyllables::YiSyllableGox => 'ꇫ',
            YiSyllables::YiSyllableGo => 'ꇬ',
            YiSyllables::YiSyllableGop => 'ꇭ',
            YiSyllables::YiSyllableGet => 'ꇮ',
            YiSyllables::YiSyllableGex => 'ꇯ',
            YiSyllables::YiSyllableGe => 'ꇰ',
            YiSyllables::YiSyllableGep => 'ꇱ',
            YiSyllables::YiSyllableGut => 'ꇲ',
            YiSyllables::YiSyllableGux => 'ꇳ',
            YiSyllables::YiSyllableGu => 'ꇴ',
            YiSyllables::YiSyllableGup => 'ꇵ',
            YiSyllables::YiSyllableGurx => 'ꇶ',
            YiSyllables::YiSyllableGur => 'ꇷ',
            YiSyllables::YiSyllableKit => 'ꇸ',
            YiSyllables::YiSyllableKix => 'ꇹ',
            YiSyllables::YiSyllableKi => 'ꇺ',
            YiSyllables::YiSyllableKip => 'ꇻ',
            YiSyllables::YiSyllableKiex => 'ꇼ',
            YiSyllables::YiSyllableKie => 'ꇽ',
            YiSyllables::YiSyllableKiep => 'ꇾ',
            YiSyllables::YiSyllableKat => 'ꇿ',
            YiSyllables::YiSyllableKax => 'ꈀ',
            YiSyllables::YiSyllableKa => 'ꈁ',
            YiSyllables::YiSyllableKap => 'ꈂ',
            YiSyllables::YiSyllableKuox => 'ꈃ',
            YiSyllables::YiSyllableKuo => 'ꈄ',
            YiSyllables::YiSyllableKuop => 'ꈅ',
            YiSyllables::YiSyllableKot => 'ꈆ',
            YiSyllables::YiSyllableKox => 'ꈇ',
            YiSyllables::YiSyllableKo => 'ꈈ',
            YiSyllables::YiSyllableKop => 'ꈉ',
            YiSyllables::YiSyllableKet => 'ꈊ',
            YiSyllables::YiSyllableKex => 'ꈋ',
            YiSyllables::YiSyllableKe => 'ꈌ',
            YiSyllables::YiSyllableKep => 'ꈍ',
            YiSyllables::YiSyllableKut => 'ꈎ',
            YiSyllables::YiSyllableKux => 'ꈏ',
            YiSyllables::YiSyllableKu => 'ꈐ',
            YiSyllables::YiSyllableKup => 'ꈑ',
            YiSyllables::YiSyllableKurx => 'ꈒ',
            YiSyllables::YiSyllableKur => 'ꈓ',
            YiSyllables::YiSyllableGgit => 'ꈔ',
            YiSyllables::YiSyllableGgix => 'ꈕ',
            YiSyllables::YiSyllableGgi => 'ꈖ',
            YiSyllables::YiSyllableGgiex => 'ꈗ',
            YiSyllables::YiSyllableGgie => 'ꈘ',
            YiSyllables::YiSyllableGgiep => 'ꈙ',
            YiSyllables::YiSyllableGgat => 'ꈚ',
            YiSyllables::YiSyllableGgax => 'ꈛ',
            YiSyllables::YiSyllableGga => 'ꈜ',
            YiSyllables::YiSyllableGgap => 'ꈝ',
            YiSyllables::YiSyllableGguot => 'ꈞ',
            YiSyllables::YiSyllableGguox => 'ꈟ',
            YiSyllables::YiSyllableGguo => 'ꈠ',
            YiSyllables::YiSyllableGguop => 'ꈡ',
            YiSyllables::YiSyllableGgot => 'ꈢ',
            YiSyllables::YiSyllableGgox => 'ꈣ',
            YiSyllables::YiSyllableGgo => 'ꈤ',
            YiSyllables::YiSyllableGgop => 'ꈥ',
            YiSyllables::YiSyllableGget => 'ꈦ',
            YiSyllables::YiSyllableGgex => 'ꈧ',
            YiSyllables::YiSyllableGge => 'ꈨ',
            YiSyllables::YiSyllableGgep => 'ꈩ',
            YiSyllables::YiSyllableGgut => 'ꈪ',
            YiSyllables::YiSyllableGgux => 'ꈫ',
            YiSyllables::YiSyllableGgu => 'ꈬ',
            YiSyllables::YiSyllableGgup => 'ꈭ',
            YiSyllables::YiSyllableGgurx => 'ꈮ',
            YiSyllables::YiSyllableGgur => 'ꈯ',
            YiSyllables::YiSyllableMgiex => 'ꈰ',
            YiSyllables::YiSyllableMgie => 'ꈱ',
            YiSyllables::YiSyllableMgat => 'ꈲ',
            YiSyllables::YiSyllableMgax => 'ꈳ',
            YiSyllables::YiSyllableMga => 'ꈴ',
            YiSyllables::YiSyllableMgap => 'ꈵ',
            YiSyllables::YiSyllableMguox => 'ꈶ',
            YiSyllables::YiSyllableMguo => 'ꈷ',
            YiSyllables::YiSyllableMguop => 'ꈸ',
            YiSyllables::YiSyllableMgot => 'ꈹ',
            YiSyllables::YiSyllableMgox => 'ꈺ',
            YiSyllables::YiSyllableMgo => 'ꈻ',
            YiSyllables::YiSyllableMgop => 'ꈼ',
            YiSyllables::YiSyllableMgex => 'ꈽ',
            YiSyllables::YiSyllableMge => 'ꈾ',
            YiSyllables::YiSyllableMgep => 'ꈿ',
            YiSyllables::YiSyllableMgut => 'ꉀ',
            YiSyllables::YiSyllableMgux => 'ꉁ',
            YiSyllables::YiSyllableMgu => 'ꉂ',
            YiSyllables::YiSyllableMgup => 'ꉃ',
            YiSyllables::YiSyllableMgurx => 'ꉄ',
            YiSyllables::YiSyllableMgur => 'ꉅ',
            YiSyllables::YiSyllableHxit => 'ꉆ',
            YiSyllables::YiSyllableHxix => 'ꉇ',
            YiSyllables::YiSyllableHxi => 'ꉈ',
            YiSyllables::YiSyllableHxip => 'ꉉ',
            YiSyllables::YiSyllableHxiet => 'ꉊ',
            YiSyllables::YiSyllableHxiex => 'ꉋ',
            YiSyllables::YiSyllableHxie => 'ꉌ',
            YiSyllables::YiSyllableHxiep => 'ꉍ',
            YiSyllables::YiSyllableHxat => 'ꉎ',
            YiSyllables::YiSyllableHxax => 'ꉏ',
            YiSyllables::YiSyllableHxa => 'ꉐ',
            YiSyllables::YiSyllableHxap => 'ꉑ',
            YiSyllables::YiSyllableHxuot => 'ꉒ',
            YiSyllables::YiSyllableHxuox => 'ꉓ',
            YiSyllables::YiSyllableHxuo => 'ꉔ',
            YiSyllables::YiSyllableHxuop => 'ꉕ',
            YiSyllables::YiSyllableHxot => 'ꉖ',
            YiSyllables::YiSyllableHxox => 'ꉗ',
            YiSyllables::YiSyllableHxo => 'ꉘ',
            YiSyllables::YiSyllableHxop => 'ꉙ',
            YiSyllables::YiSyllableHxex => 'ꉚ',
            YiSyllables::YiSyllableHxe => 'ꉛ',
            YiSyllables::YiSyllableHxep => 'ꉜ',
            YiSyllables::YiSyllableNgiex => 'ꉝ',
            YiSyllables::YiSyllableNgie => 'ꉞ',
            YiSyllables::YiSyllableNgiep => 'ꉟ',
            YiSyllables::YiSyllableNgat => 'ꉠ',
            YiSyllables::YiSyllableNgax => 'ꉡ',
            YiSyllables::YiSyllableNga => 'ꉢ',
            YiSyllables::YiSyllableNgap => 'ꉣ',
            YiSyllables::YiSyllableNguot => 'ꉤ',
            YiSyllables::YiSyllableNguox => 'ꉥ',
            YiSyllables::YiSyllableNguo => 'ꉦ',
            YiSyllables::YiSyllableNgot => 'ꉧ',
            YiSyllables::YiSyllableNgox => 'ꉨ',
            YiSyllables::YiSyllableNgo => 'ꉩ',
            YiSyllables::YiSyllableNgop => 'ꉪ',
            YiSyllables::YiSyllableNgex => 'ꉫ',
            YiSyllables::YiSyllableNge => 'ꉬ',
            YiSyllables::YiSyllableNgep => 'ꉭ',
            YiSyllables::YiSyllableHit => 'ꉮ',
            YiSyllables::YiSyllableHiex => 'ꉯ',
            YiSyllables::YiSyllableHie => 'ꉰ',
            YiSyllables::YiSyllableHat => 'ꉱ',
            YiSyllables::YiSyllableHax => 'ꉲ',
            YiSyllables::YiSyllableHa => 'ꉳ',
            YiSyllables::YiSyllableHap => 'ꉴ',
            YiSyllables::YiSyllableHuot => 'ꉵ',
            YiSyllables::YiSyllableHuox => 'ꉶ',
            YiSyllables::YiSyllableHuo => 'ꉷ',
            YiSyllables::YiSyllableHuop => 'ꉸ',
            YiSyllables::YiSyllableHot => 'ꉹ',
            YiSyllables::YiSyllableHox => 'ꉺ',
            YiSyllables::YiSyllableHo => 'ꉻ',
            YiSyllables::YiSyllableHop => 'ꉼ',
            YiSyllables::YiSyllableHex => 'ꉽ',
            YiSyllables::YiSyllableHe => 'ꉾ',
            YiSyllables::YiSyllableHep => 'ꉿ',
            YiSyllables::YiSyllableWat => 'ꊀ',
            YiSyllables::YiSyllableWax => 'ꊁ',
            YiSyllables::YiSyllableWa => 'ꊂ',
            YiSyllables::YiSyllableWap => 'ꊃ',
            YiSyllables::YiSyllableWuox => 'ꊄ',
            YiSyllables::YiSyllableWuo => 'ꊅ',
            YiSyllables::YiSyllableWuop => 'ꊆ',
            YiSyllables::YiSyllableWox => 'ꊇ',
            YiSyllables::YiSyllableWo => 'ꊈ',
            YiSyllables::YiSyllableWop => 'ꊉ',
            YiSyllables::YiSyllableWex => 'ꊊ',
            YiSyllables::YiSyllableWe => 'ꊋ',
            YiSyllables::YiSyllableWep => 'ꊌ',
            YiSyllables::YiSyllableZit => 'ꊍ',
            YiSyllables::YiSyllableZix => 'ꊎ',
            YiSyllables::YiSyllableZi => 'ꊏ',
            YiSyllables::YiSyllableZip => 'ꊐ',
            YiSyllables::YiSyllableZiex => 'ꊑ',
            YiSyllables::YiSyllableZie => 'ꊒ',
            YiSyllables::YiSyllableZiep => 'ꊓ',
            YiSyllables::YiSyllableZat => 'ꊔ',
            YiSyllables::YiSyllableZax => 'ꊕ',
            YiSyllables::YiSyllableZa => 'ꊖ',
            YiSyllables::YiSyllableZap => 'ꊗ',
            YiSyllables::YiSyllableZuox => 'ꊘ',
            YiSyllables::YiSyllableZuo => 'ꊙ',
            YiSyllables::YiSyllableZuop => 'ꊚ',
            YiSyllables::YiSyllableZot => 'ꊛ',
            YiSyllables::YiSyllableZox => 'ꊜ',
            YiSyllables::YiSyllableZo => 'ꊝ',
            YiSyllables::YiSyllableZop => 'ꊞ',
            YiSyllables::YiSyllableZex => 'ꊟ',
            YiSyllables::YiSyllableZe => 'ꊠ',
            YiSyllables::YiSyllableZep => 'ꊡ',
            YiSyllables::YiSyllableZut => 'ꊢ',
            YiSyllables::YiSyllableZux => 'ꊣ',
            YiSyllables::YiSyllableZu => 'ꊤ',
            YiSyllables::YiSyllableZup => 'ꊥ',
            YiSyllables::YiSyllableZurx => 'ꊦ',
            YiSyllables::YiSyllableZur => 'ꊧ',
            YiSyllables::YiSyllableZyt => 'ꊨ',
            YiSyllables::YiSyllableZyx => 'ꊩ',
            YiSyllables::YiSyllableZy => 'ꊪ',
            YiSyllables::YiSyllableZyp => 'ꊫ',
            YiSyllables::YiSyllableZyrx => 'ꊬ',
            YiSyllables::YiSyllableZyr => 'ꊭ',
            YiSyllables::YiSyllableCit => 'ꊮ',
            YiSyllables::YiSyllableCix => 'ꊯ',
            YiSyllables::YiSyllableCi => 'ꊰ',
            YiSyllables::YiSyllableCip => 'ꊱ',
            YiSyllables::YiSyllableCiet => 'ꊲ',
            YiSyllables::YiSyllableCiex => 'ꊳ',
            YiSyllables::YiSyllableCie => 'ꊴ',
            YiSyllables::YiSyllableCiep => 'ꊵ',
            YiSyllables::YiSyllableCat => 'ꊶ',
            YiSyllables::YiSyllableCax => 'ꊷ',
            YiSyllables::YiSyllableCa => 'ꊸ',
            YiSyllables::YiSyllableCap => 'ꊹ',
            YiSyllables::YiSyllableCuox => 'ꊺ',
            YiSyllables::YiSyllableCuo => 'ꊻ',
            YiSyllables::YiSyllableCuop => 'ꊼ',
            YiSyllables::YiSyllableCot => 'ꊽ',
            YiSyllables::YiSyllableCox => 'ꊾ',
            YiSyllables::YiSyllableCo => 'ꊿ',
            YiSyllables::YiSyllableCop => 'ꋀ',
            YiSyllables::YiSyllableCex => 'ꋁ',
            YiSyllables::YiSyllableCe => 'ꋂ',
            YiSyllables::YiSyllableCep => 'ꋃ',
            YiSyllables::YiSyllableCut => 'ꋄ',
            YiSyllables::YiSyllableCux => 'ꋅ',
            YiSyllables::YiSyllableCu => 'ꋆ',
            YiSyllables::YiSyllableCup => 'ꋇ',
            YiSyllables::YiSyllableCurx => 'ꋈ',
            YiSyllables::YiSyllableCur => 'ꋉ',
            YiSyllables::YiSyllableCyt => 'ꋊ',
            YiSyllables::YiSyllableCyx => 'ꋋ',
            YiSyllables::YiSyllableCy => 'ꋌ',
            YiSyllables::YiSyllableCyp => 'ꋍ',
            YiSyllables::YiSyllableCyrx => 'ꋎ',
            YiSyllables::YiSyllableCyr => 'ꋏ',
            YiSyllables::YiSyllableZzit => 'ꋐ',
            YiSyllables::YiSyllableZzix => 'ꋑ',
            YiSyllables::YiSyllableZzi => 'ꋒ',
            YiSyllables::YiSyllableZzip => 'ꋓ',
            YiSyllables::YiSyllableZziet => 'ꋔ',
            YiSyllables::YiSyllableZziex => 'ꋕ',
            YiSyllables::YiSyllableZzie => 'ꋖ',
            YiSyllables::YiSyllableZziep => 'ꋗ',
            YiSyllables::YiSyllableZzat => 'ꋘ',
            YiSyllables::YiSyllableZzax => 'ꋙ',
            YiSyllables::YiSyllableZza => 'ꋚ',
            YiSyllables::YiSyllableZzap => 'ꋛ',
            YiSyllables::YiSyllableZzox => 'ꋜ',
            YiSyllables::YiSyllableZzo => 'ꋝ',
            YiSyllables::YiSyllableZzop => 'ꋞ',
            YiSyllables::YiSyllableZzex => 'ꋟ',
            YiSyllables::YiSyllableZze => 'ꋠ',
            YiSyllables::YiSyllableZzep => 'ꋡ',
            YiSyllables::YiSyllableZzux => 'ꋢ',
            YiSyllables::YiSyllableZzu => 'ꋣ',
            YiSyllables::YiSyllableZzup => 'ꋤ',
            YiSyllables::YiSyllableZzurx => 'ꋥ',
            YiSyllables::YiSyllableZzur => 'ꋦ',
            YiSyllables::YiSyllableZzyt => 'ꋧ',
            YiSyllables::YiSyllableZzyx => 'ꋨ',
            YiSyllables::YiSyllableZzy => 'ꋩ',
            YiSyllables::YiSyllableZzyp => 'ꋪ',
            YiSyllables::YiSyllableZzyrx => 'ꋫ',
            YiSyllables::YiSyllableZzyr => 'ꋬ',
            YiSyllables::YiSyllableNzit => 'ꋭ',
            YiSyllables::YiSyllableNzix => 'ꋮ',
            YiSyllables::YiSyllableNzi => 'ꋯ',
            YiSyllables::YiSyllableNzip => 'ꋰ',
            YiSyllables::YiSyllableNziex => 'ꋱ',
            YiSyllables::YiSyllableNzie => 'ꋲ',
            YiSyllables::YiSyllableNziep => 'ꋳ',
            YiSyllables::YiSyllableNzat => 'ꋴ',
            YiSyllables::YiSyllableNzax => 'ꋵ',
            YiSyllables::YiSyllableNza => 'ꋶ',
            YiSyllables::YiSyllableNzap => 'ꋷ',
            YiSyllables::YiSyllableNzuox => 'ꋸ',
            YiSyllables::YiSyllableNzuo => 'ꋹ',
            YiSyllables::YiSyllableNzox => 'ꋺ',
            YiSyllables::YiSyllableNzop => 'ꋻ',
            YiSyllables::YiSyllableNzex => 'ꋼ',
            YiSyllables::YiSyllableNze => 'ꋽ',
            YiSyllables::YiSyllableNzux => 'ꋾ',
            YiSyllables::YiSyllableNzu => 'ꋿ',
            YiSyllables::YiSyllableNzup => 'ꌀ',
            YiSyllables::YiSyllableNzurx => 'ꌁ',
            YiSyllables::YiSyllableNzur => 'ꌂ',
            YiSyllables::YiSyllableNzyt => 'ꌃ',
            YiSyllables::YiSyllableNzyx => 'ꌄ',
            YiSyllables::YiSyllableNzy => 'ꌅ',
            YiSyllables::YiSyllableNzyp => 'ꌆ',
            YiSyllables::YiSyllableNzyrx => 'ꌇ',
            YiSyllables::YiSyllableNzyr => 'ꌈ',
            YiSyllables::YiSyllableSit => 'ꌉ',
            YiSyllables::YiSyllableSix => 'ꌊ',
            YiSyllables::YiSyllableSi => 'ꌋ',
            YiSyllables::YiSyllableSip => 'ꌌ',
            YiSyllables::YiSyllableSiex => 'ꌍ',
            YiSyllables::YiSyllableSie => 'ꌎ',
            YiSyllables::YiSyllableSiep => 'ꌏ',
            YiSyllables::YiSyllableSat => 'ꌐ',
            YiSyllables::YiSyllableSax => 'ꌑ',
            YiSyllables::YiSyllableSa => 'ꌒ',
            YiSyllables::YiSyllableSap => 'ꌓ',
            YiSyllables::YiSyllableSuox => 'ꌔ',
            YiSyllables::YiSyllableSuo => 'ꌕ',
            YiSyllables::YiSyllableSuop => 'ꌖ',
            YiSyllables::YiSyllableSot => 'ꌗ',
            YiSyllables::YiSyllableSox => 'ꌘ',
            YiSyllables::YiSyllableSo => 'ꌙ',
            YiSyllables::YiSyllableSop => 'ꌚ',
            YiSyllables::YiSyllableSex => 'ꌛ',
            YiSyllables::YiSyllableSe => 'ꌜ',
            YiSyllables::YiSyllableSep => 'ꌝ',
            YiSyllables::YiSyllableSut => 'ꌞ',
            YiSyllables::YiSyllableSux => 'ꌟ',
            YiSyllables::YiSyllableSu => 'ꌠ',
            YiSyllables::YiSyllableSup => 'ꌡ',
            YiSyllables::YiSyllableSurx => 'ꌢ',
            YiSyllables::YiSyllableSur => 'ꌣ',
            YiSyllables::YiSyllableSyt => 'ꌤ',
            YiSyllables::YiSyllableSyx => 'ꌥ',
            YiSyllables::YiSyllableSy => 'ꌦ',
            YiSyllables::YiSyllableSyp => 'ꌧ',
            YiSyllables::YiSyllableSyrx => 'ꌨ',
            YiSyllables::YiSyllableSyr => 'ꌩ',
            YiSyllables::YiSyllableSsit => 'ꌪ',
            YiSyllables::YiSyllableSsix => 'ꌫ',
            YiSyllables::YiSyllableSsi => 'ꌬ',
            YiSyllables::YiSyllableSsip => 'ꌭ',
            YiSyllables::YiSyllableSsiex => 'ꌮ',
            YiSyllables::YiSyllableSsie => 'ꌯ',
            YiSyllables::YiSyllableSsiep => 'ꌰ',
            YiSyllables::YiSyllableSsat => 'ꌱ',
            YiSyllables::YiSyllableSsax => 'ꌲ',
            YiSyllables::YiSyllableSsa => 'ꌳ',
            YiSyllables::YiSyllableSsap => 'ꌴ',
            YiSyllables::YiSyllableSsot => 'ꌵ',
            YiSyllables::YiSyllableSsox => 'ꌶ',
            YiSyllables::YiSyllableSso => 'ꌷ',
            YiSyllables::YiSyllableSsop => 'ꌸ',
            YiSyllables::YiSyllableSsex => 'ꌹ',
            YiSyllables::YiSyllableSse => 'ꌺ',
            YiSyllables::YiSyllableSsep => 'ꌻ',
            YiSyllables::YiSyllableSsut => 'ꌼ',
            YiSyllables::YiSyllableSsux => 'ꌽ',
            YiSyllables::YiSyllableSsu => 'ꌾ',
            YiSyllables::YiSyllableSsup => 'ꌿ',
            YiSyllables::YiSyllableSsyt => 'ꍀ',
            YiSyllables::YiSyllableSsyx => 'ꍁ',
            YiSyllables::YiSyllableSsy => 'ꍂ',
            YiSyllables::YiSyllableSsyp => 'ꍃ',
            YiSyllables::YiSyllableSsyrx => 'ꍄ',
            YiSyllables::YiSyllableSsyr => 'ꍅ',
            YiSyllables::YiSyllableZhat => 'ꍆ',
            YiSyllables::YiSyllableZhax => 'ꍇ',
            YiSyllables::YiSyllableZha => 'ꍈ',
            YiSyllables::YiSyllableZhap => 'ꍉ',
            YiSyllables::YiSyllableZhuox => 'ꍊ',
            YiSyllables::YiSyllableZhuo => 'ꍋ',
            YiSyllables::YiSyllableZhuop => 'ꍌ',
            YiSyllables::YiSyllableZhot => 'ꍍ',
            YiSyllables::YiSyllableZhox => 'ꍎ',
            YiSyllables::YiSyllableZho => 'ꍏ',
            YiSyllables::YiSyllableZhop => 'ꍐ',
            YiSyllables::YiSyllableZhet => 'ꍑ',
            YiSyllables::YiSyllableZhex => 'ꍒ',
            YiSyllables::YiSyllableZhe => 'ꍓ',
            YiSyllables::YiSyllableZhep => 'ꍔ',
            YiSyllables::YiSyllableZhut => 'ꍕ',
            YiSyllables::YiSyllableZhux => 'ꍖ',
            YiSyllables::YiSyllableZhu => 'ꍗ',
            YiSyllables::YiSyllableZhup => 'ꍘ',
            YiSyllables::YiSyllableZhurx => 'ꍙ',
            YiSyllables::YiSyllableZhur => 'ꍚ',
            YiSyllables::YiSyllableZhyt => 'ꍛ',
            YiSyllables::YiSyllableZhyx => 'ꍜ',
            YiSyllables::YiSyllableZhy => 'ꍝ',
            YiSyllables::YiSyllableZhyp => 'ꍞ',
            YiSyllables::YiSyllableZhyrx => 'ꍟ',
            YiSyllables::YiSyllableZhyr => 'ꍠ',
            YiSyllables::YiSyllableChat => 'ꍡ',
            YiSyllables::YiSyllableChax => 'ꍢ',
            YiSyllables::YiSyllableCha => 'ꍣ',
            YiSyllables::YiSyllableChap => 'ꍤ',
            YiSyllables::YiSyllableChuot => 'ꍥ',
            YiSyllables::YiSyllableChuox => 'ꍦ',
            YiSyllables::YiSyllableChuo => 'ꍧ',
            YiSyllables::YiSyllableChuop => 'ꍨ',
            YiSyllables::YiSyllableChot => 'ꍩ',
            YiSyllables::YiSyllableChox => 'ꍪ',
            YiSyllables::YiSyllableCho => 'ꍫ',
            YiSyllables::YiSyllableChop => 'ꍬ',
            YiSyllables::YiSyllableChet => 'ꍭ',
            YiSyllables::YiSyllableChex => 'ꍮ',
            YiSyllables::YiSyllableChe => 'ꍯ',
            YiSyllables::YiSyllableChep => 'ꍰ',
            YiSyllables::YiSyllableChux => 'ꍱ',
            YiSyllables::YiSyllableChu => 'ꍲ',
            YiSyllables::YiSyllableChup => 'ꍳ',
            YiSyllables::YiSyllableChurx => 'ꍴ',
            YiSyllables::YiSyllableChur => 'ꍵ',
            YiSyllables::YiSyllableChyt => 'ꍶ',
            YiSyllables::YiSyllableChyx => 'ꍷ',
            YiSyllables::YiSyllableChy => 'ꍸ',
            YiSyllables::YiSyllableChyp => 'ꍹ',
            YiSyllables::YiSyllableChyrx => 'ꍺ',
            YiSyllables::YiSyllableChyr => 'ꍻ',
            YiSyllables::YiSyllableRrax => 'ꍼ',
            YiSyllables::YiSyllableRra => 'ꍽ',
            YiSyllables::YiSyllableRruox => 'ꍾ',
            YiSyllables::YiSyllableRruo => 'ꍿ',
            YiSyllables::YiSyllableRrot => 'ꎀ',
            YiSyllables::YiSyllableRrox => 'ꎁ',
            YiSyllables::YiSyllableRro => 'ꎂ',
            YiSyllables::YiSyllableRrop => 'ꎃ',
            YiSyllables::YiSyllableRret => 'ꎄ',
            YiSyllables::YiSyllableRrex => 'ꎅ',
            YiSyllables::YiSyllableRre => 'ꎆ',
            YiSyllables::YiSyllableRrep => 'ꎇ',
            YiSyllables::YiSyllableRrut => 'ꎈ',
            YiSyllables::YiSyllableRrux => 'ꎉ',
            YiSyllables::YiSyllableRru => 'ꎊ',
            YiSyllables::YiSyllableRrup => 'ꎋ',
            YiSyllables::YiSyllableRrurx => 'ꎌ',
            YiSyllables::YiSyllableRrur => 'ꎍ',
            YiSyllables::YiSyllableRryt => 'ꎎ',
            YiSyllables::YiSyllableRryx => 'ꎏ',
            YiSyllables::YiSyllableRry => 'ꎐ',
            YiSyllables::YiSyllableRryp => 'ꎑ',
            YiSyllables::YiSyllableRryrx => 'ꎒ',
            YiSyllables::YiSyllableRryr => 'ꎓ',
            YiSyllables::YiSyllableNrat => 'ꎔ',
            YiSyllables::YiSyllableNrax => 'ꎕ',
            YiSyllables::YiSyllableNra => 'ꎖ',
            YiSyllables::YiSyllableNrap => 'ꎗ',
            YiSyllables::YiSyllableNrox => 'ꎘ',
            YiSyllables::YiSyllableNro => 'ꎙ',
            YiSyllables::YiSyllableNrop => 'ꎚ',
            YiSyllables::YiSyllableNret => 'ꎛ',
            YiSyllables::YiSyllableNrex => 'ꎜ',
            YiSyllables::YiSyllableNre => 'ꎝ',
            YiSyllables::YiSyllableNrep => 'ꎞ',
            YiSyllables::YiSyllableNrut => 'ꎟ',
            YiSyllables::YiSyllableNrux => 'ꎠ',
            YiSyllables::YiSyllableNru => 'ꎡ',
            YiSyllables::YiSyllableNrup => 'ꎢ',
            YiSyllables::YiSyllableNrurx => 'ꎣ',
            YiSyllables::YiSyllableNrur => 'ꎤ',
            YiSyllables::YiSyllableNryt => 'ꎥ',
            YiSyllables::YiSyllableNryx => 'ꎦ',
            YiSyllables::YiSyllableNry => 'ꎧ',
            YiSyllables::YiSyllableNryp => 'ꎨ',
            YiSyllables::YiSyllableNryrx => 'ꎩ',
            YiSyllables::YiSyllableNryr => 'ꎪ',
            YiSyllables::YiSyllableShat => 'ꎫ',
            YiSyllables::YiSyllableShax => 'ꎬ',
            YiSyllables::YiSyllableSha => 'ꎭ',
            YiSyllables::YiSyllableShap => 'ꎮ',
            YiSyllables::YiSyllableShuox => 'ꎯ',
            YiSyllables::YiSyllableShuo => 'ꎰ',
            YiSyllables::YiSyllableShuop => 'ꎱ',
            YiSyllables::YiSyllableShot => 'ꎲ',
            YiSyllables::YiSyllableShox => 'ꎳ',
            YiSyllables::YiSyllableSho => 'ꎴ',
            YiSyllables::YiSyllableShop => 'ꎵ',
            YiSyllables::YiSyllableShet => 'ꎶ',
            YiSyllables::YiSyllableShex => 'ꎷ',
            YiSyllables::YiSyllableShe => 'ꎸ',
            YiSyllables::YiSyllableShep => 'ꎹ',
            YiSyllables::YiSyllableShut => 'ꎺ',
            YiSyllables::YiSyllableShux => 'ꎻ',
            YiSyllables::YiSyllableShu => 'ꎼ',
            YiSyllables::YiSyllableShup => 'ꎽ',
            YiSyllables::YiSyllableShurx => 'ꎾ',
            YiSyllables::YiSyllableShur => 'ꎿ',
            YiSyllables::YiSyllableShyt => 'ꏀ',
            YiSyllables::YiSyllableShyx => 'ꏁ',
            YiSyllables::YiSyllableShy => 'ꏂ',
            YiSyllables::YiSyllableShyp => 'ꏃ',
            YiSyllables::YiSyllableShyrx => 'ꏄ',
            YiSyllables::YiSyllableShyr => 'ꏅ',
            YiSyllables::YiSyllableRat => 'ꏆ',
            YiSyllables::YiSyllableRax => 'ꏇ',
            YiSyllables::YiSyllableRa => 'ꏈ',
            YiSyllables::YiSyllableRap => 'ꏉ',
            YiSyllables::YiSyllableRuox => 'ꏊ',
            YiSyllables::YiSyllableRuo => 'ꏋ',
            YiSyllables::YiSyllableRuop => 'ꏌ',
            YiSyllables::YiSyllableRot => 'ꏍ',
            YiSyllables::YiSyllableRox => 'ꏎ',
            YiSyllables::YiSyllableRo => 'ꏏ',
            YiSyllables::YiSyllableRop => 'ꏐ',
            YiSyllables::YiSyllableRex => 'ꏑ',
            YiSyllables::YiSyllableRe => 'ꏒ',
            YiSyllables::YiSyllableRep => 'ꏓ',
            YiSyllables::YiSyllableRut => 'ꏔ',
            YiSyllables::YiSyllableRux => 'ꏕ',
            YiSyllables::YiSyllableRu => 'ꏖ',
            YiSyllables::YiSyllableRup => 'ꏗ',
            YiSyllables::YiSyllableRurx => 'ꏘ',
            YiSyllables::YiSyllableRur => 'ꏙ',
            YiSyllables::YiSyllableRyt => 'ꏚ',
            YiSyllables::YiSyllableRyx => 'ꏛ',
            YiSyllables::YiSyllableRy => 'ꏜ',
            YiSyllables::YiSyllableRyp => 'ꏝ',
            YiSyllables::YiSyllableRyrx => 'ꏞ',
            YiSyllables::YiSyllableRyr => 'ꏟ',
            YiSyllables::YiSyllableJit => 'ꏠ',
            YiSyllables::YiSyllableJix => 'ꏡ',
            YiSyllables::YiSyllableJi => 'ꏢ',
            YiSyllables::YiSyllableJip => 'ꏣ',
            YiSyllables::YiSyllableJiet => 'ꏤ',
            YiSyllables::YiSyllableJiex => 'ꏥ',
            YiSyllables::YiSyllableJie => 'ꏦ',
            YiSyllables::YiSyllableJiep => 'ꏧ',
            YiSyllables::YiSyllableJuot => 'ꏨ',
            YiSyllables::YiSyllableJuox => 'ꏩ',
            YiSyllables::YiSyllableJuo => 'ꏪ',
            YiSyllables::YiSyllableJuop => 'ꏫ',
            YiSyllables::YiSyllableJot => 'ꏬ',
            YiSyllables::YiSyllableJox => 'ꏭ',
            YiSyllables::YiSyllableJo => 'ꏮ',
            YiSyllables::YiSyllableJop => 'ꏯ',
            YiSyllables::YiSyllableJut => 'ꏰ',
            YiSyllables::YiSyllableJux => 'ꏱ',
            YiSyllables::YiSyllableJu => 'ꏲ',
            YiSyllables::YiSyllableJup => 'ꏳ',
            YiSyllables::YiSyllableJurx => 'ꏴ',
            YiSyllables::YiSyllableJur => 'ꏵ',
            YiSyllables::YiSyllableJyt => 'ꏶ',
            YiSyllables::YiSyllableJyx => 'ꏷ',
            YiSyllables::YiSyllableJy => 'ꏸ',
            YiSyllables::YiSyllableJyp => 'ꏹ',
            YiSyllables::YiSyllableJyrx => 'ꏺ',
            YiSyllables::YiSyllableJyr => 'ꏻ',
            YiSyllables::YiSyllableQit => 'ꏼ',
            YiSyllables::YiSyllableQix => 'ꏽ',
            YiSyllables::YiSyllableQi => 'ꏾ',
            YiSyllables::YiSyllableQip => 'ꏿ',
            YiSyllables::YiSyllableQiet => 'ꐀ',
            YiSyllables::YiSyllableQiex => 'ꐁ',
            YiSyllables::YiSyllableQie => 'ꐂ',
            YiSyllables::YiSyllableQiep => 'ꐃ',
            YiSyllables::YiSyllableQuot => 'ꐄ',
            YiSyllables::YiSyllableQuox => 'ꐅ',
            YiSyllables::YiSyllableQuo => 'ꐆ',
            YiSyllables::YiSyllableQuop => 'ꐇ',
            YiSyllables::YiSyllableQot => 'ꐈ',
            YiSyllables::YiSyllableQox => 'ꐉ',
            YiSyllables::YiSyllableQo => 'ꐊ',
            YiSyllables::YiSyllableQop => 'ꐋ',
            YiSyllables::YiSyllableQut => 'ꐌ',
            YiSyllables::YiSyllableQux => 'ꐍ',
            YiSyllables::YiSyllableQu => 'ꐎ',
            YiSyllables::YiSyllableQup => 'ꐏ',
            YiSyllables::YiSyllableQurx => 'ꐐ',
            YiSyllables::YiSyllableQur => 'ꐑ',
            YiSyllables::YiSyllableQyt => 'ꐒ',
            YiSyllables::YiSyllableQyx => 'ꐓ',
            YiSyllables::YiSyllableQy => 'ꐔ',
            YiSyllables::YiSyllableQyp => 'ꐕ',
            YiSyllables::YiSyllableQyrx => 'ꐖ',
            YiSyllables::YiSyllableQyr => 'ꐗ',
            YiSyllables::YiSyllableJjit => 'ꐘ',
            YiSyllables::YiSyllableJjix => 'ꐙ',
            YiSyllables::YiSyllableJji => 'ꐚ',
            YiSyllables::YiSyllableJjip => 'ꐛ',
            YiSyllables::YiSyllableJjiet => 'ꐜ',
            YiSyllables::YiSyllableJjiex => 'ꐝ',
            YiSyllables::YiSyllableJjie => 'ꐞ',
            YiSyllables::YiSyllableJjiep => 'ꐟ',
            YiSyllables::YiSyllableJjuox => 'ꐠ',
            YiSyllables::YiSyllableJjuo => 'ꐡ',
            YiSyllables::YiSyllableJjuop => 'ꐢ',
            YiSyllables::YiSyllableJjot => 'ꐣ',
            YiSyllables::YiSyllableJjox => 'ꐤ',
            YiSyllables::YiSyllableJjo => 'ꐥ',
            YiSyllables::YiSyllableJjop => 'ꐦ',
            YiSyllables::YiSyllableJjut => 'ꐧ',
            YiSyllables::YiSyllableJjux => 'ꐨ',
            YiSyllables::YiSyllableJju => 'ꐩ',
            YiSyllables::YiSyllableJjup => 'ꐪ',
            YiSyllables::YiSyllableJjurx => 'ꐫ',
            YiSyllables::YiSyllableJjur => 'ꐬ',
            YiSyllables::YiSyllableJjyt => 'ꐭ',
            YiSyllables::YiSyllableJjyx => 'ꐮ',
            YiSyllables::YiSyllableJjy => 'ꐯ',
            YiSyllables::YiSyllableJjyp => 'ꐰ',
            YiSyllables::YiSyllableNjit => 'ꐱ',
            YiSyllables::YiSyllableNjix => 'ꐲ',
            YiSyllables::YiSyllableNji => 'ꐳ',
            YiSyllables::YiSyllableNjip => 'ꐴ',
            YiSyllables::YiSyllableNjiet => 'ꐵ',
            YiSyllables::YiSyllableNjiex => 'ꐶ',
            YiSyllables::YiSyllableNjie => 'ꐷ',
            YiSyllables::YiSyllableNjiep => 'ꐸ',
            YiSyllables::YiSyllableNjuox => 'ꐹ',
            YiSyllables::YiSyllableNjuo => 'ꐺ',
            YiSyllables::YiSyllableNjot => 'ꐻ',
            YiSyllables::YiSyllableNjox => 'ꐼ',
            YiSyllables::YiSyllableNjo => 'ꐽ',
            YiSyllables::YiSyllableNjop => 'ꐾ',
            YiSyllables::YiSyllableNjux => 'ꐿ',
            YiSyllables::YiSyllableNju => 'ꑀ',
            YiSyllables::YiSyllableNjup => 'ꑁ',
            YiSyllables::YiSyllableNjurx => 'ꑂ',
            YiSyllables::YiSyllableNjur => 'ꑃ',
            YiSyllables::YiSyllableNjyt => 'ꑄ',
            YiSyllables::YiSyllableNjyx => 'ꑅ',
            YiSyllables::YiSyllableNjy => 'ꑆ',
            YiSyllables::YiSyllableNjyp => 'ꑇ',
            YiSyllables::YiSyllableNjyrx => 'ꑈ',
            YiSyllables::YiSyllableNjyr => 'ꑉ',
            YiSyllables::YiSyllableNyit => 'ꑊ',
            YiSyllables::YiSyllableNyix => 'ꑋ',
            YiSyllables::YiSyllableNyi => 'ꑌ',
            YiSyllables::YiSyllableNyip => 'ꑍ',
            YiSyllables::YiSyllableNyiet => 'ꑎ',
            YiSyllables::YiSyllableNyiex => 'ꑏ',
            YiSyllables::YiSyllableNyie => 'ꑐ',
            YiSyllables::YiSyllableNyiep => 'ꑑ',
            YiSyllables::YiSyllableNyuox => 'ꑒ',
            YiSyllables::YiSyllableNyuo => 'ꑓ',
            YiSyllables::YiSyllableNyuop => 'ꑔ',
            YiSyllables::YiSyllableNyot => 'ꑕ',
            YiSyllables::YiSyllableNyox => 'ꑖ',
            YiSyllables::YiSyllableNyo => 'ꑗ',
            YiSyllables::YiSyllableNyop => 'ꑘ',
            YiSyllables::YiSyllableNyut => 'ꑙ',
            YiSyllables::YiSyllableNyux => 'ꑚ',
            YiSyllables::YiSyllableNyu => 'ꑛ',
            YiSyllables::YiSyllableNyup => 'ꑜ',
            YiSyllables::YiSyllableXit => 'ꑝ',
            YiSyllables::YiSyllableXix => 'ꑞ',
            YiSyllables::YiSyllableXi => 'ꑟ',
            YiSyllables::YiSyllableXip => 'ꑠ',
            YiSyllables::YiSyllableXiet => 'ꑡ',
            YiSyllables::YiSyllableXiex => 'ꑢ',
            YiSyllables::YiSyllableXie => 'ꑣ',
            YiSyllables::YiSyllableXiep => 'ꑤ',
            YiSyllables::YiSyllableXuox => 'ꑥ',
            YiSyllables::YiSyllableXuo => 'ꑦ',
            YiSyllables::YiSyllableXot => 'ꑧ',
            YiSyllables::YiSyllableXox => 'ꑨ',
            YiSyllables::YiSyllableXo => 'ꑩ',
            YiSyllables::YiSyllableXop => 'ꑪ',
            YiSyllables::YiSyllableXyt => 'ꑫ',
            YiSyllables::YiSyllableXyx => 'ꑬ',
            YiSyllables::YiSyllableXy => 'ꑭ',
            YiSyllables::YiSyllableXyp => 'ꑮ',
            YiSyllables::YiSyllableXyrx => 'ꑯ',
            YiSyllables::YiSyllableXyr => 'ꑰ',
            YiSyllables::YiSyllableYit => 'ꑱ',
            YiSyllables::YiSyllableYix => 'ꑲ',
            YiSyllables::YiSyllableYi => 'ꑳ',
            YiSyllables::YiSyllableYip => 'ꑴ',
            YiSyllables::YiSyllableYiet => 'ꑵ',
            YiSyllables::YiSyllableYiex => 'ꑶ',
            YiSyllables::YiSyllableYie => 'ꑷ',
            YiSyllables::YiSyllableYiep => 'ꑸ',
            YiSyllables::YiSyllableYuot => 'ꑹ',
            YiSyllables::YiSyllableYuox => 'ꑺ',
            YiSyllables::YiSyllableYuo => 'ꑻ',
            YiSyllables::YiSyllableYuop => 'ꑼ',
            YiSyllables::YiSyllableYot => 'ꑽ',
            YiSyllables::YiSyllableYox => 'ꑾ',
            YiSyllables::YiSyllableYo => 'ꑿ',
            YiSyllables::YiSyllableYop => 'ꒀ',
            YiSyllables::YiSyllableYut => 'ꒁ',
            YiSyllables::YiSyllableYux => 'ꒂ',
            YiSyllables::YiSyllableYu => 'ꒃ',
            YiSyllables::YiSyllableYup => 'ꒄ',
            YiSyllables::YiSyllableYurx => 'ꒅ',
            YiSyllables::YiSyllableYur => 'ꒆ',
            YiSyllables::YiSyllableYyt => 'ꒇ',
            YiSyllables::YiSyllableYyx => 'ꒈ',
            YiSyllables::YiSyllableYy => 'ꒉ',
            YiSyllables::YiSyllableYyp => 'ꒊ',
            YiSyllables::YiSyllableYyrx => 'ꒋ',
            YiSyllables::YiSyllableYyr => 'ꒌ',
        }
    }
}

impl std::convert::TryFrom<char> for YiSyllables {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꀀ' => Ok(YiSyllables::YiSyllableIt),
            'ꀁ' => Ok(YiSyllables::YiSyllableIx),
            'ꀂ' => Ok(YiSyllables::YiSyllableI),
            'ꀃ' => Ok(YiSyllables::YiSyllableIp),
            'ꀄ' => Ok(YiSyllables::YiSyllableIet),
            'ꀅ' => Ok(YiSyllables::YiSyllableIex),
            'ꀆ' => Ok(YiSyllables::YiSyllableIe),
            'ꀇ' => Ok(YiSyllables::YiSyllableIep),
            'ꀈ' => Ok(YiSyllables::YiSyllableAt),
            'ꀉ' => Ok(YiSyllables::YiSyllableAx),
            'ꀊ' => Ok(YiSyllables::YiSyllableA),
            'ꀋ' => Ok(YiSyllables::YiSyllableAp),
            'ꀌ' => Ok(YiSyllables::YiSyllableUox),
            'ꀍ' => Ok(YiSyllables::YiSyllableUo),
            'ꀎ' => Ok(YiSyllables::YiSyllableUop),
            'ꀏ' => Ok(YiSyllables::YiSyllableOt),
            'ꀐ' => Ok(YiSyllables::YiSyllableOx),
            'ꀑ' => Ok(YiSyllables::YiSyllableO),
            'ꀒ' => Ok(YiSyllables::YiSyllableOp),
            'ꀓ' => Ok(YiSyllables::YiSyllableEx),
            'ꀔ' => Ok(YiSyllables::YiSyllableE),
            'ꀕ' => Ok(YiSyllables::YiSyllableWu),
            'ꀖ' => Ok(YiSyllables::YiSyllableBit),
            'ꀗ' => Ok(YiSyllables::YiSyllableBix),
            'ꀘ' => Ok(YiSyllables::YiSyllableBi),
            'ꀙ' => Ok(YiSyllables::YiSyllableBip),
            'ꀚ' => Ok(YiSyllables::YiSyllableBiet),
            'ꀛ' => Ok(YiSyllables::YiSyllableBiex),
            'ꀜ' => Ok(YiSyllables::YiSyllableBie),
            'ꀝ' => Ok(YiSyllables::YiSyllableBiep),
            'ꀞ' => Ok(YiSyllables::YiSyllableBat),
            'ꀟ' => Ok(YiSyllables::YiSyllableBax),
            'ꀠ' => Ok(YiSyllables::YiSyllableBa),
            'ꀡ' => Ok(YiSyllables::YiSyllableBap),
            'ꀢ' => Ok(YiSyllables::YiSyllableBuox),
            'ꀣ' => Ok(YiSyllables::YiSyllableBuo),
            'ꀤ' => Ok(YiSyllables::YiSyllableBuop),
            'ꀥ' => Ok(YiSyllables::YiSyllableBot),
            'ꀦ' => Ok(YiSyllables::YiSyllableBox),
            'ꀧ' => Ok(YiSyllables::YiSyllableBo),
            'ꀨ' => Ok(YiSyllables::YiSyllableBop),
            'ꀩ' => Ok(YiSyllables::YiSyllableBex),
            'ꀪ' => Ok(YiSyllables::YiSyllableBe),
            'ꀫ' => Ok(YiSyllables::YiSyllableBep),
            'ꀬ' => Ok(YiSyllables::YiSyllableBut),
            'ꀭ' => Ok(YiSyllables::YiSyllableBux),
            'ꀮ' => Ok(YiSyllables::YiSyllableBu),
            'ꀯ' => Ok(YiSyllables::YiSyllableBup),
            'ꀰ' => Ok(YiSyllables::YiSyllableBurx),
            'ꀱ' => Ok(YiSyllables::YiSyllableBur),
            'ꀲ' => Ok(YiSyllables::YiSyllableByt),
            'ꀳ' => Ok(YiSyllables::YiSyllableByx),
            'ꀴ' => Ok(YiSyllables::YiSyllableBy),
            'ꀵ' => Ok(YiSyllables::YiSyllableByp),
            'ꀶ' => Ok(YiSyllables::YiSyllableByrx),
            'ꀷ' => Ok(YiSyllables::YiSyllableByr),
            'ꀸ' => Ok(YiSyllables::YiSyllablePit),
            'ꀹ' => Ok(YiSyllables::YiSyllablePix),
            'ꀺ' => Ok(YiSyllables::YiSyllablePi),
            'ꀻ' => Ok(YiSyllables::YiSyllablePip),
            'ꀼ' => Ok(YiSyllables::YiSyllablePiex),
            'ꀽ' => Ok(YiSyllables::YiSyllablePie),
            'ꀾ' => Ok(YiSyllables::YiSyllablePiep),
            'ꀿ' => Ok(YiSyllables::YiSyllablePat),
            'ꁀ' => Ok(YiSyllables::YiSyllablePax),
            'ꁁ' => Ok(YiSyllables::YiSyllablePa),
            'ꁂ' => Ok(YiSyllables::YiSyllablePap),
            'ꁃ' => Ok(YiSyllables::YiSyllablePuox),
            'ꁄ' => Ok(YiSyllables::YiSyllablePuo),
            'ꁅ' => Ok(YiSyllables::YiSyllablePuop),
            'ꁆ' => Ok(YiSyllables::YiSyllablePot),
            'ꁇ' => Ok(YiSyllables::YiSyllablePox),
            'ꁈ' => Ok(YiSyllables::YiSyllablePo),
            'ꁉ' => Ok(YiSyllables::YiSyllablePop),
            'ꁊ' => Ok(YiSyllables::YiSyllablePut),
            'ꁋ' => Ok(YiSyllables::YiSyllablePux),
            'ꁌ' => Ok(YiSyllables::YiSyllablePu),
            'ꁍ' => Ok(YiSyllables::YiSyllablePup),
            'ꁎ' => Ok(YiSyllables::YiSyllablePurx),
            'ꁏ' => Ok(YiSyllables::YiSyllablePur),
            'ꁐ' => Ok(YiSyllables::YiSyllablePyt),
            'ꁑ' => Ok(YiSyllables::YiSyllablePyx),
            'ꁒ' => Ok(YiSyllables::YiSyllablePy),
            'ꁓ' => Ok(YiSyllables::YiSyllablePyp),
            'ꁔ' => Ok(YiSyllables::YiSyllablePyrx),
            'ꁕ' => Ok(YiSyllables::YiSyllablePyr),
            'ꁖ' => Ok(YiSyllables::YiSyllableBbit),
            'ꁗ' => Ok(YiSyllables::YiSyllableBbix),
            'ꁘ' => Ok(YiSyllables::YiSyllableBbi),
            'ꁙ' => Ok(YiSyllables::YiSyllableBbip),
            'ꁚ' => Ok(YiSyllables::YiSyllableBbiet),
            'ꁛ' => Ok(YiSyllables::YiSyllableBbiex),
            'ꁜ' => Ok(YiSyllables::YiSyllableBbie),
            'ꁝ' => Ok(YiSyllables::YiSyllableBbiep),
            'ꁞ' => Ok(YiSyllables::YiSyllableBbat),
            'ꁟ' => Ok(YiSyllables::YiSyllableBbax),
            'ꁠ' => Ok(YiSyllables::YiSyllableBba),
            'ꁡ' => Ok(YiSyllables::YiSyllableBbap),
            'ꁢ' => Ok(YiSyllables::YiSyllableBbuox),
            'ꁣ' => Ok(YiSyllables::YiSyllableBbuo),
            'ꁤ' => Ok(YiSyllables::YiSyllableBbuop),
            'ꁥ' => Ok(YiSyllables::YiSyllableBbot),
            'ꁦ' => Ok(YiSyllables::YiSyllableBbox),
            'ꁧ' => Ok(YiSyllables::YiSyllableBbo),
            'ꁨ' => Ok(YiSyllables::YiSyllableBbop),
            'ꁩ' => Ok(YiSyllables::YiSyllableBbex),
            'ꁪ' => Ok(YiSyllables::YiSyllableBbe),
            'ꁫ' => Ok(YiSyllables::YiSyllableBbep),
            'ꁬ' => Ok(YiSyllables::YiSyllableBbut),
            'ꁭ' => Ok(YiSyllables::YiSyllableBbux),
            'ꁮ' => Ok(YiSyllables::YiSyllableBbu),
            'ꁯ' => Ok(YiSyllables::YiSyllableBbup),
            'ꁰ' => Ok(YiSyllables::YiSyllableBburx),
            'ꁱ' => Ok(YiSyllables::YiSyllableBbur),
            'ꁲ' => Ok(YiSyllables::YiSyllableBbyt),
            'ꁳ' => Ok(YiSyllables::YiSyllableBbyx),
            'ꁴ' => Ok(YiSyllables::YiSyllableBby),
            'ꁵ' => Ok(YiSyllables::YiSyllableBbyp),
            'ꁶ' => Ok(YiSyllables::YiSyllableNbit),
            'ꁷ' => Ok(YiSyllables::YiSyllableNbix),
            'ꁸ' => Ok(YiSyllables::YiSyllableNbi),
            'ꁹ' => Ok(YiSyllables::YiSyllableNbip),
            'ꁺ' => Ok(YiSyllables::YiSyllableNbiex),
            'ꁻ' => Ok(YiSyllables::YiSyllableNbie),
            'ꁼ' => Ok(YiSyllables::YiSyllableNbiep),
            'ꁽ' => Ok(YiSyllables::YiSyllableNbat),
            'ꁾ' => Ok(YiSyllables::YiSyllableNbax),
            'ꁿ' => Ok(YiSyllables::YiSyllableNba),
            'ꂀ' => Ok(YiSyllables::YiSyllableNbap),
            'ꂁ' => Ok(YiSyllables::YiSyllableNbot),
            'ꂂ' => Ok(YiSyllables::YiSyllableNbox),
            'ꂃ' => Ok(YiSyllables::YiSyllableNbo),
            'ꂄ' => Ok(YiSyllables::YiSyllableNbop),
            'ꂅ' => Ok(YiSyllables::YiSyllableNbut),
            'ꂆ' => Ok(YiSyllables::YiSyllableNbux),
            'ꂇ' => Ok(YiSyllables::YiSyllableNbu),
            'ꂈ' => Ok(YiSyllables::YiSyllableNbup),
            'ꂉ' => Ok(YiSyllables::YiSyllableNburx),
            'ꂊ' => Ok(YiSyllables::YiSyllableNbur),
            'ꂋ' => Ok(YiSyllables::YiSyllableNbyt),
            'ꂌ' => Ok(YiSyllables::YiSyllableNbyx),
            'ꂍ' => Ok(YiSyllables::YiSyllableNby),
            'ꂎ' => Ok(YiSyllables::YiSyllableNbyp),
            'ꂏ' => Ok(YiSyllables::YiSyllableNbyrx),
            'ꂐ' => Ok(YiSyllables::YiSyllableNbyr),
            'ꂑ' => Ok(YiSyllables::YiSyllableHmit),
            'ꂒ' => Ok(YiSyllables::YiSyllableHmix),
            'ꂓ' => Ok(YiSyllables::YiSyllableHmi),
            'ꂔ' => Ok(YiSyllables::YiSyllableHmip),
            'ꂕ' => Ok(YiSyllables::YiSyllableHmiex),
            'ꂖ' => Ok(YiSyllables::YiSyllableHmie),
            'ꂗ' => Ok(YiSyllables::YiSyllableHmiep),
            'ꂘ' => Ok(YiSyllables::YiSyllableHmat),
            'ꂙ' => Ok(YiSyllables::YiSyllableHmax),
            'ꂚ' => Ok(YiSyllables::YiSyllableHma),
            'ꂛ' => Ok(YiSyllables::YiSyllableHmap),
            'ꂜ' => Ok(YiSyllables::YiSyllableHmuox),
            'ꂝ' => Ok(YiSyllables::YiSyllableHmuo),
            'ꂞ' => Ok(YiSyllables::YiSyllableHmuop),
            'ꂟ' => Ok(YiSyllables::YiSyllableHmot),
            'ꂠ' => Ok(YiSyllables::YiSyllableHmox),
            'ꂡ' => Ok(YiSyllables::YiSyllableHmo),
            'ꂢ' => Ok(YiSyllables::YiSyllableHmop),
            'ꂣ' => Ok(YiSyllables::YiSyllableHmut),
            'ꂤ' => Ok(YiSyllables::YiSyllableHmux),
            'ꂥ' => Ok(YiSyllables::YiSyllableHmu),
            'ꂦ' => Ok(YiSyllables::YiSyllableHmup),
            'ꂧ' => Ok(YiSyllables::YiSyllableHmurx),
            'ꂨ' => Ok(YiSyllables::YiSyllableHmur),
            'ꂩ' => Ok(YiSyllables::YiSyllableHmyx),
            'ꂪ' => Ok(YiSyllables::YiSyllableHmy),
            'ꂫ' => Ok(YiSyllables::YiSyllableHmyp),
            'ꂬ' => Ok(YiSyllables::YiSyllableHmyrx),
            'ꂭ' => Ok(YiSyllables::YiSyllableHmyr),
            'ꂮ' => Ok(YiSyllables::YiSyllableMit),
            'ꂯ' => Ok(YiSyllables::YiSyllableMix),
            'ꂰ' => Ok(YiSyllables::YiSyllableMi),
            'ꂱ' => Ok(YiSyllables::YiSyllableMip),
            'ꂲ' => Ok(YiSyllables::YiSyllableMiex),
            'ꂳ' => Ok(YiSyllables::YiSyllableMie),
            'ꂴ' => Ok(YiSyllables::YiSyllableMiep),
            'ꂵ' => Ok(YiSyllables::YiSyllableMat),
            'ꂶ' => Ok(YiSyllables::YiSyllableMax),
            'ꂷ' => Ok(YiSyllables::YiSyllableMa),
            'ꂸ' => Ok(YiSyllables::YiSyllableMap),
            'ꂹ' => Ok(YiSyllables::YiSyllableMuot),
            'ꂺ' => Ok(YiSyllables::YiSyllableMuox),
            'ꂻ' => Ok(YiSyllables::YiSyllableMuo),
            'ꂼ' => Ok(YiSyllables::YiSyllableMuop),
            'ꂽ' => Ok(YiSyllables::YiSyllableMot),
            'ꂾ' => Ok(YiSyllables::YiSyllableMox),
            'ꂿ' => Ok(YiSyllables::YiSyllableMo),
            'ꃀ' => Ok(YiSyllables::YiSyllableMop),
            'ꃁ' => Ok(YiSyllables::YiSyllableMex),
            'ꃂ' => Ok(YiSyllables::YiSyllableMe),
            'ꃃ' => Ok(YiSyllables::YiSyllableMut),
            'ꃄ' => Ok(YiSyllables::YiSyllableMux),
            'ꃅ' => Ok(YiSyllables::YiSyllableMu),
            'ꃆ' => Ok(YiSyllables::YiSyllableMup),
            'ꃇ' => Ok(YiSyllables::YiSyllableMurx),
            'ꃈ' => Ok(YiSyllables::YiSyllableMur),
            'ꃉ' => Ok(YiSyllables::YiSyllableMyt),
            'ꃊ' => Ok(YiSyllables::YiSyllableMyx),
            'ꃋ' => Ok(YiSyllables::YiSyllableMy),
            'ꃌ' => Ok(YiSyllables::YiSyllableMyp),
            'ꃍ' => Ok(YiSyllables::YiSyllableFit),
            'ꃎ' => Ok(YiSyllables::YiSyllableFix),
            'ꃏ' => Ok(YiSyllables::YiSyllableFi),
            'ꃐ' => Ok(YiSyllables::YiSyllableFip),
            'ꃑ' => Ok(YiSyllables::YiSyllableFat),
            'ꃒ' => Ok(YiSyllables::YiSyllableFax),
            'ꃓ' => Ok(YiSyllables::YiSyllableFa),
            'ꃔ' => Ok(YiSyllables::YiSyllableFap),
            'ꃕ' => Ok(YiSyllables::YiSyllableFox),
            'ꃖ' => Ok(YiSyllables::YiSyllableFo),
            'ꃗ' => Ok(YiSyllables::YiSyllableFop),
            'ꃘ' => Ok(YiSyllables::YiSyllableFut),
            'ꃙ' => Ok(YiSyllables::YiSyllableFux),
            'ꃚ' => Ok(YiSyllables::YiSyllableFu),
            'ꃛ' => Ok(YiSyllables::YiSyllableFup),
            'ꃜ' => Ok(YiSyllables::YiSyllableFurx),
            'ꃝ' => Ok(YiSyllables::YiSyllableFur),
            'ꃞ' => Ok(YiSyllables::YiSyllableFyt),
            'ꃟ' => Ok(YiSyllables::YiSyllableFyx),
            'ꃠ' => Ok(YiSyllables::YiSyllableFy),
            'ꃡ' => Ok(YiSyllables::YiSyllableFyp),
            'ꃢ' => Ok(YiSyllables::YiSyllableVit),
            'ꃣ' => Ok(YiSyllables::YiSyllableVix),
            'ꃤ' => Ok(YiSyllables::YiSyllableVi),
            'ꃥ' => Ok(YiSyllables::YiSyllableVip),
            'ꃦ' => Ok(YiSyllables::YiSyllableViet),
            'ꃧ' => Ok(YiSyllables::YiSyllableViex),
            'ꃨ' => Ok(YiSyllables::YiSyllableVie),
            'ꃩ' => Ok(YiSyllables::YiSyllableViep),
            'ꃪ' => Ok(YiSyllables::YiSyllableVat),
            'ꃫ' => Ok(YiSyllables::YiSyllableVax),
            'ꃬ' => Ok(YiSyllables::YiSyllableVa),
            'ꃭ' => Ok(YiSyllables::YiSyllableVap),
            'ꃮ' => Ok(YiSyllables::YiSyllableVot),
            'ꃯ' => Ok(YiSyllables::YiSyllableVox),
            'ꃰ' => Ok(YiSyllables::YiSyllableVo),
            'ꃱ' => Ok(YiSyllables::YiSyllableVop),
            'ꃲ' => Ok(YiSyllables::YiSyllableVex),
            'ꃳ' => Ok(YiSyllables::YiSyllableVep),
            'ꃴ' => Ok(YiSyllables::YiSyllableVut),
            'ꃵ' => Ok(YiSyllables::YiSyllableVux),
            'ꃶ' => Ok(YiSyllables::YiSyllableVu),
            'ꃷ' => Ok(YiSyllables::YiSyllableVup),
            'ꃸ' => Ok(YiSyllables::YiSyllableVurx),
            'ꃹ' => Ok(YiSyllables::YiSyllableVur),
            'ꃺ' => Ok(YiSyllables::YiSyllableVyt),
            'ꃻ' => Ok(YiSyllables::YiSyllableVyx),
            'ꃼ' => Ok(YiSyllables::YiSyllableVy),
            'ꃽ' => Ok(YiSyllables::YiSyllableVyp),
            'ꃾ' => Ok(YiSyllables::YiSyllableVyrx),
            'ꃿ' => Ok(YiSyllables::YiSyllableVyr),
            'ꄀ' => Ok(YiSyllables::YiSyllableDit),
            'ꄁ' => Ok(YiSyllables::YiSyllableDix),
            'ꄂ' => Ok(YiSyllables::YiSyllableDi),
            'ꄃ' => Ok(YiSyllables::YiSyllableDip),
            'ꄄ' => Ok(YiSyllables::YiSyllableDiex),
            'ꄅ' => Ok(YiSyllables::YiSyllableDie),
            'ꄆ' => Ok(YiSyllables::YiSyllableDiep),
            'ꄇ' => Ok(YiSyllables::YiSyllableDat),
            'ꄈ' => Ok(YiSyllables::YiSyllableDax),
            'ꄉ' => Ok(YiSyllables::YiSyllableDa),
            'ꄊ' => Ok(YiSyllables::YiSyllableDap),
            'ꄋ' => Ok(YiSyllables::YiSyllableDuox),
            'ꄌ' => Ok(YiSyllables::YiSyllableDuo),
            'ꄍ' => Ok(YiSyllables::YiSyllableDot),
            'ꄎ' => Ok(YiSyllables::YiSyllableDox),
            'ꄏ' => Ok(YiSyllables::YiSyllableDo),
            'ꄐ' => Ok(YiSyllables::YiSyllableDop),
            'ꄑ' => Ok(YiSyllables::YiSyllableDex),
            'ꄒ' => Ok(YiSyllables::YiSyllableDe),
            'ꄓ' => Ok(YiSyllables::YiSyllableDep),
            'ꄔ' => Ok(YiSyllables::YiSyllableDut),
            'ꄕ' => Ok(YiSyllables::YiSyllableDux),
            'ꄖ' => Ok(YiSyllables::YiSyllableDu),
            'ꄗ' => Ok(YiSyllables::YiSyllableDup),
            'ꄘ' => Ok(YiSyllables::YiSyllableDurx),
            'ꄙ' => Ok(YiSyllables::YiSyllableDur),
            'ꄚ' => Ok(YiSyllables::YiSyllableTit),
            'ꄛ' => Ok(YiSyllables::YiSyllableTix),
            'ꄜ' => Ok(YiSyllables::YiSyllableTi),
            'ꄝ' => Ok(YiSyllables::YiSyllableTip),
            'ꄞ' => Ok(YiSyllables::YiSyllableTiex),
            'ꄟ' => Ok(YiSyllables::YiSyllableTie),
            'ꄠ' => Ok(YiSyllables::YiSyllableTiep),
            'ꄡ' => Ok(YiSyllables::YiSyllableTat),
            'ꄢ' => Ok(YiSyllables::YiSyllableTax),
            'ꄣ' => Ok(YiSyllables::YiSyllableTa),
            'ꄤ' => Ok(YiSyllables::YiSyllableTap),
            'ꄥ' => Ok(YiSyllables::YiSyllableTuot),
            'ꄦ' => Ok(YiSyllables::YiSyllableTuox),
            'ꄧ' => Ok(YiSyllables::YiSyllableTuo),
            'ꄨ' => Ok(YiSyllables::YiSyllableTuop),
            'ꄩ' => Ok(YiSyllables::YiSyllableTot),
            'ꄪ' => Ok(YiSyllables::YiSyllableTox),
            'ꄫ' => Ok(YiSyllables::YiSyllableTo),
            'ꄬ' => Ok(YiSyllables::YiSyllableTop),
            'ꄭ' => Ok(YiSyllables::YiSyllableTex),
            'ꄮ' => Ok(YiSyllables::YiSyllableTe),
            'ꄯ' => Ok(YiSyllables::YiSyllableTep),
            'ꄰ' => Ok(YiSyllables::YiSyllableTut),
            'ꄱ' => Ok(YiSyllables::YiSyllableTux),
            'ꄲ' => Ok(YiSyllables::YiSyllableTu),
            'ꄳ' => Ok(YiSyllables::YiSyllableTup),
            'ꄴ' => Ok(YiSyllables::YiSyllableTurx),
            'ꄵ' => Ok(YiSyllables::YiSyllableTur),
            'ꄶ' => Ok(YiSyllables::YiSyllableDdit),
            'ꄷ' => Ok(YiSyllables::YiSyllableDdix),
            'ꄸ' => Ok(YiSyllables::YiSyllableDdi),
            'ꄹ' => Ok(YiSyllables::YiSyllableDdip),
            'ꄺ' => Ok(YiSyllables::YiSyllableDdiex),
            'ꄻ' => Ok(YiSyllables::YiSyllableDdie),
            'ꄼ' => Ok(YiSyllables::YiSyllableDdiep),
            'ꄽ' => Ok(YiSyllables::YiSyllableDdat),
            'ꄾ' => Ok(YiSyllables::YiSyllableDdax),
            'ꄿ' => Ok(YiSyllables::YiSyllableDda),
            'ꅀ' => Ok(YiSyllables::YiSyllableDdap),
            'ꅁ' => Ok(YiSyllables::YiSyllableDduox),
            'ꅂ' => Ok(YiSyllables::YiSyllableDduo),
            'ꅃ' => Ok(YiSyllables::YiSyllableDduop),
            'ꅄ' => Ok(YiSyllables::YiSyllableDdot),
            'ꅅ' => Ok(YiSyllables::YiSyllableDdox),
            'ꅆ' => Ok(YiSyllables::YiSyllableDdo),
            'ꅇ' => Ok(YiSyllables::YiSyllableDdop),
            'ꅈ' => Ok(YiSyllables::YiSyllableDdex),
            'ꅉ' => Ok(YiSyllables::YiSyllableDde),
            'ꅊ' => Ok(YiSyllables::YiSyllableDdep),
            'ꅋ' => Ok(YiSyllables::YiSyllableDdut),
            'ꅌ' => Ok(YiSyllables::YiSyllableDdux),
            'ꅍ' => Ok(YiSyllables::YiSyllableDdu),
            'ꅎ' => Ok(YiSyllables::YiSyllableDdup),
            'ꅏ' => Ok(YiSyllables::YiSyllableDdurx),
            'ꅐ' => Ok(YiSyllables::YiSyllableDdur),
            'ꅑ' => Ok(YiSyllables::YiSyllableNdit),
            'ꅒ' => Ok(YiSyllables::YiSyllableNdix),
            'ꅓ' => Ok(YiSyllables::YiSyllableNdi),
            'ꅔ' => Ok(YiSyllables::YiSyllableNdip),
            'ꅕ' => Ok(YiSyllables::YiSyllableNdiex),
            'ꅖ' => Ok(YiSyllables::YiSyllableNdie),
            'ꅗ' => Ok(YiSyllables::YiSyllableNdat),
            'ꅘ' => Ok(YiSyllables::YiSyllableNdax),
            'ꅙ' => Ok(YiSyllables::YiSyllableNda),
            'ꅚ' => Ok(YiSyllables::YiSyllableNdap),
            'ꅛ' => Ok(YiSyllables::YiSyllableNdot),
            'ꅜ' => Ok(YiSyllables::YiSyllableNdox),
            'ꅝ' => Ok(YiSyllables::YiSyllableNdo),
            'ꅞ' => Ok(YiSyllables::YiSyllableNdop),
            'ꅟ' => Ok(YiSyllables::YiSyllableNdex),
            'ꅠ' => Ok(YiSyllables::YiSyllableNde),
            'ꅡ' => Ok(YiSyllables::YiSyllableNdep),
            'ꅢ' => Ok(YiSyllables::YiSyllableNdut),
            'ꅣ' => Ok(YiSyllables::YiSyllableNdux),
            'ꅤ' => Ok(YiSyllables::YiSyllableNdu),
            'ꅥ' => Ok(YiSyllables::YiSyllableNdup),
            'ꅦ' => Ok(YiSyllables::YiSyllableNdurx),
            'ꅧ' => Ok(YiSyllables::YiSyllableNdur),
            'ꅨ' => Ok(YiSyllables::YiSyllableHnit),
            'ꅩ' => Ok(YiSyllables::YiSyllableHnix),
            'ꅪ' => Ok(YiSyllables::YiSyllableHni),
            'ꅫ' => Ok(YiSyllables::YiSyllableHnip),
            'ꅬ' => Ok(YiSyllables::YiSyllableHniet),
            'ꅭ' => Ok(YiSyllables::YiSyllableHniex),
            'ꅮ' => Ok(YiSyllables::YiSyllableHnie),
            'ꅯ' => Ok(YiSyllables::YiSyllableHniep),
            'ꅰ' => Ok(YiSyllables::YiSyllableHnat),
            'ꅱ' => Ok(YiSyllables::YiSyllableHnax),
            'ꅲ' => Ok(YiSyllables::YiSyllableHna),
            'ꅳ' => Ok(YiSyllables::YiSyllableHnap),
            'ꅴ' => Ok(YiSyllables::YiSyllableHnuox),
            'ꅵ' => Ok(YiSyllables::YiSyllableHnuo),
            'ꅶ' => Ok(YiSyllables::YiSyllableHnot),
            'ꅷ' => Ok(YiSyllables::YiSyllableHnox),
            'ꅸ' => Ok(YiSyllables::YiSyllableHnop),
            'ꅹ' => Ok(YiSyllables::YiSyllableHnex),
            'ꅺ' => Ok(YiSyllables::YiSyllableHne),
            'ꅻ' => Ok(YiSyllables::YiSyllableHnep),
            'ꅼ' => Ok(YiSyllables::YiSyllableHnut),
            'ꅽ' => Ok(YiSyllables::YiSyllableNit),
            'ꅾ' => Ok(YiSyllables::YiSyllableNix),
            'ꅿ' => Ok(YiSyllables::YiSyllableNi),
            'ꆀ' => Ok(YiSyllables::YiSyllableNip),
            'ꆁ' => Ok(YiSyllables::YiSyllableNiex),
            'ꆂ' => Ok(YiSyllables::YiSyllableNie),
            'ꆃ' => Ok(YiSyllables::YiSyllableNiep),
            'ꆄ' => Ok(YiSyllables::YiSyllableNax),
            'ꆅ' => Ok(YiSyllables::YiSyllableNa),
            'ꆆ' => Ok(YiSyllables::YiSyllableNap),
            'ꆇ' => Ok(YiSyllables::YiSyllableNuox),
            'ꆈ' => Ok(YiSyllables::YiSyllableNuo),
            'ꆉ' => Ok(YiSyllables::YiSyllableNuop),
            'ꆊ' => Ok(YiSyllables::YiSyllableNot),
            'ꆋ' => Ok(YiSyllables::YiSyllableNox),
            'ꆌ' => Ok(YiSyllables::YiSyllableNo),
            'ꆍ' => Ok(YiSyllables::YiSyllableNop),
            'ꆎ' => Ok(YiSyllables::YiSyllableNex),
            'ꆏ' => Ok(YiSyllables::YiSyllableNe),
            'ꆐ' => Ok(YiSyllables::YiSyllableNep),
            'ꆑ' => Ok(YiSyllables::YiSyllableNut),
            'ꆒ' => Ok(YiSyllables::YiSyllableNux),
            'ꆓ' => Ok(YiSyllables::YiSyllableNu),
            'ꆔ' => Ok(YiSyllables::YiSyllableNup),
            'ꆕ' => Ok(YiSyllables::YiSyllableNurx),
            'ꆖ' => Ok(YiSyllables::YiSyllableNur),
            'ꆗ' => Ok(YiSyllables::YiSyllableHlit),
            'ꆘ' => Ok(YiSyllables::YiSyllableHlix),
            'ꆙ' => Ok(YiSyllables::YiSyllableHli),
            'ꆚ' => Ok(YiSyllables::YiSyllableHlip),
            'ꆛ' => Ok(YiSyllables::YiSyllableHliex),
            'ꆜ' => Ok(YiSyllables::YiSyllableHlie),
            'ꆝ' => Ok(YiSyllables::YiSyllableHliep),
            'ꆞ' => Ok(YiSyllables::YiSyllableHlat),
            'ꆟ' => Ok(YiSyllables::YiSyllableHlax),
            'ꆠ' => Ok(YiSyllables::YiSyllableHla),
            'ꆡ' => Ok(YiSyllables::YiSyllableHlap),
            'ꆢ' => Ok(YiSyllables::YiSyllableHluox),
            'ꆣ' => Ok(YiSyllables::YiSyllableHluo),
            'ꆤ' => Ok(YiSyllables::YiSyllableHluop),
            'ꆥ' => Ok(YiSyllables::YiSyllableHlox),
            'ꆦ' => Ok(YiSyllables::YiSyllableHlo),
            'ꆧ' => Ok(YiSyllables::YiSyllableHlop),
            'ꆨ' => Ok(YiSyllables::YiSyllableHlex),
            'ꆩ' => Ok(YiSyllables::YiSyllableHle),
            'ꆪ' => Ok(YiSyllables::YiSyllableHlep),
            'ꆫ' => Ok(YiSyllables::YiSyllableHlut),
            'ꆬ' => Ok(YiSyllables::YiSyllableHlux),
            'ꆭ' => Ok(YiSyllables::YiSyllableHlu),
            'ꆮ' => Ok(YiSyllables::YiSyllableHlup),
            'ꆯ' => Ok(YiSyllables::YiSyllableHlurx),
            'ꆰ' => Ok(YiSyllables::YiSyllableHlur),
            'ꆱ' => Ok(YiSyllables::YiSyllableHlyt),
            'ꆲ' => Ok(YiSyllables::YiSyllableHlyx),
            'ꆳ' => Ok(YiSyllables::YiSyllableHly),
            'ꆴ' => Ok(YiSyllables::YiSyllableHlyp),
            'ꆵ' => Ok(YiSyllables::YiSyllableHlyrx),
            'ꆶ' => Ok(YiSyllables::YiSyllableHlyr),
            'ꆷ' => Ok(YiSyllables::YiSyllableLit),
            'ꆸ' => Ok(YiSyllables::YiSyllableLix),
            'ꆹ' => Ok(YiSyllables::YiSyllableLi),
            'ꆺ' => Ok(YiSyllables::YiSyllableLip),
            'ꆻ' => Ok(YiSyllables::YiSyllableLiet),
            'ꆼ' => Ok(YiSyllables::YiSyllableLiex),
            'ꆽ' => Ok(YiSyllables::YiSyllableLie),
            'ꆾ' => Ok(YiSyllables::YiSyllableLiep),
            'ꆿ' => Ok(YiSyllables::YiSyllableLat),
            'ꇀ' => Ok(YiSyllables::YiSyllableLax),
            'ꇁ' => Ok(YiSyllables::YiSyllableLa),
            'ꇂ' => Ok(YiSyllables::YiSyllableLap),
            'ꇃ' => Ok(YiSyllables::YiSyllableLuot),
            'ꇄ' => Ok(YiSyllables::YiSyllableLuox),
            'ꇅ' => Ok(YiSyllables::YiSyllableLuo),
            'ꇆ' => Ok(YiSyllables::YiSyllableLuop),
            'ꇇ' => Ok(YiSyllables::YiSyllableLot),
            'ꇈ' => Ok(YiSyllables::YiSyllableLox),
            'ꇉ' => Ok(YiSyllables::YiSyllableLo),
            'ꇊ' => Ok(YiSyllables::YiSyllableLop),
            'ꇋ' => Ok(YiSyllables::YiSyllableLex),
            'ꇌ' => Ok(YiSyllables::YiSyllableLe),
            'ꇍ' => Ok(YiSyllables::YiSyllableLep),
            'ꇎ' => Ok(YiSyllables::YiSyllableLut),
            'ꇏ' => Ok(YiSyllables::YiSyllableLux),
            'ꇐ' => Ok(YiSyllables::YiSyllableLu),
            'ꇑ' => Ok(YiSyllables::YiSyllableLup),
            'ꇒ' => Ok(YiSyllables::YiSyllableLurx),
            'ꇓ' => Ok(YiSyllables::YiSyllableLur),
            'ꇔ' => Ok(YiSyllables::YiSyllableLyt),
            'ꇕ' => Ok(YiSyllables::YiSyllableLyx),
            'ꇖ' => Ok(YiSyllables::YiSyllableLy),
            'ꇗ' => Ok(YiSyllables::YiSyllableLyp),
            'ꇘ' => Ok(YiSyllables::YiSyllableLyrx),
            'ꇙ' => Ok(YiSyllables::YiSyllableLyr),
            'ꇚ' => Ok(YiSyllables::YiSyllableGit),
            'ꇛ' => Ok(YiSyllables::YiSyllableGix),
            'ꇜ' => Ok(YiSyllables::YiSyllableGi),
            'ꇝ' => Ok(YiSyllables::YiSyllableGip),
            'ꇞ' => Ok(YiSyllables::YiSyllableGiet),
            'ꇟ' => Ok(YiSyllables::YiSyllableGiex),
            'ꇠ' => Ok(YiSyllables::YiSyllableGie),
            'ꇡ' => Ok(YiSyllables::YiSyllableGiep),
            'ꇢ' => Ok(YiSyllables::YiSyllableGat),
            'ꇣ' => Ok(YiSyllables::YiSyllableGax),
            'ꇤ' => Ok(YiSyllables::YiSyllableGa),
            'ꇥ' => Ok(YiSyllables::YiSyllableGap),
            'ꇦ' => Ok(YiSyllables::YiSyllableGuot),
            'ꇧ' => Ok(YiSyllables::YiSyllableGuox),
            'ꇨ' => Ok(YiSyllables::YiSyllableGuo),
            'ꇩ' => Ok(YiSyllables::YiSyllableGuop),
            'ꇪ' => Ok(YiSyllables::YiSyllableGot),
            'ꇫ' => Ok(YiSyllables::YiSyllableGox),
            'ꇬ' => Ok(YiSyllables::YiSyllableGo),
            'ꇭ' => Ok(YiSyllables::YiSyllableGop),
            'ꇮ' => Ok(YiSyllables::YiSyllableGet),
            'ꇯ' => Ok(YiSyllables::YiSyllableGex),
            'ꇰ' => Ok(YiSyllables::YiSyllableGe),
            'ꇱ' => Ok(YiSyllables::YiSyllableGep),
            'ꇲ' => Ok(YiSyllables::YiSyllableGut),
            'ꇳ' => Ok(YiSyllables::YiSyllableGux),
            'ꇴ' => Ok(YiSyllables::YiSyllableGu),
            'ꇵ' => Ok(YiSyllables::YiSyllableGup),
            'ꇶ' => Ok(YiSyllables::YiSyllableGurx),
            'ꇷ' => Ok(YiSyllables::YiSyllableGur),
            'ꇸ' => Ok(YiSyllables::YiSyllableKit),
            'ꇹ' => Ok(YiSyllables::YiSyllableKix),
            'ꇺ' => Ok(YiSyllables::YiSyllableKi),
            'ꇻ' => Ok(YiSyllables::YiSyllableKip),
            'ꇼ' => Ok(YiSyllables::YiSyllableKiex),
            'ꇽ' => Ok(YiSyllables::YiSyllableKie),
            'ꇾ' => Ok(YiSyllables::YiSyllableKiep),
            'ꇿ' => Ok(YiSyllables::YiSyllableKat),
            'ꈀ' => Ok(YiSyllables::YiSyllableKax),
            'ꈁ' => Ok(YiSyllables::YiSyllableKa),
            'ꈂ' => Ok(YiSyllables::YiSyllableKap),
            'ꈃ' => Ok(YiSyllables::YiSyllableKuox),
            'ꈄ' => Ok(YiSyllables::YiSyllableKuo),
            'ꈅ' => Ok(YiSyllables::YiSyllableKuop),
            'ꈆ' => Ok(YiSyllables::YiSyllableKot),
            'ꈇ' => Ok(YiSyllables::YiSyllableKox),
            'ꈈ' => Ok(YiSyllables::YiSyllableKo),
            'ꈉ' => Ok(YiSyllables::YiSyllableKop),
            'ꈊ' => Ok(YiSyllables::YiSyllableKet),
            'ꈋ' => Ok(YiSyllables::YiSyllableKex),
            'ꈌ' => Ok(YiSyllables::YiSyllableKe),
            'ꈍ' => Ok(YiSyllables::YiSyllableKep),
            'ꈎ' => Ok(YiSyllables::YiSyllableKut),
            'ꈏ' => Ok(YiSyllables::YiSyllableKux),
            'ꈐ' => Ok(YiSyllables::YiSyllableKu),
            'ꈑ' => Ok(YiSyllables::YiSyllableKup),
            'ꈒ' => Ok(YiSyllables::YiSyllableKurx),
            'ꈓ' => Ok(YiSyllables::YiSyllableKur),
            'ꈔ' => Ok(YiSyllables::YiSyllableGgit),
            'ꈕ' => Ok(YiSyllables::YiSyllableGgix),
            'ꈖ' => Ok(YiSyllables::YiSyllableGgi),
            'ꈗ' => Ok(YiSyllables::YiSyllableGgiex),
            'ꈘ' => Ok(YiSyllables::YiSyllableGgie),
            'ꈙ' => Ok(YiSyllables::YiSyllableGgiep),
            'ꈚ' => Ok(YiSyllables::YiSyllableGgat),
            'ꈛ' => Ok(YiSyllables::YiSyllableGgax),
            'ꈜ' => Ok(YiSyllables::YiSyllableGga),
            'ꈝ' => Ok(YiSyllables::YiSyllableGgap),
            'ꈞ' => Ok(YiSyllables::YiSyllableGguot),
            'ꈟ' => Ok(YiSyllables::YiSyllableGguox),
            'ꈠ' => Ok(YiSyllables::YiSyllableGguo),
            'ꈡ' => Ok(YiSyllables::YiSyllableGguop),
            'ꈢ' => Ok(YiSyllables::YiSyllableGgot),
            'ꈣ' => Ok(YiSyllables::YiSyllableGgox),
            'ꈤ' => Ok(YiSyllables::YiSyllableGgo),
            'ꈥ' => Ok(YiSyllables::YiSyllableGgop),
            'ꈦ' => Ok(YiSyllables::YiSyllableGget),
            'ꈧ' => Ok(YiSyllables::YiSyllableGgex),
            'ꈨ' => Ok(YiSyllables::YiSyllableGge),
            'ꈩ' => Ok(YiSyllables::YiSyllableGgep),
            'ꈪ' => Ok(YiSyllables::YiSyllableGgut),
            'ꈫ' => Ok(YiSyllables::YiSyllableGgux),
            'ꈬ' => Ok(YiSyllables::YiSyllableGgu),
            'ꈭ' => Ok(YiSyllables::YiSyllableGgup),
            'ꈮ' => Ok(YiSyllables::YiSyllableGgurx),
            'ꈯ' => Ok(YiSyllables::YiSyllableGgur),
            'ꈰ' => Ok(YiSyllables::YiSyllableMgiex),
            'ꈱ' => Ok(YiSyllables::YiSyllableMgie),
            'ꈲ' => Ok(YiSyllables::YiSyllableMgat),
            'ꈳ' => Ok(YiSyllables::YiSyllableMgax),
            'ꈴ' => Ok(YiSyllables::YiSyllableMga),
            'ꈵ' => Ok(YiSyllables::YiSyllableMgap),
            'ꈶ' => Ok(YiSyllables::YiSyllableMguox),
            'ꈷ' => Ok(YiSyllables::YiSyllableMguo),
            'ꈸ' => Ok(YiSyllables::YiSyllableMguop),
            'ꈹ' => Ok(YiSyllables::YiSyllableMgot),
            'ꈺ' => Ok(YiSyllables::YiSyllableMgox),
            'ꈻ' => Ok(YiSyllables::YiSyllableMgo),
            'ꈼ' => Ok(YiSyllables::YiSyllableMgop),
            'ꈽ' => Ok(YiSyllables::YiSyllableMgex),
            'ꈾ' => Ok(YiSyllables::YiSyllableMge),
            'ꈿ' => Ok(YiSyllables::YiSyllableMgep),
            'ꉀ' => Ok(YiSyllables::YiSyllableMgut),
            'ꉁ' => Ok(YiSyllables::YiSyllableMgux),
            'ꉂ' => Ok(YiSyllables::YiSyllableMgu),
            'ꉃ' => Ok(YiSyllables::YiSyllableMgup),
            'ꉄ' => Ok(YiSyllables::YiSyllableMgurx),
            'ꉅ' => Ok(YiSyllables::YiSyllableMgur),
            'ꉆ' => Ok(YiSyllables::YiSyllableHxit),
            'ꉇ' => Ok(YiSyllables::YiSyllableHxix),
            'ꉈ' => Ok(YiSyllables::YiSyllableHxi),
            'ꉉ' => Ok(YiSyllables::YiSyllableHxip),
            'ꉊ' => Ok(YiSyllables::YiSyllableHxiet),
            'ꉋ' => Ok(YiSyllables::YiSyllableHxiex),
            'ꉌ' => Ok(YiSyllables::YiSyllableHxie),
            'ꉍ' => Ok(YiSyllables::YiSyllableHxiep),
            'ꉎ' => Ok(YiSyllables::YiSyllableHxat),
            'ꉏ' => Ok(YiSyllables::YiSyllableHxax),
            'ꉐ' => Ok(YiSyllables::YiSyllableHxa),
            'ꉑ' => Ok(YiSyllables::YiSyllableHxap),
            'ꉒ' => Ok(YiSyllables::YiSyllableHxuot),
            'ꉓ' => Ok(YiSyllables::YiSyllableHxuox),
            'ꉔ' => Ok(YiSyllables::YiSyllableHxuo),
            'ꉕ' => Ok(YiSyllables::YiSyllableHxuop),
            'ꉖ' => Ok(YiSyllables::YiSyllableHxot),
            'ꉗ' => Ok(YiSyllables::YiSyllableHxox),
            'ꉘ' => Ok(YiSyllables::YiSyllableHxo),
            'ꉙ' => Ok(YiSyllables::YiSyllableHxop),
            'ꉚ' => Ok(YiSyllables::YiSyllableHxex),
            'ꉛ' => Ok(YiSyllables::YiSyllableHxe),
            'ꉜ' => Ok(YiSyllables::YiSyllableHxep),
            'ꉝ' => Ok(YiSyllables::YiSyllableNgiex),
            'ꉞ' => Ok(YiSyllables::YiSyllableNgie),
            'ꉟ' => Ok(YiSyllables::YiSyllableNgiep),
            'ꉠ' => Ok(YiSyllables::YiSyllableNgat),
            'ꉡ' => Ok(YiSyllables::YiSyllableNgax),
            'ꉢ' => Ok(YiSyllables::YiSyllableNga),
            'ꉣ' => Ok(YiSyllables::YiSyllableNgap),
            'ꉤ' => Ok(YiSyllables::YiSyllableNguot),
            'ꉥ' => Ok(YiSyllables::YiSyllableNguox),
            'ꉦ' => Ok(YiSyllables::YiSyllableNguo),
            'ꉧ' => Ok(YiSyllables::YiSyllableNgot),
            'ꉨ' => Ok(YiSyllables::YiSyllableNgox),
            'ꉩ' => Ok(YiSyllables::YiSyllableNgo),
            'ꉪ' => Ok(YiSyllables::YiSyllableNgop),
            'ꉫ' => Ok(YiSyllables::YiSyllableNgex),
            'ꉬ' => Ok(YiSyllables::YiSyllableNge),
            'ꉭ' => Ok(YiSyllables::YiSyllableNgep),
            'ꉮ' => Ok(YiSyllables::YiSyllableHit),
            'ꉯ' => Ok(YiSyllables::YiSyllableHiex),
            'ꉰ' => Ok(YiSyllables::YiSyllableHie),
            'ꉱ' => Ok(YiSyllables::YiSyllableHat),
            'ꉲ' => Ok(YiSyllables::YiSyllableHax),
            'ꉳ' => Ok(YiSyllables::YiSyllableHa),
            'ꉴ' => Ok(YiSyllables::YiSyllableHap),
            'ꉵ' => Ok(YiSyllables::YiSyllableHuot),
            'ꉶ' => Ok(YiSyllables::YiSyllableHuox),
            'ꉷ' => Ok(YiSyllables::YiSyllableHuo),
            'ꉸ' => Ok(YiSyllables::YiSyllableHuop),
            'ꉹ' => Ok(YiSyllables::YiSyllableHot),
            'ꉺ' => Ok(YiSyllables::YiSyllableHox),
            'ꉻ' => Ok(YiSyllables::YiSyllableHo),
            'ꉼ' => Ok(YiSyllables::YiSyllableHop),
            'ꉽ' => Ok(YiSyllables::YiSyllableHex),
            'ꉾ' => Ok(YiSyllables::YiSyllableHe),
            'ꉿ' => Ok(YiSyllables::YiSyllableHep),
            'ꊀ' => Ok(YiSyllables::YiSyllableWat),
            'ꊁ' => Ok(YiSyllables::YiSyllableWax),
            'ꊂ' => Ok(YiSyllables::YiSyllableWa),
            'ꊃ' => Ok(YiSyllables::YiSyllableWap),
            'ꊄ' => Ok(YiSyllables::YiSyllableWuox),
            'ꊅ' => Ok(YiSyllables::YiSyllableWuo),
            'ꊆ' => Ok(YiSyllables::YiSyllableWuop),
            'ꊇ' => Ok(YiSyllables::YiSyllableWox),
            'ꊈ' => Ok(YiSyllables::YiSyllableWo),
            'ꊉ' => Ok(YiSyllables::YiSyllableWop),
            'ꊊ' => Ok(YiSyllables::YiSyllableWex),
            'ꊋ' => Ok(YiSyllables::YiSyllableWe),
            'ꊌ' => Ok(YiSyllables::YiSyllableWep),
            'ꊍ' => Ok(YiSyllables::YiSyllableZit),
            'ꊎ' => Ok(YiSyllables::YiSyllableZix),
            'ꊏ' => Ok(YiSyllables::YiSyllableZi),
            'ꊐ' => Ok(YiSyllables::YiSyllableZip),
            'ꊑ' => Ok(YiSyllables::YiSyllableZiex),
            'ꊒ' => Ok(YiSyllables::YiSyllableZie),
            'ꊓ' => Ok(YiSyllables::YiSyllableZiep),
            'ꊔ' => Ok(YiSyllables::YiSyllableZat),
            'ꊕ' => Ok(YiSyllables::YiSyllableZax),
            'ꊖ' => Ok(YiSyllables::YiSyllableZa),
            'ꊗ' => Ok(YiSyllables::YiSyllableZap),
            'ꊘ' => Ok(YiSyllables::YiSyllableZuox),
            'ꊙ' => Ok(YiSyllables::YiSyllableZuo),
            'ꊚ' => Ok(YiSyllables::YiSyllableZuop),
            'ꊛ' => Ok(YiSyllables::YiSyllableZot),
            'ꊜ' => Ok(YiSyllables::YiSyllableZox),
            'ꊝ' => Ok(YiSyllables::YiSyllableZo),
            'ꊞ' => Ok(YiSyllables::YiSyllableZop),
            'ꊟ' => Ok(YiSyllables::YiSyllableZex),
            'ꊠ' => Ok(YiSyllables::YiSyllableZe),
            'ꊡ' => Ok(YiSyllables::YiSyllableZep),
            'ꊢ' => Ok(YiSyllables::YiSyllableZut),
            'ꊣ' => Ok(YiSyllables::YiSyllableZux),
            'ꊤ' => Ok(YiSyllables::YiSyllableZu),
            'ꊥ' => Ok(YiSyllables::YiSyllableZup),
            'ꊦ' => Ok(YiSyllables::YiSyllableZurx),
            'ꊧ' => Ok(YiSyllables::YiSyllableZur),
            'ꊨ' => Ok(YiSyllables::YiSyllableZyt),
            'ꊩ' => Ok(YiSyllables::YiSyllableZyx),
            'ꊪ' => Ok(YiSyllables::YiSyllableZy),
            'ꊫ' => Ok(YiSyllables::YiSyllableZyp),
            'ꊬ' => Ok(YiSyllables::YiSyllableZyrx),
            'ꊭ' => Ok(YiSyllables::YiSyllableZyr),
            'ꊮ' => Ok(YiSyllables::YiSyllableCit),
            'ꊯ' => Ok(YiSyllables::YiSyllableCix),
            'ꊰ' => Ok(YiSyllables::YiSyllableCi),
            'ꊱ' => Ok(YiSyllables::YiSyllableCip),
            'ꊲ' => Ok(YiSyllables::YiSyllableCiet),
            'ꊳ' => Ok(YiSyllables::YiSyllableCiex),
            'ꊴ' => Ok(YiSyllables::YiSyllableCie),
            'ꊵ' => Ok(YiSyllables::YiSyllableCiep),
            'ꊶ' => Ok(YiSyllables::YiSyllableCat),
            'ꊷ' => Ok(YiSyllables::YiSyllableCax),
            'ꊸ' => Ok(YiSyllables::YiSyllableCa),
            'ꊹ' => Ok(YiSyllables::YiSyllableCap),
            'ꊺ' => Ok(YiSyllables::YiSyllableCuox),
            'ꊻ' => Ok(YiSyllables::YiSyllableCuo),
            'ꊼ' => Ok(YiSyllables::YiSyllableCuop),
            'ꊽ' => Ok(YiSyllables::YiSyllableCot),
            'ꊾ' => Ok(YiSyllables::YiSyllableCox),
            'ꊿ' => Ok(YiSyllables::YiSyllableCo),
            'ꋀ' => Ok(YiSyllables::YiSyllableCop),
            'ꋁ' => Ok(YiSyllables::YiSyllableCex),
            'ꋂ' => Ok(YiSyllables::YiSyllableCe),
            'ꋃ' => Ok(YiSyllables::YiSyllableCep),
            'ꋄ' => Ok(YiSyllables::YiSyllableCut),
            'ꋅ' => Ok(YiSyllables::YiSyllableCux),
            'ꋆ' => Ok(YiSyllables::YiSyllableCu),
            'ꋇ' => Ok(YiSyllables::YiSyllableCup),
            'ꋈ' => Ok(YiSyllables::YiSyllableCurx),
            'ꋉ' => Ok(YiSyllables::YiSyllableCur),
            'ꋊ' => Ok(YiSyllables::YiSyllableCyt),
            'ꋋ' => Ok(YiSyllables::YiSyllableCyx),
            'ꋌ' => Ok(YiSyllables::YiSyllableCy),
            'ꋍ' => Ok(YiSyllables::YiSyllableCyp),
            'ꋎ' => Ok(YiSyllables::YiSyllableCyrx),
            'ꋏ' => Ok(YiSyllables::YiSyllableCyr),
            'ꋐ' => Ok(YiSyllables::YiSyllableZzit),
            'ꋑ' => Ok(YiSyllables::YiSyllableZzix),
            'ꋒ' => Ok(YiSyllables::YiSyllableZzi),
            'ꋓ' => Ok(YiSyllables::YiSyllableZzip),
            'ꋔ' => Ok(YiSyllables::YiSyllableZziet),
            'ꋕ' => Ok(YiSyllables::YiSyllableZziex),
            'ꋖ' => Ok(YiSyllables::YiSyllableZzie),
            'ꋗ' => Ok(YiSyllables::YiSyllableZziep),
            'ꋘ' => Ok(YiSyllables::YiSyllableZzat),
            'ꋙ' => Ok(YiSyllables::YiSyllableZzax),
            'ꋚ' => Ok(YiSyllables::YiSyllableZza),
            'ꋛ' => Ok(YiSyllables::YiSyllableZzap),
            'ꋜ' => Ok(YiSyllables::YiSyllableZzox),
            'ꋝ' => Ok(YiSyllables::YiSyllableZzo),
            'ꋞ' => Ok(YiSyllables::YiSyllableZzop),
            'ꋟ' => Ok(YiSyllables::YiSyllableZzex),
            'ꋠ' => Ok(YiSyllables::YiSyllableZze),
            'ꋡ' => Ok(YiSyllables::YiSyllableZzep),
            'ꋢ' => Ok(YiSyllables::YiSyllableZzux),
            'ꋣ' => Ok(YiSyllables::YiSyllableZzu),
            'ꋤ' => Ok(YiSyllables::YiSyllableZzup),
            'ꋥ' => Ok(YiSyllables::YiSyllableZzurx),
            'ꋦ' => Ok(YiSyllables::YiSyllableZzur),
            'ꋧ' => Ok(YiSyllables::YiSyllableZzyt),
            'ꋨ' => Ok(YiSyllables::YiSyllableZzyx),
            'ꋩ' => Ok(YiSyllables::YiSyllableZzy),
            'ꋪ' => Ok(YiSyllables::YiSyllableZzyp),
            'ꋫ' => Ok(YiSyllables::YiSyllableZzyrx),
            'ꋬ' => Ok(YiSyllables::YiSyllableZzyr),
            'ꋭ' => Ok(YiSyllables::YiSyllableNzit),
            'ꋮ' => Ok(YiSyllables::YiSyllableNzix),
            'ꋯ' => Ok(YiSyllables::YiSyllableNzi),
            'ꋰ' => Ok(YiSyllables::YiSyllableNzip),
            'ꋱ' => Ok(YiSyllables::YiSyllableNziex),
            'ꋲ' => Ok(YiSyllables::YiSyllableNzie),
            'ꋳ' => Ok(YiSyllables::YiSyllableNziep),
            'ꋴ' => Ok(YiSyllables::YiSyllableNzat),
            'ꋵ' => Ok(YiSyllables::YiSyllableNzax),
            'ꋶ' => Ok(YiSyllables::YiSyllableNza),
            'ꋷ' => Ok(YiSyllables::YiSyllableNzap),
            'ꋸ' => Ok(YiSyllables::YiSyllableNzuox),
            'ꋹ' => Ok(YiSyllables::YiSyllableNzuo),
            'ꋺ' => Ok(YiSyllables::YiSyllableNzox),
            'ꋻ' => Ok(YiSyllables::YiSyllableNzop),
            'ꋼ' => Ok(YiSyllables::YiSyllableNzex),
            'ꋽ' => Ok(YiSyllables::YiSyllableNze),
            'ꋾ' => Ok(YiSyllables::YiSyllableNzux),
            'ꋿ' => Ok(YiSyllables::YiSyllableNzu),
            'ꌀ' => Ok(YiSyllables::YiSyllableNzup),
            'ꌁ' => Ok(YiSyllables::YiSyllableNzurx),
            'ꌂ' => Ok(YiSyllables::YiSyllableNzur),
            'ꌃ' => Ok(YiSyllables::YiSyllableNzyt),
            'ꌄ' => Ok(YiSyllables::YiSyllableNzyx),
            'ꌅ' => Ok(YiSyllables::YiSyllableNzy),
            'ꌆ' => Ok(YiSyllables::YiSyllableNzyp),
            'ꌇ' => Ok(YiSyllables::YiSyllableNzyrx),
            'ꌈ' => Ok(YiSyllables::YiSyllableNzyr),
            'ꌉ' => Ok(YiSyllables::YiSyllableSit),
            'ꌊ' => Ok(YiSyllables::YiSyllableSix),
            'ꌋ' => Ok(YiSyllables::YiSyllableSi),
            'ꌌ' => Ok(YiSyllables::YiSyllableSip),
            'ꌍ' => Ok(YiSyllables::YiSyllableSiex),
            'ꌎ' => Ok(YiSyllables::YiSyllableSie),
            'ꌏ' => Ok(YiSyllables::YiSyllableSiep),
            'ꌐ' => Ok(YiSyllables::YiSyllableSat),
            'ꌑ' => Ok(YiSyllables::YiSyllableSax),
            'ꌒ' => Ok(YiSyllables::YiSyllableSa),
            'ꌓ' => Ok(YiSyllables::YiSyllableSap),
            'ꌔ' => Ok(YiSyllables::YiSyllableSuox),
            'ꌕ' => Ok(YiSyllables::YiSyllableSuo),
            'ꌖ' => Ok(YiSyllables::YiSyllableSuop),
            'ꌗ' => Ok(YiSyllables::YiSyllableSot),
            'ꌘ' => Ok(YiSyllables::YiSyllableSox),
            'ꌙ' => Ok(YiSyllables::YiSyllableSo),
            'ꌚ' => Ok(YiSyllables::YiSyllableSop),
            'ꌛ' => Ok(YiSyllables::YiSyllableSex),
            'ꌜ' => Ok(YiSyllables::YiSyllableSe),
            'ꌝ' => Ok(YiSyllables::YiSyllableSep),
            'ꌞ' => Ok(YiSyllables::YiSyllableSut),
            'ꌟ' => Ok(YiSyllables::YiSyllableSux),
            'ꌠ' => Ok(YiSyllables::YiSyllableSu),
            'ꌡ' => Ok(YiSyllables::YiSyllableSup),
            'ꌢ' => Ok(YiSyllables::YiSyllableSurx),
            'ꌣ' => Ok(YiSyllables::YiSyllableSur),
            'ꌤ' => Ok(YiSyllables::YiSyllableSyt),
            'ꌥ' => Ok(YiSyllables::YiSyllableSyx),
            'ꌦ' => Ok(YiSyllables::YiSyllableSy),
            'ꌧ' => Ok(YiSyllables::YiSyllableSyp),
            'ꌨ' => Ok(YiSyllables::YiSyllableSyrx),
            'ꌩ' => Ok(YiSyllables::YiSyllableSyr),
            'ꌪ' => Ok(YiSyllables::YiSyllableSsit),
            'ꌫ' => Ok(YiSyllables::YiSyllableSsix),
            'ꌬ' => Ok(YiSyllables::YiSyllableSsi),
            'ꌭ' => Ok(YiSyllables::YiSyllableSsip),
            'ꌮ' => Ok(YiSyllables::YiSyllableSsiex),
            'ꌯ' => Ok(YiSyllables::YiSyllableSsie),
            'ꌰ' => Ok(YiSyllables::YiSyllableSsiep),
            'ꌱ' => Ok(YiSyllables::YiSyllableSsat),
            'ꌲ' => Ok(YiSyllables::YiSyllableSsax),
            'ꌳ' => Ok(YiSyllables::YiSyllableSsa),
            'ꌴ' => Ok(YiSyllables::YiSyllableSsap),
            'ꌵ' => Ok(YiSyllables::YiSyllableSsot),
            'ꌶ' => Ok(YiSyllables::YiSyllableSsox),
            'ꌷ' => Ok(YiSyllables::YiSyllableSso),
            'ꌸ' => Ok(YiSyllables::YiSyllableSsop),
            'ꌹ' => Ok(YiSyllables::YiSyllableSsex),
            'ꌺ' => Ok(YiSyllables::YiSyllableSse),
            'ꌻ' => Ok(YiSyllables::YiSyllableSsep),
            'ꌼ' => Ok(YiSyllables::YiSyllableSsut),
            'ꌽ' => Ok(YiSyllables::YiSyllableSsux),
            'ꌾ' => Ok(YiSyllables::YiSyllableSsu),
            'ꌿ' => Ok(YiSyllables::YiSyllableSsup),
            'ꍀ' => Ok(YiSyllables::YiSyllableSsyt),
            'ꍁ' => Ok(YiSyllables::YiSyllableSsyx),
            'ꍂ' => Ok(YiSyllables::YiSyllableSsy),
            'ꍃ' => Ok(YiSyllables::YiSyllableSsyp),
            'ꍄ' => Ok(YiSyllables::YiSyllableSsyrx),
            'ꍅ' => Ok(YiSyllables::YiSyllableSsyr),
            'ꍆ' => Ok(YiSyllables::YiSyllableZhat),
            'ꍇ' => Ok(YiSyllables::YiSyllableZhax),
            'ꍈ' => Ok(YiSyllables::YiSyllableZha),
            'ꍉ' => Ok(YiSyllables::YiSyllableZhap),
            'ꍊ' => Ok(YiSyllables::YiSyllableZhuox),
            'ꍋ' => Ok(YiSyllables::YiSyllableZhuo),
            'ꍌ' => Ok(YiSyllables::YiSyllableZhuop),
            'ꍍ' => Ok(YiSyllables::YiSyllableZhot),
            'ꍎ' => Ok(YiSyllables::YiSyllableZhox),
            'ꍏ' => Ok(YiSyllables::YiSyllableZho),
            'ꍐ' => Ok(YiSyllables::YiSyllableZhop),
            'ꍑ' => Ok(YiSyllables::YiSyllableZhet),
            'ꍒ' => Ok(YiSyllables::YiSyllableZhex),
            'ꍓ' => Ok(YiSyllables::YiSyllableZhe),
            'ꍔ' => Ok(YiSyllables::YiSyllableZhep),
            'ꍕ' => Ok(YiSyllables::YiSyllableZhut),
            'ꍖ' => Ok(YiSyllables::YiSyllableZhux),
            'ꍗ' => Ok(YiSyllables::YiSyllableZhu),
            'ꍘ' => Ok(YiSyllables::YiSyllableZhup),
            'ꍙ' => Ok(YiSyllables::YiSyllableZhurx),
            'ꍚ' => Ok(YiSyllables::YiSyllableZhur),
            'ꍛ' => Ok(YiSyllables::YiSyllableZhyt),
            'ꍜ' => Ok(YiSyllables::YiSyllableZhyx),
            'ꍝ' => Ok(YiSyllables::YiSyllableZhy),
            'ꍞ' => Ok(YiSyllables::YiSyllableZhyp),
            'ꍟ' => Ok(YiSyllables::YiSyllableZhyrx),
            'ꍠ' => Ok(YiSyllables::YiSyllableZhyr),
            'ꍡ' => Ok(YiSyllables::YiSyllableChat),
            'ꍢ' => Ok(YiSyllables::YiSyllableChax),
            'ꍣ' => Ok(YiSyllables::YiSyllableCha),
            'ꍤ' => Ok(YiSyllables::YiSyllableChap),
            'ꍥ' => Ok(YiSyllables::YiSyllableChuot),
            'ꍦ' => Ok(YiSyllables::YiSyllableChuox),
            'ꍧ' => Ok(YiSyllables::YiSyllableChuo),
            'ꍨ' => Ok(YiSyllables::YiSyllableChuop),
            'ꍩ' => Ok(YiSyllables::YiSyllableChot),
            'ꍪ' => Ok(YiSyllables::YiSyllableChox),
            'ꍫ' => Ok(YiSyllables::YiSyllableCho),
            'ꍬ' => Ok(YiSyllables::YiSyllableChop),
            'ꍭ' => Ok(YiSyllables::YiSyllableChet),
            'ꍮ' => Ok(YiSyllables::YiSyllableChex),
            'ꍯ' => Ok(YiSyllables::YiSyllableChe),
            'ꍰ' => Ok(YiSyllables::YiSyllableChep),
            'ꍱ' => Ok(YiSyllables::YiSyllableChux),
            'ꍲ' => Ok(YiSyllables::YiSyllableChu),
            'ꍳ' => Ok(YiSyllables::YiSyllableChup),
            'ꍴ' => Ok(YiSyllables::YiSyllableChurx),
            'ꍵ' => Ok(YiSyllables::YiSyllableChur),
            'ꍶ' => Ok(YiSyllables::YiSyllableChyt),
            'ꍷ' => Ok(YiSyllables::YiSyllableChyx),
            'ꍸ' => Ok(YiSyllables::YiSyllableChy),
            'ꍹ' => Ok(YiSyllables::YiSyllableChyp),
            'ꍺ' => Ok(YiSyllables::YiSyllableChyrx),
            'ꍻ' => Ok(YiSyllables::YiSyllableChyr),
            'ꍼ' => Ok(YiSyllables::YiSyllableRrax),
            'ꍽ' => Ok(YiSyllables::YiSyllableRra),
            'ꍾ' => Ok(YiSyllables::YiSyllableRruox),
            'ꍿ' => Ok(YiSyllables::YiSyllableRruo),
            'ꎀ' => Ok(YiSyllables::YiSyllableRrot),
            'ꎁ' => Ok(YiSyllables::YiSyllableRrox),
            'ꎂ' => Ok(YiSyllables::YiSyllableRro),
            'ꎃ' => Ok(YiSyllables::YiSyllableRrop),
            'ꎄ' => Ok(YiSyllables::YiSyllableRret),
            'ꎅ' => Ok(YiSyllables::YiSyllableRrex),
            'ꎆ' => Ok(YiSyllables::YiSyllableRre),
            'ꎇ' => Ok(YiSyllables::YiSyllableRrep),
            'ꎈ' => Ok(YiSyllables::YiSyllableRrut),
            'ꎉ' => Ok(YiSyllables::YiSyllableRrux),
            'ꎊ' => Ok(YiSyllables::YiSyllableRru),
            'ꎋ' => Ok(YiSyllables::YiSyllableRrup),
            'ꎌ' => Ok(YiSyllables::YiSyllableRrurx),
            'ꎍ' => Ok(YiSyllables::YiSyllableRrur),
            'ꎎ' => Ok(YiSyllables::YiSyllableRryt),
            'ꎏ' => Ok(YiSyllables::YiSyllableRryx),
            'ꎐ' => Ok(YiSyllables::YiSyllableRry),
            'ꎑ' => Ok(YiSyllables::YiSyllableRryp),
            'ꎒ' => Ok(YiSyllables::YiSyllableRryrx),
            'ꎓ' => Ok(YiSyllables::YiSyllableRryr),
            'ꎔ' => Ok(YiSyllables::YiSyllableNrat),
            'ꎕ' => Ok(YiSyllables::YiSyllableNrax),
            'ꎖ' => Ok(YiSyllables::YiSyllableNra),
            'ꎗ' => Ok(YiSyllables::YiSyllableNrap),
            'ꎘ' => Ok(YiSyllables::YiSyllableNrox),
            'ꎙ' => Ok(YiSyllables::YiSyllableNro),
            'ꎚ' => Ok(YiSyllables::YiSyllableNrop),
            'ꎛ' => Ok(YiSyllables::YiSyllableNret),
            'ꎜ' => Ok(YiSyllables::YiSyllableNrex),
            'ꎝ' => Ok(YiSyllables::YiSyllableNre),
            'ꎞ' => Ok(YiSyllables::YiSyllableNrep),
            'ꎟ' => Ok(YiSyllables::YiSyllableNrut),
            'ꎠ' => Ok(YiSyllables::YiSyllableNrux),
            'ꎡ' => Ok(YiSyllables::YiSyllableNru),
            'ꎢ' => Ok(YiSyllables::YiSyllableNrup),
            'ꎣ' => Ok(YiSyllables::YiSyllableNrurx),
            'ꎤ' => Ok(YiSyllables::YiSyllableNrur),
            'ꎥ' => Ok(YiSyllables::YiSyllableNryt),
            'ꎦ' => Ok(YiSyllables::YiSyllableNryx),
            'ꎧ' => Ok(YiSyllables::YiSyllableNry),
            'ꎨ' => Ok(YiSyllables::YiSyllableNryp),
            'ꎩ' => Ok(YiSyllables::YiSyllableNryrx),
            'ꎪ' => Ok(YiSyllables::YiSyllableNryr),
            'ꎫ' => Ok(YiSyllables::YiSyllableShat),
            'ꎬ' => Ok(YiSyllables::YiSyllableShax),
            'ꎭ' => Ok(YiSyllables::YiSyllableSha),
            'ꎮ' => Ok(YiSyllables::YiSyllableShap),
            'ꎯ' => Ok(YiSyllables::YiSyllableShuox),
            'ꎰ' => Ok(YiSyllables::YiSyllableShuo),
            'ꎱ' => Ok(YiSyllables::YiSyllableShuop),
            'ꎲ' => Ok(YiSyllables::YiSyllableShot),
            'ꎳ' => Ok(YiSyllables::YiSyllableShox),
            'ꎴ' => Ok(YiSyllables::YiSyllableSho),
            'ꎵ' => Ok(YiSyllables::YiSyllableShop),
            'ꎶ' => Ok(YiSyllables::YiSyllableShet),
            'ꎷ' => Ok(YiSyllables::YiSyllableShex),
            'ꎸ' => Ok(YiSyllables::YiSyllableShe),
            'ꎹ' => Ok(YiSyllables::YiSyllableShep),
            'ꎺ' => Ok(YiSyllables::YiSyllableShut),
            'ꎻ' => Ok(YiSyllables::YiSyllableShux),
            'ꎼ' => Ok(YiSyllables::YiSyllableShu),
            'ꎽ' => Ok(YiSyllables::YiSyllableShup),
            'ꎾ' => Ok(YiSyllables::YiSyllableShurx),
            'ꎿ' => Ok(YiSyllables::YiSyllableShur),
            'ꏀ' => Ok(YiSyllables::YiSyllableShyt),
            'ꏁ' => Ok(YiSyllables::YiSyllableShyx),
            'ꏂ' => Ok(YiSyllables::YiSyllableShy),
            'ꏃ' => Ok(YiSyllables::YiSyllableShyp),
            'ꏄ' => Ok(YiSyllables::YiSyllableShyrx),
            'ꏅ' => Ok(YiSyllables::YiSyllableShyr),
            'ꏆ' => Ok(YiSyllables::YiSyllableRat),
            'ꏇ' => Ok(YiSyllables::YiSyllableRax),
            'ꏈ' => Ok(YiSyllables::YiSyllableRa),
            'ꏉ' => Ok(YiSyllables::YiSyllableRap),
            'ꏊ' => Ok(YiSyllables::YiSyllableRuox),
            'ꏋ' => Ok(YiSyllables::YiSyllableRuo),
            'ꏌ' => Ok(YiSyllables::YiSyllableRuop),
            'ꏍ' => Ok(YiSyllables::YiSyllableRot),
            'ꏎ' => Ok(YiSyllables::YiSyllableRox),
            'ꏏ' => Ok(YiSyllables::YiSyllableRo),
            'ꏐ' => Ok(YiSyllables::YiSyllableRop),
            'ꏑ' => Ok(YiSyllables::YiSyllableRex),
            'ꏒ' => Ok(YiSyllables::YiSyllableRe),
            'ꏓ' => Ok(YiSyllables::YiSyllableRep),
            'ꏔ' => Ok(YiSyllables::YiSyllableRut),
            'ꏕ' => Ok(YiSyllables::YiSyllableRux),
            'ꏖ' => Ok(YiSyllables::YiSyllableRu),
            'ꏗ' => Ok(YiSyllables::YiSyllableRup),
            'ꏘ' => Ok(YiSyllables::YiSyllableRurx),
            'ꏙ' => Ok(YiSyllables::YiSyllableRur),
            'ꏚ' => Ok(YiSyllables::YiSyllableRyt),
            'ꏛ' => Ok(YiSyllables::YiSyllableRyx),
            'ꏜ' => Ok(YiSyllables::YiSyllableRy),
            'ꏝ' => Ok(YiSyllables::YiSyllableRyp),
            'ꏞ' => Ok(YiSyllables::YiSyllableRyrx),
            'ꏟ' => Ok(YiSyllables::YiSyllableRyr),
            'ꏠ' => Ok(YiSyllables::YiSyllableJit),
            'ꏡ' => Ok(YiSyllables::YiSyllableJix),
            'ꏢ' => Ok(YiSyllables::YiSyllableJi),
            'ꏣ' => Ok(YiSyllables::YiSyllableJip),
            'ꏤ' => Ok(YiSyllables::YiSyllableJiet),
            'ꏥ' => Ok(YiSyllables::YiSyllableJiex),
            'ꏦ' => Ok(YiSyllables::YiSyllableJie),
            'ꏧ' => Ok(YiSyllables::YiSyllableJiep),
            'ꏨ' => Ok(YiSyllables::YiSyllableJuot),
            'ꏩ' => Ok(YiSyllables::YiSyllableJuox),
            'ꏪ' => Ok(YiSyllables::YiSyllableJuo),
            'ꏫ' => Ok(YiSyllables::YiSyllableJuop),
            'ꏬ' => Ok(YiSyllables::YiSyllableJot),
            'ꏭ' => Ok(YiSyllables::YiSyllableJox),
            'ꏮ' => Ok(YiSyllables::YiSyllableJo),
            'ꏯ' => Ok(YiSyllables::YiSyllableJop),
            'ꏰ' => Ok(YiSyllables::YiSyllableJut),
            'ꏱ' => Ok(YiSyllables::YiSyllableJux),
            'ꏲ' => Ok(YiSyllables::YiSyllableJu),
            'ꏳ' => Ok(YiSyllables::YiSyllableJup),
            'ꏴ' => Ok(YiSyllables::YiSyllableJurx),
            'ꏵ' => Ok(YiSyllables::YiSyllableJur),
            'ꏶ' => Ok(YiSyllables::YiSyllableJyt),
            'ꏷ' => Ok(YiSyllables::YiSyllableJyx),
            'ꏸ' => Ok(YiSyllables::YiSyllableJy),
            'ꏹ' => Ok(YiSyllables::YiSyllableJyp),
            'ꏺ' => Ok(YiSyllables::YiSyllableJyrx),
            'ꏻ' => Ok(YiSyllables::YiSyllableJyr),
            'ꏼ' => Ok(YiSyllables::YiSyllableQit),
            'ꏽ' => Ok(YiSyllables::YiSyllableQix),
            'ꏾ' => Ok(YiSyllables::YiSyllableQi),
            'ꏿ' => Ok(YiSyllables::YiSyllableQip),
            'ꐀ' => Ok(YiSyllables::YiSyllableQiet),
            'ꐁ' => Ok(YiSyllables::YiSyllableQiex),
            'ꐂ' => Ok(YiSyllables::YiSyllableQie),
            'ꐃ' => Ok(YiSyllables::YiSyllableQiep),
            'ꐄ' => Ok(YiSyllables::YiSyllableQuot),
            'ꐅ' => Ok(YiSyllables::YiSyllableQuox),
            'ꐆ' => Ok(YiSyllables::YiSyllableQuo),
            'ꐇ' => Ok(YiSyllables::YiSyllableQuop),
            'ꐈ' => Ok(YiSyllables::YiSyllableQot),
            'ꐉ' => Ok(YiSyllables::YiSyllableQox),
            'ꐊ' => Ok(YiSyllables::YiSyllableQo),
            'ꐋ' => Ok(YiSyllables::YiSyllableQop),
            'ꐌ' => Ok(YiSyllables::YiSyllableQut),
            'ꐍ' => Ok(YiSyllables::YiSyllableQux),
            'ꐎ' => Ok(YiSyllables::YiSyllableQu),
            'ꐏ' => Ok(YiSyllables::YiSyllableQup),
            'ꐐ' => Ok(YiSyllables::YiSyllableQurx),
            'ꐑ' => Ok(YiSyllables::YiSyllableQur),
            'ꐒ' => Ok(YiSyllables::YiSyllableQyt),
            'ꐓ' => Ok(YiSyllables::YiSyllableQyx),
            'ꐔ' => Ok(YiSyllables::YiSyllableQy),
            'ꐕ' => Ok(YiSyllables::YiSyllableQyp),
            'ꐖ' => Ok(YiSyllables::YiSyllableQyrx),
            'ꐗ' => Ok(YiSyllables::YiSyllableQyr),
            'ꐘ' => Ok(YiSyllables::YiSyllableJjit),
            'ꐙ' => Ok(YiSyllables::YiSyllableJjix),
            'ꐚ' => Ok(YiSyllables::YiSyllableJji),
            'ꐛ' => Ok(YiSyllables::YiSyllableJjip),
            'ꐜ' => Ok(YiSyllables::YiSyllableJjiet),
            'ꐝ' => Ok(YiSyllables::YiSyllableJjiex),
            'ꐞ' => Ok(YiSyllables::YiSyllableJjie),
            'ꐟ' => Ok(YiSyllables::YiSyllableJjiep),
            'ꐠ' => Ok(YiSyllables::YiSyllableJjuox),
            'ꐡ' => Ok(YiSyllables::YiSyllableJjuo),
            'ꐢ' => Ok(YiSyllables::YiSyllableJjuop),
            'ꐣ' => Ok(YiSyllables::YiSyllableJjot),
            'ꐤ' => Ok(YiSyllables::YiSyllableJjox),
            'ꐥ' => Ok(YiSyllables::YiSyllableJjo),
            'ꐦ' => Ok(YiSyllables::YiSyllableJjop),
            'ꐧ' => Ok(YiSyllables::YiSyllableJjut),
            'ꐨ' => Ok(YiSyllables::YiSyllableJjux),
            'ꐩ' => Ok(YiSyllables::YiSyllableJju),
            'ꐪ' => Ok(YiSyllables::YiSyllableJjup),
            'ꐫ' => Ok(YiSyllables::YiSyllableJjurx),
            'ꐬ' => Ok(YiSyllables::YiSyllableJjur),
            'ꐭ' => Ok(YiSyllables::YiSyllableJjyt),
            'ꐮ' => Ok(YiSyllables::YiSyllableJjyx),
            'ꐯ' => Ok(YiSyllables::YiSyllableJjy),
            'ꐰ' => Ok(YiSyllables::YiSyllableJjyp),
            'ꐱ' => Ok(YiSyllables::YiSyllableNjit),
            'ꐲ' => Ok(YiSyllables::YiSyllableNjix),
            'ꐳ' => Ok(YiSyllables::YiSyllableNji),
            'ꐴ' => Ok(YiSyllables::YiSyllableNjip),
            'ꐵ' => Ok(YiSyllables::YiSyllableNjiet),
            'ꐶ' => Ok(YiSyllables::YiSyllableNjiex),
            'ꐷ' => Ok(YiSyllables::YiSyllableNjie),
            'ꐸ' => Ok(YiSyllables::YiSyllableNjiep),
            'ꐹ' => Ok(YiSyllables::YiSyllableNjuox),
            'ꐺ' => Ok(YiSyllables::YiSyllableNjuo),
            'ꐻ' => Ok(YiSyllables::YiSyllableNjot),
            'ꐼ' => Ok(YiSyllables::YiSyllableNjox),
            'ꐽ' => Ok(YiSyllables::YiSyllableNjo),
            'ꐾ' => Ok(YiSyllables::YiSyllableNjop),
            'ꐿ' => Ok(YiSyllables::YiSyllableNjux),
            'ꑀ' => Ok(YiSyllables::YiSyllableNju),
            'ꑁ' => Ok(YiSyllables::YiSyllableNjup),
            'ꑂ' => Ok(YiSyllables::YiSyllableNjurx),
            'ꑃ' => Ok(YiSyllables::YiSyllableNjur),
            'ꑄ' => Ok(YiSyllables::YiSyllableNjyt),
            'ꑅ' => Ok(YiSyllables::YiSyllableNjyx),
            'ꑆ' => Ok(YiSyllables::YiSyllableNjy),
            'ꑇ' => Ok(YiSyllables::YiSyllableNjyp),
            'ꑈ' => Ok(YiSyllables::YiSyllableNjyrx),
            'ꑉ' => Ok(YiSyllables::YiSyllableNjyr),
            'ꑊ' => Ok(YiSyllables::YiSyllableNyit),
            'ꑋ' => Ok(YiSyllables::YiSyllableNyix),
            'ꑌ' => Ok(YiSyllables::YiSyllableNyi),
            'ꑍ' => Ok(YiSyllables::YiSyllableNyip),
            'ꑎ' => Ok(YiSyllables::YiSyllableNyiet),
            'ꑏ' => Ok(YiSyllables::YiSyllableNyiex),
            'ꑐ' => Ok(YiSyllables::YiSyllableNyie),
            'ꑑ' => Ok(YiSyllables::YiSyllableNyiep),
            'ꑒ' => Ok(YiSyllables::YiSyllableNyuox),
            'ꑓ' => Ok(YiSyllables::YiSyllableNyuo),
            'ꑔ' => Ok(YiSyllables::YiSyllableNyuop),
            'ꑕ' => Ok(YiSyllables::YiSyllableNyot),
            'ꑖ' => Ok(YiSyllables::YiSyllableNyox),
            'ꑗ' => Ok(YiSyllables::YiSyllableNyo),
            'ꑘ' => Ok(YiSyllables::YiSyllableNyop),
            'ꑙ' => Ok(YiSyllables::YiSyllableNyut),
            'ꑚ' => Ok(YiSyllables::YiSyllableNyux),
            'ꑛ' => Ok(YiSyllables::YiSyllableNyu),
            'ꑜ' => Ok(YiSyllables::YiSyllableNyup),
            'ꑝ' => Ok(YiSyllables::YiSyllableXit),
            'ꑞ' => Ok(YiSyllables::YiSyllableXix),
            'ꑟ' => Ok(YiSyllables::YiSyllableXi),
            'ꑠ' => Ok(YiSyllables::YiSyllableXip),
            'ꑡ' => Ok(YiSyllables::YiSyllableXiet),
            'ꑢ' => Ok(YiSyllables::YiSyllableXiex),
            'ꑣ' => Ok(YiSyllables::YiSyllableXie),
            'ꑤ' => Ok(YiSyllables::YiSyllableXiep),
            'ꑥ' => Ok(YiSyllables::YiSyllableXuox),
            'ꑦ' => Ok(YiSyllables::YiSyllableXuo),
            'ꑧ' => Ok(YiSyllables::YiSyllableXot),
            'ꑨ' => Ok(YiSyllables::YiSyllableXox),
            'ꑩ' => Ok(YiSyllables::YiSyllableXo),
            'ꑪ' => Ok(YiSyllables::YiSyllableXop),
            'ꑫ' => Ok(YiSyllables::YiSyllableXyt),
            'ꑬ' => Ok(YiSyllables::YiSyllableXyx),
            'ꑭ' => Ok(YiSyllables::YiSyllableXy),
            'ꑮ' => Ok(YiSyllables::YiSyllableXyp),
            'ꑯ' => Ok(YiSyllables::YiSyllableXyrx),
            'ꑰ' => Ok(YiSyllables::YiSyllableXyr),
            'ꑱ' => Ok(YiSyllables::YiSyllableYit),
            'ꑲ' => Ok(YiSyllables::YiSyllableYix),
            'ꑳ' => Ok(YiSyllables::YiSyllableYi),
            'ꑴ' => Ok(YiSyllables::YiSyllableYip),
            'ꑵ' => Ok(YiSyllables::YiSyllableYiet),
            'ꑶ' => Ok(YiSyllables::YiSyllableYiex),
            'ꑷ' => Ok(YiSyllables::YiSyllableYie),
            'ꑸ' => Ok(YiSyllables::YiSyllableYiep),
            'ꑹ' => Ok(YiSyllables::YiSyllableYuot),
            'ꑺ' => Ok(YiSyllables::YiSyllableYuox),
            'ꑻ' => Ok(YiSyllables::YiSyllableYuo),
            'ꑼ' => Ok(YiSyllables::YiSyllableYuop),
            'ꑽ' => Ok(YiSyllables::YiSyllableYot),
            'ꑾ' => Ok(YiSyllables::YiSyllableYox),
            'ꑿ' => Ok(YiSyllables::YiSyllableYo),
            'ꒀ' => Ok(YiSyllables::YiSyllableYop),
            'ꒁ' => Ok(YiSyllables::YiSyllableYut),
            'ꒂ' => Ok(YiSyllables::YiSyllableYux),
            'ꒃ' => Ok(YiSyllables::YiSyllableYu),
            'ꒄ' => Ok(YiSyllables::YiSyllableYup),
            'ꒅ' => Ok(YiSyllables::YiSyllableYurx),
            'ꒆ' => Ok(YiSyllables::YiSyllableYur),
            'ꒇ' => Ok(YiSyllables::YiSyllableYyt),
            'ꒈ' => Ok(YiSyllables::YiSyllableYyx),
            'ꒉ' => Ok(YiSyllables::YiSyllableYy),
            'ꒊ' => Ok(YiSyllables::YiSyllableYyp),
            'ꒋ' => Ok(YiSyllables::YiSyllableYyrx),
            'ꒌ' => Ok(YiSyllables::YiSyllableYyr),
            _ => Err(()),
        }
    }
}

impl Into<u32> for YiSyllables {
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

impl std::convert::TryFrom<u32> for YiSyllables {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for YiSyllables {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl YiSyllables {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        YiSyllables::YiSyllableIt
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            YiSyllables::YiSyllableIt => "yi syllable it",
            YiSyllables::YiSyllableIx => "yi syllable ix",
            YiSyllables::YiSyllableI => "yi syllable i",
            YiSyllables::YiSyllableIp => "yi syllable ip",
            YiSyllables::YiSyllableIet => "yi syllable iet",
            YiSyllables::YiSyllableIex => "yi syllable iex",
            YiSyllables::YiSyllableIe => "yi syllable ie",
            YiSyllables::YiSyllableIep => "yi syllable iep",
            YiSyllables::YiSyllableAt => "yi syllable at",
            YiSyllables::YiSyllableAx => "yi syllable ax",
            YiSyllables::YiSyllableA => "yi syllable a",
            YiSyllables::YiSyllableAp => "yi syllable ap",
            YiSyllables::YiSyllableUox => "yi syllable uox",
            YiSyllables::YiSyllableUo => "yi syllable uo",
            YiSyllables::YiSyllableUop => "yi syllable uop",
            YiSyllables::YiSyllableOt => "yi syllable ot",
            YiSyllables::YiSyllableOx => "yi syllable ox",
            YiSyllables::YiSyllableO => "yi syllable o",
            YiSyllables::YiSyllableOp => "yi syllable op",
            YiSyllables::YiSyllableEx => "yi syllable ex",
            YiSyllables::YiSyllableE => "yi syllable e",
            YiSyllables::YiSyllableWu => "yi syllable wu",
            YiSyllables::YiSyllableBit => "yi syllable bit",
            YiSyllables::YiSyllableBix => "yi syllable bix",
            YiSyllables::YiSyllableBi => "yi syllable bi",
            YiSyllables::YiSyllableBip => "yi syllable bip",
            YiSyllables::YiSyllableBiet => "yi syllable biet",
            YiSyllables::YiSyllableBiex => "yi syllable biex",
            YiSyllables::YiSyllableBie => "yi syllable bie",
            YiSyllables::YiSyllableBiep => "yi syllable biep",
            YiSyllables::YiSyllableBat => "yi syllable bat",
            YiSyllables::YiSyllableBax => "yi syllable bax",
            YiSyllables::YiSyllableBa => "yi syllable ba",
            YiSyllables::YiSyllableBap => "yi syllable bap",
            YiSyllables::YiSyllableBuox => "yi syllable buox",
            YiSyllables::YiSyllableBuo => "yi syllable buo",
            YiSyllables::YiSyllableBuop => "yi syllable buop",
            YiSyllables::YiSyllableBot => "yi syllable bot",
            YiSyllables::YiSyllableBox => "yi syllable box",
            YiSyllables::YiSyllableBo => "yi syllable bo",
            YiSyllables::YiSyllableBop => "yi syllable bop",
            YiSyllables::YiSyllableBex => "yi syllable bex",
            YiSyllables::YiSyllableBe => "yi syllable be",
            YiSyllables::YiSyllableBep => "yi syllable bep",
            YiSyllables::YiSyllableBut => "yi syllable but",
            YiSyllables::YiSyllableBux => "yi syllable bux",
            YiSyllables::YiSyllableBu => "yi syllable bu",
            YiSyllables::YiSyllableBup => "yi syllable bup",
            YiSyllables::YiSyllableBurx => "yi syllable burx",
            YiSyllables::YiSyllableBur => "yi syllable bur",
            YiSyllables::YiSyllableByt => "yi syllable byt",
            YiSyllables::YiSyllableByx => "yi syllable byx",
            YiSyllables::YiSyllableBy => "yi syllable by",
            YiSyllables::YiSyllableByp => "yi syllable byp",
            YiSyllables::YiSyllableByrx => "yi syllable byrx",
            YiSyllables::YiSyllableByr => "yi syllable byr",
            YiSyllables::YiSyllablePit => "yi syllable pit",
            YiSyllables::YiSyllablePix => "yi syllable pix",
            YiSyllables::YiSyllablePi => "yi syllable pi",
            YiSyllables::YiSyllablePip => "yi syllable pip",
            YiSyllables::YiSyllablePiex => "yi syllable piex",
            YiSyllables::YiSyllablePie => "yi syllable pie",
            YiSyllables::YiSyllablePiep => "yi syllable piep",
            YiSyllables::YiSyllablePat => "yi syllable pat",
            YiSyllables::YiSyllablePax => "yi syllable pax",
            YiSyllables::YiSyllablePa => "yi syllable pa",
            YiSyllables::YiSyllablePap => "yi syllable pap",
            YiSyllables::YiSyllablePuox => "yi syllable puox",
            YiSyllables::YiSyllablePuo => "yi syllable puo",
            YiSyllables::YiSyllablePuop => "yi syllable puop",
            YiSyllables::YiSyllablePot => "yi syllable pot",
            YiSyllables::YiSyllablePox => "yi syllable pox",
            YiSyllables::YiSyllablePo => "yi syllable po",
            YiSyllables::YiSyllablePop => "yi syllable pop",
            YiSyllables::YiSyllablePut => "yi syllable put",
            YiSyllables::YiSyllablePux => "yi syllable pux",
            YiSyllables::YiSyllablePu => "yi syllable pu",
            YiSyllables::YiSyllablePup => "yi syllable pup",
            YiSyllables::YiSyllablePurx => "yi syllable purx",
            YiSyllables::YiSyllablePur => "yi syllable pur",
            YiSyllables::YiSyllablePyt => "yi syllable pyt",
            YiSyllables::YiSyllablePyx => "yi syllable pyx",
            YiSyllables::YiSyllablePy => "yi syllable py",
            YiSyllables::YiSyllablePyp => "yi syllable pyp",
            YiSyllables::YiSyllablePyrx => "yi syllable pyrx",
            YiSyllables::YiSyllablePyr => "yi syllable pyr",
            YiSyllables::YiSyllableBbit => "yi syllable bbit",
            YiSyllables::YiSyllableBbix => "yi syllable bbix",
            YiSyllables::YiSyllableBbi => "yi syllable bbi",
            YiSyllables::YiSyllableBbip => "yi syllable bbip",
            YiSyllables::YiSyllableBbiet => "yi syllable bbiet",
            YiSyllables::YiSyllableBbiex => "yi syllable bbiex",
            YiSyllables::YiSyllableBbie => "yi syllable bbie",
            YiSyllables::YiSyllableBbiep => "yi syllable bbiep",
            YiSyllables::YiSyllableBbat => "yi syllable bbat",
            YiSyllables::YiSyllableBbax => "yi syllable bbax",
            YiSyllables::YiSyllableBba => "yi syllable bba",
            YiSyllables::YiSyllableBbap => "yi syllable bbap",
            YiSyllables::YiSyllableBbuox => "yi syllable bbuox",
            YiSyllables::YiSyllableBbuo => "yi syllable bbuo",
            YiSyllables::YiSyllableBbuop => "yi syllable bbuop",
            YiSyllables::YiSyllableBbot => "yi syllable bbot",
            YiSyllables::YiSyllableBbox => "yi syllable bbox",
            YiSyllables::YiSyllableBbo => "yi syllable bbo",
            YiSyllables::YiSyllableBbop => "yi syllable bbop",
            YiSyllables::YiSyllableBbex => "yi syllable bbex",
            YiSyllables::YiSyllableBbe => "yi syllable bbe",
            YiSyllables::YiSyllableBbep => "yi syllable bbep",
            YiSyllables::YiSyllableBbut => "yi syllable bbut",
            YiSyllables::YiSyllableBbux => "yi syllable bbux",
            YiSyllables::YiSyllableBbu => "yi syllable bbu",
            YiSyllables::YiSyllableBbup => "yi syllable bbup",
            YiSyllables::YiSyllableBburx => "yi syllable bburx",
            YiSyllables::YiSyllableBbur => "yi syllable bbur",
            YiSyllables::YiSyllableBbyt => "yi syllable bbyt",
            YiSyllables::YiSyllableBbyx => "yi syllable bbyx",
            YiSyllables::YiSyllableBby => "yi syllable bby",
            YiSyllables::YiSyllableBbyp => "yi syllable bbyp",
            YiSyllables::YiSyllableNbit => "yi syllable nbit",
            YiSyllables::YiSyllableNbix => "yi syllable nbix",
            YiSyllables::YiSyllableNbi => "yi syllable nbi",
            YiSyllables::YiSyllableNbip => "yi syllable nbip",
            YiSyllables::YiSyllableNbiex => "yi syllable nbiex",
            YiSyllables::YiSyllableNbie => "yi syllable nbie",
            YiSyllables::YiSyllableNbiep => "yi syllable nbiep",
            YiSyllables::YiSyllableNbat => "yi syllable nbat",
            YiSyllables::YiSyllableNbax => "yi syllable nbax",
            YiSyllables::YiSyllableNba => "yi syllable nba",
            YiSyllables::YiSyllableNbap => "yi syllable nbap",
            YiSyllables::YiSyllableNbot => "yi syllable nbot",
            YiSyllables::YiSyllableNbox => "yi syllable nbox",
            YiSyllables::YiSyllableNbo => "yi syllable nbo",
            YiSyllables::YiSyllableNbop => "yi syllable nbop",
            YiSyllables::YiSyllableNbut => "yi syllable nbut",
            YiSyllables::YiSyllableNbux => "yi syllable nbux",
            YiSyllables::YiSyllableNbu => "yi syllable nbu",
            YiSyllables::YiSyllableNbup => "yi syllable nbup",
            YiSyllables::YiSyllableNburx => "yi syllable nburx",
            YiSyllables::YiSyllableNbur => "yi syllable nbur",
            YiSyllables::YiSyllableNbyt => "yi syllable nbyt",
            YiSyllables::YiSyllableNbyx => "yi syllable nbyx",
            YiSyllables::YiSyllableNby => "yi syllable nby",
            YiSyllables::YiSyllableNbyp => "yi syllable nbyp",
            YiSyllables::YiSyllableNbyrx => "yi syllable nbyrx",
            YiSyllables::YiSyllableNbyr => "yi syllable nbyr",
            YiSyllables::YiSyllableHmit => "yi syllable hmit",
            YiSyllables::YiSyllableHmix => "yi syllable hmix",
            YiSyllables::YiSyllableHmi => "yi syllable hmi",
            YiSyllables::YiSyllableHmip => "yi syllable hmip",
            YiSyllables::YiSyllableHmiex => "yi syllable hmiex",
            YiSyllables::YiSyllableHmie => "yi syllable hmie",
            YiSyllables::YiSyllableHmiep => "yi syllable hmiep",
            YiSyllables::YiSyllableHmat => "yi syllable hmat",
            YiSyllables::YiSyllableHmax => "yi syllable hmax",
            YiSyllables::YiSyllableHma => "yi syllable hma",
            YiSyllables::YiSyllableHmap => "yi syllable hmap",
            YiSyllables::YiSyllableHmuox => "yi syllable hmuox",
            YiSyllables::YiSyllableHmuo => "yi syllable hmuo",
            YiSyllables::YiSyllableHmuop => "yi syllable hmuop",
            YiSyllables::YiSyllableHmot => "yi syllable hmot",
            YiSyllables::YiSyllableHmox => "yi syllable hmox",
            YiSyllables::YiSyllableHmo => "yi syllable hmo",
            YiSyllables::YiSyllableHmop => "yi syllable hmop",
            YiSyllables::YiSyllableHmut => "yi syllable hmut",
            YiSyllables::YiSyllableHmux => "yi syllable hmux",
            YiSyllables::YiSyllableHmu => "yi syllable hmu",
            YiSyllables::YiSyllableHmup => "yi syllable hmup",
            YiSyllables::YiSyllableHmurx => "yi syllable hmurx",
            YiSyllables::YiSyllableHmur => "yi syllable hmur",
            YiSyllables::YiSyllableHmyx => "yi syllable hmyx",
            YiSyllables::YiSyllableHmy => "yi syllable hmy",
            YiSyllables::YiSyllableHmyp => "yi syllable hmyp",
            YiSyllables::YiSyllableHmyrx => "yi syllable hmyrx",
            YiSyllables::YiSyllableHmyr => "yi syllable hmyr",
            YiSyllables::YiSyllableMit => "yi syllable mit",
            YiSyllables::YiSyllableMix => "yi syllable mix",
            YiSyllables::YiSyllableMi => "yi syllable mi",
            YiSyllables::YiSyllableMip => "yi syllable mip",
            YiSyllables::YiSyllableMiex => "yi syllable miex",
            YiSyllables::YiSyllableMie => "yi syllable mie",
            YiSyllables::YiSyllableMiep => "yi syllable miep",
            YiSyllables::YiSyllableMat => "yi syllable mat",
            YiSyllables::YiSyllableMax => "yi syllable max",
            YiSyllables::YiSyllableMa => "yi syllable ma",
            YiSyllables::YiSyllableMap => "yi syllable map",
            YiSyllables::YiSyllableMuot => "yi syllable muot",
            YiSyllables::YiSyllableMuox => "yi syllable muox",
            YiSyllables::YiSyllableMuo => "yi syllable muo",
            YiSyllables::YiSyllableMuop => "yi syllable muop",
            YiSyllables::YiSyllableMot => "yi syllable mot",
            YiSyllables::YiSyllableMox => "yi syllable mox",
            YiSyllables::YiSyllableMo => "yi syllable mo",
            YiSyllables::YiSyllableMop => "yi syllable mop",
            YiSyllables::YiSyllableMex => "yi syllable mex",
            YiSyllables::YiSyllableMe => "yi syllable me",
            YiSyllables::YiSyllableMut => "yi syllable mut",
            YiSyllables::YiSyllableMux => "yi syllable mux",
            YiSyllables::YiSyllableMu => "yi syllable mu",
            YiSyllables::YiSyllableMup => "yi syllable mup",
            YiSyllables::YiSyllableMurx => "yi syllable murx",
            YiSyllables::YiSyllableMur => "yi syllable mur",
            YiSyllables::YiSyllableMyt => "yi syllable myt",
            YiSyllables::YiSyllableMyx => "yi syllable myx",
            YiSyllables::YiSyllableMy => "yi syllable my",
            YiSyllables::YiSyllableMyp => "yi syllable myp",
            YiSyllables::YiSyllableFit => "yi syllable fit",
            YiSyllables::YiSyllableFix => "yi syllable fix",
            YiSyllables::YiSyllableFi => "yi syllable fi",
            YiSyllables::YiSyllableFip => "yi syllable fip",
            YiSyllables::YiSyllableFat => "yi syllable fat",
            YiSyllables::YiSyllableFax => "yi syllable fax",
            YiSyllables::YiSyllableFa => "yi syllable fa",
            YiSyllables::YiSyllableFap => "yi syllable fap",
            YiSyllables::YiSyllableFox => "yi syllable fox",
            YiSyllables::YiSyllableFo => "yi syllable fo",
            YiSyllables::YiSyllableFop => "yi syllable fop",
            YiSyllables::YiSyllableFut => "yi syllable fut",
            YiSyllables::YiSyllableFux => "yi syllable fux",
            YiSyllables::YiSyllableFu => "yi syllable fu",
            YiSyllables::YiSyllableFup => "yi syllable fup",
            YiSyllables::YiSyllableFurx => "yi syllable furx",
            YiSyllables::YiSyllableFur => "yi syllable fur",
            YiSyllables::YiSyllableFyt => "yi syllable fyt",
            YiSyllables::YiSyllableFyx => "yi syllable fyx",
            YiSyllables::YiSyllableFy => "yi syllable fy",
            YiSyllables::YiSyllableFyp => "yi syllable fyp",
            YiSyllables::YiSyllableVit => "yi syllable vit",
            YiSyllables::YiSyllableVix => "yi syllable vix",
            YiSyllables::YiSyllableVi => "yi syllable vi",
            YiSyllables::YiSyllableVip => "yi syllable vip",
            YiSyllables::YiSyllableViet => "yi syllable viet",
            YiSyllables::YiSyllableViex => "yi syllable viex",
            YiSyllables::YiSyllableVie => "yi syllable vie",
            YiSyllables::YiSyllableViep => "yi syllable viep",
            YiSyllables::YiSyllableVat => "yi syllable vat",
            YiSyllables::YiSyllableVax => "yi syllable vax",
            YiSyllables::YiSyllableVa => "yi syllable va",
            YiSyllables::YiSyllableVap => "yi syllable vap",
            YiSyllables::YiSyllableVot => "yi syllable vot",
            YiSyllables::YiSyllableVox => "yi syllable vox",
            YiSyllables::YiSyllableVo => "yi syllable vo",
            YiSyllables::YiSyllableVop => "yi syllable vop",
            YiSyllables::YiSyllableVex => "yi syllable vex",
            YiSyllables::YiSyllableVep => "yi syllable vep",
            YiSyllables::YiSyllableVut => "yi syllable vut",
            YiSyllables::YiSyllableVux => "yi syllable vux",
            YiSyllables::YiSyllableVu => "yi syllable vu",
            YiSyllables::YiSyllableVup => "yi syllable vup",
            YiSyllables::YiSyllableVurx => "yi syllable vurx",
            YiSyllables::YiSyllableVur => "yi syllable vur",
            YiSyllables::YiSyllableVyt => "yi syllable vyt",
            YiSyllables::YiSyllableVyx => "yi syllable vyx",
            YiSyllables::YiSyllableVy => "yi syllable vy",
            YiSyllables::YiSyllableVyp => "yi syllable vyp",
            YiSyllables::YiSyllableVyrx => "yi syllable vyrx",
            YiSyllables::YiSyllableVyr => "yi syllable vyr",
            YiSyllables::YiSyllableDit => "yi syllable dit",
            YiSyllables::YiSyllableDix => "yi syllable dix",
            YiSyllables::YiSyllableDi => "yi syllable di",
            YiSyllables::YiSyllableDip => "yi syllable dip",
            YiSyllables::YiSyllableDiex => "yi syllable diex",
            YiSyllables::YiSyllableDie => "yi syllable die",
            YiSyllables::YiSyllableDiep => "yi syllable diep",
            YiSyllables::YiSyllableDat => "yi syllable dat",
            YiSyllables::YiSyllableDax => "yi syllable dax",
            YiSyllables::YiSyllableDa => "yi syllable da",
            YiSyllables::YiSyllableDap => "yi syllable dap",
            YiSyllables::YiSyllableDuox => "yi syllable duox",
            YiSyllables::YiSyllableDuo => "yi syllable duo",
            YiSyllables::YiSyllableDot => "yi syllable dot",
            YiSyllables::YiSyllableDox => "yi syllable dox",
            YiSyllables::YiSyllableDo => "yi syllable do",
            YiSyllables::YiSyllableDop => "yi syllable dop",
            YiSyllables::YiSyllableDex => "yi syllable dex",
            YiSyllables::YiSyllableDe => "yi syllable de",
            YiSyllables::YiSyllableDep => "yi syllable dep",
            YiSyllables::YiSyllableDut => "yi syllable dut",
            YiSyllables::YiSyllableDux => "yi syllable dux",
            YiSyllables::YiSyllableDu => "yi syllable du",
            YiSyllables::YiSyllableDup => "yi syllable dup",
            YiSyllables::YiSyllableDurx => "yi syllable durx",
            YiSyllables::YiSyllableDur => "yi syllable dur",
            YiSyllables::YiSyllableTit => "yi syllable tit",
            YiSyllables::YiSyllableTix => "yi syllable tix",
            YiSyllables::YiSyllableTi => "yi syllable ti",
            YiSyllables::YiSyllableTip => "yi syllable tip",
            YiSyllables::YiSyllableTiex => "yi syllable tiex",
            YiSyllables::YiSyllableTie => "yi syllable tie",
            YiSyllables::YiSyllableTiep => "yi syllable tiep",
            YiSyllables::YiSyllableTat => "yi syllable tat",
            YiSyllables::YiSyllableTax => "yi syllable tax",
            YiSyllables::YiSyllableTa => "yi syllable ta",
            YiSyllables::YiSyllableTap => "yi syllable tap",
            YiSyllables::YiSyllableTuot => "yi syllable tuot",
            YiSyllables::YiSyllableTuox => "yi syllable tuox",
            YiSyllables::YiSyllableTuo => "yi syllable tuo",
            YiSyllables::YiSyllableTuop => "yi syllable tuop",
            YiSyllables::YiSyllableTot => "yi syllable tot",
            YiSyllables::YiSyllableTox => "yi syllable tox",
            YiSyllables::YiSyllableTo => "yi syllable to",
            YiSyllables::YiSyllableTop => "yi syllable top",
            YiSyllables::YiSyllableTex => "yi syllable tex",
            YiSyllables::YiSyllableTe => "yi syllable te",
            YiSyllables::YiSyllableTep => "yi syllable tep",
            YiSyllables::YiSyllableTut => "yi syllable tut",
            YiSyllables::YiSyllableTux => "yi syllable tux",
            YiSyllables::YiSyllableTu => "yi syllable tu",
            YiSyllables::YiSyllableTup => "yi syllable tup",
            YiSyllables::YiSyllableTurx => "yi syllable turx",
            YiSyllables::YiSyllableTur => "yi syllable tur",
            YiSyllables::YiSyllableDdit => "yi syllable ddit",
            YiSyllables::YiSyllableDdix => "yi syllable ddix",
            YiSyllables::YiSyllableDdi => "yi syllable ddi",
            YiSyllables::YiSyllableDdip => "yi syllable ddip",
            YiSyllables::YiSyllableDdiex => "yi syllable ddiex",
            YiSyllables::YiSyllableDdie => "yi syllable ddie",
            YiSyllables::YiSyllableDdiep => "yi syllable ddiep",
            YiSyllables::YiSyllableDdat => "yi syllable ddat",
            YiSyllables::YiSyllableDdax => "yi syllable ddax",
            YiSyllables::YiSyllableDda => "yi syllable dda",
            YiSyllables::YiSyllableDdap => "yi syllable ddap",
            YiSyllables::YiSyllableDduox => "yi syllable dduox",
            YiSyllables::YiSyllableDduo => "yi syllable dduo",
            YiSyllables::YiSyllableDduop => "yi syllable dduop",
            YiSyllables::YiSyllableDdot => "yi syllable ddot",
            YiSyllables::YiSyllableDdox => "yi syllable ddox",
            YiSyllables::YiSyllableDdo => "yi syllable ddo",
            YiSyllables::YiSyllableDdop => "yi syllable ddop",
            YiSyllables::YiSyllableDdex => "yi syllable ddex",
            YiSyllables::YiSyllableDde => "yi syllable dde",
            YiSyllables::YiSyllableDdep => "yi syllable ddep",
            YiSyllables::YiSyllableDdut => "yi syllable ddut",
            YiSyllables::YiSyllableDdux => "yi syllable ddux",
            YiSyllables::YiSyllableDdu => "yi syllable ddu",
            YiSyllables::YiSyllableDdup => "yi syllable ddup",
            YiSyllables::YiSyllableDdurx => "yi syllable ddurx",
            YiSyllables::YiSyllableDdur => "yi syllable ddur",
            YiSyllables::YiSyllableNdit => "yi syllable ndit",
            YiSyllables::YiSyllableNdix => "yi syllable ndix",
            YiSyllables::YiSyllableNdi => "yi syllable ndi",
            YiSyllables::YiSyllableNdip => "yi syllable ndip",
            YiSyllables::YiSyllableNdiex => "yi syllable ndiex",
            YiSyllables::YiSyllableNdie => "yi syllable ndie",
            YiSyllables::YiSyllableNdat => "yi syllable ndat",
            YiSyllables::YiSyllableNdax => "yi syllable ndax",
            YiSyllables::YiSyllableNda => "yi syllable nda",
            YiSyllables::YiSyllableNdap => "yi syllable ndap",
            YiSyllables::YiSyllableNdot => "yi syllable ndot",
            YiSyllables::YiSyllableNdox => "yi syllable ndox",
            YiSyllables::YiSyllableNdo => "yi syllable ndo",
            YiSyllables::YiSyllableNdop => "yi syllable ndop",
            YiSyllables::YiSyllableNdex => "yi syllable ndex",
            YiSyllables::YiSyllableNde => "yi syllable nde",
            YiSyllables::YiSyllableNdep => "yi syllable ndep",
            YiSyllables::YiSyllableNdut => "yi syllable ndut",
            YiSyllables::YiSyllableNdux => "yi syllable ndux",
            YiSyllables::YiSyllableNdu => "yi syllable ndu",
            YiSyllables::YiSyllableNdup => "yi syllable ndup",
            YiSyllables::YiSyllableNdurx => "yi syllable ndurx",
            YiSyllables::YiSyllableNdur => "yi syllable ndur",
            YiSyllables::YiSyllableHnit => "yi syllable hnit",
            YiSyllables::YiSyllableHnix => "yi syllable hnix",
            YiSyllables::YiSyllableHni => "yi syllable hni",
            YiSyllables::YiSyllableHnip => "yi syllable hnip",
            YiSyllables::YiSyllableHniet => "yi syllable hniet",
            YiSyllables::YiSyllableHniex => "yi syllable hniex",
            YiSyllables::YiSyllableHnie => "yi syllable hnie",
            YiSyllables::YiSyllableHniep => "yi syllable hniep",
            YiSyllables::YiSyllableHnat => "yi syllable hnat",
            YiSyllables::YiSyllableHnax => "yi syllable hnax",
            YiSyllables::YiSyllableHna => "yi syllable hna",
            YiSyllables::YiSyllableHnap => "yi syllable hnap",
            YiSyllables::YiSyllableHnuox => "yi syllable hnuox",
            YiSyllables::YiSyllableHnuo => "yi syllable hnuo",
            YiSyllables::YiSyllableHnot => "yi syllable hnot",
            YiSyllables::YiSyllableHnox => "yi syllable hnox",
            YiSyllables::YiSyllableHnop => "yi syllable hnop",
            YiSyllables::YiSyllableHnex => "yi syllable hnex",
            YiSyllables::YiSyllableHne => "yi syllable hne",
            YiSyllables::YiSyllableHnep => "yi syllable hnep",
            YiSyllables::YiSyllableHnut => "yi syllable hnut",
            YiSyllables::YiSyllableNit => "yi syllable nit",
            YiSyllables::YiSyllableNix => "yi syllable nix",
            YiSyllables::YiSyllableNi => "yi syllable ni",
            YiSyllables::YiSyllableNip => "yi syllable nip",
            YiSyllables::YiSyllableNiex => "yi syllable niex",
            YiSyllables::YiSyllableNie => "yi syllable nie",
            YiSyllables::YiSyllableNiep => "yi syllable niep",
            YiSyllables::YiSyllableNax => "yi syllable nax",
            YiSyllables::YiSyllableNa => "yi syllable na",
            YiSyllables::YiSyllableNap => "yi syllable nap",
            YiSyllables::YiSyllableNuox => "yi syllable nuox",
            YiSyllables::YiSyllableNuo => "yi syllable nuo",
            YiSyllables::YiSyllableNuop => "yi syllable nuop",
            YiSyllables::YiSyllableNot => "yi syllable not",
            YiSyllables::YiSyllableNox => "yi syllable nox",
            YiSyllables::YiSyllableNo => "yi syllable no",
            YiSyllables::YiSyllableNop => "yi syllable nop",
            YiSyllables::YiSyllableNex => "yi syllable nex",
            YiSyllables::YiSyllableNe => "yi syllable ne",
            YiSyllables::YiSyllableNep => "yi syllable nep",
            YiSyllables::YiSyllableNut => "yi syllable nut",
            YiSyllables::YiSyllableNux => "yi syllable nux",
            YiSyllables::YiSyllableNu => "yi syllable nu",
            YiSyllables::YiSyllableNup => "yi syllable nup",
            YiSyllables::YiSyllableNurx => "yi syllable nurx",
            YiSyllables::YiSyllableNur => "yi syllable nur",
            YiSyllables::YiSyllableHlit => "yi syllable hlit",
            YiSyllables::YiSyllableHlix => "yi syllable hlix",
            YiSyllables::YiSyllableHli => "yi syllable hli",
            YiSyllables::YiSyllableHlip => "yi syllable hlip",
            YiSyllables::YiSyllableHliex => "yi syllable hliex",
            YiSyllables::YiSyllableHlie => "yi syllable hlie",
            YiSyllables::YiSyllableHliep => "yi syllable hliep",
            YiSyllables::YiSyllableHlat => "yi syllable hlat",
            YiSyllables::YiSyllableHlax => "yi syllable hlax",
            YiSyllables::YiSyllableHla => "yi syllable hla",
            YiSyllables::YiSyllableHlap => "yi syllable hlap",
            YiSyllables::YiSyllableHluox => "yi syllable hluox",
            YiSyllables::YiSyllableHluo => "yi syllable hluo",
            YiSyllables::YiSyllableHluop => "yi syllable hluop",
            YiSyllables::YiSyllableHlox => "yi syllable hlox",
            YiSyllables::YiSyllableHlo => "yi syllable hlo",
            YiSyllables::YiSyllableHlop => "yi syllable hlop",
            YiSyllables::YiSyllableHlex => "yi syllable hlex",
            YiSyllables::YiSyllableHle => "yi syllable hle",
            YiSyllables::YiSyllableHlep => "yi syllable hlep",
            YiSyllables::YiSyllableHlut => "yi syllable hlut",
            YiSyllables::YiSyllableHlux => "yi syllable hlux",
            YiSyllables::YiSyllableHlu => "yi syllable hlu",
            YiSyllables::YiSyllableHlup => "yi syllable hlup",
            YiSyllables::YiSyllableHlurx => "yi syllable hlurx",
            YiSyllables::YiSyllableHlur => "yi syllable hlur",
            YiSyllables::YiSyllableHlyt => "yi syllable hlyt",
            YiSyllables::YiSyllableHlyx => "yi syllable hlyx",
            YiSyllables::YiSyllableHly => "yi syllable hly",
            YiSyllables::YiSyllableHlyp => "yi syllable hlyp",
            YiSyllables::YiSyllableHlyrx => "yi syllable hlyrx",
            YiSyllables::YiSyllableHlyr => "yi syllable hlyr",
            YiSyllables::YiSyllableLit => "yi syllable lit",
            YiSyllables::YiSyllableLix => "yi syllable lix",
            YiSyllables::YiSyllableLi => "yi syllable li",
            YiSyllables::YiSyllableLip => "yi syllable lip",
            YiSyllables::YiSyllableLiet => "yi syllable liet",
            YiSyllables::YiSyllableLiex => "yi syllable liex",
            YiSyllables::YiSyllableLie => "yi syllable lie",
            YiSyllables::YiSyllableLiep => "yi syllable liep",
            YiSyllables::YiSyllableLat => "yi syllable lat",
            YiSyllables::YiSyllableLax => "yi syllable lax",
            YiSyllables::YiSyllableLa => "yi syllable la",
            YiSyllables::YiSyllableLap => "yi syllable lap",
            YiSyllables::YiSyllableLuot => "yi syllable luot",
            YiSyllables::YiSyllableLuox => "yi syllable luox",
            YiSyllables::YiSyllableLuo => "yi syllable luo",
            YiSyllables::YiSyllableLuop => "yi syllable luop",
            YiSyllables::YiSyllableLot => "yi syllable lot",
            YiSyllables::YiSyllableLox => "yi syllable lox",
            YiSyllables::YiSyllableLo => "yi syllable lo",
            YiSyllables::YiSyllableLop => "yi syllable lop",
            YiSyllables::YiSyllableLex => "yi syllable lex",
            YiSyllables::YiSyllableLe => "yi syllable le",
            YiSyllables::YiSyllableLep => "yi syllable lep",
            YiSyllables::YiSyllableLut => "yi syllable lut",
            YiSyllables::YiSyllableLux => "yi syllable lux",
            YiSyllables::YiSyllableLu => "yi syllable lu",
            YiSyllables::YiSyllableLup => "yi syllable lup",
            YiSyllables::YiSyllableLurx => "yi syllable lurx",
            YiSyllables::YiSyllableLur => "yi syllable lur",
            YiSyllables::YiSyllableLyt => "yi syllable lyt",
            YiSyllables::YiSyllableLyx => "yi syllable lyx",
            YiSyllables::YiSyllableLy => "yi syllable ly",
            YiSyllables::YiSyllableLyp => "yi syllable lyp",
            YiSyllables::YiSyllableLyrx => "yi syllable lyrx",
            YiSyllables::YiSyllableLyr => "yi syllable lyr",
            YiSyllables::YiSyllableGit => "yi syllable git",
            YiSyllables::YiSyllableGix => "yi syllable gix",
            YiSyllables::YiSyllableGi => "yi syllable gi",
            YiSyllables::YiSyllableGip => "yi syllable gip",
            YiSyllables::YiSyllableGiet => "yi syllable giet",
            YiSyllables::YiSyllableGiex => "yi syllable giex",
            YiSyllables::YiSyllableGie => "yi syllable gie",
            YiSyllables::YiSyllableGiep => "yi syllable giep",
            YiSyllables::YiSyllableGat => "yi syllable gat",
            YiSyllables::YiSyllableGax => "yi syllable gax",
            YiSyllables::YiSyllableGa => "yi syllable ga",
            YiSyllables::YiSyllableGap => "yi syllable gap",
            YiSyllables::YiSyllableGuot => "yi syllable guot",
            YiSyllables::YiSyllableGuox => "yi syllable guox",
            YiSyllables::YiSyllableGuo => "yi syllable guo",
            YiSyllables::YiSyllableGuop => "yi syllable guop",
            YiSyllables::YiSyllableGot => "yi syllable got",
            YiSyllables::YiSyllableGox => "yi syllable gox",
            YiSyllables::YiSyllableGo => "yi syllable go",
            YiSyllables::YiSyllableGop => "yi syllable gop",
            YiSyllables::YiSyllableGet => "yi syllable get",
            YiSyllables::YiSyllableGex => "yi syllable gex",
            YiSyllables::YiSyllableGe => "yi syllable ge",
            YiSyllables::YiSyllableGep => "yi syllable gep",
            YiSyllables::YiSyllableGut => "yi syllable gut",
            YiSyllables::YiSyllableGux => "yi syllable gux",
            YiSyllables::YiSyllableGu => "yi syllable gu",
            YiSyllables::YiSyllableGup => "yi syllable gup",
            YiSyllables::YiSyllableGurx => "yi syllable gurx",
            YiSyllables::YiSyllableGur => "yi syllable gur",
            YiSyllables::YiSyllableKit => "yi syllable kit",
            YiSyllables::YiSyllableKix => "yi syllable kix",
            YiSyllables::YiSyllableKi => "yi syllable ki",
            YiSyllables::YiSyllableKip => "yi syllable kip",
            YiSyllables::YiSyllableKiex => "yi syllable kiex",
            YiSyllables::YiSyllableKie => "yi syllable kie",
            YiSyllables::YiSyllableKiep => "yi syllable kiep",
            YiSyllables::YiSyllableKat => "yi syllable kat",
            YiSyllables::YiSyllableKax => "yi syllable kax",
            YiSyllables::YiSyllableKa => "yi syllable ka",
            YiSyllables::YiSyllableKap => "yi syllable kap",
            YiSyllables::YiSyllableKuox => "yi syllable kuox",
            YiSyllables::YiSyllableKuo => "yi syllable kuo",
            YiSyllables::YiSyllableKuop => "yi syllable kuop",
            YiSyllables::YiSyllableKot => "yi syllable kot",
            YiSyllables::YiSyllableKox => "yi syllable kox",
            YiSyllables::YiSyllableKo => "yi syllable ko",
            YiSyllables::YiSyllableKop => "yi syllable kop",
            YiSyllables::YiSyllableKet => "yi syllable ket",
            YiSyllables::YiSyllableKex => "yi syllable kex",
            YiSyllables::YiSyllableKe => "yi syllable ke",
            YiSyllables::YiSyllableKep => "yi syllable kep",
            YiSyllables::YiSyllableKut => "yi syllable kut",
            YiSyllables::YiSyllableKux => "yi syllable kux",
            YiSyllables::YiSyllableKu => "yi syllable ku",
            YiSyllables::YiSyllableKup => "yi syllable kup",
            YiSyllables::YiSyllableKurx => "yi syllable kurx",
            YiSyllables::YiSyllableKur => "yi syllable kur",
            YiSyllables::YiSyllableGgit => "yi syllable ggit",
            YiSyllables::YiSyllableGgix => "yi syllable ggix",
            YiSyllables::YiSyllableGgi => "yi syllable ggi",
            YiSyllables::YiSyllableGgiex => "yi syllable ggiex",
            YiSyllables::YiSyllableGgie => "yi syllable ggie",
            YiSyllables::YiSyllableGgiep => "yi syllable ggiep",
            YiSyllables::YiSyllableGgat => "yi syllable ggat",
            YiSyllables::YiSyllableGgax => "yi syllable ggax",
            YiSyllables::YiSyllableGga => "yi syllable gga",
            YiSyllables::YiSyllableGgap => "yi syllable ggap",
            YiSyllables::YiSyllableGguot => "yi syllable gguot",
            YiSyllables::YiSyllableGguox => "yi syllable gguox",
            YiSyllables::YiSyllableGguo => "yi syllable gguo",
            YiSyllables::YiSyllableGguop => "yi syllable gguop",
            YiSyllables::YiSyllableGgot => "yi syllable ggot",
            YiSyllables::YiSyllableGgox => "yi syllable ggox",
            YiSyllables::YiSyllableGgo => "yi syllable ggo",
            YiSyllables::YiSyllableGgop => "yi syllable ggop",
            YiSyllables::YiSyllableGget => "yi syllable gget",
            YiSyllables::YiSyllableGgex => "yi syllable ggex",
            YiSyllables::YiSyllableGge => "yi syllable gge",
            YiSyllables::YiSyllableGgep => "yi syllable ggep",
            YiSyllables::YiSyllableGgut => "yi syllable ggut",
            YiSyllables::YiSyllableGgux => "yi syllable ggux",
            YiSyllables::YiSyllableGgu => "yi syllable ggu",
            YiSyllables::YiSyllableGgup => "yi syllable ggup",
            YiSyllables::YiSyllableGgurx => "yi syllable ggurx",
            YiSyllables::YiSyllableGgur => "yi syllable ggur",
            YiSyllables::YiSyllableMgiex => "yi syllable mgiex",
            YiSyllables::YiSyllableMgie => "yi syllable mgie",
            YiSyllables::YiSyllableMgat => "yi syllable mgat",
            YiSyllables::YiSyllableMgax => "yi syllable mgax",
            YiSyllables::YiSyllableMga => "yi syllable mga",
            YiSyllables::YiSyllableMgap => "yi syllable mgap",
            YiSyllables::YiSyllableMguox => "yi syllable mguox",
            YiSyllables::YiSyllableMguo => "yi syllable mguo",
            YiSyllables::YiSyllableMguop => "yi syllable mguop",
            YiSyllables::YiSyllableMgot => "yi syllable mgot",
            YiSyllables::YiSyllableMgox => "yi syllable mgox",
            YiSyllables::YiSyllableMgo => "yi syllable mgo",
            YiSyllables::YiSyllableMgop => "yi syllable mgop",
            YiSyllables::YiSyllableMgex => "yi syllable mgex",
            YiSyllables::YiSyllableMge => "yi syllable mge",
            YiSyllables::YiSyllableMgep => "yi syllable mgep",
            YiSyllables::YiSyllableMgut => "yi syllable mgut",
            YiSyllables::YiSyllableMgux => "yi syllable mgux",
            YiSyllables::YiSyllableMgu => "yi syllable mgu",
            YiSyllables::YiSyllableMgup => "yi syllable mgup",
            YiSyllables::YiSyllableMgurx => "yi syllable mgurx",
            YiSyllables::YiSyllableMgur => "yi syllable mgur",
            YiSyllables::YiSyllableHxit => "yi syllable hxit",
            YiSyllables::YiSyllableHxix => "yi syllable hxix",
            YiSyllables::YiSyllableHxi => "yi syllable hxi",
            YiSyllables::YiSyllableHxip => "yi syllable hxip",
            YiSyllables::YiSyllableHxiet => "yi syllable hxiet",
            YiSyllables::YiSyllableHxiex => "yi syllable hxiex",
            YiSyllables::YiSyllableHxie => "yi syllable hxie",
            YiSyllables::YiSyllableHxiep => "yi syllable hxiep",
            YiSyllables::YiSyllableHxat => "yi syllable hxat",
            YiSyllables::YiSyllableHxax => "yi syllable hxax",
            YiSyllables::YiSyllableHxa => "yi syllable hxa",
            YiSyllables::YiSyllableHxap => "yi syllable hxap",
            YiSyllables::YiSyllableHxuot => "yi syllable hxuot",
            YiSyllables::YiSyllableHxuox => "yi syllable hxuox",
            YiSyllables::YiSyllableHxuo => "yi syllable hxuo",
            YiSyllables::YiSyllableHxuop => "yi syllable hxuop",
            YiSyllables::YiSyllableHxot => "yi syllable hxot",
            YiSyllables::YiSyllableHxox => "yi syllable hxox",
            YiSyllables::YiSyllableHxo => "yi syllable hxo",
            YiSyllables::YiSyllableHxop => "yi syllable hxop",
            YiSyllables::YiSyllableHxex => "yi syllable hxex",
            YiSyllables::YiSyllableHxe => "yi syllable hxe",
            YiSyllables::YiSyllableHxep => "yi syllable hxep",
            YiSyllables::YiSyllableNgiex => "yi syllable ngiex",
            YiSyllables::YiSyllableNgie => "yi syllable ngie",
            YiSyllables::YiSyllableNgiep => "yi syllable ngiep",
            YiSyllables::YiSyllableNgat => "yi syllable ngat",
            YiSyllables::YiSyllableNgax => "yi syllable ngax",
            YiSyllables::YiSyllableNga => "yi syllable nga",
            YiSyllables::YiSyllableNgap => "yi syllable ngap",
            YiSyllables::YiSyllableNguot => "yi syllable nguot",
            YiSyllables::YiSyllableNguox => "yi syllable nguox",
            YiSyllables::YiSyllableNguo => "yi syllable nguo",
            YiSyllables::YiSyllableNgot => "yi syllable ngot",
            YiSyllables::YiSyllableNgox => "yi syllable ngox",
            YiSyllables::YiSyllableNgo => "yi syllable ngo",
            YiSyllables::YiSyllableNgop => "yi syllable ngop",
            YiSyllables::YiSyllableNgex => "yi syllable ngex",
            YiSyllables::YiSyllableNge => "yi syllable nge",
            YiSyllables::YiSyllableNgep => "yi syllable ngep",
            YiSyllables::YiSyllableHit => "yi syllable hit",
            YiSyllables::YiSyllableHiex => "yi syllable hiex",
            YiSyllables::YiSyllableHie => "yi syllable hie",
            YiSyllables::YiSyllableHat => "yi syllable hat",
            YiSyllables::YiSyllableHax => "yi syllable hax",
            YiSyllables::YiSyllableHa => "yi syllable ha",
            YiSyllables::YiSyllableHap => "yi syllable hap",
            YiSyllables::YiSyllableHuot => "yi syllable huot",
            YiSyllables::YiSyllableHuox => "yi syllable huox",
            YiSyllables::YiSyllableHuo => "yi syllable huo",
            YiSyllables::YiSyllableHuop => "yi syllable huop",
            YiSyllables::YiSyllableHot => "yi syllable hot",
            YiSyllables::YiSyllableHox => "yi syllable hox",
            YiSyllables::YiSyllableHo => "yi syllable ho",
            YiSyllables::YiSyllableHop => "yi syllable hop",
            YiSyllables::YiSyllableHex => "yi syllable hex",
            YiSyllables::YiSyllableHe => "yi syllable he",
            YiSyllables::YiSyllableHep => "yi syllable hep",
            YiSyllables::YiSyllableWat => "yi syllable wat",
            YiSyllables::YiSyllableWax => "yi syllable wax",
            YiSyllables::YiSyllableWa => "yi syllable wa",
            YiSyllables::YiSyllableWap => "yi syllable wap",
            YiSyllables::YiSyllableWuox => "yi syllable wuox",
            YiSyllables::YiSyllableWuo => "yi syllable wuo",
            YiSyllables::YiSyllableWuop => "yi syllable wuop",
            YiSyllables::YiSyllableWox => "yi syllable wox",
            YiSyllables::YiSyllableWo => "yi syllable wo",
            YiSyllables::YiSyllableWop => "yi syllable wop",
            YiSyllables::YiSyllableWex => "yi syllable wex",
            YiSyllables::YiSyllableWe => "yi syllable we",
            YiSyllables::YiSyllableWep => "yi syllable wep",
            YiSyllables::YiSyllableZit => "yi syllable zit",
            YiSyllables::YiSyllableZix => "yi syllable zix",
            YiSyllables::YiSyllableZi => "yi syllable zi",
            YiSyllables::YiSyllableZip => "yi syllable zip",
            YiSyllables::YiSyllableZiex => "yi syllable ziex",
            YiSyllables::YiSyllableZie => "yi syllable zie",
            YiSyllables::YiSyllableZiep => "yi syllable ziep",
            YiSyllables::YiSyllableZat => "yi syllable zat",
            YiSyllables::YiSyllableZax => "yi syllable zax",
            YiSyllables::YiSyllableZa => "yi syllable za",
            YiSyllables::YiSyllableZap => "yi syllable zap",
            YiSyllables::YiSyllableZuox => "yi syllable zuox",
            YiSyllables::YiSyllableZuo => "yi syllable zuo",
            YiSyllables::YiSyllableZuop => "yi syllable zuop",
            YiSyllables::YiSyllableZot => "yi syllable zot",
            YiSyllables::YiSyllableZox => "yi syllable zox",
            YiSyllables::YiSyllableZo => "yi syllable zo",
            YiSyllables::YiSyllableZop => "yi syllable zop",
            YiSyllables::YiSyllableZex => "yi syllable zex",
            YiSyllables::YiSyllableZe => "yi syllable ze",
            YiSyllables::YiSyllableZep => "yi syllable zep",
            YiSyllables::YiSyllableZut => "yi syllable zut",
            YiSyllables::YiSyllableZux => "yi syllable zux",
            YiSyllables::YiSyllableZu => "yi syllable zu",
            YiSyllables::YiSyllableZup => "yi syllable zup",
            YiSyllables::YiSyllableZurx => "yi syllable zurx",
            YiSyllables::YiSyllableZur => "yi syllable zur",
            YiSyllables::YiSyllableZyt => "yi syllable zyt",
            YiSyllables::YiSyllableZyx => "yi syllable zyx",
            YiSyllables::YiSyllableZy => "yi syllable zy",
            YiSyllables::YiSyllableZyp => "yi syllable zyp",
            YiSyllables::YiSyllableZyrx => "yi syllable zyrx",
            YiSyllables::YiSyllableZyr => "yi syllable zyr",
            YiSyllables::YiSyllableCit => "yi syllable cit",
            YiSyllables::YiSyllableCix => "yi syllable cix",
            YiSyllables::YiSyllableCi => "yi syllable ci",
            YiSyllables::YiSyllableCip => "yi syllable cip",
            YiSyllables::YiSyllableCiet => "yi syllable ciet",
            YiSyllables::YiSyllableCiex => "yi syllable ciex",
            YiSyllables::YiSyllableCie => "yi syllable cie",
            YiSyllables::YiSyllableCiep => "yi syllable ciep",
            YiSyllables::YiSyllableCat => "yi syllable cat",
            YiSyllables::YiSyllableCax => "yi syllable cax",
            YiSyllables::YiSyllableCa => "yi syllable ca",
            YiSyllables::YiSyllableCap => "yi syllable cap",
            YiSyllables::YiSyllableCuox => "yi syllable cuox",
            YiSyllables::YiSyllableCuo => "yi syllable cuo",
            YiSyllables::YiSyllableCuop => "yi syllable cuop",
            YiSyllables::YiSyllableCot => "yi syllable cot",
            YiSyllables::YiSyllableCox => "yi syllable cox",
            YiSyllables::YiSyllableCo => "yi syllable co",
            YiSyllables::YiSyllableCop => "yi syllable cop",
            YiSyllables::YiSyllableCex => "yi syllable cex",
            YiSyllables::YiSyllableCe => "yi syllable ce",
            YiSyllables::YiSyllableCep => "yi syllable cep",
            YiSyllables::YiSyllableCut => "yi syllable cut",
            YiSyllables::YiSyllableCux => "yi syllable cux",
            YiSyllables::YiSyllableCu => "yi syllable cu",
            YiSyllables::YiSyllableCup => "yi syllable cup",
            YiSyllables::YiSyllableCurx => "yi syllable curx",
            YiSyllables::YiSyllableCur => "yi syllable cur",
            YiSyllables::YiSyllableCyt => "yi syllable cyt",
            YiSyllables::YiSyllableCyx => "yi syllable cyx",
            YiSyllables::YiSyllableCy => "yi syllable cy",
            YiSyllables::YiSyllableCyp => "yi syllable cyp",
            YiSyllables::YiSyllableCyrx => "yi syllable cyrx",
            YiSyllables::YiSyllableCyr => "yi syllable cyr",
            YiSyllables::YiSyllableZzit => "yi syllable zzit",
            YiSyllables::YiSyllableZzix => "yi syllable zzix",
            YiSyllables::YiSyllableZzi => "yi syllable zzi",
            YiSyllables::YiSyllableZzip => "yi syllable zzip",
            YiSyllables::YiSyllableZziet => "yi syllable zziet",
            YiSyllables::YiSyllableZziex => "yi syllable zziex",
            YiSyllables::YiSyllableZzie => "yi syllable zzie",
            YiSyllables::YiSyllableZziep => "yi syllable zziep",
            YiSyllables::YiSyllableZzat => "yi syllable zzat",
            YiSyllables::YiSyllableZzax => "yi syllable zzax",
            YiSyllables::YiSyllableZza => "yi syllable zza",
            YiSyllables::YiSyllableZzap => "yi syllable zzap",
            YiSyllables::YiSyllableZzox => "yi syllable zzox",
            YiSyllables::YiSyllableZzo => "yi syllable zzo",
            YiSyllables::YiSyllableZzop => "yi syllable zzop",
            YiSyllables::YiSyllableZzex => "yi syllable zzex",
            YiSyllables::YiSyllableZze => "yi syllable zze",
            YiSyllables::YiSyllableZzep => "yi syllable zzep",
            YiSyllables::YiSyllableZzux => "yi syllable zzux",
            YiSyllables::YiSyllableZzu => "yi syllable zzu",
            YiSyllables::YiSyllableZzup => "yi syllable zzup",
            YiSyllables::YiSyllableZzurx => "yi syllable zzurx",
            YiSyllables::YiSyllableZzur => "yi syllable zzur",
            YiSyllables::YiSyllableZzyt => "yi syllable zzyt",
            YiSyllables::YiSyllableZzyx => "yi syllable zzyx",
            YiSyllables::YiSyllableZzy => "yi syllable zzy",
            YiSyllables::YiSyllableZzyp => "yi syllable zzyp",
            YiSyllables::YiSyllableZzyrx => "yi syllable zzyrx",
            YiSyllables::YiSyllableZzyr => "yi syllable zzyr",
            YiSyllables::YiSyllableNzit => "yi syllable nzit",
            YiSyllables::YiSyllableNzix => "yi syllable nzix",
            YiSyllables::YiSyllableNzi => "yi syllable nzi",
            YiSyllables::YiSyllableNzip => "yi syllable nzip",
            YiSyllables::YiSyllableNziex => "yi syllable nziex",
            YiSyllables::YiSyllableNzie => "yi syllable nzie",
            YiSyllables::YiSyllableNziep => "yi syllable nziep",
            YiSyllables::YiSyllableNzat => "yi syllable nzat",
            YiSyllables::YiSyllableNzax => "yi syllable nzax",
            YiSyllables::YiSyllableNza => "yi syllable nza",
            YiSyllables::YiSyllableNzap => "yi syllable nzap",
            YiSyllables::YiSyllableNzuox => "yi syllable nzuox",
            YiSyllables::YiSyllableNzuo => "yi syllable nzuo",
            YiSyllables::YiSyllableNzox => "yi syllable nzox",
            YiSyllables::YiSyllableNzop => "yi syllable nzop",
            YiSyllables::YiSyllableNzex => "yi syllable nzex",
            YiSyllables::YiSyllableNze => "yi syllable nze",
            YiSyllables::YiSyllableNzux => "yi syllable nzux",
            YiSyllables::YiSyllableNzu => "yi syllable nzu",
            YiSyllables::YiSyllableNzup => "yi syllable nzup",
            YiSyllables::YiSyllableNzurx => "yi syllable nzurx",
            YiSyllables::YiSyllableNzur => "yi syllable nzur",
            YiSyllables::YiSyllableNzyt => "yi syllable nzyt",
            YiSyllables::YiSyllableNzyx => "yi syllable nzyx",
            YiSyllables::YiSyllableNzy => "yi syllable nzy",
            YiSyllables::YiSyllableNzyp => "yi syllable nzyp",
            YiSyllables::YiSyllableNzyrx => "yi syllable nzyrx",
            YiSyllables::YiSyllableNzyr => "yi syllable nzyr",
            YiSyllables::YiSyllableSit => "yi syllable sit",
            YiSyllables::YiSyllableSix => "yi syllable six",
            YiSyllables::YiSyllableSi => "yi syllable si",
            YiSyllables::YiSyllableSip => "yi syllable sip",
            YiSyllables::YiSyllableSiex => "yi syllable siex",
            YiSyllables::YiSyllableSie => "yi syllable sie",
            YiSyllables::YiSyllableSiep => "yi syllable siep",
            YiSyllables::YiSyllableSat => "yi syllable sat",
            YiSyllables::YiSyllableSax => "yi syllable sax",
            YiSyllables::YiSyllableSa => "yi syllable sa",
            YiSyllables::YiSyllableSap => "yi syllable sap",
            YiSyllables::YiSyllableSuox => "yi syllable suox",
            YiSyllables::YiSyllableSuo => "yi syllable suo",
            YiSyllables::YiSyllableSuop => "yi syllable suop",
            YiSyllables::YiSyllableSot => "yi syllable sot",
            YiSyllables::YiSyllableSox => "yi syllable sox",
            YiSyllables::YiSyllableSo => "yi syllable so",
            YiSyllables::YiSyllableSop => "yi syllable sop",
            YiSyllables::YiSyllableSex => "yi syllable sex",
            YiSyllables::YiSyllableSe => "yi syllable se",
            YiSyllables::YiSyllableSep => "yi syllable sep",
            YiSyllables::YiSyllableSut => "yi syllable sut",
            YiSyllables::YiSyllableSux => "yi syllable sux",
            YiSyllables::YiSyllableSu => "yi syllable su",
            YiSyllables::YiSyllableSup => "yi syllable sup",
            YiSyllables::YiSyllableSurx => "yi syllable surx",
            YiSyllables::YiSyllableSur => "yi syllable sur",
            YiSyllables::YiSyllableSyt => "yi syllable syt",
            YiSyllables::YiSyllableSyx => "yi syllable syx",
            YiSyllables::YiSyllableSy => "yi syllable sy",
            YiSyllables::YiSyllableSyp => "yi syllable syp",
            YiSyllables::YiSyllableSyrx => "yi syllable syrx",
            YiSyllables::YiSyllableSyr => "yi syllable syr",
            YiSyllables::YiSyllableSsit => "yi syllable ssit",
            YiSyllables::YiSyllableSsix => "yi syllable ssix",
            YiSyllables::YiSyllableSsi => "yi syllable ssi",
            YiSyllables::YiSyllableSsip => "yi syllable ssip",
            YiSyllables::YiSyllableSsiex => "yi syllable ssiex",
            YiSyllables::YiSyllableSsie => "yi syllable ssie",
            YiSyllables::YiSyllableSsiep => "yi syllable ssiep",
            YiSyllables::YiSyllableSsat => "yi syllable ssat",
            YiSyllables::YiSyllableSsax => "yi syllable ssax",
            YiSyllables::YiSyllableSsa => "yi syllable ssa",
            YiSyllables::YiSyllableSsap => "yi syllable ssap",
            YiSyllables::YiSyllableSsot => "yi syllable ssot",
            YiSyllables::YiSyllableSsox => "yi syllable ssox",
            YiSyllables::YiSyllableSso => "yi syllable sso",
            YiSyllables::YiSyllableSsop => "yi syllable ssop",
            YiSyllables::YiSyllableSsex => "yi syllable ssex",
            YiSyllables::YiSyllableSse => "yi syllable sse",
            YiSyllables::YiSyllableSsep => "yi syllable ssep",
            YiSyllables::YiSyllableSsut => "yi syllable ssut",
            YiSyllables::YiSyllableSsux => "yi syllable ssux",
            YiSyllables::YiSyllableSsu => "yi syllable ssu",
            YiSyllables::YiSyllableSsup => "yi syllable ssup",
            YiSyllables::YiSyllableSsyt => "yi syllable ssyt",
            YiSyllables::YiSyllableSsyx => "yi syllable ssyx",
            YiSyllables::YiSyllableSsy => "yi syllable ssy",
            YiSyllables::YiSyllableSsyp => "yi syllable ssyp",
            YiSyllables::YiSyllableSsyrx => "yi syllable ssyrx",
            YiSyllables::YiSyllableSsyr => "yi syllable ssyr",
            YiSyllables::YiSyllableZhat => "yi syllable zhat",
            YiSyllables::YiSyllableZhax => "yi syllable zhax",
            YiSyllables::YiSyllableZha => "yi syllable zha",
            YiSyllables::YiSyllableZhap => "yi syllable zhap",
            YiSyllables::YiSyllableZhuox => "yi syllable zhuox",
            YiSyllables::YiSyllableZhuo => "yi syllable zhuo",
            YiSyllables::YiSyllableZhuop => "yi syllable zhuop",
            YiSyllables::YiSyllableZhot => "yi syllable zhot",
            YiSyllables::YiSyllableZhox => "yi syllable zhox",
            YiSyllables::YiSyllableZho => "yi syllable zho",
            YiSyllables::YiSyllableZhop => "yi syllable zhop",
            YiSyllables::YiSyllableZhet => "yi syllable zhet",
            YiSyllables::YiSyllableZhex => "yi syllable zhex",
            YiSyllables::YiSyllableZhe => "yi syllable zhe",
            YiSyllables::YiSyllableZhep => "yi syllable zhep",
            YiSyllables::YiSyllableZhut => "yi syllable zhut",
            YiSyllables::YiSyllableZhux => "yi syllable zhux",
            YiSyllables::YiSyllableZhu => "yi syllable zhu",
            YiSyllables::YiSyllableZhup => "yi syllable zhup",
            YiSyllables::YiSyllableZhurx => "yi syllable zhurx",
            YiSyllables::YiSyllableZhur => "yi syllable zhur",
            YiSyllables::YiSyllableZhyt => "yi syllable zhyt",
            YiSyllables::YiSyllableZhyx => "yi syllable zhyx",
            YiSyllables::YiSyllableZhy => "yi syllable zhy",
            YiSyllables::YiSyllableZhyp => "yi syllable zhyp",
            YiSyllables::YiSyllableZhyrx => "yi syllable zhyrx",
            YiSyllables::YiSyllableZhyr => "yi syllable zhyr",
            YiSyllables::YiSyllableChat => "yi syllable chat",
            YiSyllables::YiSyllableChax => "yi syllable chax",
            YiSyllables::YiSyllableCha => "yi syllable cha",
            YiSyllables::YiSyllableChap => "yi syllable chap",
            YiSyllables::YiSyllableChuot => "yi syllable chuot",
            YiSyllables::YiSyllableChuox => "yi syllable chuox",
            YiSyllables::YiSyllableChuo => "yi syllable chuo",
            YiSyllables::YiSyllableChuop => "yi syllable chuop",
            YiSyllables::YiSyllableChot => "yi syllable chot",
            YiSyllables::YiSyllableChox => "yi syllable chox",
            YiSyllables::YiSyllableCho => "yi syllable cho",
            YiSyllables::YiSyllableChop => "yi syllable chop",
            YiSyllables::YiSyllableChet => "yi syllable chet",
            YiSyllables::YiSyllableChex => "yi syllable chex",
            YiSyllables::YiSyllableChe => "yi syllable che",
            YiSyllables::YiSyllableChep => "yi syllable chep",
            YiSyllables::YiSyllableChux => "yi syllable chux",
            YiSyllables::YiSyllableChu => "yi syllable chu",
            YiSyllables::YiSyllableChup => "yi syllable chup",
            YiSyllables::YiSyllableChurx => "yi syllable churx",
            YiSyllables::YiSyllableChur => "yi syllable chur",
            YiSyllables::YiSyllableChyt => "yi syllable chyt",
            YiSyllables::YiSyllableChyx => "yi syllable chyx",
            YiSyllables::YiSyllableChy => "yi syllable chy",
            YiSyllables::YiSyllableChyp => "yi syllable chyp",
            YiSyllables::YiSyllableChyrx => "yi syllable chyrx",
            YiSyllables::YiSyllableChyr => "yi syllable chyr",
            YiSyllables::YiSyllableRrax => "yi syllable rrax",
            YiSyllables::YiSyllableRra => "yi syllable rra",
            YiSyllables::YiSyllableRruox => "yi syllable rruox",
            YiSyllables::YiSyllableRruo => "yi syllable rruo",
            YiSyllables::YiSyllableRrot => "yi syllable rrot",
            YiSyllables::YiSyllableRrox => "yi syllable rrox",
            YiSyllables::YiSyllableRro => "yi syllable rro",
            YiSyllables::YiSyllableRrop => "yi syllable rrop",
            YiSyllables::YiSyllableRret => "yi syllable rret",
            YiSyllables::YiSyllableRrex => "yi syllable rrex",
            YiSyllables::YiSyllableRre => "yi syllable rre",
            YiSyllables::YiSyllableRrep => "yi syllable rrep",
            YiSyllables::YiSyllableRrut => "yi syllable rrut",
            YiSyllables::YiSyllableRrux => "yi syllable rrux",
            YiSyllables::YiSyllableRru => "yi syllable rru",
            YiSyllables::YiSyllableRrup => "yi syllable rrup",
            YiSyllables::YiSyllableRrurx => "yi syllable rrurx",
            YiSyllables::YiSyllableRrur => "yi syllable rrur",
            YiSyllables::YiSyllableRryt => "yi syllable rryt",
            YiSyllables::YiSyllableRryx => "yi syllable rryx",
            YiSyllables::YiSyllableRry => "yi syllable rry",
            YiSyllables::YiSyllableRryp => "yi syllable rryp",
            YiSyllables::YiSyllableRryrx => "yi syllable rryrx",
            YiSyllables::YiSyllableRryr => "yi syllable rryr",
            YiSyllables::YiSyllableNrat => "yi syllable nrat",
            YiSyllables::YiSyllableNrax => "yi syllable nrax",
            YiSyllables::YiSyllableNra => "yi syllable nra",
            YiSyllables::YiSyllableNrap => "yi syllable nrap",
            YiSyllables::YiSyllableNrox => "yi syllable nrox",
            YiSyllables::YiSyllableNro => "yi syllable nro",
            YiSyllables::YiSyllableNrop => "yi syllable nrop",
            YiSyllables::YiSyllableNret => "yi syllable nret",
            YiSyllables::YiSyllableNrex => "yi syllable nrex",
            YiSyllables::YiSyllableNre => "yi syllable nre",
            YiSyllables::YiSyllableNrep => "yi syllable nrep",
            YiSyllables::YiSyllableNrut => "yi syllable nrut",
            YiSyllables::YiSyllableNrux => "yi syllable nrux",
            YiSyllables::YiSyllableNru => "yi syllable nru",
            YiSyllables::YiSyllableNrup => "yi syllable nrup",
            YiSyllables::YiSyllableNrurx => "yi syllable nrurx",
            YiSyllables::YiSyllableNrur => "yi syllable nrur",
            YiSyllables::YiSyllableNryt => "yi syllable nryt",
            YiSyllables::YiSyllableNryx => "yi syllable nryx",
            YiSyllables::YiSyllableNry => "yi syllable nry",
            YiSyllables::YiSyllableNryp => "yi syllable nryp",
            YiSyllables::YiSyllableNryrx => "yi syllable nryrx",
            YiSyllables::YiSyllableNryr => "yi syllable nryr",
            YiSyllables::YiSyllableShat => "yi syllable shat",
            YiSyllables::YiSyllableShax => "yi syllable shax",
            YiSyllables::YiSyllableSha => "yi syllable sha",
            YiSyllables::YiSyllableShap => "yi syllable shap",
            YiSyllables::YiSyllableShuox => "yi syllable shuox",
            YiSyllables::YiSyllableShuo => "yi syllable shuo",
            YiSyllables::YiSyllableShuop => "yi syllable shuop",
            YiSyllables::YiSyllableShot => "yi syllable shot",
            YiSyllables::YiSyllableShox => "yi syllable shox",
            YiSyllables::YiSyllableSho => "yi syllable sho",
            YiSyllables::YiSyllableShop => "yi syllable shop",
            YiSyllables::YiSyllableShet => "yi syllable shet",
            YiSyllables::YiSyllableShex => "yi syllable shex",
            YiSyllables::YiSyllableShe => "yi syllable she",
            YiSyllables::YiSyllableShep => "yi syllable shep",
            YiSyllables::YiSyllableShut => "yi syllable shut",
            YiSyllables::YiSyllableShux => "yi syllable shux",
            YiSyllables::YiSyllableShu => "yi syllable shu",
            YiSyllables::YiSyllableShup => "yi syllable shup",
            YiSyllables::YiSyllableShurx => "yi syllable shurx",
            YiSyllables::YiSyllableShur => "yi syllable shur",
            YiSyllables::YiSyllableShyt => "yi syllable shyt",
            YiSyllables::YiSyllableShyx => "yi syllable shyx",
            YiSyllables::YiSyllableShy => "yi syllable shy",
            YiSyllables::YiSyllableShyp => "yi syllable shyp",
            YiSyllables::YiSyllableShyrx => "yi syllable shyrx",
            YiSyllables::YiSyllableShyr => "yi syllable shyr",
            YiSyllables::YiSyllableRat => "yi syllable rat",
            YiSyllables::YiSyllableRax => "yi syllable rax",
            YiSyllables::YiSyllableRa => "yi syllable ra",
            YiSyllables::YiSyllableRap => "yi syllable rap",
            YiSyllables::YiSyllableRuox => "yi syllable ruox",
            YiSyllables::YiSyllableRuo => "yi syllable ruo",
            YiSyllables::YiSyllableRuop => "yi syllable ruop",
            YiSyllables::YiSyllableRot => "yi syllable rot",
            YiSyllables::YiSyllableRox => "yi syllable rox",
            YiSyllables::YiSyllableRo => "yi syllable ro",
            YiSyllables::YiSyllableRop => "yi syllable rop",
            YiSyllables::YiSyllableRex => "yi syllable rex",
            YiSyllables::YiSyllableRe => "yi syllable re",
            YiSyllables::YiSyllableRep => "yi syllable rep",
            YiSyllables::YiSyllableRut => "yi syllable rut",
            YiSyllables::YiSyllableRux => "yi syllable rux",
            YiSyllables::YiSyllableRu => "yi syllable ru",
            YiSyllables::YiSyllableRup => "yi syllable rup",
            YiSyllables::YiSyllableRurx => "yi syllable rurx",
            YiSyllables::YiSyllableRur => "yi syllable rur",
            YiSyllables::YiSyllableRyt => "yi syllable ryt",
            YiSyllables::YiSyllableRyx => "yi syllable ryx",
            YiSyllables::YiSyllableRy => "yi syllable ry",
            YiSyllables::YiSyllableRyp => "yi syllable ryp",
            YiSyllables::YiSyllableRyrx => "yi syllable ryrx",
            YiSyllables::YiSyllableRyr => "yi syllable ryr",
            YiSyllables::YiSyllableJit => "yi syllable jit",
            YiSyllables::YiSyllableJix => "yi syllable jix",
            YiSyllables::YiSyllableJi => "yi syllable ji",
            YiSyllables::YiSyllableJip => "yi syllable jip",
            YiSyllables::YiSyllableJiet => "yi syllable jiet",
            YiSyllables::YiSyllableJiex => "yi syllable jiex",
            YiSyllables::YiSyllableJie => "yi syllable jie",
            YiSyllables::YiSyllableJiep => "yi syllable jiep",
            YiSyllables::YiSyllableJuot => "yi syllable juot",
            YiSyllables::YiSyllableJuox => "yi syllable juox",
            YiSyllables::YiSyllableJuo => "yi syllable juo",
            YiSyllables::YiSyllableJuop => "yi syllable juop",
            YiSyllables::YiSyllableJot => "yi syllable jot",
            YiSyllables::YiSyllableJox => "yi syllable jox",
            YiSyllables::YiSyllableJo => "yi syllable jo",
            YiSyllables::YiSyllableJop => "yi syllable jop",
            YiSyllables::YiSyllableJut => "yi syllable jut",
            YiSyllables::YiSyllableJux => "yi syllable jux",
            YiSyllables::YiSyllableJu => "yi syllable ju",
            YiSyllables::YiSyllableJup => "yi syllable jup",
            YiSyllables::YiSyllableJurx => "yi syllable jurx",
            YiSyllables::YiSyllableJur => "yi syllable jur",
            YiSyllables::YiSyllableJyt => "yi syllable jyt",
            YiSyllables::YiSyllableJyx => "yi syllable jyx",
            YiSyllables::YiSyllableJy => "yi syllable jy",
            YiSyllables::YiSyllableJyp => "yi syllable jyp",
            YiSyllables::YiSyllableJyrx => "yi syllable jyrx",
            YiSyllables::YiSyllableJyr => "yi syllable jyr",
            YiSyllables::YiSyllableQit => "yi syllable qit",
            YiSyllables::YiSyllableQix => "yi syllable qix",
            YiSyllables::YiSyllableQi => "yi syllable qi",
            YiSyllables::YiSyllableQip => "yi syllable qip",
            YiSyllables::YiSyllableQiet => "yi syllable qiet",
            YiSyllables::YiSyllableQiex => "yi syllable qiex",
            YiSyllables::YiSyllableQie => "yi syllable qie",
            YiSyllables::YiSyllableQiep => "yi syllable qiep",
            YiSyllables::YiSyllableQuot => "yi syllable quot",
            YiSyllables::YiSyllableQuox => "yi syllable quox",
            YiSyllables::YiSyllableQuo => "yi syllable quo",
            YiSyllables::YiSyllableQuop => "yi syllable quop",
            YiSyllables::YiSyllableQot => "yi syllable qot",
            YiSyllables::YiSyllableQox => "yi syllable qox",
            YiSyllables::YiSyllableQo => "yi syllable qo",
            YiSyllables::YiSyllableQop => "yi syllable qop",
            YiSyllables::YiSyllableQut => "yi syllable qut",
            YiSyllables::YiSyllableQux => "yi syllable qux",
            YiSyllables::YiSyllableQu => "yi syllable qu",
            YiSyllables::YiSyllableQup => "yi syllable qup",
            YiSyllables::YiSyllableQurx => "yi syllable qurx",
            YiSyllables::YiSyllableQur => "yi syllable qur",
            YiSyllables::YiSyllableQyt => "yi syllable qyt",
            YiSyllables::YiSyllableQyx => "yi syllable qyx",
            YiSyllables::YiSyllableQy => "yi syllable qy",
            YiSyllables::YiSyllableQyp => "yi syllable qyp",
            YiSyllables::YiSyllableQyrx => "yi syllable qyrx",
            YiSyllables::YiSyllableQyr => "yi syllable qyr",
            YiSyllables::YiSyllableJjit => "yi syllable jjit",
            YiSyllables::YiSyllableJjix => "yi syllable jjix",
            YiSyllables::YiSyllableJji => "yi syllable jji",
            YiSyllables::YiSyllableJjip => "yi syllable jjip",
            YiSyllables::YiSyllableJjiet => "yi syllable jjiet",
            YiSyllables::YiSyllableJjiex => "yi syllable jjiex",
            YiSyllables::YiSyllableJjie => "yi syllable jjie",
            YiSyllables::YiSyllableJjiep => "yi syllable jjiep",
            YiSyllables::YiSyllableJjuox => "yi syllable jjuox",
            YiSyllables::YiSyllableJjuo => "yi syllable jjuo",
            YiSyllables::YiSyllableJjuop => "yi syllable jjuop",
            YiSyllables::YiSyllableJjot => "yi syllable jjot",
            YiSyllables::YiSyllableJjox => "yi syllable jjox",
            YiSyllables::YiSyllableJjo => "yi syllable jjo",
            YiSyllables::YiSyllableJjop => "yi syllable jjop",
            YiSyllables::YiSyllableJjut => "yi syllable jjut",
            YiSyllables::YiSyllableJjux => "yi syllable jjux",
            YiSyllables::YiSyllableJju => "yi syllable jju",
            YiSyllables::YiSyllableJjup => "yi syllable jjup",
            YiSyllables::YiSyllableJjurx => "yi syllable jjurx",
            YiSyllables::YiSyllableJjur => "yi syllable jjur",
            YiSyllables::YiSyllableJjyt => "yi syllable jjyt",
            YiSyllables::YiSyllableJjyx => "yi syllable jjyx",
            YiSyllables::YiSyllableJjy => "yi syllable jjy",
            YiSyllables::YiSyllableJjyp => "yi syllable jjyp",
            YiSyllables::YiSyllableNjit => "yi syllable njit",
            YiSyllables::YiSyllableNjix => "yi syllable njix",
            YiSyllables::YiSyllableNji => "yi syllable nji",
            YiSyllables::YiSyllableNjip => "yi syllable njip",
            YiSyllables::YiSyllableNjiet => "yi syllable njiet",
            YiSyllables::YiSyllableNjiex => "yi syllable njiex",
            YiSyllables::YiSyllableNjie => "yi syllable njie",
            YiSyllables::YiSyllableNjiep => "yi syllable njiep",
            YiSyllables::YiSyllableNjuox => "yi syllable njuox",
            YiSyllables::YiSyllableNjuo => "yi syllable njuo",
            YiSyllables::YiSyllableNjot => "yi syllable njot",
            YiSyllables::YiSyllableNjox => "yi syllable njox",
            YiSyllables::YiSyllableNjo => "yi syllable njo",
            YiSyllables::YiSyllableNjop => "yi syllable njop",
            YiSyllables::YiSyllableNjux => "yi syllable njux",
            YiSyllables::YiSyllableNju => "yi syllable nju",
            YiSyllables::YiSyllableNjup => "yi syllable njup",
            YiSyllables::YiSyllableNjurx => "yi syllable njurx",
            YiSyllables::YiSyllableNjur => "yi syllable njur",
            YiSyllables::YiSyllableNjyt => "yi syllable njyt",
            YiSyllables::YiSyllableNjyx => "yi syllable njyx",
            YiSyllables::YiSyllableNjy => "yi syllable njy",
            YiSyllables::YiSyllableNjyp => "yi syllable njyp",
            YiSyllables::YiSyllableNjyrx => "yi syllable njyrx",
            YiSyllables::YiSyllableNjyr => "yi syllable njyr",
            YiSyllables::YiSyllableNyit => "yi syllable nyit",
            YiSyllables::YiSyllableNyix => "yi syllable nyix",
            YiSyllables::YiSyllableNyi => "yi syllable nyi",
            YiSyllables::YiSyllableNyip => "yi syllable nyip",
            YiSyllables::YiSyllableNyiet => "yi syllable nyiet",
            YiSyllables::YiSyllableNyiex => "yi syllable nyiex",
            YiSyllables::YiSyllableNyie => "yi syllable nyie",
            YiSyllables::YiSyllableNyiep => "yi syllable nyiep",
            YiSyllables::YiSyllableNyuox => "yi syllable nyuox",
            YiSyllables::YiSyllableNyuo => "yi syllable nyuo",
            YiSyllables::YiSyllableNyuop => "yi syllable nyuop",
            YiSyllables::YiSyllableNyot => "yi syllable nyot",
            YiSyllables::YiSyllableNyox => "yi syllable nyox",
            YiSyllables::YiSyllableNyo => "yi syllable nyo",
            YiSyllables::YiSyllableNyop => "yi syllable nyop",
            YiSyllables::YiSyllableNyut => "yi syllable nyut",
            YiSyllables::YiSyllableNyux => "yi syllable nyux",
            YiSyllables::YiSyllableNyu => "yi syllable nyu",
            YiSyllables::YiSyllableNyup => "yi syllable nyup",
            YiSyllables::YiSyllableXit => "yi syllable xit",
            YiSyllables::YiSyllableXix => "yi syllable xix",
            YiSyllables::YiSyllableXi => "yi syllable xi",
            YiSyllables::YiSyllableXip => "yi syllable xip",
            YiSyllables::YiSyllableXiet => "yi syllable xiet",
            YiSyllables::YiSyllableXiex => "yi syllable xiex",
            YiSyllables::YiSyllableXie => "yi syllable xie",
            YiSyllables::YiSyllableXiep => "yi syllable xiep",
            YiSyllables::YiSyllableXuox => "yi syllable xuox",
            YiSyllables::YiSyllableXuo => "yi syllable xuo",
            YiSyllables::YiSyllableXot => "yi syllable xot",
            YiSyllables::YiSyllableXox => "yi syllable xox",
            YiSyllables::YiSyllableXo => "yi syllable xo",
            YiSyllables::YiSyllableXop => "yi syllable xop",
            YiSyllables::YiSyllableXyt => "yi syllable xyt",
            YiSyllables::YiSyllableXyx => "yi syllable xyx",
            YiSyllables::YiSyllableXy => "yi syllable xy",
            YiSyllables::YiSyllableXyp => "yi syllable xyp",
            YiSyllables::YiSyllableXyrx => "yi syllable xyrx",
            YiSyllables::YiSyllableXyr => "yi syllable xyr",
            YiSyllables::YiSyllableYit => "yi syllable yit",
            YiSyllables::YiSyllableYix => "yi syllable yix",
            YiSyllables::YiSyllableYi => "yi syllable yi",
            YiSyllables::YiSyllableYip => "yi syllable yip",
            YiSyllables::YiSyllableYiet => "yi syllable yiet",
            YiSyllables::YiSyllableYiex => "yi syllable yiex",
            YiSyllables::YiSyllableYie => "yi syllable yie",
            YiSyllables::YiSyllableYiep => "yi syllable yiep",
            YiSyllables::YiSyllableYuot => "yi syllable yuot",
            YiSyllables::YiSyllableYuox => "yi syllable yuox",
            YiSyllables::YiSyllableYuo => "yi syllable yuo",
            YiSyllables::YiSyllableYuop => "yi syllable yuop",
            YiSyllables::YiSyllableYot => "yi syllable yot",
            YiSyllables::YiSyllableYox => "yi syllable yox",
            YiSyllables::YiSyllableYo => "yi syllable yo",
            YiSyllables::YiSyllableYop => "yi syllable yop",
            YiSyllables::YiSyllableYut => "yi syllable yut",
            YiSyllables::YiSyllableYux => "yi syllable yux",
            YiSyllables::YiSyllableYu => "yi syllable yu",
            YiSyllables::YiSyllableYup => "yi syllable yup",
            YiSyllables::YiSyllableYurx => "yi syllable yurx",
            YiSyllables::YiSyllableYur => "yi syllable yur",
            YiSyllables::YiSyllableYyt => "yi syllable yyt",
            YiSyllables::YiSyllableYyx => "yi syllable yyx",
            YiSyllables::YiSyllableYy => "yi syllable yy",
            YiSyllables::YiSyllableYyp => "yi syllable yyp",
            YiSyllables::YiSyllableYyrx => "yi syllable yyrx",
            YiSyllables::YiSyllableYyr => "yi syllable yyr",
        }
    }
}
