
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsili => "byzantine musical symbol psili",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDaseia => "byzantine musical symbol daseia",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPerispomeni => "byzantine musical symbol perispomeni",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaEkfonitikon => "byzantine musical symbol oxeia ekfonitikon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaDipli => "byzantine musical symbol oxeia dipli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaEkfonitikon => "byzantine musical symbol vareia ekfonitikon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaDipli => "byzantine musical symbol vareia dipli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKathisti => "byzantine musical symbol kathisti",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSyrmatiki => "byzantine musical symbol syrmatiki",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitiki => "byzantine musical symbol paraklitiki",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYpokrisis => "byzantine musical symbol ypokrisis",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYpokrisisDipli => "byzantine musical symbol ypokrisis dipli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKremasti => "byzantine musical symbol kremasti",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApesoEkfonitikon => "byzantine musical symbol apeso ekfonitikon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolExoEkfonitikon => "byzantine musical symbol exo ekfonitikon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTeleia => "byzantine musical symbol teleia",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimata => "byzantine musical symbol kentimata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofos => "byzantine musical symbol apostrofos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofosDipli => "byzantine musical symbol apostrofos dipli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSynevma => "byzantine musical symbol synevma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThita => "byzantine musical symbol thita",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOligonArchaion => "byzantine musical symbol oligon archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonArchaion => "byzantine musical symbol gorgon archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsilon => "byzantine musical symbol psilon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChamilon => "byzantine musical symbol chamilon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVathy => "byzantine musical symbol vathy",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolIsonArchaion => "byzantine musical symbol ison archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaArchaion => "byzantine musical symbol kentima archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataArchaion => "byzantine musical symbol kentimata archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSaximata => "byzantine musical symbol saximata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParichon => "byzantine musical symbol parichon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolStavrosApodexia => "byzantine musical symbol stavros apodexia",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaiArchaion => "byzantine musical symbol oxeiai archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaiArchaion => "byzantine musical symbol vareiai archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApodermaArchaion => "byzantine musical symbol apoderma archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApothema => "byzantine musical symbol apothema",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasma => "byzantine musical symbol klasma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolRevma => "byzantine musical symbol revma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPiasmaArchaion => "byzantine musical symbol piasma archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTinagma => "byzantine musical symbol tinagma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAnatrichisma => "byzantine musical symbol anatrichisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSeisma => "byzantine musical symbol seisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaArchaion => "byzantine musical symbol synagma archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaMetaStavrou => "byzantine musical symbol synagma meta stavrou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOyranismaArchaion => "byzantine musical symbol oyranisma archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThema => "byzantine musical symbol thema",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLemoi => "byzantine musical symbol lemoi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDyo => "byzantine musical symbol dyo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTria => "byzantine musical symbol tria",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTessera => "byzantine musical symbol tessera",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimata => "byzantine musical symbol kratimata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApesoExoNeo => "byzantine musical symbol apeso exo neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraArchaion => "byzantine musical symbol fthora archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolImifthora => "byzantine musical symbol imifthora",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonArchaion => "byzantine musical symbol tromikon archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKatavaTromikon => "byzantine musical symbol katava tromikon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPelaston => "byzantine musical symbol pelaston",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifiston => "byzantine musical symbol psifiston",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKontevma => "byzantine musical symbol kontevma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChorevmaArchaion => "byzantine musical symbol chorevma archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolRapisma => "byzantine musical symbol rapisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParakalesmaArchaion => "byzantine musical symbol parakalesma archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitikiArchaion => "byzantine musical symbol paraklitiki archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolIchadin => "byzantine musical symbol ichadin",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolNana => "byzantine musical symbol nana",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPetasma => "byzantine musical symbol petasma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKontevmaAllo => "byzantine musical symbol kontevma allo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonAllo => "byzantine musical symbol tromikon allo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolStraggismata => "byzantine musical symbol straggismata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGronthismata => "byzantine musical symbol gronthismata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolIsonNeo => "byzantine musical symbol ison neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOligonNeo => "byzantine musical symbol oligon neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOxeiaNeo => "byzantine musical symbol oxeia neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPetasti => "byzantine musical symbol petasti",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKoufisma => "byzantine musical symbol koufisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPetastokoufisma => "byzantine musical symbol petastokoufisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimokoufisma => "byzantine musical symbol kratimokoufisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPelastonNeo => "byzantine musical symbol pelaston neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoAno => "byzantine musical symbol kentimata neo ano",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoAno => "byzantine musical symbol kentima neo ano",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYpsili => "byzantine musical symbol ypsili",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofosNeo => "byzantine musical symbol apostrofos neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofoiSyndesmosNeo => "byzantine musical symbol apostrofoi syndesmos neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYporroi => "byzantine musical symbol yporroi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimoyporroon => "byzantine musical symbol kratimoyporroon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolElafron => "byzantine musical symbol elafron",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChamili => "byzantine musical symbol chamili",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMikronIson => "byzantine musical symbol mikron ison",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolVareiaNeo => "byzantine musical symbol vareia neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPiasmaNeo => "byzantine musical symbol piasma neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistonNeo => "byzantine musical symbol psifiston neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOmalon => "byzantine musical symbol omalon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAntikenoma => "byzantine musical symbol antikenoma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLygisma => "byzantine musical symbol lygisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParaklitikiNeo => "byzantine musical symbol paraklitiki neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolParakalesmaNeo => "byzantine musical symbol parakalesma neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEteronParakalesma => "byzantine musical symbol eteron parakalesma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKylisma => "byzantine musical symbol kylisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAntikenokylisma => "byzantine musical symbol antikenokylisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikonNeo => "byzantine musical symbol tromikon neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEkstrepton => "byzantine musical symbol ekstrepton",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSynagmaNeo => "byzantine musical symbol synagma neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSyrma => "byzantine musical symbol syrma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChorevmaNeo => "byzantine musical symbol chorevma neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEpegerma => "byzantine musical symbol epegerma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSeismaNeo => "byzantine musical symbol seisma neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolXironKlasma => "byzantine musical symbol xiron klasma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikopsifiston => "byzantine musical symbol tromikopsifiston",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistolygisma => "byzantine musical symbol psifistolygisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikolygisma => "byzantine musical symbol tromikolygisma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikoparakalesma => "byzantine musical symbol tromikoparakalesma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistoparakalesma => "byzantine musical symbol psifistoparakalesma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTromikosynagma => "byzantine musical symbol tromikosynagma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolPsifistosynagma => "byzantine musical symbol psifistosynagma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgosyntheton => "byzantine musical symbol gorgosyntheton",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArgosyntheton => "byzantine musical symbol argosyntheton",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEteronArgosyntheton => "byzantine musical symbol eteron argosyntheton",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolOyranismaNeo => "byzantine musical symbol oyranisma neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThematismosEso => "byzantine musical symbol thematismos eso",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThematismosExo => "byzantine musical symbol thematismos exo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThemaAploun => "byzantine musical symbol thema aploun",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolThesKaiApothes => "byzantine musical symbol thes kai apothes",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKatavasma => "byzantine musical symbol katavasma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEndofonon => "byzantine musical symbol endofonon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfenKato => "byzantine musical symbol yfen kato",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfenAno => "byzantine musical symbol yfen ano",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolStavros => "byzantine musical symbol stavros",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasmaAno => "byzantine musical symbol klasma ano",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDipliArchaion => "byzantine musical symbol dipli archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaArchaion => "byzantine musical symbol kratima archaion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaAllo => "byzantine musical symbol kratima allo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKratimaNeo => "byzantine musical symbol kratima neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApodermaNeo => "byzantine musical symbol apoderma neo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApli => "byzantine musical symbol apli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDipli => "byzantine musical symbol dipli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTripli => "byzantine musical symbol tripli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTetrapli => "byzantine musical symbol tetrapli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKoronis => "byzantine musical symbol koronis",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaEnosChronou => "byzantine musical symbol leimma enos chronou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaDyoChronon => "byzantine musical symbol leimma dyo chronon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaTrionChronon => "byzantine musical symbol leimma trion chronon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaTessaronChronon => "byzantine musical symbol leimma tessaron chronon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolLeimmaImiseosChronou => "byzantine musical symbol leimma imiseos chronou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonNeoAno => "byzantine musical symbol gorgon neo ano",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonParestigmenonAristera => "byzantine musical symbol gorgon parestigmenon aristera",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonParestigmenonDexia => "byzantine musical symbol gorgon parestigmenon dexia",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgon => "byzantine musical symbol digorgon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonAristeraKato => "byzantine musical symbol digorgon parestigmenon aristera kato",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonAristeraAno => "byzantine musical symbol digorgon parestigmenon aristera ano",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigorgonParestigmenonDexia => "byzantine musical symbol digorgon parestigmenon dexia",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolTrigorgon => "byzantine musical symbol trigorgon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArgon => "byzantine musical symbol argon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolImidiargon => "byzantine musical symbol imidiargon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiargon => "byzantine musical symbol diargon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiPoliArgi => "byzantine musical symbol agogi poli argi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiArgoteri => "byzantine musical symbol agogi argoteri",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiArgi => "byzantine musical symbol agogi argi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiMetria => "byzantine musical symbol agogi metria",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiMesi => "byzantine musical symbol agogi mesi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiGorgi => "byzantine musical symbol agogi gorgi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiGorgoteri => "byzantine musical symbol agogi gorgoteri",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolAgogiPoliGorgi => "byzantine musical symbol agogi poli gorgi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaProtosIchos => "byzantine musical symbol martyria protos ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaAlliProtosIchos => "byzantine musical symbol martyria alli protos ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaDeyterosIchos => "byzantine musical symbol martyria deyteros ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaAlliDeyterosIchos => "byzantine musical symbol martyria alli deyteros ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTritosIchos => "byzantine musical symbol martyria tritos ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTrifonias => "byzantine musical symbol martyria trifonias",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTetartosIchos => "byzantine musical symbol martyria tetartos ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaTetartosLegetosIchos => "byzantine musical symbol martyria tetartos legetos ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaLegetosIchos => "byzantine musical symbol martyria legetos ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaPlagiosIchos => "byzantine musical symbol martyria plagios ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolIsakiaTelousIchimatos => "byzantine musical symbol isakia telous ichimatos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolApostrofoiTelousIchimatos => "byzantine musical symbol apostrofoi telous ichimatos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisTetrafonias => "byzantine musical symbol fanerosis tetrafonias",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisMonofonias => "byzantine musical symbol fanerosis monofonias",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFanerosisDifonias => "byzantine musical symbol fanerosis difonias",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaVarysIchos => "byzantine musical symbol martyria varys ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaProtovarysIchos => "byzantine musical symbol martyria protovarys ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolMartyriaPlagiosTetartosIchos => "byzantine musical symbol martyria plagios tetartos ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorthmikonNAploun => "byzantine musical symbol gorthmikon n aploun",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorthmikonNDiploun => "byzantine musical symbol gorthmikon n diploun",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolEnarxisKaiFthoraVou => "byzantine musical symbol enarxis kai fthora vou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolImifonon => "byzantine musical symbol imifonon",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolImifthoron => "byzantine musical symbol imifthoron",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraArchaionDeyterouIchou => "byzantine musical symbol fthora archaion deyterou ichou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiPa => "byzantine musical symbol fthora diatoniki pa",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNana => "byzantine musical symbol fthora diatoniki nana",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraNaosIchos => "byzantine musical symbol fthora naos ichos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiDi => "byzantine musical symbol fthora diatoniki di",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraSklironDiatononDi => "byzantine musical symbol fthora skliron diatonon di",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiKe => "byzantine musical symbol fthora diatoniki ke",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiZo => "byzantine musical symbol fthora diatoniki zo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNiKato => "byzantine musical symbol fthora diatoniki ni kato",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraDiatonikiNiAno => "byzantine musical symbol fthora diatoniki ni ano",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraMalakonChromaDifonias => "byzantine musical symbol fthora malakon chroma difonias",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraMalakonChromaMonofonias => "byzantine musical symbol fthora malakon chroma monofonias",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFhtoraSklironChromaVasis => "byzantine musical symbol fhtora skliron chroma vasis",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraSklironChromaSynafi => "byzantine musical symbol fthora skliron chroma synafi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraNenano => "byzantine musical symbol fthora nenano",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaZygos => "byzantine musical symbol chroa zygos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaKliton => "byzantine musical symbol chroa kliton",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolChroaSpathi => "byzantine musical symbol chroa spathi",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraIYfesisTetartimorion => "byzantine musical symbol fthora i yfesis tetartimorion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolFthoraEnarmoniosAntifonia => "byzantine musical symbol fthora enarmonios antifonia",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisTritimorion => "byzantine musical symbol yfesis tritimorion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTritimorion => "byzantine musical symbol diesis tritimorion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTetartimorion => "byzantine musical symbol diesis tetartimorion",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisApliDyoDodekata => "byzantine musical symbol diesis apli dyo dodekata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisMonogrammosTesseraDodekata => "byzantine musical symbol diesis monogrammos tessera dodekata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisDigrammosExDodekata => "byzantine musical symbol diesis digrammos ex dodekata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiesisTrigrammosOktoDodekata => "byzantine musical symbol diesis trigrammos okto dodekata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisApliDyoDodekata => "byzantine musical symbol yfesis apli dyo dodekata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisMonogrammosTesseraDodekata => "byzantine musical symbol yfesis monogrammos tessera dodekata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisDigrammosExDodekata => "byzantine musical symbol yfesis digrammos ex dodekata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolYfesisTrigrammosOktoDodekata => "byzantine musical symbol yfesis trigrammos okto dodekata",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGenikiDiesis => "byzantine musical symbol geniki diesis",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGenikiYfesis => "byzantine musical symbol geniki yfesis",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliApliMikri => "byzantine musical symbol diastoli apli mikri",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliApliMegali => "byzantine musical symbol diastoli apli megali",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliDipli => "byzantine musical symbol diastoli dipli",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiastoliTheseos => "byzantine musical symbol diastoli theseos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseos => "byzantine musical symbol simansis theseos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosDisimou => "byzantine musical symbol simansis theseos disimou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosTrisimou => "byzantine musical symbol simansis theseos trisimou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisTheseosTetrasimou => "byzantine musical symbol simansis theseos tetrasimou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseos => "byzantine musical symbol simansis arseos",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosDisimou => "byzantine musical symbol simansis arseos disimou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosTrisimou => "byzantine musical symbol simansis arseos trisimou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolSimansisArseosTetrasimou => "byzantine musical symbol simansis arseos tetrasimou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDigrammaGg => "byzantine musical symbol digramma gg",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolDiftoggosOu => "byzantine musical symbol diftoggos ou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolStigma => "byzantine musical symbol stigma",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoPa => "byzantine musical symbol arktiko pa",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoVou => "byzantine musical symbol arktiko vou",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoGa => "byzantine musical symbol arktiko ga",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoDi => "byzantine musical symbol arktiko di",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoKe => "byzantine musical symbol arktiko ke",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoZo => "byzantine musical symbol arktiko zo",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolArktikoNi => "byzantine musical symbol arktiko ni",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoMeso => "byzantine musical symbol kentimata neo meso",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoMeso => "byzantine musical symbol kentima neo meso",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimataNeoKato => "byzantine musical symbol kentimata neo kato",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKentimaNeoKato => "byzantine musical symbol kentima neo kato",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolKlasmaKato => "byzantine musical symbol klasma kato",
            ByzantineMusicalSymbols::ByzantineMusicalSymbolGorgonNeoKato => "byzantine musical symbol gorgon neo kato",
        }
    }
}
