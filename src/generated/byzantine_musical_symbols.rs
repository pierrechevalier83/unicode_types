
/// An enum to represent all characters in the ByzantineMusicalSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ByzantineMusicalSymbols {
    /// \u{1d000}: '𝀀'
    ByzantineMusicalSymbolPsili,
    /// \u{1d001}: '𝀁'
    ByzantineMusicalSymbolDaseia,
    /// \u{1d002}: '𝀂'
    ByzantineMusicalSymbolPerispomeni,
    /// \u{1d003}: '𝀃'
    ByzantineMusicalSymbolOxeiaEkfonitikon,
    /// \u{1d004}: '𝀄'
    ByzantineMusicalSymbolOxeiaDipli,
    /// \u{1d005}: '𝀅'
    ByzantineMusicalSymbolVareiaEkfonitikon,
    /// \u{1d006}: '𝀆'
    ByzantineMusicalSymbolVareiaDipli,
    /// \u{1d007}: '𝀇'
    ByzantineMusicalSymbolKathisti,
    /// \u{1d008}: '𝀈'
    ByzantineMusicalSymbolSyrmatiki,
    /// \u{1d009}: '𝀉'
    ByzantineMusicalSymbolParaklitiki,
    /// \u{1d00a}: '𝀊'
    ByzantineMusicalSymbolYpokrisis,
    /// \u{1d00b}: '𝀋'
    ByzantineMusicalSymbolYpokrisisDipli,
    /// \u{1d00c}: '𝀌'
    ByzantineMusicalSymbolKremasti,
    /// \u{1d00d}: '𝀍'
    ByzantineMusicalSymbolApesoEkfonitikon,
    /// \u{1d00e}: '𝀎'
    ByzantineMusicalSymbolExoEkfonitikon,
    /// \u{1d00f}: '𝀏'
    ByzantineMusicalSymbolTeleia,
    /// \u{1d010}: '𝀐'
    ByzantineMusicalSymbolKentimata,
    /// \u{1d011}: '𝀑'
    ByzantineMusicalSymbolApostrofos,
    /// \u{1d012}: '𝀒'
    ByzantineMusicalSymbolApostrofosDipli,
    /// \u{1d013}: '𝀓'
    ByzantineMusicalSymbolSynevma,
    /// \u{1d014}: '𝀔'
    ByzantineMusicalSymbolThita,
    /// \u{1d015}: '𝀕'
    ByzantineMusicalSymbolOligonArchaion,
    /// \u{1d016}: '𝀖'
    ByzantineMusicalSymbolGorgonArchaion,
    /// \u{1d017}: '𝀗'
    ByzantineMusicalSymbolPsilon,
    /// \u{1d018}: '𝀘'
    ByzantineMusicalSymbolChamilon,
    /// \u{1d019}: '𝀙'
    ByzantineMusicalSymbolVathy,
    /// \u{1d01a}: '𝀚'
    ByzantineMusicalSymbolIsonArchaion,
    /// \u{1d01b}: '𝀛'
    ByzantineMusicalSymbolKentimaArchaion,
    /// \u{1d01c}: '𝀜'
    ByzantineMusicalSymbolKentimataArchaion,
    /// \u{1d01d}: '𝀝'
    ByzantineMusicalSymbolSaximata,
    /// \u{1d01e}: '𝀞'
    ByzantineMusicalSymbolParichon,
    /// \u{1d01f}: '𝀟'
    ByzantineMusicalSymbolStavrosApodexia,
    /// \u{1d020}: '𝀠'
    ByzantineMusicalSymbolOxeiaiArchaion,
    /// \u{1d021}: '𝀡'
    ByzantineMusicalSymbolVareiaiArchaion,
    /// \u{1d022}: '𝀢'
    ByzantineMusicalSymbolApodermaArchaion,
    /// \u{1d023}: '𝀣'
    ByzantineMusicalSymbolApothema,
    /// \u{1d024}: '𝀤'
    ByzantineMusicalSymbolKlasma,
    /// \u{1d025}: '𝀥'
    ByzantineMusicalSymbolRevma,
    /// \u{1d026}: '𝀦'
    ByzantineMusicalSymbolPiasmaArchaion,
    /// \u{1d027}: '𝀧'
    ByzantineMusicalSymbolTinagma,
    /// \u{1d028}: '𝀨'
    ByzantineMusicalSymbolAnatrichisma,
    /// \u{1d029}: '𝀩'
    ByzantineMusicalSymbolSeisma,
    /// \u{1d02a}: '𝀪'
    ByzantineMusicalSymbolSynagmaArchaion,
    /// \u{1d02b}: '𝀫'
    ByzantineMusicalSymbolSynagmaMetaStavrou,
    /// \u{1d02c}: '𝀬'
    ByzantineMusicalSymbolOyranismaArchaion,
    /// \u{1d02d}: '𝀭'
    ByzantineMusicalSymbolThema,
    /// \u{1d02e}: '𝀮'
    ByzantineMusicalSymbolLemoi,
    /// \u{1d02f}: '𝀯'
    ByzantineMusicalSymbolDyo,
    /// \u{1d030}: '𝀰'
    ByzantineMusicalSymbolTria,
    /// \u{1d031}: '𝀱'
    ByzantineMusicalSymbolTessera,
    /// \u{1d032}: '𝀲'
    ByzantineMusicalSymbolKratimata,
    /// \u{1d033}: '𝀳'
    ByzantineMusicalSymbolApesoExoNeo,
    /// \u{1d034}: '𝀴'
    ByzantineMusicalSymbolFthoraArchaion,
    /// \u{1d035}: '𝀵'
    ByzantineMusicalSymbolImifthora,
    /// \u{1d036}: '𝀶'
    ByzantineMusicalSymbolTromikonArchaion,
    /// \u{1d037}: '𝀷'
    ByzantineMusicalSymbolKatavaTromikon,
    /// \u{1d038}: '𝀸'
    ByzantineMusicalSymbolPelaston,
    /// \u{1d039}: '𝀹'
    ByzantineMusicalSymbolPsifiston,
    /// \u{1d03a}: '𝀺'
    ByzantineMusicalSymbolKontevma,
    /// \u{1d03b}: '𝀻'
    ByzantineMusicalSymbolChorevmaArchaion,
    /// \u{1d03c}: '𝀼'
    ByzantineMusicalSymbolRapisma,
    /// \u{1d03d}: '𝀽'
    ByzantineMusicalSymbolParakalesmaArchaion,
    /// \u{1d03e}: '𝀾'
    ByzantineMusicalSymbolParaklitikiArchaion,
    /// \u{1d03f}: '𝀿'
    ByzantineMusicalSymbolIchadin,
    /// \u{1d040}: '𝁀'
    ByzantineMusicalSymbolNana,
    /// \u{1d041}: '𝁁'
    ByzantineMusicalSymbolPetasma,
    /// \u{1d042}: '𝁂'
    ByzantineMusicalSymbolKontevmaAllo,
    /// \u{1d043}: '𝁃'
    ByzantineMusicalSymbolTromikonAllo,
    /// \u{1d044}: '𝁄'
    ByzantineMusicalSymbolStraggismata,
    /// \u{1d045}: '𝁅'
    ByzantineMusicalSymbolGronthismata,
    /// \u{1d046}: '𝁆'
    ByzantineMusicalSymbolIsonNeo,
    /// \u{1d047}: '𝁇'
    ByzantineMusicalSymbolOligonNeo,
    /// \u{1d048}: '𝁈'
    ByzantineMusicalSymbolOxeiaNeo,
    /// \u{1d049}: '𝁉'
    ByzantineMusicalSymbolPetasti,
    /// \u{1d04a}: '𝁊'
    ByzantineMusicalSymbolKoufisma,
    /// \u{1d04b}: '𝁋'
    ByzantineMusicalSymbolPetastokoufisma,
    /// \u{1d04c}: '𝁌'
    ByzantineMusicalSymbolKratimokoufisma,
    /// \u{1d04d}: '𝁍'
    ByzantineMusicalSymbolPelastonNeo,
    /// \u{1d04e}: '𝁎'
    ByzantineMusicalSymbolKentimataNeoAno,
    /// \u{1d04f}: '𝁏'
    ByzantineMusicalSymbolKentimaNeoAno,
    /// \u{1d050}: '𝁐'
    ByzantineMusicalSymbolYpsili,
    /// \u{1d051}: '𝁑'
    ByzantineMusicalSymbolApostrofosNeo,
    /// \u{1d052}: '𝁒'
    ByzantineMusicalSymbolApostrofoiSyndesmosNeo,
    /// \u{1d053}: '𝁓'
    ByzantineMusicalSymbolYporroi,
    /// \u{1d054}: '𝁔'
    ByzantineMusicalSymbolKratimoyporroon,
    /// \u{1d055}: '𝁕'
    ByzantineMusicalSymbolElafron,
    /// \u{1d056}: '𝁖'
    ByzantineMusicalSymbolChamili,
    /// \u{1d057}: '𝁗'
    ByzantineMusicalSymbolMikronIson,
    /// \u{1d058}: '𝁘'
    ByzantineMusicalSymbolVareiaNeo,
    /// \u{1d059}: '𝁙'
    ByzantineMusicalSymbolPiasmaNeo,
    /// \u{1d05a}: '𝁚'
    ByzantineMusicalSymbolPsifistonNeo,
    /// \u{1d05b}: '𝁛'
    ByzantineMusicalSymbolOmalon,
    /// \u{1d05c}: '𝁜'
    ByzantineMusicalSymbolAntikenoma,
    /// \u{1d05d}: '𝁝'
    ByzantineMusicalSymbolLygisma,
    /// \u{1d05e}: '𝁞'
    ByzantineMusicalSymbolParaklitikiNeo,
    /// \u{1d05f}: '𝁟'
    ByzantineMusicalSymbolParakalesmaNeo,
    /// \u{1d060}: '𝁠'
    ByzantineMusicalSymbolEteronParakalesma,
    /// \u{1d061}: '𝁡'
    ByzantineMusicalSymbolKylisma,
    /// \u{1d062}: '𝁢'
    ByzantineMusicalSymbolAntikenokylisma,
    /// \u{1d063}: '𝁣'
    ByzantineMusicalSymbolTromikonNeo,
    /// \u{1d064}: '𝁤'
    ByzantineMusicalSymbolEkstrepton,
    /// \u{1d065}: '𝁥'
    ByzantineMusicalSymbolSynagmaNeo,
    /// \u{1d066}: '𝁦'
    ByzantineMusicalSymbolSyrma,
    /// \u{1d067}: '𝁧'
    ByzantineMusicalSymbolChorevmaNeo,
    /// \u{1d068}: '𝁨'
    ByzantineMusicalSymbolEpegerma,
    /// \u{1d069}: '𝁩'
    ByzantineMusicalSymbolSeismaNeo,
    /// \u{1d06a}: '𝁪'
    ByzantineMusicalSymbolXironKlasma,
    /// \u{1d06b}: '𝁫'
    ByzantineMusicalSymbolTromikopsifiston,
    /// \u{1d06c}: '𝁬'
    ByzantineMusicalSymbolPsifistolygisma,
    /// \u{1d06d}: '𝁭'
    ByzantineMusicalSymbolTromikolygisma,
    /// \u{1d06e}: '𝁮'
    ByzantineMusicalSymbolTromikoparakalesma,
    /// \u{1d06f}: '𝁯'
    ByzantineMusicalSymbolPsifistoparakalesma,
    /// \u{1d070}: '𝁰'
    ByzantineMusicalSymbolTromikosynagma,
    /// \u{1d071}: '𝁱'
    ByzantineMusicalSymbolPsifistosynagma,
    /// \u{1d072}: '𝁲'
    ByzantineMusicalSymbolGorgosyntheton,
    /// \u{1d073}: '𝁳'
    ByzantineMusicalSymbolArgosyntheton,
    /// \u{1d074}: '𝁴'
    ByzantineMusicalSymbolEteronArgosyntheton,
    /// \u{1d075}: '𝁵'
    ByzantineMusicalSymbolOyranismaNeo,
    /// \u{1d076}: '𝁶'
    ByzantineMusicalSymbolThematismosEso,
    /// \u{1d077}: '𝁷'
    ByzantineMusicalSymbolThematismosExo,
    /// \u{1d078}: '𝁸'
    ByzantineMusicalSymbolThemaAploun,
    /// \u{1d079}: '𝁹'
    ByzantineMusicalSymbolThesKaiApothes,
    /// \u{1d07a}: '𝁺'
    ByzantineMusicalSymbolKatavasma,
    /// \u{1d07b}: '𝁻'
    ByzantineMusicalSymbolEndofonon,
    /// \u{1d07c}: '𝁼'
    ByzantineMusicalSymbolYfenKato,
    /// \u{1d07d}: '𝁽'
    ByzantineMusicalSymbolYfenAno,
    /// \u{1d07e}: '𝁾'
    ByzantineMusicalSymbolStavros,
    /// \u{1d07f}: '𝁿'
    ByzantineMusicalSymbolKlasmaAno,
    /// \u{1d080}: '𝂀'
    ByzantineMusicalSymbolDipliArchaion,
    /// \u{1d081}: '𝂁'
    ByzantineMusicalSymbolKratimaArchaion,
    /// \u{1d082}: '𝂂'
    ByzantineMusicalSymbolKratimaAllo,
    /// \u{1d083}: '𝂃'
    ByzantineMusicalSymbolKratimaNeo,
    /// \u{1d084}: '𝂄'
    ByzantineMusicalSymbolApodermaNeo,
    /// \u{1d085}: '𝂅'
    ByzantineMusicalSymbolApli,
    /// \u{1d086}: '𝂆'
    ByzantineMusicalSymbolDipli,
    /// \u{1d087}: '𝂇'
    ByzantineMusicalSymbolTripli,
    /// \u{1d088}: '𝂈'
    ByzantineMusicalSymbolTetrapli,
    /// \u{1d089}: '𝂉'
    ByzantineMusicalSymbolKoronis,
    /// \u{1d08a}: '𝂊'
    ByzantineMusicalSymbolLeimmaEnosChronou,
    /// \u{1d08b}: '𝂋'
    ByzantineMusicalSymbolLeimmaDyoChronon,
    /// \u{1d08c}: '𝂌'
    ByzantineMusicalSymbolLeimmaTrionChronon,
    /// \u{1d08d}: '𝂍'
    ByzantineMusicalSymbolLeimmaTessaronChronon,
    /// \u{1d08e}: '𝂎'
    ByzantineMusicalSymbolLeimmaImiseosChronou,
    /// \u{1d08f}: '𝂏'
    ByzantineMusicalSymbolGorgonNeoAno,
    /// \u{1d090}: '𝂐'
    ByzantineMusicalSymbolGorgonParestigmenonAristera,
    /// \u{1d091}: '𝂑'
    ByzantineMusicalSymbolGorgonParestigmenonDexia,
    /// \u{1d092}: '𝂒'
    ByzantineMusicalSymbolDigorgon,
    /// \u{1d093}: '𝂓'
    ByzantineMusicalSymbolDigorgonParestigmenonAristeraKato,
    /// \u{1d094}: '𝂔'
    ByzantineMusicalSymbolDigorgonParestigmenonAristeraAno,
    /// \u{1d095}: '𝂕'
    ByzantineMusicalSymbolDigorgonParestigmenonDexia,
    /// \u{1d096}: '𝂖'
    ByzantineMusicalSymbolTrigorgon,
    /// \u{1d097}: '𝂗'
    ByzantineMusicalSymbolArgon,
    /// \u{1d098}: '𝂘'
    ByzantineMusicalSymbolImidiargon,
    /// \u{1d099}: '𝂙'
    ByzantineMusicalSymbolDiargon,
    /// \u{1d09a}: '𝂚'
    ByzantineMusicalSymbolAgogiPoliArgi,
    /// \u{1d09b}: '𝂛'
    ByzantineMusicalSymbolAgogiArgoteri,
    /// \u{1d09c}: '𝂜'
    ByzantineMusicalSymbolAgogiArgi,
    /// \u{1d09d}: '𝂝'
    ByzantineMusicalSymbolAgogiMetria,
    /// \u{1d09e}: '𝂞'
    ByzantineMusicalSymbolAgogiMesi,
    /// \u{1d09f}: '𝂟'
    ByzantineMusicalSymbolAgogiGorgi,
    /// \u{1d0a0}: '𝂠'
    ByzantineMusicalSymbolAgogiGorgoteri,
    /// \u{1d0a1}: '𝂡'
    ByzantineMusicalSymbolAgogiPoliGorgi,
    /// \u{1d0a2}: '𝂢'
    ByzantineMusicalSymbolMartyriaProtosIchos,
    /// \u{1d0a3}: '𝂣'
    ByzantineMusicalSymbolMartyriaAlliProtosIchos,
    /// \u{1d0a4}: '𝂤'
    ByzantineMusicalSymbolMartyriaDeyterosIchos,
    /// \u{1d0a5}: '𝂥'
    ByzantineMusicalSymbolMartyriaAlliDeyterosIchos,
    /// \u{1d0a6}: '𝂦'
    ByzantineMusicalSymbolMartyriaTritosIchos,
    /// \u{1d0a7}: '𝂧'
    ByzantineMusicalSymbolMartyriaTrifonias,
    /// \u{1d0a8}: '𝂨'
    ByzantineMusicalSymbolMartyriaTetartosIchos,
    /// \u{1d0a9}: '𝂩'
    ByzantineMusicalSymbolMartyriaTetartosLegetosIchos,
    /// \u{1d0aa}: '𝂪'
    ByzantineMusicalSymbolMartyriaLegetosIchos,
    /// \u{1d0ab}: '𝂫'
    ByzantineMusicalSymbolMartyriaPlagiosIchos,
    /// \u{1d0ac}: '𝂬'
    ByzantineMusicalSymbolIsakiaTelousIchimatos,
    /// \u{1d0ad}: '𝂭'
    ByzantineMusicalSymbolApostrofoiTelousIchimatos,
    /// \u{1d0ae}: '𝂮'
    ByzantineMusicalSymbolFanerosisTetrafonias,
    /// \u{1d0af}: '𝂯'
    ByzantineMusicalSymbolFanerosisMonofonias,
    /// \u{1d0b0}: '𝂰'
    ByzantineMusicalSymbolFanerosisDifonias,
    /// \u{1d0b1}: '𝂱'
    ByzantineMusicalSymbolMartyriaVarysIchos,
    /// \u{1d0b2}: '𝂲'
    ByzantineMusicalSymbolMartyriaProtovarysIchos,
    /// \u{1d0b3}: '𝂳'
    ByzantineMusicalSymbolMartyriaPlagiosTetartosIchos,
    /// \u{1d0b4}: '𝂴'
    ByzantineMusicalSymbolGorthmikonNAploun,
    /// \u{1d0b5}: '𝂵'
    ByzantineMusicalSymbolGorthmikonNDiploun,
    /// \u{1d0b6}: '𝂶'
    ByzantineMusicalSymbolEnarxisKaiFthoraVou,
    /// \u{1d0b7}: '𝂷'
    ByzantineMusicalSymbolImifonon,
    /// \u{1d0b8}: '𝂸'
    ByzantineMusicalSymbolImifthoron,
    /// \u{1d0b9}: '𝂹'
    ByzantineMusicalSymbolFthoraArchaionDeyterouIchou,
    /// \u{1d0ba}: '𝂺'
    ByzantineMusicalSymbolFthoraDiatonikiPa,
    /// \u{1d0bb}: '𝂻'
    ByzantineMusicalSymbolFthoraDiatonikiNana,
    /// \u{1d0bc}: '𝂼'
    ByzantineMusicalSymbolFthoraNaosIchos,
    /// \u{1d0bd}: '𝂽'
    ByzantineMusicalSymbolFthoraDiatonikiDi,
    /// \u{1d0be}: '𝂾'
    ByzantineMusicalSymbolFthoraSklironDiatononDi,
    /// \u{1d0bf}: '𝂿'
    ByzantineMusicalSymbolFthoraDiatonikiKe,
    /// \u{1d0c0}: '𝃀'
    ByzantineMusicalSymbolFthoraDiatonikiZo,
    /// \u{1d0c1}: '𝃁'
    ByzantineMusicalSymbolFthoraDiatonikiNiKato,
    /// \u{1d0c2}: '𝃂'
    ByzantineMusicalSymbolFthoraDiatonikiNiAno,
    /// \u{1d0c3}: '𝃃'
    ByzantineMusicalSymbolFthoraMalakonChromaDifonias,
    /// \u{1d0c4}: '𝃄'
    ByzantineMusicalSymbolFthoraMalakonChromaMonofonias,
    /// \u{1d0c5}: '𝃅'
    ByzantineMusicalSymbolFhtoraSklironChromaVasis,
    /// \u{1d0c6}: '𝃆'
    ByzantineMusicalSymbolFthoraSklironChromaSynafi,
    /// \u{1d0c7}: '𝃇'
    ByzantineMusicalSymbolFthoraNenano,
    /// \u{1d0c8}: '𝃈'
    ByzantineMusicalSymbolChroaZygos,
    /// \u{1d0c9}: '𝃉'
    ByzantineMusicalSymbolChroaKliton,
    /// \u{1d0ca}: '𝃊'
    ByzantineMusicalSymbolChroaSpathi,
    /// \u{1d0cb}: '𝃋'
    ByzantineMusicalSymbolFthoraIYfesisTetartimorion,
    /// \u{1d0cc}: '𝃌'
    ByzantineMusicalSymbolFthoraEnarmoniosAntifonia,
    /// \u{1d0cd}: '𝃍'
    ByzantineMusicalSymbolYfesisTritimorion,
    /// \u{1d0ce}: '𝃎'
    ByzantineMusicalSymbolDiesisTritimorion,
    /// \u{1d0cf}: '𝃏'
    ByzantineMusicalSymbolDiesisTetartimorion,
    /// \u{1d0d0}: '𝃐'
    ByzantineMusicalSymbolDiesisApliDyoDodekata,
    /// \u{1d0d1}: '𝃑'
    ByzantineMusicalSymbolDiesisMonogrammosTesseraDodekata,
    /// \u{1d0d2}: '𝃒'
    ByzantineMusicalSymbolDiesisDigrammosExDodekata,
    /// \u{1d0d3}: '𝃓'
    ByzantineMusicalSymbolDiesisTrigrammosOktoDodekata,
    /// \u{1d0d4}: '𝃔'
    ByzantineMusicalSymbolYfesisApliDyoDodekata,
    /// \u{1d0d5}: '𝃕'
    ByzantineMusicalSymbolYfesisMonogrammosTesseraDodekata,
    /// \u{1d0d6}: '𝃖'
    ByzantineMusicalSymbolYfesisDigrammosExDodekata,
    /// \u{1d0d7}: '𝃗'
    ByzantineMusicalSymbolYfesisTrigrammosOktoDodekata,
    /// \u{1d0d8}: '𝃘'
    ByzantineMusicalSymbolGenikiDiesis,
    /// \u{1d0d9}: '𝃙'
    ByzantineMusicalSymbolGenikiYfesis,
    /// \u{1d0da}: '𝃚'
    ByzantineMusicalSymbolDiastoliApliMikri,
    /// \u{1d0db}: '𝃛'
    ByzantineMusicalSymbolDiastoliApliMegali,
    /// \u{1d0dc}: '𝃜'
    ByzantineMusicalSymbolDiastoliDipli,
    /// \u{1d0dd}: '𝃝'
    ByzantineMusicalSymbolDiastoliTheseos,
    /// \u{1d0de}: '𝃞'
    ByzantineMusicalSymbolSimansisTheseos,
    /// \u{1d0df}: '𝃟'
    ByzantineMusicalSymbolSimansisTheseosDisimou,
    /// \u{1d0e0}: '𝃠'
    ByzantineMusicalSymbolSimansisTheseosTrisimou,
    /// \u{1d0e1}: '𝃡'
    ByzantineMusicalSymbolSimansisTheseosTetrasimou,
    /// \u{1d0e2}: '𝃢'
    ByzantineMusicalSymbolSimansisArseos,
    /// \u{1d0e3}: '𝃣'
    ByzantineMusicalSymbolSimansisArseosDisimou,
    /// \u{1d0e4}: '𝃤'
    ByzantineMusicalSymbolSimansisArseosTrisimou,
    /// \u{1d0e5}: '𝃥'
    ByzantineMusicalSymbolSimansisArseosTetrasimou,
    /// \u{1d0e6}: '𝃦'
    ByzantineMusicalSymbolDigrammaGg,
    /// \u{1d0e7}: '𝃧'
    ByzantineMusicalSymbolDiftoggosOu,
    /// \u{1d0e8}: '𝃨'
    ByzantineMusicalSymbolStigma,
    /// \u{1d0e9}: '𝃩'
    ByzantineMusicalSymbolArktikoPa,
    /// \u{1d0ea}: '𝃪'
    ByzantineMusicalSymbolArktikoVou,
    /// \u{1d0eb}: '𝃫'
    ByzantineMusicalSymbolArktikoGa,
    /// \u{1d0ec}: '𝃬'
    ByzantineMusicalSymbolArktikoDi,
    /// \u{1d0ed}: '𝃭'
    ByzantineMusicalSymbolArktikoKe,
    /// \u{1d0ee}: '𝃮'
    ByzantineMusicalSymbolArktikoZo,
    /// \u{1d0ef}: '𝃯'
    ByzantineMusicalSymbolArktikoNi,
    /// \u{1d0f0}: '𝃰'
    ByzantineMusicalSymbolKentimataNeoMeso,
    /// \u{1d0f1}: '𝃱'
    ByzantineMusicalSymbolKentimaNeoMeso,
    /// \u{1d0f2}: '𝃲'
    ByzantineMusicalSymbolKentimataNeoKato,
    /// \u{1d0f3}: '𝃳'
    ByzantineMusicalSymbolKentimaNeoKato,
    /// \u{1d0f4}: '𝃴'
    ByzantineMusicalSymbolKlasmaKato,
    /// \u{1d0f5}: '𝃵'
    ByzantineMusicalSymbolGorgonNeoKato,
}

impl Into<char> for ByzantineMusicalSymbols {
    fn into(self) -> char {
        match self {
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsili => '𝀀',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDaseia => '𝀁',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPerispomeni => '𝀂',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaEkfonitikon => '𝀃',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaDipli => '𝀄',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaEkfonitikon => '𝀅',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaDipli => '𝀆',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKathisti => '𝀇',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSyrmatiki => '𝀈',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitiki => '𝀉',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYpokrisis => '𝀊',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYpokrisisDipli => '𝀋',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKremasti => '𝀌',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApesoEkfonitikon => '𝀍',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolExoEkfonitikon => '𝀎',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTeleia => '𝀏',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimata => '𝀐',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofos => '𝀑',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofosDipli => '𝀒',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSynevma => '𝀓',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThita => '𝀔',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOligonArchaion => '𝀕',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonArchaion => '𝀖',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsilon => '𝀗',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChamilon => '𝀘',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVathy => '𝀙',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolIsonArchaion => '𝀚',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaArchaion => '𝀛',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataArchaion => '𝀜',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSaximata => '𝀝',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParichon => '𝀞',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolStavrosApodexia => '𝀟',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaiArchaion => '𝀠',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaiArchaion => '𝀡',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApodermaArchaion => '𝀢',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApothema => '𝀣',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasma => '𝀤',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolRevma => '𝀥',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPiasmaArchaion => '𝀦',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTinagma => '𝀧',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAnatrichisma => '𝀨',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSeisma => '𝀩',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaArchaion => '𝀪',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaMetaStavrou => '𝀫',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOyranismaArchaion => '𝀬',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThema => '𝀭',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLemoi => '𝀮',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDyo => '𝀯',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTria => '𝀰',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTessera => '𝀱',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimata => '𝀲',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApesoExoNeo => '𝀳',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraArchaion => '𝀴',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolImifthora => '𝀵',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonArchaion => '𝀶',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKatavaTromikon => '𝀷',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPelaston => '𝀸',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifiston => '𝀹',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKontevma => '𝀺',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChorevmaArchaion => '𝀻',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolRapisma => '𝀼',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParakalesmaArchaion => '𝀽',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitikiArchaion => '𝀾',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolIchadin => '𝀿',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolNana => '𝁀',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPetasma => '𝁁',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKontevmaAllo => '𝁂',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonAllo => '𝁃',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolStraggismata => '𝁄',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGronthismata => '𝁅',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolIsonNeo => '𝁆',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOligonNeo => '𝁇',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaNeo => '𝁈',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPetasti => '𝁉',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKoufisma => '𝁊',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPetastokoufisma => '𝁋',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimokoufisma => '𝁌',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPelastonNeo => '𝁍',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoAno => '𝁎',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoAno => '𝁏',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYpsili => '𝁐',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofosNeo => '𝁑',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofoiSyndesmosNeo => '𝁒',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYporroi => '𝁓',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimoyporroon => '𝁔',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolElafron => '𝁕',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChamili => '𝁖',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMikronIson => '𝁗',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaNeo => '𝁘',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPiasmaNeo => '𝁙',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistonNeo => '𝁚',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOmalon => '𝁛',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAntikenoma => '𝁜',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLygisma => '𝁝',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitikiNeo => '𝁞',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParakalesmaNeo => '𝁟',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEteronParakalesma => '𝁠',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKylisma => '𝁡',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAntikenokylisma => '𝁢',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonNeo => '𝁣',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEkstrepton => '𝁤',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaNeo => '𝁥',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSyrma => '𝁦',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChorevmaNeo => '𝁧',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEpegerma => '𝁨',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSeismaNeo => '𝁩',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolXironKlasma => '𝁪',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikopsifiston => '𝁫',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistolygisma => '𝁬',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikolygisma => '𝁭',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikoparakalesma => '𝁮',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistoparakalesma => '𝁯',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikosynagma => '𝁰',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistosynagma => '𝁱',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgosyntheton => '𝁲',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArgosyntheton => '𝁳',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEteronArgosyntheton => '𝁴',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOyranismaNeo => '𝁵',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThematismosEso => '𝁶',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThematismosExo => '𝁷',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThemaAploun => '𝁸',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThesKaiApothes => '𝁹',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKatavasma => '𝁺',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEndofonon => '𝁻',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfenKato => '𝁼',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfenAno => '𝁽',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolStavros => '𝁾',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasmaAno => '𝁿',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDipliArchaion => '𝂀',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaArchaion => '𝂁',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaAllo => '𝂂',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaNeo => '𝂃',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApodermaNeo => '𝂄',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApli => '𝂅',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDipli => '𝂆',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTripli => '𝂇',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTetrapli => '𝂈',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKoronis => '𝂉',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaEnosChronou => '𝂊',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaDyoChronon => '𝂋',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaTrionChronon => '𝂌',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaTessaronChronon => '𝂍',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaImiseosChronou => '𝂎',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonNeoAno => '𝂏',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonParestigmenonAristera => '𝂐',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonParestigmenonDexia => '𝂑',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgon => '𝂒',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonAristeraKato => '𝂓',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonAristeraAno => '𝂔',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonDexia => '𝂕',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTrigorgon => '𝂖',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArgon => '𝂗',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolImidiargon => '𝂘',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiargon => '𝂙',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiPoliArgi => '𝂚',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiArgoteri => '𝂛',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiArgi => '𝂜',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiMetria => '𝂝',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiMesi => '𝂞',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiGorgi => '𝂟',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiGorgoteri => '𝂠',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiPoliGorgi => '𝂡',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaProtosIchos => '𝂢',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaAlliProtosIchos => '𝂣',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaDeyterosIchos => '𝂤',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaAlliDeyterosIchos => '𝂥',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTritosIchos => '𝂦',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTrifonias => '𝂧',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTetartosIchos => '𝂨',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTetartosLegetosIchos => '𝂩',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaLegetosIchos => '𝂪',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaPlagiosIchos => '𝂫',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolIsakiaTelousIchimatos => '𝂬',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofoiTelousIchimatos => '𝂭',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisTetrafonias => '𝂮',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisMonofonias => '𝂯',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisDifonias => '𝂰',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaVarysIchos => '𝂱',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaProtovarysIchos => '𝂲',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaPlagiosTetartosIchos => '𝂳',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorthmikonNAploun => '𝂴',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorthmikonNDiploun => '𝂵',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEnarxisKaiFthoraVou => '𝂶',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolImifonon => '𝂷',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolImifthoron => '𝂸',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraArchaionDeyterouIchou => '𝂹',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiPa => '𝂺',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNana => '𝂻',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraNaosIchos => '𝂼',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiDi => '𝂽',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraSklironDiatononDi => '𝂾',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiKe => '𝂿',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiZo => '𝃀',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNiKato => '𝃁',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNiAno => '𝃂',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraMalakonChromaDifonias => '𝃃',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraMalakonChromaMonofonias => '𝃄',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFhtoraSklironChromaVasis => '𝃅',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraSklironChromaSynafi => '𝃆',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraNenano => '𝃇',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaZygos => '𝃈',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaKliton => '𝃉',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaSpathi => '𝃊',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraIYfesisTetartimorion => '𝃋',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraEnarmoniosAntifonia => '𝃌',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisTritimorion => '𝃍',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTritimorion => '𝃎',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTetartimorion => '𝃏',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisApliDyoDodekata => '𝃐',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisMonogrammosTesseraDodekata => '𝃑',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisDigrammosExDodekata => '𝃒',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTrigrammosOktoDodekata => '𝃓',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisApliDyoDodekata => '𝃔',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisMonogrammosTesseraDodekata => '𝃕',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisDigrammosExDodekata => '𝃖',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisTrigrammosOktoDodekata => '𝃗',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGenikiDiesis => '𝃘',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGenikiYfesis => '𝃙',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliApliMikri => '𝃚',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliApliMegali => '𝃛',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliDipli => '𝃜',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliTheseos => '𝃝',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseos => '𝃞',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosDisimou => '𝃟',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosTrisimou => '𝃠',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosTetrasimou => '𝃡',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseos => '𝃢',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosDisimou => '𝃣',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosTrisimou => '𝃤',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosTetrasimou => '𝃥',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigrammaGg => '𝃦',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiftoggosOu => '𝃧',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolStigma => '𝃨',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoPa => '𝃩',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoVou => '𝃪',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoGa => '𝃫',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoDi => '𝃬',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoKe => '𝃭',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoZo => '𝃮',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoNi => '𝃯',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoMeso => '𝃰',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoMeso => '𝃱',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoKato => '𝃲',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoKato => '𝃳',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasmaKato => '𝃴',
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonNeoKato => '𝃵',
        }
    }
}

impl std::convert::TryFrom<char> for ByzantineMusicalSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𝀀' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPsili),
            '𝀁' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDaseia),
            '𝀂' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPerispomeni),
            '𝀃' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaEkfonitikon),
            '𝀄' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaDipli),
            '𝀅' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaEkfonitikon),
            '𝀆' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaDipli),
            '𝀇' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKathisti),
            '𝀈' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSyrmatiki),
            '𝀉' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitiki),
            '𝀊' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYpokrisis),
            '𝀋' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYpokrisisDipli),
            '𝀌' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKremasti),
            '𝀍' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApesoEkfonitikon),
            '𝀎' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolExoEkfonitikon),
            '𝀏' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTeleia),
            '𝀐' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimata),
            '𝀑' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofos),
            '𝀒' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofosDipli),
            '𝀓' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSynevma),
            '𝀔' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolThita),
            '𝀕' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOligonArchaion),
            '𝀖' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonArchaion),
            '𝀗' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPsilon),
            '𝀘' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolChamilon),
            '𝀙' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolVathy),
            '𝀚' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolIsonArchaion),
            '𝀛' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaArchaion),
            '𝀜' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataArchaion),
            '𝀝' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSaximata),
            '𝀞' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolParichon),
            '𝀟' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolStavrosApodexia),
            '𝀠' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaiArchaion),
            '𝀡' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaiArchaion),
            '𝀢' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApodermaArchaion),
            '𝀣' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApothema),
            '𝀤' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasma),
            '𝀥' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolRevma),
            '𝀦' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPiasmaArchaion),
            '𝀧' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTinagma),
            '𝀨' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAnatrichisma),
            '𝀩' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSeisma),
            '𝀪' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaArchaion),
            '𝀫' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaMetaStavrou),
            '𝀬' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOyranismaArchaion),
            '𝀭' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolThema),
            '𝀮' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolLemoi),
            '𝀯' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDyo),
            '𝀰' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTria),
            '𝀱' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTessera),
            '𝀲' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimata),
            '𝀳' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApesoExoNeo),
            '𝀴' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraArchaion),
            '𝀵' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolImifthora),
            '𝀶' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonArchaion),
            '𝀷' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKatavaTromikon),
            '𝀸' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPelaston),
            '𝀹' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifiston),
            '𝀺' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKontevma),
            '𝀻' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolChorevmaArchaion),
            '𝀼' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolRapisma),
            '𝀽' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolParakalesmaArchaion),
            '𝀾' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitikiArchaion),
            '𝀿' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolIchadin),
            '𝁀' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolNana),
            '𝁁' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPetasma),
            '𝁂' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKontevmaAllo),
            '𝁃' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonAllo),
            '𝁄' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolStraggismata),
            '𝁅' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGronthismata),
            '𝁆' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolIsonNeo),
            '𝁇' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOligonNeo),
            '𝁈' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaNeo),
            '𝁉' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPetasti),
            '𝁊' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKoufisma),
            '𝁋' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPetastokoufisma),
            '𝁌' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimokoufisma),
            '𝁍' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPelastonNeo),
            '𝁎' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoAno),
            '𝁏' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoAno),
            '𝁐' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYpsili),
            '𝁑' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofosNeo),
            '𝁒' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofoiSyndesmosNeo),
            '𝁓' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYporroi),
            '𝁔' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimoyporroon),
            '𝁕' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolElafron),
            '𝁖' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolChamili),
            '𝁗' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMikronIson),
            '𝁘' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaNeo),
            '𝁙' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPiasmaNeo),
            '𝁚' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistonNeo),
            '𝁛' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOmalon),
            '𝁜' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAntikenoma),
            '𝁝' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolLygisma),
            '𝁞' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitikiNeo),
            '𝁟' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolParakalesmaNeo),
            '𝁠' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolEteronParakalesma),
            '𝁡' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKylisma),
            '𝁢' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAntikenokylisma),
            '𝁣' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonNeo),
            '𝁤' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolEkstrepton),
            '𝁥' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaNeo),
            '𝁦' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSyrma),
            '𝁧' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolChorevmaNeo),
            '𝁨' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolEpegerma),
            '𝁩' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSeismaNeo),
            '𝁪' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolXironKlasma),
            '𝁫' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikopsifiston),
            '𝁬' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistolygisma),
            '𝁭' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikolygisma),
            '𝁮' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikoparakalesma),
            '𝁯' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistoparakalesma),
            '𝁰' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikosynagma),
            '𝁱' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistosynagma),
            '𝁲' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgosyntheton),
            '𝁳' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArgosyntheton),
            '𝁴' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolEteronArgosyntheton),
            '𝁵' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolOyranismaNeo),
            '𝁶' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolThematismosEso),
            '𝁷' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolThematismosExo),
            '𝁸' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolThemaAploun),
            '𝁹' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolThesKaiApothes),
            '𝁺' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKatavasma),
            '𝁻' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolEndofonon),
            '𝁼' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYfenKato),
            '𝁽' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYfenAno),
            '𝁾' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolStavros),
            '𝁿' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasmaAno),
            '𝂀' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDipliArchaion),
            '𝂁' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaArchaion),
            '𝂂' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaAllo),
            '𝂃' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaNeo),
            '𝂄' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApodermaNeo),
            '𝂅' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApli),
            '𝂆' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDipli),
            '𝂇' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTripli),
            '𝂈' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTetrapli),
            '𝂉' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKoronis),
            '𝂊' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaEnosChronou),
            '𝂋' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaDyoChronon),
            '𝂌' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaTrionChronon),
            '𝂍' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaTessaronChronon),
            '𝂎' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaImiseosChronou),
            '𝂏' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonNeoAno),
            '𝂐' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonParestigmenonAristera),
            '𝂑' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonParestigmenonDexia),
            '𝂒' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgon),
            '𝂓' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonAristeraKato),
            '𝂔' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonAristeraAno),
            '𝂕' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonDexia),
            '𝂖' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolTrigorgon),
            '𝂗' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArgon),
            '𝂘' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolImidiargon),
            '𝂙' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiargon),
            '𝂚' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiPoliArgi),
            '𝂛' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiArgoteri),
            '𝂜' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiArgi),
            '𝂝' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiMetria),
            '𝂞' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiMesi),
            '𝂟' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiGorgi),
            '𝂠' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiGorgoteri),
            '𝂡' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiPoliGorgi),
            '𝂢' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaProtosIchos),
            '𝂣' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaAlliProtosIchos),
            '𝂤' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaDeyterosIchos),
            '𝂥' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaAlliDeyterosIchos),
            '𝂦' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTritosIchos),
            '𝂧' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTrifonias),
            '𝂨' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTetartosIchos),
            '𝂩' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTetartosLegetosIchos),
            '𝂪' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaLegetosIchos),
            '𝂫' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaPlagiosIchos),
            '𝂬' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolIsakiaTelousIchimatos),
            '𝂭' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofoiTelousIchimatos),
            '𝂮' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisTetrafonias),
            '𝂯' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisMonofonias),
            '𝂰' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisDifonias),
            '𝂱' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaVarysIchos),
            '𝂲' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaProtovarysIchos),
            '𝂳' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaPlagiosTetartosIchos),
            '𝂴' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGorthmikonNAploun),
            '𝂵' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGorthmikonNDiploun),
            '𝂶' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolEnarxisKaiFthoraVou),
            '𝂷' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolImifonon),
            '𝂸' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolImifthoron),
            '𝂹' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraArchaionDeyterouIchou),
            '𝂺' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiPa),
            '𝂻' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNana),
            '𝂼' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraNaosIchos),
            '𝂽' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiDi),
            '𝂾' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraSklironDiatononDi),
            '𝂿' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiKe),
            '𝃀' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiZo),
            '𝃁' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNiKato),
            '𝃂' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNiAno),
            '𝃃' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraMalakonChromaDifonias),
            '𝃄' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraMalakonChromaMonofonias),
            '𝃅' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFhtoraSklironChromaVasis),
            '𝃆' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraSklironChromaSynafi),
            '𝃇' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraNenano),
            '𝃈' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaZygos),
            '𝃉' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaKliton),
            '𝃊' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaSpathi),
            '𝃋' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraIYfesisTetartimorion),
            '𝃌' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraEnarmoniosAntifonia),
            '𝃍' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisTritimorion),
            '𝃎' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTritimorion),
            '𝃏' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTetartimorion),
            '𝃐' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisApliDyoDodekata),
            '𝃑' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisMonogrammosTesseraDodekata),
            '𝃒' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisDigrammosExDodekata),
            '𝃓' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTrigrammosOktoDodekata),
            '𝃔' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisApliDyoDodekata),
            '𝃕' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisMonogrammosTesseraDodekata),
            '𝃖' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisDigrammosExDodekata),
            '𝃗' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisTrigrammosOktoDodekata),
            '𝃘' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGenikiDiesis),
            '𝃙' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGenikiYfesis),
            '𝃚' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliApliMikri),
            '𝃛' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliApliMegali),
            '𝃜' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliDipli),
            '𝃝' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliTheseos),
            '𝃞' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseos),
            '𝃟' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosDisimou),
            '𝃠' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosTrisimou),
            '𝃡' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosTetrasimou),
            '𝃢' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseos),
            '𝃣' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosDisimou),
            '𝃤' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosTrisimou),
            '𝃥' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosTetrasimou),
            '𝃦' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDigrammaGg),
            '𝃧' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolDiftoggosOu),
            '𝃨' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolStigma),
            '𝃩' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoPa),
            '𝃪' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoVou),
            '𝃫' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoGa),
            '𝃬' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoDi),
            '𝃭' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoKe),
            '𝃮' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoZo),
            '𝃯' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoNi),
            '𝃰' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoMeso),
            '𝃱' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoMeso),
            '𝃲' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoKato),
            '𝃳' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoKato),
            '𝃴' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasmaKato),
            '𝃵' => Ok(ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonNeoKato),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ByzantineMusicalSymbols {
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

impl std::convert::TryFrom<u32> for ByzantineMusicalSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ByzantineMusicalSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ByzantineMusicalSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ByzantineMusicalSymbols::ByzantineMusicalSymbolPsili
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ByzantineMusicalSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
