
/// An enum to represent all characters in the Cuneiform block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Cuneiform {
    /// \u{12000}: '𒀀'
    SignA,
    /// \u{12001}: '𒀁'
    SignATimesA,
    /// \u{12002}: '𒀂'
    SignATimesBad,
    /// \u{12003}: '𒀃'
    SignATimesGan2Tenu,
    /// \u{12004}: '𒀄'
    SignATimesHa,
    /// \u{12005}: '𒀅'
    SignATimesIgi,
    /// \u{12006}: '𒀆'
    SignATimesLagarGunu,
    /// \u{12007}: '𒀇'
    SignATimesMush,
    /// \u{12008}: '𒀈'
    SignATimesSag,
    /// \u{12009}: '𒀉'
    SignA2,
    /// \u{1200a}: '𒀊'
    SignAb,
    /// \u{1200b}: '𒀋'
    SignAbTimesAsh2,
    /// \u{1200c}: '𒀌'
    SignAbTimesDun3Gunu,
    /// \u{1200d}: '𒀍'
    SignAbTimesGal,
    /// \u{1200e}: '𒀎'
    SignAbTimesGan2Tenu,
    /// \u{1200f}: '𒀏'
    SignAbTimesHa,
    /// \u{12010}: '𒀐'
    SignAbTimesIgiGunu,
    /// \u{12011}: '𒀑'
    SignAbTimesImin,
    /// \u{12012}: '𒀒'
    SignAbTimesLagab,
    /// \u{12013}: '𒀓'
    SignAbTimesShesh,
    /// \u{12014}: '𒀔'
    SignAbTimesUPlusUPlusU,
    /// \u{12015}: '𒀕'
    SignAbGunu,
    /// \u{12016}: '𒀖'
    SignAb2,
    /// \u{12017}: '𒀗'
    SignAb2TimesBalag,
    /// \u{12018}: '𒀘'
    SignAb2TimesGan2Tenu,
    /// \u{12019}: '𒀙'
    SignAb2TimesMePlusEn,
    /// \u{1201a}: '𒀚'
    SignAb2TimesSha3,
    /// \u{1201b}: '𒀛'
    SignAb2TimesTak4,
    /// \u{1201c}: '𒀜'
    SignAd,
    /// \u{1201d}: '𒀝'
    SignAk,
    /// \u{1201e}: '𒀞'
    SignAkTimesErin2,
    /// \u{1201f}: '𒀟'
    SignAkTimesShitaPlusGish,
    /// \u{12020}: '𒀠'
    SignAl,
    /// \u{12021}: '𒀡'
    SignAlTimesAl,
    /// \u{12022}: '𒀢'
    SignAlTimesDim2,
    /// \u{12023}: '𒀣'
    SignAlTimesGish,
    /// \u{12024}: '𒀤'
    SignAlTimesHa,
    /// \u{12025}: '𒀥'
    SignAlTimesKad3,
    /// \u{12026}: '𒀦'
    SignAlTimesKi,
    /// \u{12027}: '𒀧'
    SignAlTimesShe,
    /// \u{12028}: '𒀨'
    SignAlTimesUsh,
    /// \u{12029}: '𒀩'
    SignAlan,
    /// \u{1202a}: '𒀪'
    SignAleph,
    /// \u{1202b}: '𒀫'
    SignAmar,
    /// \u{1202c}: '𒀬'
    SignAmarTimesShe,
    /// \u{1202d}: '𒀭'
    SignAn,
    /// \u{1202e}: '𒀮'
    SignAnOverAn,
    /// \u{1202f}: '𒀯'
    SignAnThreeTimes,
    /// \u{12030}: '𒀰'
    SignAnPlusNagaOpposingAnPlusNaga,
    /// \u{12031}: '𒀱'
    SignAnPlusNagaSquared,
    /// \u{12032}: '𒀲'
    SignAnshe,
    /// \u{12033}: '𒀳'
    SignApin,
    /// \u{12034}: '𒀴'
    SignArad,
    /// \u{12035}: '𒀵'
    SignAradTimesKur,
    /// \u{12036}: '𒀶'
    SignArkab,
    /// \u{12037}: '𒀷'
    SignAsal2,
    /// \u{12038}: '𒀸'
    SignAsh,
    /// \u{12039}: '𒀹'
    SignAshZidaTenu,
    /// \u{1203a}: '𒀺'
    SignAshKabaTenu,
    /// \u{1203b}: '𒀻'
    SignAshOverAshTug2OverTug2Tug2OverTug2Pap,
    /// \u{1203c}: '𒀼'
    SignAshOverAshOverAsh,
    /// \u{1203d}: '𒀽'
    SignAshOverAshOverAshCrossingAshOverAshOverAsh,
    /// \u{1203e}: '𒀾'
    SignAsh2,
    /// \u{1203f}: '𒀿'
    SignAshgab,
    /// \u{12040}: '𒁀'
    SignBa,
    /// \u{12041}: '𒁁'
    SignBad,
    /// \u{12042}: '𒁂'
    SignBag3,
    /// \u{12043}: '𒁃'
    SignBahar2,
    /// \u{12044}: '𒁄'
    SignBal,
    /// \u{12045}: '𒁅'
    SignBalOverBal,
    /// \u{12046}: '𒁆'
    SignBalag,
    /// \u{12047}: '𒁇'
    SignBar,
    /// \u{12048}: '𒁈'
    SignBara2,
    /// \u{12049}: '𒁉'
    SignBi,
    /// \u{1204a}: '𒁊'
    SignBiTimesA,
    /// \u{1204b}: '𒁋'
    SignBiTimesGar,
    /// \u{1204c}: '𒁌'
    SignBiTimesIgiGunu,
    /// \u{1204d}: '𒁍'
    SignBu,
    /// \u{1204e}: '𒁎'
    SignBuOverBuAb,
    /// \u{1204f}: '𒁏'
    SignBuOverBuUn,
    /// \u{12050}: '𒁐'
    SignBuCrossingBu,
    /// \u{12051}: '𒁑'
    SignBulug,
    /// \u{12052}: '𒁒'
    SignBulugOverBulug,
    /// \u{12053}: '𒁓'
    SignBur,
    /// \u{12054}: '𒁔'
    SignBur2,
    /// \u{12055}: '𒁕'
    SignDa,
    /// \u{12056}: '𒁖'
    SignDag,
    /// \u{12057}: '𒁗'
    SignDagKisim5TimesAPlusMash,
    /// \u{12058}: '𒁘'
    SignDagKisim5TimesAmar,
    /// \u{12059}: '𒁙'
    SignDagKisim5TimesBalag,
    /// \u{1205a}: '𒁚'
    SignDagKisim5TimesBi,
    /// \u{1205b}: '𒁛'
    SignDagKisim5TimesGa,
    /// \u{1205c}: '𒁜'
    SignDagKisim5TimesGaPlusMash,
    /// \u{1205d}: '𒁝'
    SignDagKisim5TimesGi,
    /// \u{1205e}: '𒁞'
    SignDagKisim5TimesGir2,
    /// \u{1205f}: '𒁟'
    SignDagKisim5TimesGud,
    /// \u{12060}: '𒁠'
    SignDagKisim5TimesHa,
    /// \u{12061}: '𒁡'
    SignDagKisim5TimesIr,
    /// \u{12062}: '𒁢'
    SignDagKisim5TimesIrPlusLu,
    /// \u{12063}: '𒁣'
    SignDagKisim5TimesKak,
    /// \u{12064}: '𒁤'
    SignDagKisim5TimesLa,
    /// \u{12065}: '𒁥'
    SignDagKisim5TimesLu,
    /// \u{12066}: '𒁦'
    SignDagKisim5TimesLuPlusMash2,
    /// \u{12067}: '𒁧'
    SignDagKisim5TimesLum,
    /// \u{12068}: '𒁨'
    SignDagKisim5TimesNe,
    /// \u{12069}: '𒁩'
    SignDagKisim5TimesPapPlusPap,
    /// \u{1206a}: '𒁪'
    SignDagKisim5TimesSi,
    /// \u{1206b}: '𒁫'
    SignDagKisim5TimesTak4,
    /// \u{1206c}: '𒁬'
    SignDagKisim5TimesU2PlusGir2,
    /// \u{1206d}: '𒁭'
    SignDagKisim5TimesUsh,
    /// \u{1206e}: '𒁮'
    SignDam,
    /// \u{1206f}: '𒁯'
    SignDar,
    /// \u{12070}: '𒁰'
    SignDara3,
    /// \u{12071}: '𒁱'
    SignDara4,
    /// \u{12072}: '𒁲'
    SignDi,
    /// \u{12073}: '𒁳'
    SignDib,
    /// \u{12074}: '𒁴'
    SignDim,
    /// \u{12075}: '𒁵'
    SignDimTimesShe,
    /// \u{12076}: '𒁶'
    SignDim2,
    /// \u{12077}: '𒁷'
    SignDin,
    /// \u{12078}: '𒁸'
    SignDinKaskalUGunuDish,
    /// \u{12079}: '𒁹'
    SignDish,
    /// \u{1207a}: '𒁺'
    SignDu,
    /// \u{1207b}: '𒁻'
    SignDuOverDu,
    /// \u{1207c}: '𒁼'
    SignDuGunu,
    /// \u{1207d}: '𒁽'
    SignDuSheshig,
    /// \u{1207e}: '𒁾'
    SignDub,
    /// \u{1207f}: '𒁿'
    SignDubTimesEsh2,
    /// \u{12080}: '𒂀'
    SignDub2,
    /// \u{12081}: '𒂁'
    SignDug,
    /// \u{12082}: '𒂂'
    SignDugud,
    /// \u{12083}: '𒂃'
    SignDuh,
    /// \u{12084}: '𒂄'
    SignDun,
    /// \u{12085}: '𒂅'
    SignDun3,
    /// \u{12086}: '𒂆'
    SignDun3Gunu,
    /// \u{12087}: '𒂇'
    SignDun3GunuGunu,
    /// \u{12088}: '𒂈'
    SignDun4,
    /// \u{12089}: '𒂉'
    SignDur2,
    /// \u{1208a}: '𒂊'
    SignE,
    /// \u{1208b}: '𒂋'
    SignETimesPap,
    /// \u{1208c}: '𒂌'
    SignEOverENunOverNun,
    /// \u{1208d}: '𒂍'
    SignE2,
    /// \u{1208e}: '𒂎'
    SignE2TimesAPlusHaPlusDa,
    /// \u{1208f}: '𒂏'
    SignE2TimesGar,
    /// \u{12090}: '𒂐'
    SignE2TimesMi,
    /// \u{12091}: '𒂑'
    SignE2TimesSal,
    /// \u{12092}: '𒂒'
    SignE2TimesShe,
    /// \u{12093}: '𒂓'
    SignE2TimesU,
    /// \u{12094}: '𒂔'
    SignEdin,
    /// \u{12095}: '𒂕'
    SignEgir,
    /// \u{12096}: '𒂖'
    SignEl,
    /// \u{12097}: '𒂗'
    SignEn,
    /// \u{12098}: '𒂘'
    SignEnTimesGan2,
    /// \u{12099}: '𒂙'
    SignEnTimesGan2Tenu,
    /// \u{1209a}: '𒂚'
    SignEnTimesMe,
    /// \u{1209b}: '𒂛'
    SignEnCrossingEn,
    /// \u{1209c}: '𒂜'
    SignEnOpposingEn,
    /// \u{1209d}: '𒂝'
    SignEnSquared,
    /// \u{1209e}: '𒂞'
    SignEren,
    /// \u{1209f}: '𒂟'
    SignErin2,
    /// \u{120a0}: '𒂠'
    SignEsh2,
    /// \u{120a1}: '𒂡'
    SignEzen,
    /// \u{120a2}: '𒂢'
    SignEzenTimesA,
    /// \u{120a3}: '𒂣'
    SignEzenTimesAPlusLal,
    /// \u{120a4}: '𒂤'
    SignEzenTimesAPlusLalTimesLal,
    /// \u{120a5}: '𒂥'
    SignEzenTimesAn,
    /// \u{120a6}: '𒂦'
    SignEzenTimesBad,
    /// \u{120a7}: '𒂧'
    SignEzenTimesDun3Gunu,
    /// \u{120a8}: '𒂨'
    SignEzenTimesDun3GunuGunu,
    /// \u{120a9}: '𒂩'
    SignEzenTimesHa,
    /// \u{120aa}: '𒂪'
    SignEzenTimesHaGunu,
    /// \u{120ab}: '𒂫'
    SignEzenTimesIgiGunu,
    /// \u{120ac}: '𒂬'
    SignEzenTimesKaskal,
    /// \u{120ad}: '𒂭'
    SignEzenTimesKaskalSquared,
    /// \u{120ae}: '𒂮'
    SignEzenTimesKu3,
    /// \u{120af}: '𒂯'
    SignEzenTimesLa,
    /// \u{120b0}: '𒂰'
    SignEzenTimesLalTimesLal,
    /// \u{120b1}: '𒂱'
    SignEzenTimesLi,
    /// \u{120b2}: '𒂲'
    SignEzenTimesLu,
    /// \u{120b3}: '𒂳'
    SignEzenTimesU2,
    /// \u{120b4}: '𒂴'
    SignEzenTimesUd,
    /// \u{120b5}: '𒂵'
    SignGa,
    /// \u{120b6}: '𒂶'
    SignGaGunu,
    /// \u{120b7}: '𒂷'
    SignGa2,
    /// \u{120b8}: '𒂸'
    SignGa2TimesAPlusDaPlusHa,
    /// \u{120b9}: '𒂹'
    SignGa2TimesAPlusHa,
    /// \u{120ba}: '𒂺'
    SignGa2TimesAPlusIgi,
    /// \u{120bb}: '𒂻'
    SignGa2TimesAb2TenuPlusTab,
    /// \u{120bc}: '𒂼'
    SignGa2TimesAn,
    /// \u{120bd}: '𒂽'
    SignGa2TimesAsh,
    /// \u{120be}: '𒂾'
    SignGa2TimesAsh2PlusGal,
    /// \u{120bf}: '𒂿'
    SignGa2TimesBad,
    /// \u{120c0}: '𒃀'
    SignGa2TimesBarPlusRa,
    /// \u{120c1}: '𒃁'
    SignGa2TimesBur,
    /// \u{120c2}: '𒃂'
    SignGa2TimesBurPlusRa,
    /// \u{120c3}: '𒃃'
    SignGa2TimesDa,
    /// \u{120c4}: '𒃄'
    SignGa2TimesDi,
    /// \u{120c5}: '𒃅'
    SignGa2TimesDimTimesShe,
    /// \u{120c6}: '𒃆'
    SignGa2TimesDub,
    /// \u{120c7}: '𒃇'
    SignGa2TimesEl,
    /// \u{120c8}: '𒃈'
    SignGa2TimesElPlusLa,
    /// \u{120c9}: '𒃉'
    SignGa2TimesEn,
    /// \u{120ca}: '𒃊'
    SignGa2TimesEnTimesGan2Tenu,
    /// \u{120cb}: '𒃋'
    SignGa2TimesGan2Tenu,
    /// \u{120cc}: '𒃌'
    SignGa2TimesGar,
    /// \u{120cd}: '𒃍'
    SignGa2TimesGi,
    /// \u{120ce}: '𒃎'
    SignGa2TimesGi4,
    /// \u{120cf}: '𒃏'
    SignGa2TimesGi4PlusA,
    /// \u{120d0}: '𒃐'
    SignGa2TimesGir2PlusSu,
    /// \u{120d1}: '𒃑'
    SignGa2TimesHaPlusLuPlusEsh2,
    /// \u{120d2}: '𒃒'
    SignGa2TimesHal,
    /// \u{120d3}: '𒃓'
    SignGa2TimesHalPlusLa,
    /// \u{120d4}: '𒃔'
    SignGa2TimesHiPlusLi,
    /// \u{120d5}: '𒃕'
    SignGa2TimesHub2,
    /// \u{120d6}: '𒃖'
    SignGa2TimesIgiGunu,
    /// \u{120d7}: '𒃗'
    SignGa2TimesIshPlusHuPlusAsh,
    /// \u{120d8}: '𒃘'
    SignGa2TimesKak,
    /// \u{120d9}: '𒃙'
    SignGa2TimesKaskal,
    /// \u{120da}: '𒃚'
    SignGa2TimesKid,
    /// \u{120db}: '𒃛'
    SignGa2TimesKidPlusLal,
    /// \u{120dc}: '𒃜'
    SignGa2TimesKu3PlusAn,
    /// \u{120dd}: '𒃝'
    SignGa2TimesLa,
    /// \u{120de}: '𒃞'
    SignGa2TimesMePlusEn,
    /// \u{120df}: '𒃟'
    SignGa2TimesMi,
    /// \u{120e0}: '𒃠'
    SignGa2TimesNun,
    /// \u{120e1}: '𒃡'
    SignGa2TimesNunOverNun,
    /// \u{120e2}: '𒃢'
    SignGa2TimesPa,
    /// \u{120e3}: '𒃣'
    SignGa2TimesSal,
    /// \u{120e4}: '𒃤'
    SignGa2TimesSar,
    /// \u{120e5}: '𒃥'
    SignGa2TimesShe,
    /// \u{120e6}: '𒃦'
    SignGa2TimesShePlusTur,
    /// \u{120e7}: '𒃧'
    SignGa2TimesShid,
    /// \u{120e8}: '𒃨'
    SignGa2TimesSum,
    /// \u{120e9}: '𒃩'
    SignGa2TimesTak4,
    /// \u{120ea}: '𒃪'
    SignGa2TimesU,
    /// \u{120eb}: '𒃫'
    SignGa2TimesUd,
    /// \u{120ec}: '𒃬'
    SignGa2TimesUdPlusDu,
    /// \u{120ed}: '𒃭'
    SignGa2OverGa2,
    /// \u{120ee}: '𒃮'
    SignGaba,
    /// \u{120ef}: '𒃯'
    SignGabaCrossingGaba,
    /// \u{120f0}: '𒃰'
    SignGad,
    /// \u{120f1}: '𒃱'
    SignGadOverGadGarOverGar,
    /// \u{120f2}: '𒃲'
    SignGal,
    /// \u{120f3}: '𒃳'
    SignGalGadOverGadGarOverGar,
    /// \u{120f4}: '𒃴'
    SignGalam,
    /// \u{120f5}: '𒃵'
    SignGam,
    /// \u{120f6}: '𒃶'
    SignGan,
    /// \u{120f7}: '𒃷'
    SignGan2,
    /// \u{120f8}: '𒃸'
    SignGan2Tenu,
    /// \u{120f9}: '𒃹'
    SignGan2OverGan2,
    /// \u{120fa}: '𒃺'
    SignGan2CrossingGan2,
    /// \u{120fb}: '𒃻'
    SignGar,
    /// \u{120fc}: '𒃼'
    SignGar3,
    /// \u{120fd}: '𒃽'
    SignGashan,
    /// \u{120fe}: '𒃾'
    SignGeshtin,
    /// \u{120ff}: '𒃿'
    SignGeshtinTimesKur,
    /// \u{12100}: '𒄀'
    SignGi,
    /// \u{12101}: '𒄁'
    SignGiTimesE,
    /// \u{12102}: '𒄂'
    SignGiTimesU,
    /// \u{12103}: '𒄃'
    SignGiCrossingGi,
    /// \u{12104}: '𒄄'
    SignGi4,
    /// \u{12105}: '𒄅'
    SignGi4OverGi4,
    /// \u{12106}: '𒄆'
    SignGi4CrossingGi4,
    /// \u{12107}: '𒄇'
    SignGidim,
    /// \u{12108}: '𒄈'
    SignGir2,
    /// \u{12109}: '𒄉'
    SignGir2Gunu,
    /// \u{1210a}: '𒄊'
    SignGir3,
    /// \u{1210b}: '𒄋'
    SignGir3TimesAPlusIgi,
    /// \u{1210c}: '𒄌'
    SignGir3TimesGan2Tenu,
    /// \u{1210d}: '𒄍'
    SignGir3TimesIgi,
    /// \u{1210e}: '𒄎'
    SignGir3TimesLuPlusIgi,
    /// \u{1210f}: '𒄏'
    SignGir3TimesPa,
    /// \u{12110}: '𒄐'
    SignGisal,
    /// \u{12111}: '𒄑'
    SignGish,
    /// \u{12112}: '𒄒'
    SignGishCrossingGish,
    /// \u{12113}: '𒄓'
    SignGishTimesBad,
    /// \u{12114}: '𒄔'
    SignGishTimesTak4,
    /// \u{12115}: '𒄕'
    SignGishTenu,
    /// \u{12116}: '𒄖'
    SignGu,
    /// \u{12117}: '𒄗'
    SignGuCrossingGu,
    /// \u{12118}: '𒄘'
    SignGu2,
    /// \u{12119}: '𒄙'
    SignGu2TimesKak,
    /// \u{1211a}: '𒄚'
    SignGu2TimesKakTimesIgiGunu,
    /// \u{1211b}: '𒄛'
    SignGu2TimesNun,
    /// \u{1211c}: '𒄜'
    SignGu2TimesSalPlusTug2,
    /// \u{1211d}: '𒄝'
    SignGu2Gunu,
    /// \u{1211e}: '𒄞'
    SignGud,
    /// \u{1211f}: '𒄟'
    SignGudTimesAPlusKur,
    /// \u{12120}: '𒄠'
    SignGudTimesKur,
    /// \u{12121}: '𒄡'
    SignGudOverGudLugal,
    /// \u{12122}: '𒄢'
    SignGul,
    /// \u{12123}: '𒄣'
    SignGum,
    /// \u{12124}: '𒄤'
    SignGumTimesShe,
    /// \u{12125}: '𒄥'
    SignGur,
    /// \u{12126}: '𒄦'
    SignGur7,
    /// \u{12127}: '𒄧'
    SignGurun,
    /// \u{12128}: '𒄨'
    SignGurush,
    /// \u{12129}: '𒄩'
    SignHa,
    /// \u{1212a}: '𒄪'
    SignHaTenu,
    /// \u{1212b}: '𒄫'
    SignHaGunu,
    /// \u{1212c}: '𒄬'
    SignHal,
    /// \u{1212d}: '𒄭'
    SignHi,
    /// \u{1212e}: '𒄮'
    SignHiTimesAsh,
    /// \u{1212f}: '𒄯'
    SignHiTimesAsh2,
    /// \u{12130}: '𒄰'
    SignHiTimesBad,
    /// \u{12131}: '𒄱'
    SignHiTimesDish,
    /// \u{12132}: '𒄲'
    SignHiTimesGad,
    /// \u{12133}: '𒄳'
    SignHiTimesKin,
    /// \u{12134}: '𒄴'
    SignHiTimesNun,
    /// \u{12135}: '𒄵'
    SignHiTimesShe,
    /// \u{12136}: '𒄶'
    SignHiTimesU,
    /// \u{12137}: '𒄷'
    SignHu,
    /// \u{12138}: '𒄸'
    SignHub2,
    /// \u{12139}: '𒄹'
    SignHub2TimesAn,
    /// \u{1213a}: '𒄺'
    SignHub2TimesHal,
    /// \u{1213b}: '𒄻'
    SignHub2TimesKaskal,
    /// \u{1213c}: '𒄼'
    SignHub2TimesLish,
    /// \u{1213d}: '𒄽'
    SignHub2TimesUd,
    /// \u{1213e}: '𒄾'
    SignHul2,
    /// \u{1213f}: '𒄿'
    SignI,
    /// \u{12140}: '𒅀'
    SignIA,
    /// \u{12141}: '𒅁'
    SignIb,
    /// \u{12142}: '𒅂'
    SignIdim,
    /// \u{12143}: '𒅃'
    SignIdimOverIdimBur,
    /// \u{12144}: '𒅄'
    SignIdimOverIdimSquared,
    /// \u{12145}: '𒅅'
    SignIg,
    /// \u{12146}: '𒅆'
    SignIgi,
    /// \u{12147}: '𒅇'
    SignIgiDib,
    /// \u{12148}: '𒅈'
    SignIgiRi,
    /// \u{12149}: '𒅉'
    SignIgiOverIgiShirOverShirUdOverUd,
    /// \u{1214a}: '𒅊'
    SignIgiGunu,
    /// \u{1214b}: '𒅋'
    SignIl,
    /// \u{1214c}: '𒅌'
    SignIlTimesGan2Tenu,
    /// \u{1214d}: '𒅍'
    SignIl2,
    /// \u{1214e}: '𒅎'
    SignIm,
    /// \u{1214f}: '𒅏'
    SignImTimesTak4,
    /// \u{12150}: '𒅐'
    SignImCrossingIm,
    /// \u{12151}: '𒅑'
    SignImOpposingIm,
    /// \u{12152}: '𒅒'
    SignImSquared,
    /// \u{12153}: '𒅓'
    SignImin,
    /// \u{12154}: '𒅔'
    SignIn,
    /// \u{12155}: '𒅕'
    SignIr,
    /// \u{12156}: '𒅖'
    SignIsh,
    /// \u{12157}: '𒅗'
    SignKa,
    /// \u{12158}: '𒅘'
    SignKaTimesA,
    /// \u{12159}: '𒅙'
    SignKaTimesAd,
    /// \u{1215a}: '𒅚'
    SignKaTimesAdPlusKu3,
    /// \u{1215b}: '𒅛'
    SignKaTimesAsh2,
    /// \u{1215c}: '𒅜'
    SignKaTimesBad,
    /// \u{1215d}: '𒅝'
    SignKaTimesBalag,
    /// \u{1215e}: '𒅞'
    SignKaTimesBar,
    /// \u{1215f}: '𒅟'
    SignKaTimesBi,
    /// \u{12160}: '𒅠'
    SignKaTimesErin2,
    /// \u{12161}: '𒅡'
    SignKaTimesEsh2,
    /// \u{12162}: '𒅢'
    SignKaTimesGa,
    /// \u{12163}: '𒅣'
    SignKaTimesGal,
    /// \u{12164}: '𒅤'
    SignKaTimesGan2Tenu,
    /// \u{12165}: '𒅥'
    SignKaTimesGar,
    /// \u{12166}: '𒅦'
    SignKaTimesGarPlusSha3PlusA,
    /// \u{12167}: '𒅧'
    SignKaTimesGi,
    /// \u{12168}: '𒅨'
    SignKaTimesGir2,
    /// \u{12169}: '𒅩'
    SignKaTimesGishPlusSar,
    /// \u{1216a}: '𒅪'
    SignKaTimesGishCrossingGish,
    /// \u{1216b}: '𒅫'
    SignKaTimesGu,
    /// \u{1216c}: '𒅬'
    SignKaTimesGur7,
    /// \u{1216d}: '𒅭'
    SignKaTimesIgi,
    /// \u{1216e}: '𒅮'
    SignKaTimesIm,
    /// \u{1216f}: '𒅯'
    SignKaTimesKak,
    /// \u{12170}: '𒅰'
    SignKaTimesKi,
    /// \u{12171}: '𒅱'
    SignKaTimesKid,
    /// \u{12172}: '𒅲'
    SignKaTimesLi,
    /// \u{12173}: '𒅳'
    SignKaTimesLu,
    /// \u{12174}: '𒅴'
    SignKaTimesMe,
    /// \u{12175}: '𒅵'
    SignKaTimesMePlusDu,
    /// \u{12176}: '𒅶'
    SignKaTimesMePlusGi,
    /// \u{12177}: '𒅷'
    SignKaTimesMePlusTe,
    /// \u{12178}: '𒅸'
    SignKaTimesMi,
    /// \u{12179}: '𒅹'
    SignKaTimesMiPlusNunuz,
    /// \u{1217a}: '𒅺'
    SignKaTimesNe,
    /// \u{1217b}: '𒅻'
    SignKaTimesNun,
    /// \u{1217c}: '𒅼'
    SignKaTimesPi,
    /// \u{1217d}: '𒅽'
    SignKaTimesRu,
    /// \u{1217e}: '𒅾'
    SignKaTimesSa,
    /// \u{1217f}: '𒅿'
    SignKaTimesSar,
    /// \u{12180}: '𒆀'
    SignKaTimesSha,
    /// \u{12181}: '𒆁'
    SignKaTimesShe,
    /// \u{12182}: '𒆂'
    SignKaTimesShid,
    /// \u{12183}: '𒆃'
    SignKaTimesShu,
    /// \u{12184}: '𒆄'
    SignKaTimesSig,
    /// \u{12185}: '𒆅'
    SignKaTimesSuhur,
    /// \u{12186}: '𒆆'
    SignKaTimesTar,
    /// \u{12187}: '𒆇'
    SignKaTimesU,
    /// \u{12188}: '𒆈'
    SignKaTimesU2,
    /// \u{12189}: '𒆉'
    SignKaTimesUd,
    /// \u{1218a}: '𒆊'
    SignKaTimesUmumTimesPa,
    /// \u{1218b}: '𒆋'
    SignKaTimesUsh,
    /// \u{1218c}: '𒆌'
    SignKaTimesZi,
    /// \u{1218d}: '𒆍'
    SignKa2,
    /// \u{1218e}: '𒆎'
    SignKa2CrossingKa2,
    /// \u{1218f}: '𒆏'
    SignKab,
    /// \u{12190}: '𒆐'
    SignKad2,
    /// \u{12191}: '𒆑'
    SignKad3,
    /// \u{12192}: '𒆒'
    SignKad4,
    /// \u{12193}: '𒆓'
    SignKad5,
    /// \u{12194}: '𒆔'
    SignKad5OverKad5,
    /// \u{12195}: '𒆕'
    SignKak,
    /// \u{12196}: '𒆖'
    SignKakTimesIgiGunu,
    /// \u{12197}: '𒆗'
    SignKal,
    /// \u{12198}: '𒆘'
    SignKalTimesBad,
    /// \u{12199}: '𒆙'
    SignKalCrossingKal,
    /// \u{1219a}: '𒆚'
    SignKam2,
    /// \u{1219b}: '𒆛'
    SignKam4,
    /// \u{1219c}: '𒆜'
    SignKaskal,
    /// \u{1219d}: '𒆝'
    SignKaskalLagabTimesUOverLagabTimesU,
    /// \u{1219e}: '𒆞'
    SignKaskalOverKaskalLagabTimesUOverLagabTimesU,
    /// \u{1219f}: '𒆟'
    SignKesh2,
    /// \u{121a0}: '𒆠'
    SignKi,
    /// \u{121a1}: '𒆡'
    SignKiTimesBad,
    /// \u{121a2}: '𒆢'
    SignKiTimesU,
    /// \u{121a3}: '𒆣'
    SignKiTimesUd,
    /// \u{121a4}: '𒆤'
    SignKid,
    /// \u{121a5}: '𒆥'
    SignKin,
    /// \u{121a6}: '𒆦'
    SignKisal,
    /// \u{121a7}: '𒆧'
    SignKish,
    /// \u{121a8}: '𒆨'
    SignKisim5,
    /// \u{121a9}: '𒆩'
    SignKisim5OverKisim5,
    /// \u{121aa}: '𒆪'
    SignKu,
    /// \u{121ab}: '𒆫'
    SignKuOverHiTimesAsh2KuOverHiTimesAsh2,
    /// \u{121ac}: '𒆬'
    SignKu3,
    /// \u{121ad}: '𒆭'
    SignKu4,
    /// \u{121ae}: '𒆮'
    SignKu4VariantForm,
    /// \u{121af}: '𒆯'
    SignKu7,
    /// \u{121b0}: '𒆰'
    SignKul,
    /// \u{121b1}: '𒆱'
    SignKulGunu,
    /// \u{121b2}: '𒆲'
    SignKun,
    /// \u{121b3}: '𒆳'
    SignKur,
    /// \u{121b4}: '𒆴'
    SignKurOpposingKur,
    /// \u{121b5}: '𒆵'
    SignKushu2,
    /// \u{121b6}: '𒆶'
    SignKwu318,
    /// \u{121b7}: '𒆷'
    SignLa,
    /// \u{121b8}: '𒆸'
    SignLagab,
    /// \u{121b9}: '𒆹'
    SignLagabTimesA,
    /// \u{121ba}: '𒆺'
    SignLagabTimesAPlusDaPlusHa,
    /// \u{121bb}: '𒆻'
    SignLagabTimesAPlusGar,
    /// \u{121bc}: '𒆼'
    SignLagabTimesAPlusLal,
    /// \u{121bd}: '𒆽'
    SignLagabTimesAl,
    /// \u{121be}: '𒆾'
    SignLagabTimesAn,
    /// \u{121bf}: '𒆿'
    SignLagabTimesAshZidaTenu,
    /// \u{121c0}: '𒇀'
    SignLagabTimesBad,
    /// \u{121c1}: '𒇁'
    SignLagabTimesBi,
    /// \u{121c2}: '𒇂'
    SignLagabTimesDar,
    /// \u{121c3}: '𒇃'
    SignLagabTimesEn,
    /// \u{121c4}: '𒇄'
    SignLagabTimesGa,
    /// \u{121c5}: '𒇅'
    SignLagabTimesGar,
    /// \u{121c6}: '𒇆'
    SignLagabTimesGud,
    /// \u{121c7}: '𒇇'
    SignLagabTimesGudPlusGud,
    /// \u{121c8}: '𒇈'
    SignLagabTimesHa,
    /// \u{121c9}: '𒇉'
    SignLagabTimesHal,
    /// \u{121ca}: '𒇊'
    SignLagabTimesHiTimesNun,
    /// \u{121cb}: '𒇋'
    SignLagabTimesIgiGunu,
    /// \u{121cc}: '𒇌'
    SignLagabTimesIm,
    /// \u{121cd}: '𒇍'
    SignLagabTimesImPlusHa,
    /// \u{121ce}: '𒇎'
    SignLagabTimesImPlusLu,
    /// \u{121cf}: '𒇏'
    SignLagabTimesKi,
    /// \u{121d0}: '𒇐'
    SignLagabTimesKin,
    /// \u{121d1}: '𒇑'
    SignLagabTimesKu3,
    /// \u{121d2}: '𒇒'
    SignLagabTimesKul,
    /// \u{121d3}: '𒇓'
    SignLagabTimesKulPlusHiPlusA,
    /// \u{121d4}: '𒇔'
    SignLagabTimesLagab,
    /// \u{121d5}: '𒇕'
    SignLagabTimesLish,
    /// \u{121d6}: '𒇖'
    SignLagabTimesLu,
    /// \u{121d7}: '𒇗'
    SignLagabTimesLul,
    /// \u{121d8}: '𒇘'
    SignLagabTimesMe,
    /// \u{121d9}: '𒇙'
    SignLagabTimesMePlusEn,
    /// \u{121da}: '𒇚'
    SignLagabTimesMush,
    /// \u{121db}: '𒇛'
    SignLagabTimesNe,
    /// \u{121dc}: '𒇜'
    SignLagabTimesShePlusSum,
    /// \u{121dd}: '𒇝'
    SignLagabTimesShitaPlusGishPlusErin2,
    /// \u{121de}: '𒇞'
    SignLagabTimesShitaPlusGishTenu,
    /// \u{121df}: '𒇟'
    SignLagabTimesShu2,
    /// \u{121e0}: '𒇠'
    SignLagabTimesShu2PlusShu2,
    /// \u{121e1}: '𒇡'
    SignLagabTimesSum,
    /// \u{121e2}: '𒇢'
    SignLagabTimesTag,
    /// \u{121e3}: '𒇣'
    SignLagabTimesTak4,
    /// \u{121e4}: '𒇤'
    SignLagabTimesTePlusAPlusSuPlusNa,
    /// \u{121e5}: '𒇥'
    SignLagabTimesU,
    /// \u{121e6}: '𒇦'
    SignLagabTimesUPlusA,
    /// \u{121e7}: '𒇧'
    SignLagabTimesUPlusUPlusU,
    /// \u{121e8}: '𒇨'
    SignLagabTimesU2PlusAsh,
    /// \u{121e9}: '𒇩'
    SignLagabTimesUd,
    /// \u{121ea}: '𒇪'
    SignLagabTimesUsh,
    /// \u{121eb}: '𒇫'
    SignLagabSquared,
    /// \u{121ec}: '𒇬'
    SignLagar,
    /// \u{121ed}: '𒇭'
    SignLagarTimesShe,
    /// \u{121ee}: '𒇮'
    SignLagarTimesShePlusSum,
    /// \u{121ef}: '𒇯'
    SignLagarGunu,
    /// \u{121f0}: '𒇰'
    SignLagarGunuOverLagarGunuShe,
    /// \u{121f1}: '𒇱'
    SignLahshu,
    /// \u{121f2}: '𒇲'
    SignLal,
    /// \u{121f3}: '𒇳'
    SignLalTimesLal,
    /// \u{121f4}: '𒇴'
    SignLam,
    /// \u{121f5}: '𒇵'
    SignLamTimesKur,
    /// \u{121f6}: '𒇶'
    SignLamTimesKurPlusRu,
    /// \u{121f7}: '𒇷'
    SignLi,
    /// \u{121f8}: '𒇸'
    SignLil,
    /// \u{121f9}: '𒇹'
    SignLimmu2,
    /// \u{121fa}: '𒇺'
    SignLish,
    /// \u{121fb}: '𒇻'
    SignLu,
    /// \u{121fc}: '𒇼'
    SignLuTimesBad,
    /// \u{121fd}: '𒇽'
    SignLu2,
    /// \u{121fe}: '𒇾'
    SignLu2TimesAl,
    /// \u{121ff}: '𒇿'
    SignLu2TimesBad,
    /// \u{12200}: '𒈀'
    SignLu2TimesEsh2,
    /// \u{12201}: '𒈁'
    SignLu2TimesEsh2Tenu,
    /// \u{12202}: '𒈂'
    SignLu2TimesGan2Tenu,
    /// \u{12203}: '𒈃'
    SignLu2TimesHiTimesBad,
    /// \u{12204}: '𒈄'
    SignLu2TimesIm,
    /// \u{12205}: '𒈅'
    SignLu2TimesKad2,
    /// \u{12206}: '𒈆'
    SignLu2TimesKad3,
    /// \u{12207}: '𒈇'
    SignLu2TimesKad3PlusAsh,
    /// \u{12208}: '𒈈'
    SignLu2TimesKi,
    /// \u{12209}: '𒈉'
    SignLu2TimesLaPlusAsh,
    /// \u{1220a}: '𒈊'
    SignLu2TimesLagab,
    /// \u{1220b}: '𒈋'
    SignLu2TimesMePlusEn,
    /// \u{1220c}: '𒈌'
    SignLu2TimesNe,
    /// \u{1220d}: '𒈍'
    SignLu2TimesNu,
    /// \u{1220e}: '𒈎'
    SignLu2TimesSiPlusAsh,
    /// \u{1220f}: '𒈏'
    SignLu2TimesSik2PlusBu,
    /// \u{12210}: '𒈐'
    SignLu2TimesTug2,
    /// \u{12211}: '𒈑'
    SignLu2Tenu,
    /// \u{12212}: '𒈒'
    SignLu2CrossingLu2,
    /// \u{12213}: '𒈓'
    SignLu2OpposingLu2,
    /// \u{12214}: '𒈔'
    SignLu2Squared,
    /// \u{12215}: '𒈕'
    SignLu2Sheshig,
    /// \u{12216}: '𒈖'
    SignLu3,
    /// \u{12217}: '𒈗'
    SignLugal,
    /// \u{12218}: '𒈘'
    SignLugalOverLugal,
    /// \u{12219}: '𒈙'
    SignLugalOpposingLugal,
    /// \u{1221a}: '𒈚'
    SignLugalSheshig,
    /// \u{1221b}: '𒈛'
    SignLuh,
    /// \u{1221c}: '𒈜'
    SignLul,
    /// \u{1221d}: '𒈝'
    SignLum,
    /// \u{1221e}: '𒈞'
    SignLumOverLum,
    /// \u{1221f}: '𒈟'
    SignLumOverLumGarOverGar,
    /// \u{12220}: '𒈠'
    SignMa,
    /// \u{12221}: '𒈡'
    SignMaTimesTak4,
    /// \u{12222}: '𒈢'
    SignMaGunu,
    /// \u{12223}: '𒈣'
    SignMa2,
    /// \u{12224}: '𒈤'
    SignMah,
    /// \u{12225}: '𒈥'
    SignMar,
    /// \u{12226}: '𒈦'
    SignMash,
    /// \u{12227}: '𒈧'
    SignMash2,
    /// \u{12228}: '𒈨'
    SignMe,
    /// \u{12229}: '𒈩'
    SignMes,
    /// \u{1222a}: '𒈪'
    SignMi,
    /// \u{1222b}: '𒈫'
    SignMin,
    /// \u{1222c}: '𒈬'
    SignMu,
    /// \u{1222d}: '𒈭'
    SignMuOverMu,
    /// \u{1222e}: '𒈮'
    SignMug,
    /// \u{1222f}: '𒈯'
    SignMugGunu,
    /// \u{12230}: '𒈰'
    SignMunsub,
    /// \u{12231}: '𒈱'
    SignMurgu2,
    /// \u{12232}: '𒈲'
    SignMush,
    /// \u{12233}: '𒈳'
    SignMushTimesA,
    /// \u{12234}: '𒈴'
    SignMushTimesKur,
    /// \u{12235}: '𒈵'
    SignMushTimesZa,
    /// \u{12236}: '𒈶'
    SignMushOverMush,
    /// \u{12237}: '𒈷'
    SignMushOverMushTimesAPlusNa,
    /// \u{12238}: '𒈸'
    SignMushCrossingMush,
    /// \u{12239}: '𒈹'
    SignMush3,
    /// \u{1223a}: '𒈺'
    SignMush3TimesA,
    /// \u{1223b}: '𒈻'
    SignMush3TimesAPlusDi,
    /// \u{1223c}: '𒈼'
    SignMush3TimesDi,
    /// \u{1223d}: '𒈽'
    SignMush3Gunu,
    /// \u{1223e}: '𒈾'
    SignNa,
    /// \u{1223f}: '𒈿'
    SignNa2,
    /// \u{12240}: '𒉀'
    SignNaga,
    /// \u{12241}: '𒉁'
    SignNagaInverted,
    /// \u{12242}: '𒉂'
    SignNagaTimesShuTenu,
    /// \u{12243}: '𒉃'
    SignNagaOpposingNaga,
    /// \u{12244}: '𒉄'
    SignNagar,
    /// \u{12245}: '𒉅'
    SignNamNutillu,
    /// \u{12246}: '𒉆'
    SignNam,
    /// \u{12247}: '𒉇'
    SignNam2,
    /// \u{12248}: '𒉈'
    SignNe,
    /// \u{12249}: '𒉉'
    SignNeTimesA,
    /// \u{1224a}: '𒉊'
    SignNeTimesUd,
    /// \u{1224b}: '𒉋'
    SignNeSheshig,
    /// \u{1224c}: '𒉌'
    SignNi,
    /// \u{1224d}: '𒉍'
    SignNiTimesE,
    /// \u{1224e}: '𒉎'
    SignNi2,
    /// \u{1224f}: '𒉏'
    SignNim,
    /// \u{12250}: '𒉐'
    SignNimTimesGan2Tenu,
    /// \u{12251}: '𒉑'
    SignNimTimesGarPlusGan2Tenu,
    /// \u{12252}: '𒉒'
    SignNinda2,
    /// \u{12253}: '𒉓'
    SignNinda2TimesAn,
    /// \u{12254}: '𒉔'
    SignNinda2TimesAsh,
    /// \u{12255}: '𒉕'
    SignNinda2TimesAshPlusAsh,
    /// \u{12256}: '𒉖'
    SignNinda2TimesGud,
    /// \u{12257}: '𒉗'
    SignNinda2TimesMePlusGan2Tenu,
    /// \u{12258}: '𒉘'
    SignNinda2TimesNe,
    /// \u{12259}: '𒉙'
    SignNinda2TimesNun,
    /// \u{1225a}: '𒉚'
    SignNinda2TimesShe,
    /// \u{1225b}: '𒉛'
    SignNinda2TimesShePlusAAn,
    /// \u{1225c}: '𒉜'
    SignNinda2TimesShePlusAsh,
    /// \u{1225d}: '𒉝'
    SignNinda2TimesShePlusAshPlusAsh,
    /// \u{1225e}: '𒉞'
    SignNinda2TimesU2PlusAsh,
    /// \u{1225f}: '𒉟'
    SignNinda2TimesUsh,
    /// \u{12260}: '𒉠'
    SignNisag,
    /// \u{12261}: '𒉡'
    SignNu,
    /// \u{12262}: '𒉢'
    SignNu11,
    /// \u{12263}: '𒉣'
    SignNun,
    /// \u{12264}: '𒉤'
    SignNunLagarTimesGar,
    /// \u{12265}: '𒉥'
    SignNunLagarTimesMash,
    /// \u{12266}: '𒉦'
    SignNunLagarTimesSal,
    /// \u{12267}: '𒉧'
    SignNunLagarTimesSalOverNunLagarTimesSal,
    /// \u{12268}: '𒉨'
    SignNunLagarTimesUsh,
    /// \u{12269}: '𒉩'
    SignNunTenu,
    /// \u{1226a}: '𒉪'
    SignNunOverNun,
    /// \u{1226b}: '𒉫'
    SignNunCrossingNun,
    /// \u{1226c}: '𒉬'
    SignNunCrossingNunLagarOverLagar,
    /// \u{1226d}: '𒉭'
    SignNunuz,
    /// \u{1226e}: '𒉮'
    SignNunuzAb2TimesAshgab,
    /// \u{1226f}: '𒉯'
    SignNunuzAb2TimesBi,
    /// \u{12270}: '𒉰'
    SignNunuzAb2TimesDug,
    /// \u{12271}: '𒉱'
    SignNunuzAb2TimesGud,
    /// \u{12272}: '𒉲'
    SignNunuzAb2TimesIgiGunu,
    /// \u{12273}: '𒉳'
    SignNunuzAb2TimesKad3,
    /// \u{12274}: '𒉴'
    SignNunuzAb2TimesLa,
    /// \u{12275}: '𒉵'
    SignNunuzAb2TimesNe,
    /// \u{12276}: '𒉶'
    SignNunuzAb2TimesSila3,
    /// \u{12277}: '𒉷'
    SignNunuzAb2TimesU2,
    /// \u{12278}: '𒉸'
    SignNunuzKisim5TimesBi,
    /// \u{12279}: '𒉹'
    SignNunuzKisim5TimesBiU,
    /// \u{1227a}: '𒉺'
    SignPa,
    /// \u{1227b}: '𒉻'
    SignPad,
    /// \u{1227c}: '𒉼'
    SignPan,
    /// \u{1227d}: '𒉽'
    SignPap,
    /// \u{1227e}: '𒉾'
    SignPesh2,
    /// \u{1227f}: '𒉿'
    SignPi,
    /// \u{12280}: '𒊀'
    SignPiTimesA,
    /// \u{12281}: '𒊁'
    SignPiTimesAb,
    /// \u{12282}: '𒊂'
    SignPiTimesBi,
    /// \u{12283}: '𒊃'
    SignPiTimesBu,
    /// \u{12284}: '𒊄'
    SignPiTimesE,
    /// \u{12285}: '𒊅'
    SignPiTimesI,
    /// \u{12286}: '𒊆'
    SignPiTimesIb,
    /// \u{12287}: '𒊇'
    SignPiTimesU,
    /// \u{12288}: '𒊈'
    SignPiTimesU2,
    /// \u{12289}: '𒊉'
    SignPiCrossingPi,
    /// \u{1228a}: '𒊊'
    SignPirig,
    /// \u{1228b}: '𒊋'
    SignPirigTimesKal,
    /// \u{1228c}: '𒊌'
    SignPirigTimesUd,
    /// \u{1228d}: '𒊍'
    SignPirigTimesZa,
    /// \u{1228e}: '𒊎'
    SignPirigOpposingPirig,
    /// \u{1228f}: '𒊏'
    SignRa,
    /// \u{12290}: '𒊐'
    SignRab,
    /// \u{12291}: '𒊑'
    SignRi,
    /// \u{12292}: '𒊒'
    SignRu,
    /// \u{12293}: '𒊓'
    SignSa,
    /// \u{12294}: '𒊔'
    SignSagNutillu,
    /// \u{12295}: '𒊕'
    SignSag,
    /// \u{12296}: '𒊖'
    SignSagTimesA,
    /// \u{12297}: '𒊗'
    SignSagTimesDu,
    /// \u{12298}: '𒊘'
    SignSagTimesDub,
    /// \u{12299}: '𒊙'
    SignSagTimesHa,
    /// \u{1229a}: '𒊚'
    SignSagTimesKak,
    /// \u{1229b}: '𒊛'
    SignSagTimesKur,
    /// \u{1229c}: '𒊜'
    SignSagTimesLum,
    /// \u{1229d}: '𒊝'
    SignSagTimesMi,
    /// \u{1229e}: '𒊞'
    SignSagTimesNun,
    /// \u{1229f}: '𒊟'
    SignSagTimesSal,
    /// \u{122a0}: '𒊠'
    SignSagTimesShid,
    /// \u{122a1}: '𒊡'
    SignSagTimesTab,
    /// \u{122a2}: '𒊢'
    SignSagTimesU2,
    /// \u{122a3}: '𒊣'
    SignSagTimesUb,
    /// \u{122a4}: '𒊤'
    SignSagTimesUm,
    /// \u{122a5}: '𒊥'
    SignSagTimesUr,
    /// \u{122a6}: '𒊦'
    SignSagTimesUsh,
    /// \u{122a7}: '𒊧'
    SignSagOverSag,
    /// \u{122a8}: '𒊨'
    SignSagGunu,
    /// \u{122a9}: '𒊩'
    SignSal,
    /// \u{122aa}: '𒊪'
    SignSalLagabTimesAsh2,
    /// \u{122ab}: '𒊫'
    SignSanga2,
    /// \u{122ac}: '𒊬'
    SignSar,
    /// \u{122ad}: '𒊭'
    SignSha,
    /// \u{122ae}: '𒊮'
    SignSha3,
    /// \u{122af}: '𒊯'
    SignSha3TimesA,
    /// \u{122b0}: '𒊰'
    SignSha3TimesBad,
    /// \u{122b1}: '𒊱'
    SignSha3TimesGish,
    /// \u{122b2}: '𒊲'
    SignSha3TimesNe,
    /// \u{122b3}: '𒊳'
    SignSha3TimesShu2,
    /// \u{122b4}: '𒊴'
    SignSha3TimesTur,
    /// \u{122b5}: '𒊵'
    SignSha3TimesU,
    /// \u{122b6}: '𒊶'
    SignSha3TimesUPlusA,
    /// \u{122b7}: '𒊷'
    SignSha6,
    /// \u{122b8}: '𒊸'
    SignShab6,
    /// \u{122b9}: '𒊹'
    SignShar2,
    /// \u{122ba}: '𒊺'
    SignShe,
    /// \u{122bb}: '𒊻'
    SignSheHu,
    /// \u{122bc}: '𒊼'
    SignSheOverSheGadOverGadGarOverGar,
    /// \u{122bd}: '𒊽'
    SignSheOverSheTabOverTabGarOverGar,
    /// \u{122be}: '𒊾'
    SignSheg9,
    /// \u{122bf}: '𒊿'
    SignShen,
    /// \u{122c0}: '𒋀'
    SignShesh,
    /// \u{122c1}: '𒋁'
    SignShesh2,
    /// \u{122c2}: '𒋂'
    SignSheshlam,
    /// \u{122c3}: '𒋃'
    SignShid,
    /// \u{122c4}: '𒋄'
    SignShidTimesA,
    /// \u{122c5}: '𒋅'
    SignShidTimesIm,
    /// \u{122c6}: '𒋆'
    SignShim,
    /// \u{122c7}: '𒋇'
    SignShimTimesA,
    /// \u{122c8}: '𒋈'
    SignShimTimesBal,
    /// \u{122c9}: '𒋉'
    SignShimTimesBulug,
    /// \u{122ca}: '𒋊'
    SignShimTimesDin,
    /// \u{122cb}: '𒋋'
    SignShimTimesGar,
    /// \u{122cc}: '𒋌'
    SignShimTimesIgi,
    /// \u{122cd}: '𒋍'
    SignShimTimesIgiGunu,
    /// \u{122ce}: '𒋎'
    SignShimTimesKushu2,
    /// \u{122cf}: '𒋏'
    SignShimTimesLul,
    /// \u{122d0}: '𒋐'
    SignShimTimesMug,
    /// \u{122d1}: '𒋑'
    SignShimTimesSal,
    /// \u{122d2}: '𒋒'
    SignShinig,
    /// \u{122d3}: '𒋓'
    SignShir,
    /// \u{122d4}: '𒋔'
    SignShirTenu,
    /// \u{122d5}: '𒋕'
    SignShirOverShirBurOverBur,
    /// \u{122d6}: '𒋖'
    SignShita,
    /// \u{122d7}: '𒋗'
    SignShu,
    /// \u{122d8}: '𒋘'
    SignShuOverInvertedShu,
    /// \u{122d9}: '𒋙'
    SignShu2,
    /// \u{122da}: '𒋚'
    SignShubur,
    /// \u{122db}: '𒋛'
    SignSi,
    /// \u{122dc}: '𒋜'
    SignSiGunu,
    /// \u{122dd}: '𒋝'
    SignSig,
    /// \u{122de}: '𒋞'
    SignSig4,
    /// \u{122df}: '𒋟'
    SignSig4OverSig4Shu2,
    /// \u{122e0}: '𒋠'
    SignSik2,
    /// \u{122e1}: '𒋡'
    SignSila3,
    /// \u{122e2}: '𒋢'
    SignSu,
    /// \u{122e3}: '𒋣'
    SignSuOverSu,
    /// \u{122e4}: '𒋤'
    SignSud,
    /// \u{122e5}: '𒋥'
    SignSud2,
    /// \u{122e6}: '𒋦'
    SignSuhur,
    /// \u{122e7}: '𒋧'
    SignSum,
    /// \u{122e8}: '𒋨'
    SignSumash,
    /// \u{122e9}: '𒋩'
    SignSur,
    /// \u{122ea}: '𒋪'
    SignSur9,
    /// \u{122eb}: '𒋫'
    SignTa,
    /// \u{122ec}: '𒋬'
    SignTaAsterisk,
    /// \u{122ed}: '𒋭'
    SignTaTimesHi,
    /// \u{122ee}: '𒋮'
    SignTaTimesMi,
    /// \u{122ef}: '𒋯'
    SignTaGunu,
    /// \u{122f0}: '𒋰'
    SignTab,
    /// \u{122f1}: '𒋱'
    SignTabOverTabNiOverNiDishOverDish,
    /// \u{122f2}: '𒋲'
    SignTabSquared,
    /// \u{122f3}: '𒋳'
    SignTag,
    /// \u{122f4}: '𒋴'
    SignTagTimesBi,
    /// \u{122f5}: '𒋵'
    SignTagTimesGud,
    /// \u{122f6}: '𒋶'
    SignTagTimesShe,
    /// \u{122f7}: '𒋷'
    SignTagTimesShu,
    /// \u{122f8}: '𒋸'
    SignTagTimesTug2,
    /// \u{122f9}: '𒋹'
    SignTagTimesUd,
    /// \u{122fa}: '𒋺'
    SignTak4,
    /// \u{122fb}: '𒋻'
    SignTar,
    /// \u{122fc}: '𒋼'
    SignTe,
    /// \u{122fd}: '𒋽'
    SignTeGunu,
    /// \u{122fe}: '𒋾'
    SignTi,
    /// \u{122ff}: '𒋿'
    SignTiTenu,
    /// \u{12300}: '𒌀'
    SignTil,
    /// \u{12301}: '𒌁'
    SignTir,
    /// \u{12302}: '𒌂'
    SignTirTimesTak4,
    /// \u{12303}: '𒌃'
    SignTirOverTir,
    /// \u{12304}: '𒌄'
    SignTirOverTirGadOverGadGarOverGar,
    /// \u{12305}: '𒌅'
    SignTu,
    /// \u{12306}: '𒌆'
    SignTug2,
    /// \u{12307}: '𒌇'
    SignTuk,
    /// \u{12308}: '𒌈'
    SignTum,
    /// \u{12309}: '𒌉'
    SignTur,
    /// \u{1230a}: '𒌊'
    SignTurOverTurZaOverZa,
    /// \u{1230b}: '𒌋'
    SignU,
    /// \u{1230c}: '𒌌'
    SignUGud,
    /// \u{1230d}: '𒌍'
    SignUUU,
    /// \u{1230e}: '𒌎'
    SignUOverUPaOverPaGarOverGar,
    /// \u{1230f}: '𒌏'
    SignUOverUSurOverSur,
    /// \u{12310}: '𒌐'
    SignUOverUUReversedOverUReversed,
    /// \u{12311}: '𒌑'
    SignU2,
    /// \u{12312}: '𒌒'
    SignUb,
    /// \u{12313}: '𒌓'
    SignUd,
    /// \u{12314}: '𒌔'
    SignUdKushu2,
    /// \u{12315}: '𒌕'
    SignUdTimesBad,
    /// \u{12316}: '𒌖'
    SignUdTimesMi,
    /// \u{12317}: '𒌗'
    SignUdTimesUPlusUPlusU,
    /// \u{12318}: '𒌘'
    SignUdTimesUPlusUPlusUGunu,
    /// \u{12319}: '𒌙'
    SignUdGunu,
    /// \u{1231a}: '𒌚'
    SignUdSheshig,
    /// \u{1231b}: '𒌛'
    SignUdSheshigTimesBad,
    /// \u{1231c}: '𒌜'
    SignUdug,
    /// \u{1231d}: '𒌝'
    SignUm,
    /// \u{1231e}: '𒌞'
    SignUmTimesLagab,
    /// \u{1231f}: '𒌟'
    SignUmTimesMePlusDa,
    /// \u{12320}: '𒌠'
    SignUmTimesSha3,
    /// \u{12321}: '𒌡'
    SignUmTimesU,
    /// \u{12322}: '𒌢'
    SignUmbin,
    /// \u{12323}: '𒌣'
    SignUmum,
    /// \u{12324}: '𒌤'
    SignUmumTimesKaskal,
    /// \u{12325}: '𒌥'
    SignUmumTimesPa,
    /// \u{12326}: '𒌦'
    SignUn,
    /// \u{12327}: '𒌧'
    SignUnGunu,
    /// \u{12328}: '𒌨'
    SignUr,
    /// \u{12329}: '𒌩'
    SignUrCrossingUr,
    /// \u{1232a}: '𒌪'
    SignUrSheshig,
    /// \u{1232b}: '𒌫'
    SignUr2,
    /// \u{1232c}: '𒌬'
    SignUr2TimesAPlusHa,
    /// \u{1232d}: '𒌭'
    SignUr2TimesAPlusNa,
    /// \u{1232e}: '𒌮'
    SignUr2TimesAl,
    /// \u{1232f}: '𒌯'
    SignUr2TimesHa,
    /// \u{12330}: '𒌰'
    SignUr2TimesNun,
    /// \u{12331}: '𒌱'
    SignUr2TimesU2,
    /// \u{12332}: '𒌲'
    SignUr2TimesU2PlusAsh,
    /// \u{12333}: '𒌳'
    SignUr2TimesU2PlusBi,
    /// \u{12334}: '𒌴'
    SignUr4,
    /// \u{12335}: '𒌵'
    SignUri,
    /// \u{12336}: '𒌶'
    SignUri3,
    /// \u{12337}: '𒌷'
    SignUru,
    /// \u{12338}: '𒌸'
    SignUruTimesA,
    /// \u{12339}: '𒌹'
    SignUruTimesAshgab,
    /// \u{1233a}: '𒌺'
    SignUruTimesBar,
    /// \u{1233b}: '𒌻'
    SignUruTimesDun,
    /// \u{1233c}: '𒌼'
    SignUruTimesGa,
    /// \u{1233d}: '𒌽'
    SignUruTimesGal,
    /// \u{1233e}: '𒌾'
    SignUruTimesGan2Tenu,
    /// \u{1233f}: '𒌿'
    SignUruTimesGar,
    /// \u{12340}: '𒍀'
    SignUruTimesGu,
    /// \u{12341}: '𒍁'
    SignUruTimesHa,
    /// \u{12342}: '𒍂'
    SignUruTimesIgi,
    /// \u{12343}: '𒍃'
    SignUruTimesIm,
    /// \u{12344}: '𒍄'
    SignUruTimesIsh,
    /// \u{12345}: '𒍅'
    SignUruTimesKi,
    /// \u{12346}: '𒍆'
    SignUruTimesLum,
    /// \u{12347}: '𒍇'
    SignUruTimesMin,
    /// \u{12348}: '𒍈'
    SignUruTimesPa,
    /// \u{12349}: '𒍉'
    SignUruTimesShe,
    /// \u{1234a}: '𒍊'
    SignUruTimesSig4,
    /// \u{1234b}: '𒍋'
    SignUruTimesTu,
    /// \u{1234c}: '𒍌'
    SignUruTimesUPlusGud,
    /// \u{1234d}: '𒍍'
    SignUruTimesUd,
    /// \u{1234e}: '𒍎'
    SignUruTimesUruda,
    /// \u{1234f}: '𒍏'
    SignUruda,
    /// \u{12350}: '𒍐'
    SignUrudaTimesU,
    /// \u{12351}: '𒍑'
    SignUsh,
    /// \u{12352}: '𒍒'
    SignUshTimesA,
    /// \u{12353}: '𒍓'
    SignUshTimesKu,
    /// \u{12354}: '𒍔'
    SignUshTimesKur,
    /// \u{12355}: '𒍕'
    SignUshTimesTak4,
    /// \u{12356}: '𒍖'
    SignUshx,
    /// \u{12357}: '𒍗'
    SignUsh2,
    /// \u{12358}: '𒍘'
    SignUshumx,
    /// \u{12359}: '𒍙'
    SignUtuki,
    /// \u{1235a}: '𒍚'
    SignUz3,
    /// \u{1235b}: '𒍛'
    SignUz3TimesKaskal,
    /// \u{1235c}: '𒍜'
    SignUzu,
    /// \u{1235d}: '𒍝'
    SignZa,
    /// \u{1235e}: '𒍞'
    SignZaTenu,
    /// \u{1235f}: '𒍟'
    SignZaSquaredTimesKur,
    /// \u{12360}: '𒍠'
    SignZag,
    /// \u{12361}: '𒍡'
    SignZamx,
    /// \u{12362}: '𒍢'
    SignZe2,
    /// \u{12363}: '𒍣'
    SignZi,
    /// \u{12364}: '𒍤'
    SignZiOverZi,
    /// \u{12365}: '𒍥'
    SignZi3,
    /// \u{12366}: '𒍦'
    SignZib,
    /// \u{12367}: '𒍧'
    SignZibKabaTenu,
    /// \u{12368}: '𒍨'
    SignZig,
    /// \u{12369}: '𒍩'
    SignZiz2,
    /// \u{1236a}: '𒍪'
    SignZu,
    /// \u{1236b}: '𒍫'
    SignZu5,
    /// \u{1236c}: '𒍬'
    SignZu5TimesA,
    /// \u{1236d}: '𒍭'
    SignZubur,
    /// \u{1236e}: '𒍮'
    SignZum,
    /// \u{1236f}: '𒍯'
    SignKapElamite,
    /// \u{12370}: '𒍰'
    SignAbTimesNun,
    /// \u{12371}: '𒍱'
    SignAb2TimesA,
    /// \u{12372}: '𒍲'
    SignAmarTimesKug,
    /// \u{12373}: '𒍳'
    SignDagKisim5TimesU2PlusMash,
    /// \u{12374}: '𒍴'
    SignDag3,
    /// \u{12375}: '𒍵'
    SignDishPlusShu,
    /// \u{12376}: '𒍶'
    SignDubTimesShe,
    /// \u{12377}: '𒍷'
    SignEzenTimesGud,
    /// \u{12378}: '𒍸'
    SignEzenTimesShe,
    /// \u{12379}: '𒍹'
    SignGa2TimesAnPlusKakPlusA,
    /// \u{1237a}: '𒍺'
    SignGa2TimesAsh2,
    /// \u{1237b}: '𒍻'
    SignGe22,
    /// \u{1237c}: '𒍼'
    SignGig,
    /// \u{1237d}: '𒍽'
    SignHush,
    /// \u{1237e}: '𒍾'
    SignKaTimesAnshe,
    /// \u{1237f}: '𒍿'
    SignKaTimesAsh3,
    /// \u{12380}: '𒎀'
    SignKaTimesGish,
    /// \u{12381}: '𒎁'
    SignKaTimesGud,
    /// \u{12382}: '𒎂'
    SignKaTimesHiTimesAsh2,
    /// \u{12383}: '𒎃'
    SignKaTimesLum,
    /// \u{12384}: '𒎄'
    SignKaTimesPa,
    /// \u{12385}: '𒎅'
    SignKaTimesShul,
    /// \u{12386}: '𒎆'
    SignKaTimesTu,
    /// \u{12387}: '𒎇'
    SignKaTimesUr2,
    /// \u{12388}: '𒎈'
    SignLagabTimesGi,
    /// \u{12389}: '𒎉'
    SignLu2SheshigTimesBad,
    /// \u{1238a}: '𒎊'
    SignLu2TimesEsh2PlusLal,
    /// \u{1238b}: '𒎋'
    SignLu2TimesShu,
    /// \u{1238c}: '𒎌'
    SignMesh,
    /// \u{1238d}: '𒎍'
    SignMush3TimesZa,
    /// \u{1238e}: '𒎎'
    SignNa4,
    /// \u{1238f}: '𒎏'
    SignNin,
    /// \u{12390}: '𒎐'
    SignNin9,
    /// \u{12391}: '𒎑'
    SignNinda2TimesBal,
    /// \u{12392}: '𒎒'
    SignNinda2TimesGi,
    /// \u{12393}: '𒎓'
    SignNu11RotatedNinetyDegrees,
    /// \u{12394}: '𒎔'
    SignPesh2Asterisk,
    /// \u{12395}: '𒎕'
    SignPir2,
    /// \u{12396}: '𒎖'
    SignSagTimesIgiGunu,
    /// \u{12397}: '𒎗'
    SignTi2,
    /// \u{12398}: '𒎘'
    SignUmTimesMe,
    /// \u{12399}: '𒎙'
    SignUU,
}

impl Into<char> for Cuneiform {
    fn into(self) -> char {
        match self {
            Cuneiform::SignA => '𒀀',
            Cuneiform::SignATimesA => '𒀁',
            Cuneiform::SignATimesBad => '𒀂',
            Cuneiform::SignATimesGan2Tenu => '𒀃',
            Cuneiform::SignATimesHa => '𒀄',
            Cuneiform::SignATimesIgi => '𒀅',
            Cuneiform::SignATimesLagarGunu => '𒀆',
            Cuneiform::SignATimesMush => '𒀇',
            Cuneiform::SignATimesSag => '𒀈',
            Cuneiform::SignA2 => '𒀉',
            Cuneiform::SignAb => '𒀊',
            Cuneiform::SignAbTimesAsh2 => '𒀋',
            Cuneiform::SignAbTimesDun3Gunu => '𒀌',
            Cuneiform::SignAbTimesGal => '𒀍',
            Cuneiform::SignAbTimesGan2Tenu => '𒀎',
            Cuneiform::SignAbTimesHa => '𒀏',
            Cuneiform::SignAbTimesIgiGunu => '𒀐',
            Cuneiform::SignAbTimesImin => '𒀑',
            Cuneiform::SignAbTimesLagab => '𒀒',
            Cuneiform::SignAbTimesShesh => '𒀓',
            Cuneiform::SignAbTimesUPlusUPlusU => '𒀔',
            Cuneiform::SignAbGunu => '𒀕',
            Cuneiform::SignAb2 => '𒀖',
            Cuneiform::SignAb2TimesBalag => '𒀗',
            Cuneiform::SignAb2TimesGan2Tenu => '𒀘',
            Cuneiform::SignAb2TimesMePlusEn => '𒀙',
            Cuneiform::SignAb2TimesSha3 => '𒀚',
            Cuneiform::SignAb2TimesTak4 => '𒀛',
            Cuneiform::SignAd => '𒀜',
            Cuneiform::SignAk => '𒀝',
            Cuneiform::SignAkTimesErin2 => '𒀞',
            Cuneiform::SignAkTimesShitaPlusGish => '𒀟',
            Cuneiform::SignAl => '𒀠',
            Cuneiform::SignAlTimesAl => '𒀡',
            Cuneiform::SignAlTimesDim2 => '𒀢',
            Cuneiform::SignAlTimesGish => '𒀣',
            Cuneiform::SignAlTimesHa => '𒀤',
            Cuneiform::SignAlTimesKad3 => '𒀥',
            Cuneiform::SignAlTimesKi => '𒀦',
            Cuneiform::SignAlTimesShe => '𒀧',
            Cuneiform::SignAlTimesUsh => '𒀨',
            Cuneiform::SignAlan => '𒀩',
            Cuneiform::SignAleph => '𒀪',
            Cuneiform::SignAmar => '𒀫',
            Cuneiform::SignAmarTimesShe => '𒀬',
            Cuneiform::SignAn => '𒀭',
            Cuneiform::SignAnOverAn => '𒀮',
            Cuneiform::SignAnThreeTimes => '𒀯',
            Cuneiform::SignAnPlusNagaOpposingAnPlusNaga => '𒀰',
            Cuneiform::SignAnPlusNagaSquared => '𒀱',
            Cuneiform::SignAnshe => '𒀲',
            Cuneiform::SignApin => '𒀳',
            Cuneiform::SignArad => '𒀴',
            Cuneiform::SignAradTimesKur => '𒀵',
            Cuneiform::SignArkab => '𒀶',
            Cuneiform::SignAsal2 => '𒀷',
            Cuneiform::SignAsh => '𒀸',
            Cuneiform::SignAshZidaTenu => '𒀹',
            Cuneiform::SignAshKabaTenu => '𒀺',
            Cuneiform::SignAshOverAshTug2OverTug2Tug2OverTug2Pap => '𒀻',
            Cuneiform::SignAshOverAshOverAsh => '𒀼',
            Cuneiform::SignAshOverAshOverAshCrossingAshOverAshOverAsh => '𒀽',
            Cuneiform::SignAsh2 => '𒀾',
            Cuneiform::SignAshgab => '𒀿',
            Cuneiform::SignBa => '𒁀',
            Cuneiform::SignBad => '𒁁',
            Cuneiform::SignBag3 => '𒁂',
            Cuneiform::SignBahar2 => '𒁃',
            Cuneiform::SignBal => '𒁄',
            Cuneiform::SignBalOverBal => '𒁅',
            Cuneiform::SignBalag => '𒁆',
            Cuneiform::SignBar => '𒁇',
            Cuneiform::SignBara2 => '𒁈',
            Cuneiform::SignBi => '𒁉',
            Cuneiform::SignBiTimesA => '𒁊',
            Cuneiform::SignBiTimesGar => '𒁋',
            Cuneiform::SignBiTimesIgiGunu => '𒁌',
            Cuneiform::SignBu => '𒁍',
            Cuneiform::SignBuOverBuAb => '𒁎',
            Cuneiform::SignBuOverBuUn => '𒁏',
            Cuneiform::SignBuCrossingBu => '𒁐',
            Cuneiform::SignBulug => '𒁑',
            Cuneiform::SignBulugOverBulug => '𒁒',
            Cuneiform::SignBur => '𒁓',
            Cuneiform::SignBur2 => '𒁔',
            Cuneiform::SignDa => '𒁕',
            Cuneiform::SignDag => '𒁖',
            Cuneiform::SignDagKisim5TimesAPlusMash => '𒁗',
            Cuneiform::SignDagKisim5TimesAmar => '𒁘',
            Cuneiform::SignDagKisim5TimesBalag => '𒁙',
            Cuneiform::SignDagKisim5TimesBi => '𒁚',
            Cuneiform::SignDagKisim5TimesGa => '𒁛',
            Cuneiform::SignDagKisim5TimesGaPlusMash => '𒁜',
            Cuneiform::SignDagKisim5TimesGi => '𒁝',
            Cuneiform::SignDagKisim5TimesGir2 => '𒁞',
            Cuneiform::SignDagKisim5TimesGud => '𒁟',
            Cuneiform::SignDagKisim5TimesHa => '𒁠',
            Cuneiform::SignDagKisim5TimesIr => '𒁡',
            Cuneiform::SignDagKisim5TimesIrPlusLu => '𒁢',
            Cuneiform::SignDagKisim5TimesKak => '𒁣',
            Cuneiform::SignDagKisim5TimesLa => '𒁤',
            Cuneiform::SignDagKisim5TimesLu => '𒁥',
            Cuneiform::SignDagKisim5TimesLuPlusMash2 => '𒁦',
            Cuneiform::SignDagKisim5TimesLum => '𒁧',
            Cuneiform::SignDagKisim5TimesNe => '𒁨',
            Cuneiform::SignDagKisim5TimesPapPlusPap => '𒁩',
            Cuneiform::SignDagKisim5TimesSi => '𒁪',
            Cuneiform::SignDagKisim5TimesTak4 => '𒁫',
            Cuneiform::SignDagKisim5TimesU2PlusGir2 => '𒁬',
            Cuneiform::SignDagKisim5TimesUsh => '𒁭',
            Cuneiform::SignDam => '𒁮',
            Cuneiform::SignDar => '𒁯',
            Cuneiform::SignDara3 => '𒁰',
            Cuneiform::SignDara4 => '𒁱',
            Cuneiform::SignDi => '𒁲',
            Cuneiform::SignDib => '𒁳',
            Cuneiform::SignDim => '𒁴',
            Cuneiform::SignDimTimesShe => '𒁵',
            Cuneiform::SignDim2 => '𒁶',
            Cuneiform::SignDin => '𒁷',
            Cuneiform::SignDinKaskalUGunuDish => '𒁸',
            Cuneiform::SignDish => '𒁹',
            Cuneiform::SignDu => '𒁺',
            Cuneiform::SignDuOverDu => '𒁻',
            Cuneiform::SignDuGunu => '𒁼',
            Cuneiform::SignDuSheshig => '𒁽',
            Cuneiform::SignDub => '𒁾',
            Cuneiform::SignDubTimesEsh2 => '𒁿',
            Cuneiform::SignDub2 => '𒂀',
            Cuneiform::SignDug => '𒂁',
            Cuneiform::SignDugud => '𒂂',
            Cuneiform::SignDuh => '𒂃',
            Cuneiform::SignDun => '𒂄',
            Cuneiform::SignDun3 => '𒂅',
            Cuneiform::SignDun3Gunu => '𒂆',
            Cuneiform::SignDun3GunuGunu => '𒂇',
            Cuneiform::SignDun4 => '𒂈',
            Cuneiform::SignDur2 => '𒂉',
            Cuneiform::SignE => '𒂊',
            Cuneiform::SignETimesPap => '𒂋',
            Cuneiform::SignEOverENunOverNun => '𒂌',
            Cuneiform::SignE2 => '𒂍',
            Cuneiform::SignE2TimesAPlusHaPlusDa => '𒂎',
            Cuneiform::SignE2TimesGar => '𒂏',
            Cuneiform::SignE2TimesMi => '𒂐',
            Cuneiform::SignE2TimesSal => '𒂑',
            Cuneiform::SignE2TimesShe => '𒂒',
            Cuneiform::SignE2TimesU => '𒂓',
            Cuneiform::SignEdin => '𒂔',
            Cuneiform::SignEgir => '𒂕',
            Cuneiform::SignEl => '𒂖',
            Cuneiform::SignEn => '𒂗',
            Cuneiform::SignEnTimesGan2 => '𒂘',
            Cuneiform::SignEnTimesGan2Tenu => '𒂙',
            Cuneiform::SignEnTimesMe => '𒂚',
            Cuneiform::SignEnCrossingEn => '𒂛',
            Cuneiform::SignEnOpposingEn => '𒂜',
            Cuneiform::SignEnSquared => '𒂝',
            Cuneiform::SignEren => '𒂞',
            Cuneiform::SignErin2 => '𒂟',
            Cuneiform::SignEsh2 => '𒂠',
            Cuneiform::SignEzen => '𒂡',
            Cuneiform::SignEzenTimesA => '𒂢',
            Cuneiform::SignEzenTimesAPlusLal => '𒂣',
            Cuneiform::SignEzenTimesAPlusLalTimesLal => '𒂤',
            Cuneiform::SignEzenTimesAn => '𒂥',
            Cuneiform::SignEzenTimesBad => '𒂦',
            Cuneiform::SignEzenTimesDun3Gunu => '𒂧',
            Cuneiform::SignEzenTimesDun3GunuGunu => '𒂨',
            Cuneiform::SignEzenTimesHa => '𒂩',
            Cuneiform::SignEzenTimesHaGunu => '𒂪',
            Cuneiform::SignEzenTimesIgiGunu => '𒂫',
            Cuneiform::SignEzenTimesKaskal => '𒂬',
            Cuneiform::SignEzenTimesKaskalSquared => '𒂭',
            Cuneiform::SignEzenTimesKu3 => '𒂮',
            Cuneiform::SignEzenTimesLa => '𒂯',
            Cuneiform::SignEzenTimesLalTimesLal => '𒂰',
            Cuneiform::SignEzenTimesLi => '𒂱',
            Cuneiform::SignEzenTimesLu => '𒂲',
            Cuneiform::SignEzenTimesU2 => '𒂳',
            Cuneiform::SignEzenTimesUd => '𒂴',
            Cuneiform::SignGa => '𒂵',
            Cuneiform::SignGaGunu => '𒂶',
            Cuneiform::SignGa2 => '𒂷',
            Cuneiform::SignGa2TimesAPlusDaPlusHa => '𒂸',
            Cuneiform::SignGa2TimesAPlusHa => '𒂹',
            Cuneiform::SignGa2TimesAPlusIgi => '𒂺',
            Cuneiform::SignGa2TimesAb2TenuPlusTab => '𒂻',
            Cuneiform::SignGa2TimesAn => '𒂼',
            Cuneiform::SignGa2TimesAsh => '𒂽',
            Cuneiform::SignGa2TimesAsh2PlusGal => '𒂾',
            Cuneiform::SignGa2TimesBad => '𒂿',
            Cuneiform::SignGa2TimesBarPlusRa => '𒃀',
            Cuneiform::SignGa2TimesBur => '𒃁',
            Cuneiform::SignGa2TimesBurPlusRa => '𒃂',
            Cuneiform::SignGa2TimesDa => '𒃃',
            Cuneiform::SignGa2TimesDi => '𒃄',
            Cuneiform::SignGa2TimesDimTimesShe => '𒃅',
            Cuneiform::SignGa2TimesDub => '𒃆',
            Cuneiform::SignGa2TimesEl => '𒃇',
            Cuneiform::SignGa2TimesElPlusLa => '𒃈',
            Cuneiform::SignGa2TimesEn => '𒃉',
            Cuneiform::SignGa2TimesEnTimesGan2Tenu => '𒃊',
            Cuneiform::SignGa2TimesGan2Tenu => '𒃋',
            Cuneiform::SignGa2TimesGar => '𒃌',
            Cuneiform::SignGa2TimesGi => '𒃍',
            Cuneiform::SignGa2TimesGi4 => '𒃎',
            Cuneiform::SignGa2TimesGi4PlusA => '𒃏',
            Cuneiform::SignGa2TimesGir2PlusSu => '𒃐',
            Cuneiform::SignGa2TimesHaPlusLuPlusEsh2 => '𒃑',
            Cuneiform::SignGa2TimesHal => '𒃒',
            Cuneiform::SignGa2TimesHalPlusLa => '𒃓',
            Cuneiform::SignGa2TimesHiPlusLi => '𒃔',
            Cuneiform::SignGa2TimesHub2 => '𒃕',
            Cuneiform::SignGa2TimesIgiGunu => '𒃖',
            Cuneiform::SignGa2TimesIshPlusHuPlusAsh => '𒃗',
            Cuneiform::SignGa2TimesKak => '𒃘',
            Cuneiform::SignGa2TimesKaskal => '𒃙',
            Cuneiform::SignGa2TimesKid => '𒃚',
            Cuneiform::SignGa2TimesKidPlusLal => '𒃛',
            Cuneiform::SignGa2TimesKu3PlusAn => '𒃜',
            Cuneiform::SignGa2TimesLa => '𒃝',
            Cuneiform::SignGa2TimesMePlusEn => '𒃞',
            Cuneiform::SignGa2TimesMi => '𒃟',
            Cuneiform::SignGa2TimesNun => '𒃠',
            Cuneiform::SignGa2TimesNunOverNun => '𒃡',
            Cuneiform::SignGa2TimesPa => '𒃢',
            Cuneiform::SignGa2TimesSal => '𒃣',
            Cuneiform::SignGa2TimesSar => '𒃤',
            Cuneiform::SignGa2TimesShe => '𒃥',
            Cuneiform::SignGa2TimesShePlusTur => '𒃦',
            Cuneiform::SignGa2TimesShid => '𒃧',
            Cuneiform::SignGa2TimesSum => '𒃨',
            Cuneiform::SignGa2TimesTak4 => '𒃩',
            Cuneiform::SignGa2TimesU => '𒃪',
            Cuneiform::SignGa2TimesUd => '𒃫',
            Cuneiform::SignGa2TimesUdPlusDu => '𒃬',
            Cuneiform::SignGa2OverGa2 => '𒃭',
            Cuneiform::SignGaba => '𒃮',
            Cuneiform::SignGabaCrossingGaba => '𒃯',
            Cuneiform::SignGad => '𒃰',
            Cuneiform::SignGadOverGadGarOverGar => '𒃱',
            Cuneiform::SignGal => '𒃲',
            Cuneiform::SignGalGadOverGadGarOverGar => '𒃳',
            Cuneiform::SignGalam => '𒃴',
            Cuneiform::SignGam => '𒃵',
            Cuneiform::SignGan => '𒃶',
            Cuneiform::SignGan2 => '𒃷',
            Cuneiform::SignGan2Tenu => '𒃸',
            Cuneiform::SignGan2OverGan2 => '𒃹',
            Cuneiform::SignGan2CrossingGan2 => '𒃺',
            Cuneiform::SignGar => '𒃻',
            Cuneiform::SignGar3 => '𒃼',
            Cuneiform::SignGashan => '𒃽',
            Cuneiform::SignGeshtin => '𒃾',
            Cuneiform::SignGeshtinTimesKur => '𒃿',
            Cuneiform::SignGi => '𒄀',
            Cuneiform::SignGiTimesE => '𒄁',
            Cuneiform::SignGiTimesU => '𒄂',
            Cuneiform::SignGiCrossingGi => '𒄃',
            Cuneiform::SignGi4 => '𒄄',
            Cuneiform::SignGi4OverGi4 => '𒄅',
            Cuneiform::SignGi4CrossingGi4 => '𒄆',
            Cuneiform::SignGidim => '𒄇',
            Cuneiform::SignGir2 => '𒄈',
            Cuneiform::SignGir2Gunu => '𒄉',
            Cuneiform::SignGir3 => '𒄊',
            Cuneiform::SignGir3TimesAPlusIgi => '𒄋',
            Cuneiform::SignGir3TimesGan2Tenu => '𒄌',
            Cuneiform::SignGir3TimesIgi => '𒄍',
            Cuneiform::SignGir3TimesLuPlusIgi => '𒄎',
            Cuneiform::SignGir3TimesPa => '𒄏',
            Cuneiform::SignGisal => '𒄐',
            Cuneiform::SignGish => '𒄑',
            Cuneiform::SignGishCrossingGish => '𒄒',
            Cuneiform::SignGishTimesBad => '𒄓',
            Cuneiform::SignGishTimesTak4 => '𒄔',
            Cuneiform::SignGishTenu => '𒄕',
            Cuneiform::SignGu => '𒄖',
            Cuneiform::SignGuCrossingGu => '𒄗',
            Cuneiform::SignGu2 => '𒄘',
            Cuneiform::SignGu2TimesKak => '𒄙',
            Cuneiform::SignGu2TimesKakTimesIgiGunu => '𒄚',
            Cuneiform::SignGu2TimesNun => '𒄛',
            Cuneiform::SignGu2TimesSalPlusTug2 => '𒄜',
            Cuneiform::SignGu2Gunu => '𒄝',
            Cuneiform::SignGud => '𒄞',
            Cuneiform::SignGudTimesAPlusKur => '𒄟',
            Cuneiform::SignGudTimesKur => '𒄠',
            Cuneiform::SignGudOverGudLugal => '𒄡',
            Cuneiform::SignGul => '𒄢',
            Cuneiform::SignGum => '𒄣',
            Cuneiform::SignGumTimesShe => '𒄤',
            Cuneiform::SignGur => '𒄥',
            Cuneiform::SignGur7 => '𒄦',
            Cuneiform::SignGurun => '𒄧',
            Cuneiform::SignGurush => '𒄨',
            Cuneiform::SignHa => '𒄩',
            Cuneiform::SignHaTenu => '𒄪',
            Cuneiform::SignHaGunu => '𒄫',
            Cuneiform::SignHal => '𒄬',
            Cuneiform::SignHi => '𒄭',
            Cuneiform::SignHiTimesAsh => '𒄮',
            Cuneiform::SignHiTimesAsh2 => '𒄯',
            Cuneiform::SignHiTimesBad => '𒄰',
            Cuneiform::SignHiTimesDish => '𒄱',
            Cuneiform::SignHiTimesGad => '𒄲',
            Cuneiform::SignHiTimesKin => '𒄳',
            Cuneiform::SignHiTimesNun => '𒄴',
            Cuneiform::SignHiTimesShe => '𒄵',
            Cuneiform::SignHiTimesU => '𒄶',
            Cuneiform::SignHu => '𒄷',
            Cuneiform::SignHub2 => '𒄸',
            Cuneiform::SignHub2TimesAn => '𒄹',
            Cuneiform::SignHub2TimesHal => '𒄺',
            Cuneiform::SignHub2TimesKaskal => '𒄻',
            Cuneiform::SignHub2TimesLish => '𒄼',
            Cuneiform::SignHub2TimesUd => '𒄽',
            Cuneiform::SignHul2 => '𒄾',
            Cuneiform::SignI => '𒄿',
            Cuneiform::SignIA => '𒅀',
            Cuneiform::SignIb => '𒅁',
            Cuneiform::SignIdim => '𒅂',
            Cuneiform::SignIdimOverIdimBur => '𒅃',
            Cuneiform::SignIdimOverIdimSquared => '𒅄',
            Cuneiform::SignIg => '𒅅',
            Cuneiform::SignIgi => '𒅆',
            Cuneiform::SignIgiDib => '𒅇',
            Cuneiform::SignIgiRi => '𒅈',
            Cuneiform::SignIgiOverIgiShirOverShirUdOverUd => '𒅉',
            Cuneiform::SignIgiGunu => '𒅊',
            Cuneiform::SignIl => '𒅋',
            Cuneiform::SignIlTimesGan2Tenu => '𒅌',
            Cuneiform::SignIl2 => '𒅍',
            Cuneiform::SignIm => '𒅎',
            Cuneiform::SignImTimesTak4 => '𒅏',
            Cuneiform::SignImCrossingIm => '𒅐',
            Cuneiform::SignImOpposingIm => '𒅑',
            Cuneiform::SignImSquared => '𒅒',
            Cuneiform::SignImin => '𒅓',
            Cuneiform::SignIn => '𒅔',
            Cuneiform::SignIr => '𒅕',
            Cuneiform::SignIsh => '𒅖',
            Cuneiform::SignKa => '𒅗',
            Cuneiform::SignKaTimesA => '𒅘',
            Cuneiform::SignKaTimesAd => '𒅙',
            Cuneiform::SignKaTimesAdPlusKu3 => '𒅚',
            Cuneiform::SignKaTimesAsh2 => '𒅛',
            Cuneiform::SignKaTimesBad => '𒅜',
            Cuneiform::SignKaTimesBalag => '𒅝',
            Cuneiform::SignKaTimesBar => '𒅞',
            Cuneiform::SignKaTimesBi => '𒅟',
            Cuneiform::SignKaTimesErin2 => '𒅠',
            Cuneiform::SignKaTimesEsh2 => '𒅡',
            Cuneiform::SignKaTimesGa => '𒅢',
            Cuneiform::SignKaTimesGal => '𒅣',
            Cuneiform::SignKaTimesGan2Tenu => '𒅤',
            Cuneiform::SignKaTimesGar => '𒅥',
            Cuneiform::SignKaTimesGarPlusSha3PlusA => '𒅦',
            Cuneiform::SignKaTimesGi => '𒅧',
            Cuneiform::SignKaTimesGir2 => '𒅨',
            Cuneiform::SignKaTimesGishPlusSar => '𒅩',
            Cuneiform::SignKaTimesGishCrossingGish => '𒅪',
            Cuneiform::SignKaTimesGu => '𒅫',
            Cuneiform::SignKaTimesGur7 => '𒅬',
            Cuneiform::SignKaTimesIgi => '𒅭',
            Cuneiform::SignKaTimesIm => '𒅮',
            Cuneiform::SignKaTimesKak => '𒅯',
            Cuneiform::SignKaTimesKi => '𒅰',
            Cuneiform::SignKaTimesKid => '𒅱',
            Cuneiform::SignKaTimesLi => '𒅲',
            Cuneiform::SignKaTimesLu => '𒅳',
            Cuneiform::SignKaTimesMe => '𒅴',
            Cuneiform::SignKaTimesMePlusDu => '𒅵',
            Cuneiform::SignKaTimesMePlusGi => '𒅶',
            Cuneiform::SignKaTimesMePlusTe => '𒅷',
            Cuneiform::SignKaTimesMi => '𒅸',
            Cuneiform::SignKaTimesMiPlusNunuz => '𒅹',
            Cuneiform::SignKaTimesNe => '𒅺',
            Cuneiform::SignKaTimesNun => '𒅻',
            Cuneiform::SignKaTimesPi => '𒅼',
            Cuneiform::SignKaTimesRu => '𒅽',
            Cuneiform::SignKaTimesSa => '𒅾',
            Cuneiform::SignKaTimesSar => '𒅿',
            Cuneiform::SignKaTimesSha => '𒆀',
            Cuneiform::SignKaTimesShe => '𒆁',
            Cuneiform::SignKaTimesShid => '𒆂',
            Cuneiform::SignKaTimesShu => '𒆃',
            Cuneiform::SignKaTimesSig => '𒆄',
            Cuneiform::SignKaTimesSuhur => '𒆅',
            Cuneiform::SignKaTimesTar => '𒆆',
            Cuneiform::SignKaTimesU => '𒆇',
            Cuneiform::SignKaTimesU2 => '𒆈',
            Cuneiform::SignKaTimesUd => '𒆉',
            Cuneiform::SignKaTimesUmumTimesPa => '𒆊',
            Cuneiform::SignKaTimesUsh => '𒆋',
            Cuneiform::SignKaTimesZi => '𒆌',
            Cuneiform::SignKa2 => '𒆍',
            Cuneiform::SignKa2CrossingKa2 => '𒆎',
            Cuneiform::SignKab => '𒆏',
            Cuneiform::SignKad2 => '𒆐',
            Cuneiform::SignKad3 => '𒆑',
            Cuneiform::SignKad4 => '𒆒',
            Cuneiform::SignKad5 => '𒆓',
            Cuneiform::SignKad5OverKad5 => '𒆔',
            Cuneiform::SignKak => '𒆕',
            Cuneiform::SignKakTimesIgiGunu => '𒆖',
            Cuneiform::SignKal => '𒆗',
            Cuneiform::SignKalTimesBad => '𒆘',
            Cuneiform::SignKalCrossingKal => '𒆙',
            Cuneiform::SignKam2 => '𒆚',
            Cuneiform::SignKam4 => '𒆛',
            Cuneiform::SignKaskal => '𒆜',
            Cuneiform::SignKaskalLagabTimesUOverLagabTimesU => '𒆝',
            Cuneiform::SignKaskalOverKaskalLagabTimesUOverLagabTimesU => '𒆞',
            Cuneiform::SignKesh2 => '𒆟',
            Cuneiform::SignKi => '𒆠',
            Cuneiform::SignKiTimesBad => '𒆡',
            Cuneiform::SignKiTimesU => '𒆢',
            Cuneiform::SignKiTimesUd => '𒆣',
            Cuneiform::SignKid => '𒆤',
            Cuneiform::SignKin => '𒆥',
            Cuneiform::SignKisal => '𒆦',
            Cuneiform::SignKish => '𒆧',
            Cuneiform::SignKisim5 => '𒆨',
            Cuneiform::SignKisim5OverKisim5 => '𒆩',
            Cuneiform::SignKu => '𒆪',
            Cuneiform::SignKuOverHiTimesAsh2KuOverHiTimesAsh2 => '𒆫',
            Cuneiform::SignKu3 => '𒆬',
            Cuneiform::SignKu4 => '𒆭',
            Cuneiform::SignKu4VariantForm => '𒆮',
            Cuneiform::SignKu7 => '𒆯',
            Cuneiform::SignKul => '𒆰',
            Cuneiform::SignKulGunu => '𒆱',
            Cuneiform::SignKun => '𒆲',
            Cuneiform::SignKur => '𒆳',
            Cuneiform::SignKurOpposingKur => '𒆴',
            Cuneiform::SignKushu2 => '𒆵',
            Cuneiform::SignKwu318 => '𒆶',
            Cuneiform::SignLa => '𒆷',
            Cuneiform::SignLagab => '𒆸',
            Cuneiform::SignLagabTimesA => '𒆹',
            Cuneiform::SignLagabTimesAPlusDaPlusHa => '𒆺',
            Cuneiform::SignLagabTimesAPlusGar => '𒆻',
            Cuneiform::SignLagabTimesAPlusLal => '𒆼',
            Cuneiform::SignLagabTimesAl => '𒆽',
            Cuneiform::SignLagabTimesAn => '𒆾',
            Cuneiform::SignLagabTimesAshZidaTenu => '𒆿',
            Cuneiform::SignLagabTimesBad => '𒇀',
            Cuneiform::SignLagabTimesBi => '𒇁',
            Cuneiform::SignLagabTimesDar => '𒇂',
            Cuneiform::SignLagabTimesEn => '𒇃',
            Cuneiform::SignLagabTimesGa => '𒇄',
            Cuneiform::SignLagabTimesGar => '𒇅',
            Cuneiform::SignLagabTimesGud => '𒇆',
            Cuneiform::SignLagabTimesGudPlusGud => '𒇇',
            Cuneiform::SignLagabTimesHa => '𒇈',
            Cuneiform::SignLagabTimesHal => '𒇉',
            Cuneiform::SignLagabTimesHiTimesNun => '𒇊',
            Cuneiform::SignLagabTimesIgiGunu => '𒇋',
            Cuneiform::SignLagabTimesIm => '𒇌',
            Cuneiform::SignLagabTimesImPlusHa => '𒇍',
            Cuneiform::SignLagabTimesImPlusLu => '𒇎',
            Cuneiform::SignLagabTimesKi => '𒇏',
            Cuneiform::SignLagabTimesKin => '𒇐',
            Cuneiform::SignLagabTimesKu3 => '𒇑',
            Cuneiform::SignLagabTimesKul => '𒇒',
            Cuneiform::SignLagabTimesKulPlusHiPlusA => '𒇓',
            Cuneiform::SignLagabTimesLagab => '𒇔',
            Cuneiform::SignLagabTimesLish => '𒇕',
            Cuneiform::SignLagabTimesLu => '𒇖',
            Cuneiform::SignLagabTimesLul => '𒇗',
            Cuneiform::SignLagabTimesMe => '𒇘',
            Cuneiform::SignLagabTimesMePlusEn => '𒇙',
            Cuneiform::SignLagabTimesMush => '𒇚',
            Cuneiform::SignLagabTimesNe => '𒇛',
            Cuneiform::SignLagabTimesShePlusSum => '𒇜',
            Cuneiform::SignLagabTimesShitaPlusGishPlusErin2 => '𒇝',
            Cuneiform::SignLagabTimesShitaPlusGishTenu => '𒇞',
            Cuneiform::SignLagabTimesShu2 => '𒇟',
            Cuneiform::SignLagabTimesShu2PlusShu2 => '𒇠',
            Cuneiform::SignLagabTimesSum => '𒇡',
            Cuneiform::SignLagabTimesTag => '𒇢',
            Cuneiform::SignLagabTimesTak4 => '𒇣',
            Cuneiform::SignLagabTimesTePlusAPlusSuPlusNa => '𒇤',
            Cuneiform::SignLagabTimesU => '𒇥',
            Cuneiform::SignLagabTimesUPlusA => '𒇦',
            Cuneiform::SignLagabTimesUPlusUPlusU => '𒇧',
            Cuneiform::SignLagabTimesU2PlusAsh => '𒇨',
            Cuneiform::SignLagabTimesUd => '𒇩',
            Cuneiform::SignLagabTimesUsh => '𒇪',
            Cuneiform::SignLagabSquared => '𒇫',
            Cuneiform::SignLagar => '𒇬',
            Cuneiform::SignLagarTimesShe => '𒇭',
            Cuneiform::SignLagarTimesShePlusSum => '𒇮',
            Cuneiform::SignLagarGunu => '𒇯',
            Cuneiform::SignLagarGunuOverLagarGunuShe => '𒇰',
            Cuneiform::SignLahshu => '𒇱',
            Cuneiform::SignLal => '𒇲',
            Cuneiform::SignLalTimesLal => '𒇳',
            Cuneiform::SignLam => '𒇴',
            Cuneiform::SignLamTimesKur => '𒇵',
            Cuneiform::SignLamTimesKurPlusRu => '𒇶',
            Cuneiform::SignLi => '𒇷',
            Cuneiform::SignLil => '𒇸',
            Cuneiform::SignLimmu2 => '𒇹',
            Cuneiform::SignLish => '𒇺',
            Cuneiform::SignLu => '𒇻',
            Cuneiform::SignLuTimesBad => '𒇼',
            Cuneiform::SignLu2 => '𒇽',
            Cuneiform::SignLu2TimesAl => '𒇾',
            Cuneiform::SignLu2TimesBad => '𒇿',
            Cuneiform::SignLu2TimesEsh2 => '𒈀',
            Cuneiform::SignLu2TimesEsh2Tenu => '𒈁',
            Cuneiform::SignLu2TimesGan2Tenu => '𒈂',
            Cuneiform::SignLu2TimesHiTimesBad => '𒈃',
            Cuneiform::SignLu2TimesIm => '𒈄',
            Cuneiform::SignLu2TimesKad2 => '𒈅',
            Cuneiform::SignLu2TimesKad3 => '𒈆',
            Cuneiform::SignLu2TimesKad3PlusAsh => '𒈇',
            Cuneiform::SignLu2TimesKi => '𒈈',
            Cuneiform::SignLu2TimesLaPlusAsh => '𒈉',
            Cuneiform::SignLu2TimesLagab => '𒈊',
            Cuneiform::SignLu2TimesMePlusEn => '𒈋',
            Cuneiform::SignLu2TimesNe => '𒈌',
            Cuneiform::SignLu2TimesNu => '𒈍',
            Cuneiform::SignLu2TimesSiPlusAsh => '𒈎',
            Cuneiform::SignLu2TimesSik2PlusBu => '𒈏',
            Cuneiform::SignLu2TimesTug2 => '𒈐',
            Cuneiform::SignLu2Tenu => '𒈑',
            Cuneiform::SignLu2CrossingLu2 => '𒈒',
            Cuneiform::SignLu2OpposingLu2 => '𒈓',
            Cuneiform::SignLu2Squared => '𒈔',
            Cuneiform::SignLu2Sheshig => '𒈕',
            Cuneiform::SignLu3 => '𒈖',
            Cuneiform::SignLugal => '𒈗',
            Cuneiform::SignLugalOverLugal => '𒈘',
            Cuneiform::SignLugalOpposingLugal => '𒈙',
            Cuneiform::SignLugalSheshig => '𒈚',
            Cuneiform::SignLuh => '𒈛',
            Cuneiform::SignLul => '𒈜',
            Cuneiform::SignLum => '𒈝',
            Cuneiform::SignLumOverLum => '𒈞',
            Cuneiform::SignLumOverLumGarOverGar => '𒈟',
            Cuneiform::SignMa => '𒈠',
            Cuneiform::SignMaTimesTak4 => '𒈡',
            Cuneiform::SignMaGunu => '𒈢',
            Cuneiform::SignMa2 => '𒈣',
            Cuneiform::SignMah => '𒈤',
            Cuneiform::SignMar => '𒈥',
            Cuneiform::SignMash => '𒈦',
            Cuneiform::SignMash2 => '𒈧',
            Cuneiform::SignMe => '𒈨',
            Cuneiform::SignMes => '𒈩',
            Cuneiform::SignMi => '𒈪',
            Cuneiform::SignMin => '𒈫',
            Cuneiform::SignMu => '𒈬',
            Cuneiform::SignMuOverMu => '𒈭',
            Cuneiform::SignMug => '𒈮',
            Cuneiform::SignMugGunu => '𒈯',
            Cuneiform::SignMunsub => '𒈰',
            Cuneiform::SignMurgu2 => '𒈱',
            Cuneiform::SignMush => '𒈲',
            Cuneiform::SignMushTimesA => '𒈳',
            Cuneiform::SignMushTimesKur => '𒈴',
            Cuneiform::SignMushTimesZa => '𒈵',
            Cuneiform::SignMushOverMush => '𒈶',
            Cuneiform::SignMushOverMushTimesAPlusNa => '𒈷',
            Cuneiform::SignMushCrossingMush => '𒈸',
            Cuneiform::SignMush3 => '𒈹',
            Cuneiform::SignMush3TimesA => '𒈺',
            Cuneiform::SignMush3TimesAPlusDi => '𒈻',
            Cuneiform::SignMush3TimesDi => '𒈼',
            Cuneiform::SignMush3Gunu => '𒈽',
            Cuneiform::SignNa => '𒈾',
            Cuneiform::SignNa2 => '𒈿',
            Cuneiform::SignNaga => '𒉀',
            Cuneiform::SignNagaInverted => '𒉁',
            Cuneiform::SignNagaTimesShuTenu => '𒉂',
            Cuneiform::SignNagaOpposingNaga => '𒉃',
            Cuneiform::SignNagar => '𒉄',
            Cuneiform::SignNamNutillu => '𒉅',
            Cuneiform::SignNam => '𒉆',
            Cuneiform::SignNam2 => '𒉇',
            Cuneiform::SignNe => '𒉈',
            Cuneiform::SignNeTimesA => '𒉉',
            Cuneiform::SignNeTimesUd => '𒉊',
            Cuneiform::SignNeSheshig => '𒉋',
            Cuneiform::SignNi => '𒉌',
            Cuneiform::SignNiTimesE => '𒉍',
            Cuneiform::SignNi2 => '𒉎',
            Cuneiform::SignNim => '𒉏',
            Cuneiform::SignNimTimesGan2Tenu => '𒉐',
            Cuneiform::SignNimTimesGarPlusGan2Tenu => '𒉑',
            Cuneiform::SignNinda2 => '𒉒',
            Cuneiform::SignNinda2TimesAn => '𒉓',
            Cuneiform::SignNinda2TimesAsh => '𒉔',
            Cuneiform::SignNinda2TimesAshPlusAsh => '𒉕',
            Cuneiform::SignNinda2TimesGud => '𒉖',
            Cuneiform::SignNinda2TimesMePlusGan2Tenu => '𒉗',
            Cuneiform::SignNinda2TimesNe => '𒉘',
            Cuneiform::SignNinda2TimesNun => '𒉙',
            Cuneiform::SignNinda2TimesShe => '𒉚',
            Cuneiform::SignNinda2TimesShePlusAAn => '𒉛',
            Cuneiform::SignNinda2TimesShePlusAsh => '𒉜',
            Cuneiform::SignNinda2TimesShePlusAshPlusAsh => '𒉝',
            Cuneiform::SignNinda2TimesU2PlusAsh => '𒉞',
            Cuneiform::SignNinda2TimesUsh => '𒉟',
            Cuneiform::SignNisag => '𒉠',
            Cuneiform::SignNu => '𒉡',
            Cuneiform::SignNu11 => '𒉢',
            Cuneiform::SignNun => '𒉣',
            Cuneiform::SignNunLagarTimesGar => '𒉤',
            Cuneiform::SignNunLagarTimesMash => '𒉥',
            Cuneiform::SignNunLagarTimesSal => '𒉦',
            Cuneiform::SignNunLagarTimesSalOverNunLagarTimesSal => '𒉧',
            Cuneiform::SignNunLagarTimesUsh => '𒉨',
            Cuneiform::SignNunTenu => '𒉩',
            Cuneiform::SignNunOverNun => '𒉪',
            Cuneiform::SignNunCrossingNun => '𒉫',
            Cuneiform::SignNunCrossingNunLagarOverLagar => '𒉬',
            Cuneiform::SignNunuz => '𒉭',
            Cuneiform::SignNunuzAb2TimesAshgab => '𒉮',
            Cuneiform::SignNunuzAb2TimesBi => '𒉯',
            Cuneiform::SignNunuzAb2TimesDug => '𒉰',
            Cuneiform::SignNunuzAb2TimesGud => '𒉱',
            Cuneiform::SignNunuzAb2TimesIgiGunu => '𒉲',
            Cuneiform::SignNunuzAb2TimesKad3 => '𒉳',
            Cuneiform::SignNunuzAb2TimesLa => '𒉴',
            Cuneiform::SignNunuzAb2TimesNe => '𒉵',
            Cuneiform::SignNunuzAb2TimesSila3 => '𒉶',
            Cuneiform::SignNunuzAb2TimesU2 => '𒉷',
            Cuneiform::SignNunuzKisim5TimesBi => '𒉸',
            Cuneiform::SignNunuzKisim5TimesBiU => '𒉹',
            Cuneiform::SignPa => '𒉺',
            Cuneiform::SignPad => '𒉻',
            Cuneiform::SignPan => '𒉼',
            Cuneiform::SignPap => '𒉽',
            Cuneiform::SignPesh2 => '𒉾',
            Cuneiform::SignPi => '𒉿',
            Cuneiform::SignPiTimesA => '𒊀',
            Cuneiform::SignPiTimesAb => '𒊁',
            Cuneiform::SignPiTimesBi => '𒊂',
            Cuneiform::SignPiTimesBu => '𒊃',
            Cuneiform::SignPiTimesE => '𒊄',
            Cuneiform::SignPiTimesI => '𒊅',
            Cuneiform::SignPiTimesIb => '𒊆',
            Cuneiform::SignPiTimesU => '𒊇',
            Cuneiform::SignPiTimesU2 => '𒊈',
            Cuneiform::SignPiCrossingPi => '𒊉',
            Cuneiform::SignPirig => '𒊊',
            Cuneiform::SignPirigTimesKal => '𒊋',
            Cuneiform::SignPirigTimesUd => '𒊌',
            Cuneiform::SignPirigTimesZa => '𒊍',
            Cuneiform::SignPirigOpposingPirig => '𒊎',
            Cuneiform::SignRa => '𒊏',
            Cuneiform::SignRab => '𒊐',
            Cuneiform::SignRi => '𒊑',
            Cuneiform::SignRu => '𒊒',
            Cuneiform::SignSa => '𒊓',
            Cuneiform::SignSagNutillu => '𒊔',
            Cuneiform::SignSag => '𒊕',
            Cuneiform::SignSagTimesA => '𒊖',
            Cuneiform::SignSagTimesDu => '𒊗',
            Cuneiform::SignSagTimesDub => '𒊘',
            Cuneiform::SignSagTimesHa => '𒊙',
            Cuneiform::SignSagTimesKak => '𒊚',
            Cuneiform::SignSagTimesKur => '𒊛',
            Cuneiform::SignSagTimesLum => '𒊜',
            Cuneiform::SignSagTimesMi => '𒊝',
            Cuneiform::SignSagTimesNun => '𒊞',
            Cuneiform::SignSagTimesSal => '𒊟',
            Cuneiform::SignSagTimesShid => '𒊠',
            Cuneiform::SignSagTimesTab => '𒊡',
            Cuneiform::SignSagTimesU2 => '𒊢',
            Cuneiform::SignSagTimesUb => '𒊣',
            Cuneiform::SignSagTimesUm => '𒊤',
            Cuneiform::SignSagTimesUr => '𒊥',
            Cuneiform::SignSagTimesUsh => '𒊦',
            Cuneiform::SignSagOverSag => '𒊧',
            Cuneiform::SignSagGunu => '𒊨',
            Cuneiform::SignSal => '𒊩',
            Cuneiform::SignSalLagabTimesAsh2 => '𒊪',
            Cuneiform::SignSanga2 => '𒊫',
            Cuneiform::SignSar => '𒊬',
            Cuneiform::SignSha => '𒊭',
            Cuneiform::SignSha3 => '𒊮',
            Cuneiform::SignSha3TimesA => '𒊯',
            Cuneiform::SignSha3TimesBad => '𒊰',
            Cuneiform::SignSha3TimesGish => '𒊱',
            Cuneiform::SignSha3TimesNe => '𒊲',
            Cuneiform::SignSha3TimesShu2 => '𒊳',
            Cuneiform::SignSha3TimesTur => '𒊴',
            Cuneiform::SignSha3TimesU => '𒊵',
            Cuneiform::SignSha3TimesUPlusA => '𒊶',
            Cuneiform::SignSha6 => '𒊷',
            Cuneiform::SignShab6 => '𒊸',
            Cuneiform::SignShar2 => '𒊹',
            Cuneiform::SignShe => '𒊺',
            Cuneiform::SignSheHu => '𒊻',
            Cuneiform::SignSheOverSheGadOverGadGarOverGar => '𒊼',
            Cuneiform::SignSheOverSheTabOverTabGarOverGar => '𒊽',
            Cuneiform::SignSheg9 => '𒊾',
            Cuneiform::SignShen => '𒊿',
            Cuneiform::SignShesh => '𒋀',
            Cuneiform::SignShesh2 => '𒋁',
            Cuneiform::SignSheshlam => '𒋂',
            Cuneiform::SignShid => '𒋃',
            Cuneiform::SignShidTimesA => '𒋄',
            Cuneiform::SignShidTimesIm => '𒋅',
            Cuneiform::SignShim => '𒋆',
            Cuneiform::SignShimTimesA => '𒋇',
            Cuneiform::SignShimTimesBal => '𒋈',
            Cuneiform::SignShimTimesBulug => '𒋉',
            Cuneiform::SignShimTimesDin => '𒋊',
            Cuneiform::SignShimTimesGar => '𒋋',
            Cuneiform::SignShimTimesIgi => '𒋌',
            Cuneiform::SignShimTimesIgiGunu => '𒋍',
            Cuneiform::SignShimTimesKushu2 => '𒋎',
            Cuneiform::SignShimTimesLul => '𒋏',
            Cuneiform::SignShimTimesMug => '𒋐',
            Cuneiform::SignShimTimesSal => '𒋑',
            Cuneiform::SignShinig => '𒋒',
            Cuneiform::SignShir => '𒋓',
            Cuneiform::SignShirTenu => '𒋔',
            Cuneiform::SignShirOverShirBurOverBur => '𒋕',
            Cuneiform::SignShita => '𒋖',
            Cuneiform::SignShu => '𒋗',
            Cuneiform::SignShuOverInvertedShu => '𒋘',
            Cuneiform::SignShu2 => '𒋙',
            Cuneiform::SignShubur => '𒋚',
            Cuneiform::SignSi => '𒋛',
            Cuneiform::SignSiGunu => '𒋜',
            Cuneiform::SignSig => '𒋝',
            Cuneiform::SignSig4 => '𒋞',
            Cuneiform::SignSig4OverSig4Shu2 => '𒋟',
            Cuneiform::SignSik2 => '𒋠',
            Cuneiform::SignSila3 => '𒋡',
            Cuneiform::SignSu => '𒋢',
            Cuneiform::SignSuOverSu => '𒋣',
            Cuneiform::SignSud => '𒋤',
            Cuneiform::SignSud2 => '𒋥',
            Cuneiform::SignSuhur => '𒋦',
            Cuneiform::SignSum => '𒋧',
            Cuneiform::SignSumash => '𒋨',
            Cuneiform::SignSur => '𒋩',
            Cuneiform::SignSur9 => '𒋪',
            Cuneiform::SignTa => '𒋫',
            Cuneiform::SignTaAsterisk => '𒋬',
            Cuneiform::SignTaTimesHi => '𒋭',
            Cuneiform::SignTaTimesMi => '𒋮',
            Cuneiform::SignTaGunu => '𒋯',
            Cuneiform::SignTab => '𒋰',
            Cuneiform::SignTabOverTabNiOverNiDishOverDish => '𒋱',
            Cuneiform::SignTabSquared => '𒋲',
            Cuneiform::SignTag => '𒋳',
            Cuneiform::SignTagTimesBi => '𒋴',
            Cuneiform::SignTagTimesGud => '𒋵',
            Cuneiform::SignTagTimesShe => '𒋶',
            Cuneiform::SignTagTimesShu => '𒋷',
            Cuneiform::SignTagTimesTug2 => '𒋸',
            Cuneiform::SignTagTimesUd => '𒋹',
            Cuneiform::SignTak4 => '𒋺',
            Cuneiform::SignTar => '𒋻',
            Cuneiform::SignTe => '𒋼',
            Cuneiform::SignTeGunu => '𒋽',
            Cuneiform::SignTi => '𒋾',
            Cuneiform::SignTiTenu => '𒋿',
            Cuneiform::SignTil => '𒌀',
            Cuneiform::SignTir => '𒌁',
            Cuneiform::SignTirTimesTak4 => '𒌂',
            Cuneiform::SignTirOverTir => '𒌃',
            Cuneiform::SignTirOverTirGadOverGadGarOverGar => '𒌄',
            Cuneiform::SignTu => '𒌅',
            Cuneiform::SignTug2 => '𒌆',
            Cuneiform::SignTuk => '𒌇',
            Cuneiform::SignTum => '𒌈',
            Cuneiform::SignTur => '𒌉',
            Cuneiform::SignTurOverTurZaOverZa => '𒌊',
            Cuneiform::SignU => '𒌋',
            Cuneiform::SignUGud => '𒌌',
            Cuneiform::SignUUU => '𒌍',
            Cuneiform::SignUOverUPaOverPaGarOverGar => '𒌎',
            Cuneiform::SignUOverUSurOverSur => '𒌏',
            Cuneiform::SignUOverUUReversedOverUReversed => '𒌐',
            Cuneiform::SignU2 => '𒌑',
            Cuneiform::SignUb => '𒌒',
            Cuneiform::SignUd => '𒌓',
            Cuneiform::SignUdKushu2 => '𒌔',
            Cuneiform::SignUdTimesBad => '𒌕',
            Cuneiform::SignUdTimesMi => '𒌖',
            Cuneiform::SignUdTimesUPlusUPlusU => '𒌗',
            Cuneiform::SignUdTimesUPlusUPlusUGunu => '𒌘',
            Cuneiform::SignUdGunu => '𒌙',
            Cuneiform::SignUdSheshig => '𒌚',
            Cuneiform::SignUdSheshigTimesBad => '𒌛',
            Cuneiform::SignUdug => '𒌜',
            Cuneiform::SignUm => '𒌝',
            Cuneiform::SignUmTimesLagab => '𒌞',
            Cuneiform::SignUmTimesMePlusDa => '𒌟',
            Cuneiform::SignUmTimesSha3 => '𒌠',
            Cuneiform::SignUmTimesU => '𒌡',
            Cuneiform::SignUmbin => '𒌢',
            Cuneiform::SignUmum => '𒌣',
            Cuneiform::SignUmumTimesKaskal => '𒌤',
            Cuneiform::SignUmumTimesPa => '𒌥',
            Cuneiform::SignUn => '𒌦',
            Cuneiform::SignUnGunu => '𒌧',
            Cuneiform::SignUr => '𒌨',
            Cuneiform::SignUrCrossingUr => '𒌩',
            Cuneiform::SignUrSheshig => '𒌪',
            Cuneiform::SignUr2 => '𒌫',
            Cuneiform::SignUr2TimesAPlusHa => '𒌬',
            Cuneiform::SignUr2TimesAPlusNa => '𒌭',
            Cuneiform::SignUr2TimesAl => '𒌮',
            Cuneiform::SignUr2TimesHa => '𒌯',
            Cuneiform::SignUr2TimesNun => '𒌰',
            Cuneiform::SignUr2TimesU2 => '𒌱',
            Cuneiform::SignUr2TimesU2PlusAsh => '𒌲',
            Cuneiform::SignUr2TimesU2PlusBi => '𒌳',
            Cuneiform::SignUr4 => '𒌴',
            Cuneiform::SignUri => '𒌵',
            Cuneiform::SignUri3 => '𒌶',
            Cuneiform::SignUru => '𒌷',
            Cuneiform::SignUruTimesA => '𒌸',
            Cuneiform::SignUruTimesAshgab => '𒌹',
            Cuneiform::SignUruTimesBar => '𒌺',
            Cuneiform::SignUruTimesDun => '𒌻',
            Cuneiform::SignUruTimesGa => '𒌼',
            Cuneiform::SignUruTimesGal => '𒌽',
            Cuneiform::SignUruTimesGan2Tenu => '𒌾',
            Cuneiform::SignUruTimesGar => '𒌿',
            Cuneiform::SignUruTimesGu => '𒍀',
            Cuneiform::SignUruTimesHa => '𒍁',
            Cuneiform::SignUruTimesIgi => '𒍂',
            Cuneiform::SignUruTimesIm => '𒍃',
            Cuneiform::SignUruTimesIsh => '𒍄',
            Cuneiform::SignUruTimesKi => '𒍅',
            Cuneiform::SignUruTimesLum => '𒍆',
            Cuneiform::SignUruTimesMin => '𒍇',
            Cuneiform::SignUruTimesPa => '𒍈',
            Cuneiform::SignUruTimesShe => '𒍉',
            Cuneiform::SignUruTimesSig4 => '𒍊',
            Cuneiform::SignUruTimesTu => '𒍋',
            Cuneiform::SignUruTimesUPlusGud => '𒍌',
            Cuneiform::SignUruTimesUd => '𒍍',
            Cuneiform::SignUruTimesUruda => '𒍎',
            Cuneiform::SignUruda => '𒍏',
            Cuneiform::SignUrudaTimesU => '𒍐',
            Cuneiform::SignUsh => '𒍑',
            Cuneiform::SignUshTimesA => '𒍒',
            Cuneiform::SignUshTimesKu => '𒍓',
            Cuneiform::SignUshTimesKur => '𒍔',
            Cuneiform::SignUshTimesTak4 => '𒍕',
            Cuneiform::SignUshx => '𒍖',
            Cuneiform::SignUsh2 => '𒍗',
            Cuneiform::SignUshumx => '𒍘',
            Cuneiform::SignUtuki => '𒍙',
            Cuneiform::SignUz3 => '𒍚',
            Cuneiform::SignUz3TimesKaskal => '𒍛',
            Cuneiform::SignUzu => '𒍜',
            Cuneiform::SignZa => '𒍝',
            Cuneiform::SignZaTenu => '𒍞',
            Cuneiform::SignZaSquaredTimesKur => '𒍟',
            Cuneiform::SignZag => '𒍠',
            Cuneiform::SignZamx => '𒍡',
            Cuneiform::SignZe2 => '𒍢',
            Cuneiform::SignZi => '𒍣',
            Cuneiform::SignZiOverZi => '𒍤',
            Cuneiform::SignZi3 => '𒍥',
            Cuneiform::SignZib => '𒍦',
            Cuneiform::SignZibKabaTenu => '𒍧',
            Cuneiform::SignZig => '𒍨',
            Cuneiform::SignZiz2 => '𒍩',
            Cuneiform::SignZu => '𒍪',
            Cuneiform::SignZu5 => '𒍫',
            Cuneiform::SignZu5TimesA => '𒍬',
            Cuneiform::SignZubur => '𒍭',
            Cuneiform::SignZum => '𒍮',
            Cuneiform::SignKapElamite => '𒍯',
            Cuneiform::SignAbTimesNun => '𒍰',
            Cuneiform::SignAb2TimesA => '𒍱',
            Cuneiform::SignAmarTimesKug => '𒍲',
            Cuneiform::SignDagKisim5TimesU2PlusMash => '𒍳',
            Cuneiform::SignDag3 => '𒍴',
            Cuneiform::SignDishPlusShu => '𒍵',
            Cuneiform::SignDubTimesShe => '𒍶',
            Cuneiform::SignEzenTimesGud => '𒍷',
            Cuneiform::SignEzenTimesShe => '𒍸',
            Cuneiform::SignGa2TimesAnPlusKakPlusA => '𒍹',
            Cuneiform::SignGa2TimesAsh2 => '𒍺',
            Cuneiform::SignGe22 => '𒍻',
            Cuneiform::SignGig => '𒍼',
            Cuneiform::SignHush => '𒍽',
            Cuneiform::SignKaTimesAnshe => '𒍾',
            Cuneiform::SignKaTimesAsh3 => '𒍿',
            Cuneiform::SignKaTimesGish => '𒎀',
            Cuneiform::SignKaTimesGud => '𒎁',
            Cuneiform::SignKaTimesHiTimesAsh2 => '𒎂',
            Cuneiform::SignKaTimesLum => '𒎃',
            Cuneiform::SignKaTimesPa => '𒎄',
            Cuneiform::SignKaTimesShul => '𒎅',
            Cuneiform::SignKaTimesTu => '𒎆',
            Cuneiform::SignKaTimesUr2 => '𒎇',
            Cuneiform::SignLagabTimesGi => '𒎈',
            Cuneiform::SignLu2SheshigTimesBad => '𒎉',
            Cuneiform::SignLu2TimesEsh2PlusLal => '𒎊',
            Cuneiform::SignLu2TimesShu => '𒎋',
            Cuneiform::SignMesh => '𒎌',
            Cuneiform::SignMush3TimesZa => '𒎍',
            Cuneiform::SignNa4 => '𒎎',
            Cuneiform::SignNin => '𒎏',
            Cuneiform::SignNin9 => '𒎐',
            Cuneiform::SignNinda2TimesBal => '𒎑',
            Cuneiform::SignNinda2TimesGi => '𒎒',
            Cuneiform::SignNu11RotatedNinetyDegrees => '𒎓',
            Cuneiform::SignPesh2Asterisk => '𒎔',
            Cuneiform::SignPir2 => '𒎕',
            Cuneiform::SignSagTimesIgiGunu => '𒎖',
            Cuneiform::SignTi2 => '𒎗',
            Cuneiform::SignUmTimesMe => '𒎘',
            Cuneiform::SignUU => '𒎙',
        }
    }
}

impl std::convert::TryFrom<char> for Cuneiform {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𒀀' => Ok(Cuneiform::SignA),
            '𒀁' => Ok(Cuneiform::SignATimesA),
            '𒀂' => Ok(Cuneiform::SignATimesBad),
            '𒀃' => Ok(Cuneiform::SignATimesGan2Tenu),
            '𒀄' => Ok(Cuneiform::SignATimesHa),
            '𒀅' => Ok(Cuneiform::SignATimesIgi),
            '𒀆' => Ok(Cuneiform::SignATimesLagarGunu),
            '𒀇' => Ok(Cuneiform::SignATimesMush),
            '𒀈' => Ok(Cuneiform::SignATimesSag),
            '𒀉' => Ok(Cuneiform::SignA2),
            '𒀊' => Ok(Cuneiform::SignAb),
            '𒀋' => Ok(Cuneiform::SignAbTimesAsh2),
            '𒀌' => Ok(Cuneiform::SignAbTimesDun3Gunu),
            '𒀍' => Ok(Cuneiform::SignAbTimesGal),
            '𒀎' => Ok(Cuneiform::SignAbTimesGan2Tenu),
            '𒀏' => Ok(Cuneiform::SignAbTimesHa),
            '𒀐' => Ok(Cuneiform::SignAbTimesIgiGunu),
            '𒀑' => Ok(Cuneiform::SignAbTimesImin),
            '𒀒' => Ok(Cuneiform::SignAbTimesLagab),
            '𒀓' => Ok(Cuneiform::SignAbTimesShesh),
            '𒀔' => Ok(Cuneiform::SignAbTimesUPlusUPlusU),
            '𒀕' => Ok(Cuneiform::SignAbGunu),
            '𒀖' => Ok(Cuneiform::SignAb2),
            '𒀗' => Ok(Cuneiform::SignAb2TimesBalag),
            '𒀘' => Ok(Cuneiform::SignAb2TimesGan2Tenu),
            '𒀙' => Ok(Cuneiform::SignAb2TimesMePlusEn),
            '𒀚' => Ok(Cuneiform::SignAb2TimesSha3),
            '𒀛' => Ok(Cuneiform::SignAb2TimesTak4),
            '𒀜' => Ok(Cuneiform::SignAd),
            '𒀝' => Ok(Cuneiform::SignAk),
            '𒀞' => Ok(Cuneiform::SignAkTimesErin2),
            '𒀟' => Ok(Cuneiform::SignAkTimesShitaPlusGish),
            '𒀠' => Ok(Cuneiform::SignAl),
            '𒀡' => Ok(Cuneiform::SignAlTimesAl),
            '𒀢' => Ok(Cuneiform::SignAlTimesDim2),
            '𒀣' => Ok(Cuneiform::SignAlTimesGish),
            '𒀤' => Ok(Cuneiform::SignAlTimesHa),
            '𒀥' => Ok(Cuneiform::SignAlTimesKad3),
            '𒀦' => Ok(Cuneiform::SignAlTimesKi),
            '𒀧' => Ok(Cuneiform::SignAlTimesShe),
            '𒀨' => Ok(Cuneiform::SignAlTimesUsh),
            '𒀩' => Ok(Cuneiform::SignAlan),
            '𒀪' => Ok(Cuneiform::SignAleph),
            '𒀫' => Ok(Cuneiform::SignAmar),
            '𒀬' => Ok(Cuneiform::SignAmarTimesShe),
            '𒀭' => Ok(Cuneiform::SignAn),
            '𒀮' => Ok(Cuneiform::SignAnOverAn),
            '𒀯' => Ok(Cuneiform::SignAnThreeTimes),
            '𒀰' => Ok(Cuneiform::SignAnPlusNagaOpposingAnPlusNaga),
            '𒀱' => Ok(Cuneiform::SignAnPlusNagaSquared),
            '𒀲' => Ok(Cuneiform::SignAnshe),
            '𒀳' => Ok(Cuneiform::SignApin),
            '𒀴' => Ok(Cuneiform::SignArad),
            '𒀵' => Ok(Cuneiform::SignAradTimesKur),
            '𒀶' => Ok(Cuneiform::SignArkab),
            '𒀷' => Ok(Cuneiform::SignAsal2),
            '𒀸' => Ok(Cuneiform::SignAsh),
            '𒀹' => Ok(Cuneiform::SignAshZidaTenu),
            '𒀺' => Ok(Cuneiform::SignAshKabaTenu),
            '𒀻' => Ok(Cuneiform::SignAshOverAshTug2OverTug2Tug2OverTug2Pap),
            '𒀼' => Ok(Cuneiform::SignAshOverAshOverAsh),
            '𒀽' => Ok(Cuneiform::SignAshOverAshOverAshCrossingAshOverAshOverAsh),
            '𒀾' => Ok(Cuneiform::SignAsh2),
            '𒀿' => Ok(Cuneiform::SignAshgab),
            '𒁀' => Ok(Cuneiform::SignBa),
            '𒁁' => Ok(Cuneiform::SignBad),
            '𒁂' => Ok(Cuneiform::SignBag3),
            '𒁃' => Ok(Cuneiform::SignBahar2),
            '𒁄' => Ok(Cuneiform::SignBal),
            '𒁅' => Ok(Cuneiform::SignBalOverBal),
            '𒁆' => Ok(Cuneiform::SignBalag),
            '𒁇' => Ok(Cuneiform::SignBar),
            '𒁈' => Ok(Cuneiform::SignBara2),
            '𒁉' => Ok(Cuneiform::SignBi),
            '𒁊' => Ok(Cuneiform::SignBiTimesA),
            '𒁋' => Ok(Cuneiform::SignBiTimesGar),
            '𒁌' => Ok(Cuneiform::SignBiTimesIgiGunu),
            '𒁍' => Ok(Cuneiform::SignBu),
            '𒁎' => Ok(Cuneiform::SignBuOverBuAb),
            '𒁏' => Ok(Cuneiform::SignBuOverBuUn),
            '𒁐' => Ok(Cuneiform::SignBuCrossingBu),
            '𒁑' => Ok(Cuneiform::SignBulug),
            '𒁒' => Ok(Cuneiform::SignBulugOverBulug),
            '𒁓' => Ok(Cuneiform::SignBur),
            '𒁔' => Ok(Cuneiform::SignBur2),
            '𒁕' => Ok(Cuneiform::SignDa),
            '𒁖' => Ok(Cuneiform::SignDag),
            '𒁗' => Ok(Cuneiform::SignDagKisim5TimesAPlusMash),
            '𒁘' => Ok(Cuneiform::SignDagKisim5TimesAmar),
            '𒁙' => Ok(Cuneiform::SignDagKisim5TimesBalag),
            '𒁚' => Ok(Cuneiform::SignDagKisim5TimesBi),
            '𒁛' => Ok(Cuneiform::SignDagKisim5TimesGa),
            '𒁜' => Ok(Cuneiform::SignDagKisim5TimesGaPlusMash),
            '𒁝' => Ok(Cuneiform::SignDagKisim5TimesGi),
            '𒁞' => Ok(Cuneiform::SignDagKisim5TimesGir2),
            '𒁟' => Ok(Cuneiform::SignDagKisim5TimesGud),
            '𒁠' => Ok(Cuneiform::SignDagKisim5TimesHa),
            '𒁡' => Ok(Cuneiform::SignDagKisim5TimesIr),
            '𒁢' => Ok(Cuneiform::SignDagKisim5TimesIrPlusLu),
            '𒁣' => Ok(Cuneiform::SignDagKisim5TimesKak),
            '𒁤' => Ok(Cuneiform::SignDagKisim5TimesLa),
            '𒁥' => Ok(Cuneiform::SignDagKisim5TimesLu),
            '𒁦' => Ok(Cuneiform::SignDagKisim5TimesLuPlusMash2),
            '𒁧' => Ok(Cuneiform::SignDagKisim5TimesLum),
            '𒁨' => Ok(Cuneiform::SignDagKisim5TimesNe),
            '𒁩' => Ok(Cuneiform::SignDagKisim5TimesPapPlusPap),
            '𒁪' => Ok(Cuneiform::SignDagKisim5TimesSi),
            '𒁫' => Ok(Cuneiform::SignDagKisim5TimesTak4),
            '𒁬' => Ok(Cuneiform::SignDagKisim5TimesU2PlusGir2),
            '𒁭' => Ok(Cuneiform::SignDagKisim5TimesUsh),
            '𒁮' => Ok(Cuneiform::SignDam),
            '𒁯' => Ok(Cuneiform::SignDar),
            '𒁰' => Ok(Cuneiform::SignDara3),
            '𒁱' => Ok(Cuneiform::SignDara4),
            '𒁲' => Ok(Cuneiform::SignDi),
            '𒁳' => Ok(Cuneiform::SignDib),
            '𒁴' => Ok(Cuneiform::SignDim),
            '𒁵' => Ok(Cuneiform::SignDimTimesShe),
            '𒁶' => Ok(Cuneiform::SignDim2),
            '𒁷' => Ok(Cuneiform::SignDin),
            '𒁸' => Ok(Cuneiform::SignDinKaskalUGunuDish),
            '𒁹' => Ok(Cuneiform::SignDish),
            '𒁺' => Ok(Cuneiform::SignDu),
            '𒁻' => Ok(Cuneiform::SignDuOverDu),
            '𒁼' => Ok(Cuneiform::SignDuGunu),
            '𒁽' => Ok(Cuneiform::SignDuSheshig),
            '𒁾' => Ok(Cuneiform::SignDub),
            '𒁿' => Ok(Cuneiform::SignDubTimesEsh2),
            '𒂀' => Ok(Cuneiform::SignDub2),
            '𒂁' => Ok(Cuneiform::SignDug),
            '𒂂' => Ok(Cuneiform::SignDugud),
            '𒂃' => Ok(Cuneiform::SignDuh),
            '𒂄' => Ok(Cuneiform::SignDun),
            '𒂅' => Ok(Cuneiform::SignDun3),
            '𒂆' => Ok(Cuneiform::SignDun3Gunu),
            '𒂇' => Ok(Cuneiform::SignDun3GunuGunu),
            '𒂈' => Ok(Cuneiform::SignDun4),
            '𒂉' => Ok(Cuneiform::SignDur2),
            '𒂊' => Ok(Cuneiform::SignE),
            '𒂋' => Ok(Cuneiform::SignETimesPap),
            '𒂌' => Ok(Cuneiform::SignEOverENunOverNun),
            '𒂍' => Ok(Cuneiform::SignE2),
            '𒂎' => Ok(Cuneiform::SignE2TimesAPlusHaPlusDa),
            '𒂏' => Ok(Cuneiform::SignE2TimesGar),
            '𒂐' => Ok(Cuneiform::SignE2TimesMi),
            '𒂑' => Ok(Cuneiform::SignE2TimesSal),
            '𒂒' => Ok(Cuneiform::SignE2TimesShe),
            '𒂓' => Ok(Cuneiform::SignE2TimesU),
            '𒂔' => Ok(Cuneiform::SignEdin),
            '𒂕' => Ok(Cuneiform::SignEgir),
            '𒂖' => Ok(Cuneiform::SignEl),
            '𒂗' => Ok(Cuneiform::SignEn),
            '𒂘' => Ok(Cuneiform::SignEnTimesGan2),
            '𒂙' => Ok(Cuneiform::SignEnTimesGan2Tenu),
            '𒂚' => Ok(Cuneiform::SignEnTimesMe),
            '𒂛' => Ok(Cuneiform::SignEnCrossingEn),
            '𒂜' => Ok(Cuneiform::SignEnOpposingEn),
            '𒂝' => Ok(Cuneiform::SignEnSquared),
            '𒂞' => Ok(Cuneiform::SignEren),
            '𒂟' => Ok(Cuneiform::SignErin2),
            '𒂠' => Ok(Cuneiform::SignEsh2),
            '𒂡' => Ok(Cuneiform::SignEzen),
            '𒂢' => Ok(Cuneiform::SignEzenTimesA),
            '𒂣' => Ok(Cuneiform::SignEzenTimesAPlusLal),
            '𒂤' => Ok(Cuneiform::SignEzenTimesAPlusLalTimesLal),
            '𒂥' => Ok(Cuneiform::SignEzenTimesAn),
            '𒂦' => Ok(Cuneiform::SignEzenTimesBad),
            '𒂧' => Ok(Cuneiform::SignEzenTimesDun3Gunu),
            '𒂨' => Ok(Cuneiform::SignEzenTimesDun3GunuGunu),
            '𒂩' => Ok(Cuneiform::SignEzenTimesHa),
            '𒂪' => Ok(Cuneiform::SignEzenTimesHaGunu),
            '𒂫' => Ok(Cuneiform::SignEzenTimesIgiGunu),
            '𒂬' => Ok(Cuneiform::SignEzenTimesKaskal),
            '𒂭' => Ok(Cuneiform::SignEzenTimesKaskalSquared),
            '𒂮' => Ok(Cuneiform::SignEzenTimesKu3),
            '𒂯' => Ok(Cuneiform::SignEzenTimesLa),
            '𒂰' => Ok(Cuneiform::SignEzenTimesLalTimesLal),
            '𒂱' => Ok(Cuneiform::SignEzenTimesLi),
            '𒂲' => Ok(Cuneiform::SignEzenTimesLu),
            '𒂳' => Ok(Cuneiform::SignEzenTimesU2),
            '𒂴' => Ok(Cuneiform::SignEzenTimesUd),
            '𒂵' => Ok(Cuneiform::SignGa),
            '𒂶' => Ok(Cuneiform::SignGaGunu),
            '𒂷' => Ok(Cuneiform::SignGa2),
            '𒂸' => Ok(Cuneiform::SignGa2TimesAPlusDaPlusHa),
            '𒂹' => Ok(Cuneiform::SignGa2TimesAPlusHa),
            '𒂺' => Ok(Cuneiform::SignGa2TimesAPlusIgi),
            '𒂻' => Ok(Cuneiform::SignGa2TimesAb2TenuPlusTab),
            '𒂼' => Ok(Cuneiform::SignGa2TimesAn),
            '𒂽' => Ok(Cuneiform::SignGa2TimesAsh),
            '𒂾' => Ok(Cuneiform::SignGa2TimesAsh2PlusGal),
            '𒂿' => Ok(Cuneiform::SignGa2TimesBad),
            '𒃀' => Ok(Cuneiform::SignGa2TimesBarPlusRa),
            '𒃁' => Ok(Cuneiform::SignGa2TimesBur),
            '𒃂' => Ok(Cuneiform::SignGa2TimesBurPlusRa),
            '𒃃' => Ok(Cuneiform::SignGa2TimesDa),
            '𒃄' => Ok(Cuneiform::SignGa2TimesDi),
            '𒃅' => Ok(Cuneiform::SignGa2TimesDimTimesShe),
            '𒃆' => Ok(Cuneiform::SignGa2TimesDub),
            '𒃇' => Ok(Cuneiform::SignGa2TimesEl),
            '𒃈' => Ok(Cuneiform::SignGa2TimesElPlusLa),
            '𒃉' => Ok(Cuneiform::SignGa2TimesEn),
            '𒃊' => Ok(Cuneiform::SignGa2TimesEnTimesGan2Tenu),
            '𒃋' => Ok(Cuneiform::SignGa2TimesGan2Tenu),
            '𒃌' => Ok(Cuneiform::SignGa2TimesGar),
            '𒃍' => Ok(Cuneiform::SignGa2TimesGi),
            '𒃎' => Ok(Cuneiform::SignGa2TimesGi4),
            '𒃏' => Ok(Cuneiform::SignGa2TimesGi4PlusA),
            '𒃐' => Ok(Cuneiform::SignGa2TimesGir2PlusSu),
            '𒃑' => Ok(Cuneiform::SignGa2TimesHaPlusLuPlusEsh2),
            '𒃒' => Ok(Cuneiform::SignGa2TimesHal),
            '𒃓' => Ok(Cuneiform::SignGa2TimesHalPlusLa),
            '𒃔' => Ok(Cuneiform::SignGa2TimesHiPlusLi),
            '𒃕' => Ok(Cuneiform::SignGa2TimesHub2),
            '𒃖' => Ok(Cuneiform::SignGa2TimesIgiGunu),
            '𒃗' => Ok(Cuneiform::SignGa2TimesIshPlusHuPlusAsh),
            '𒃘' => Ok(Cuneiform::SignGa2TimesKak),
            '𒃙' => Ok(Cuneiform::SignGa2TimesKaskal),
            '𒃚' => Ok(Cuneiform::SignGa2TimesKid),
            '𒃛' => Ok(Cuneiform::SignGa2TimesKidPlusLal),
            '𒃜' => Ok(Cuneiform::SignGa2TimesKu3PlusAn),
            '𒃝' => Ok(Cuneiform::SignGa2TimesLa),
            '𒃞' => Ok(Cuneiform::SignGa2TimesMePlusEn),
            '𒃟' => Ok(Cuneiform::SignGa2TimesMi),
            '𒃠' => Ok(Cuneiform::SignGa2TimesNun),
            '𒃡' => Ok(Cuneiform::SignGa2TimesNunOverNun),
            '𒃢' => Ok(Cuneiform::SignGa2TimesPa),
            '𒃣' => Ok(Cuneiform::SignGa2TimesSal),
            '𒃤' => Ok(Cuneiform::SignGa2TimesSar),
            '𒃥' => Ok(Cuneiform::SignGa2TimesShe),
            '𒃦' => Ok(Cuneiform::SignGa2TimesShePlusTur),
            '𒃧' => Ok(Cuneiform::SignGa2TimesShid),
            '𒃨' => Ok(Cuneiform::SignGa2TimesSum),
            '𒃩' => Ok(Cuneiform::SignGa2TimesTak4),
            '𒃪' => Ok(Cuneiform::SignGa2TimesU),
            '𒃫' => Ok(Cuneiform::SignGa2TimesUd),
            '𒃬' => Ok(Cuneiform::SignGa2TimesUdPlusDu),
            '𒃭' => Ok(Cuneiform::SignGa2OverGa2),
            '𒃮' => Ok(Cuneiform::SignGaba),
            '𒃯' => Ok(Cuneiform::SignGabaCrossingGaba),
            '𒃰' => Ok(Cuneiform::SignGad),
            '𒃱' => Ok(Cuneiform::SignGadOverGadGarOverGar),
            '𒃲' => Ok(Cuneiform::SignGal),
            '𒃳' => Ok(Cuneiform::SignGalGadOverGadGarOverGar),
            '𒃴' => Ok(Cuneiform::SignGalam),
            '𒃵' => Ok(Cuneiform::SignGam),
            '𒃶' => Ok(Cuneiform::SignGan),
            '𒃷' => Ok(Cuneiform::SignGan2),
            '𒃸' => Ok(Cuneiform::SignGan2Tenu),
            '𒃹' => Ok(Cuneiform::SignGan2OverGan2),
            '𒃺' => Ok(Cuneiform::SignGan2CrossingGan2),
            '𒃻' => Ok(Cuneiform::SignGar),
            '𒃼' => Ok(Cuneiform::SignGar3),
            '𒃽' => Ok(Cuneiform::SignGashan),
            '𒃾' => Ok(Cuneiform::SignGeshtin),
            '𒃿' => Ok(Cuneiform::SignGeshtinTimesKur),
            '𒄀' => Ok(Cuneiform::SignGi),
            '𒄁' => Ok(Cuneiform::SignGiTimesE),
            '𒄂' => Ok(Cuneiform::SignGiTimesU),
            '𒄃' => Ok(Cuneiform::SignGiCrossingGi),
            '𒄄' => Ok(Cuneiform::SignGi4),
            '𒄅' => Ok(Cuneiform::SignGi4OverGi4),
            '𒄆' => Ok(Cuneiform::SignGi4CrossingGi4),
            '𒄇' => Ok(Cuneiform::SignGidim),
            '𒄈' => Ok(Cuneiform::SignGir2),
            '𒄉' => Ok(Cuneiform::SignGir2Gunu),
            '𒄊' => Ok(Cuneiform::SignGir3),
            '𒄋' => Ok(Cuneiform::SignGir3TimesAPlusIgi),
            '𒄌' => Ok(Cuneiform::SignGir3TimesGan2Tenu),
            '𒄍' => Ok(Cuneiform::SignGir3TimesIgi),
            '𒄎' => Ok(Cuneiform::SignGir3TimesLuPlusIgi),
            '𒄏' => Ok(Cuneiform::SignGir3TimesPa),
            '𒄐' => Ok(Cuneiform::SignGisal),
            '𒄑' => Ok(Cuneiform::SignGish),
            '𒄒' => Ok(Cuneiform::SignGishCrossingGish),
            '𒄓' => Ok(Cuneiform::SignGishTimesBad),
            '𒄔' => Ok(Cuneiform::SignGishTimesTak4),
            '𒄕' => Ok(Cuneiform::SignGishTenu),
            '𒄖' => Ok(Cuneiform::SignGu),
            '𒄗' => Ok(Cuneiform::SignGuCrossingGu),
            '𒄘' => Ok(Cuneiform::SignGu2),
            '𒄙' => Ok(Cuneiform::SignGu2TimesKak),
            '𒄚' => Ok(Cuneiform::SignGu2TimesKakTimesIgiGunu),
            '𒄛' => Ok(Cuneiform::SignGu2TimesNun),
            '𒄜' => Ok(Cuneiform::SignGu2TimesSalPlusTug2),
            '𒄝' => Ok(Cuneiform::SignGu2Gunu),
            '𒄞' => Ok(Cuneiform::SignGud),
            '𒄟' => Ok(Cuneiform::SignGudTimesAPlusKur),
            '𒄠' => Ok(Cuneiform::SignGudTimesKur),
            '𒄡' => Ok(Cuneiform::SignGudOverGudLugal),
            '𒄢' => Ok(Cuneiform::SignGul),
            '𒄣' => Ok(Cuneiform::SignGum),
            '𒄤' => Ok(Cuneiform::SignGumTimesShe),
            '𒄥' => Ok(Cuneiform::SignGur),
            '𒄦' => Ok(Cuneiform::SignGur7),
            '𒄧' => Ok(Cuneiform::SignGurun),
            '𒄨' => Ok(Cuneiform::SignGurush),
            '𒄩' => Ok(Cuneiform::SignHa),
            '𒄪' => Ok(Cuneiform::SignHaTenu),
            '𒄫' => Ok(Cuneiform::SignHaGunu),
            '𒄬' => Ok(Cuneiform::SignHal),
            '𒄭' => Ok(Cuneiform::SignHi),
            '𒄮' => Ok(Cuneiform::SignHiTimesAsh),
            '𒄯' => Ok(Cuneiform::SignHiTimesAsh2),
            '𒄰' => Ok(Cuneiform::SignHiTimesBad),
            '𒄱' => Ok(Cuneiform::SignHiTimesDish),
            '𒄲' => Ok(Cuneiform::SignHiTimesGad),
            '𒄳' => Ok(Cuneiform::SignHiTimesKin),
            '𒄴' => Ok(Cuneiform::SignHiTimesNun),
            '𒄵' => Ok(Cuneiform::SignHiTimesShe),
            '𒄶' => Ok(Cuneiform::SignHiTimesU),
            '𒄷' => Ok(Cuneiform::SignHu),
            '𒄸' => Ok(Cuneiform::SignHub2),
            '𒄹' => Ok(Cuneiform::SignHub2TimesAn),
            '𒄺' => Ok(Cuneiform::SignHub2TimesHal),
            '𒄻' => Ok(Cuneiform::SignHub2TimesKaskal),
            '𒄼' => Ok(Cuneiform::SignHub2TimesLish),
            '𒄽' => Ok(Cuneiform::SignHub2TimesUd),
            '𒄾' => Ok(Cuneiform::SignHul2),
            '𒄿' => Ok(Cuneiform::SignI),
            '𒅀' => Ok(Cuneiform::SignIA),
            '𒅁' => Ok(Cuneiform::SignIb),
            '𒅂' => Ok(Cuneiform::SignIdim),
            '𒅃' => Ok(Cuneiform::SignIdimOverIdimBur),
            '𒅄' => Ok(Cuneiform::SignIdimOverIdimSquared),
            '𒅅' => Ok(Cuneiform::SignIg),
            '𒅆' => Ok(Cuneiform::SignIgi),
            '𒅇' => Ok(Cuneiform::SignIgiDib),
            '𒅈' => Ok(Cuneiform::SignIgiRi),
            '𒅉' => Ok(Cuneiform::SignIgiOverIgiShirOverShirUdOverUd),
            '𒅊' => Ok(Cuneiform::SignIgiGunu),
            '𒅋' => Ok(Cuneiform::SignIl),
            '𒅌' => Ok(Cuneiform::SignIlTimesGan2Tenu),
            '𒅍' => Ok(Cuneiform::SignIl2),
            '𒅎' => Ok(Cuneiform::SignIm),
            '𒅏' => Ok(Cuneiform::SignImTimesTak4),
            '𒅐' => Ok(Cuneiform::SignImCrossingIm),
            '𒅑' => Ok(Cuneiform::SignImOpposingIm),
            '𒅒' => Ok(Cuneiform::SignImSquared),
            '𒅓' => Ok(Cuneiform::SignImin),
            '𒅔' => Ok(Cuneiform::SignIn),
            '𒅕' => Ok(Cuneiform::SignIr),
            '𒅖' => Ok(Cuneiform::SignIsh),
            '𒅗' => Ok(Cuneiform::SignKa),
            '𒅘' => Ok(Cuneiform::SignKaTimesA),
            '𒅙' => Ok(Cuneiform::SignKaTimesAd),
            '𒅚' => Ok(Cuneiform::SignKaTimesAdPlusKu3),
            '𒅛' => Ok(Cuneiform::SignKaTimesAsh2),
            '𒅜' => Ok(Cuneiform::SignKaTimesBad),
            '𒅝' => Ok(Cuneiform::SignKaTimesBalag),
            '𒅞' => Ok(Cuneiform::SignKaTimesBar),
            '𒅟' => Ok(Cuneiform::SignKaTimesBi),
            '𒅠' => Ok(Cuneiform::SignKaTimesErin2),
            '𒅡' => Ok(Cuneiform::SignKaTimesEsh2),
            '𒅢' => Ok(Cuneiform::SignKaTimesGa),
            '𒅣' => Ok(Cuneiform::SignKaTimesGal),
            '𒅤' => Ok(Cuneiform::SignKaTimesGan2Tenu),
            '𒅥' => Ok(Cuneiform::SignKaTimesGar),
            '𒅦' => Ok(Cuneiform::SignKaTimesGarPlusSha3PlusA),
            '𒅧' => Ok(Cuneiform::SignKaTimesGi),
            '𒅨' => Ok(Cuneiform::SignKaTimesGir2),
            '𒅩' => Ok(Cuneiform::SignKaTimesGishPlusSar),
            '𒅪' => Ok(Cuneiform::SignKaTimesGishCrossingGish),
            '𒅫' => Ok(Cuneiform::SignKaTimesGu),
            '𒅬' => Ok(Cuneiform::SignKaTimesGur7),
            '𒅭' => Ok(Cuneiform::SignKaTimesIgi),
            '𒅮' => Ok(Cuneiform::SignKaTimesIm),
            '𒅯' => Ok(Cuneiform::SignKaTimesKak),
            '𒅰' => Ok(Cuneiform::SignKaTimesKi),
            '𒅱' => Ok(Cuneiform::SignKaTimesKid),
            '𒅲' => Ok(Cuneiform::SignKaTimesLi),
            '𒅳' => Ok(Cuneiform::SignKaTimesLu),
            '𒅴' => Ok(Cuneiform::SignKaTimesMe),
            '𒅵' => Ok(Cuneiform::SignKaTimesMePlusDu),
            '𒅶' => Ok(Cuneiform::SignKaTimesMePlusGi),
            '𒅷' => Ok(Cuneiform::SignKaTimesMePlusTe),
            '𒅸' => Ok(Cuneiform::SignKaTimesMi),
            '𒅹' => Ok(Cuneiform::SignKaTimesMiPlusNunuz),
            '𒅺' => Ok(Cuneiform::SignKaTimesNe),
            '𒅻' => Ok(Cuneiform::SignKaTimesNun),
            '𒅼' => Ok(Cuneiform::SignKaTimesPi),
            '𒅽' => Ok(Cuneiform::SignKaTimesRu),
            '𒅾' => Ok(Cuneiform::SignKaTimesSa),
            '𒅿' => Ok(Cuneiform::SignKaTimesSar),
            '𒆀' => Ok(Cuneiform::SignKaTimesSha),
            '𒆁' => Ok(Cuneiform::SignKaTimesShe),
            '𒆂' => Ok(Cuneiform::SignKaTimesShid),
            '𒆃' => Ok(Cuneiform::SignKaTimesShu),
            '𒆄' => Ok(Cuneiform::SignKaTimesSig),
            '𒆅' => Ok(Cuneiform::SignKaTimesSuhur),
            '𒆆' => Ok(Cuneiform::SignKaTimesTar),
            '𒆇' => Ok(Cuneiform::SignKaTimesU),
            '𒆈' => Ok(Cuneiform::SignKaTimesU2),
            '𒆉' => Ok(Cuneiform::SignKaTimesUd),
            '𒆊' => Ok(Cuneiform::SignKaTimesUmumTimesPa),
            '𒆋' => Ok(Cuneiform::SignKaTimesUsh),
            '𒆌' => Ok(Cuneiform::SignKaTimesZi),
            '𒆍' => Ok(Cuneiform::SignKa2),
            '𒆎' => Ok(Cuneiform::SignKa2CrossingKa2),
            '𒆏' => Ok(Cuneiform::SignKab),
            '𒆐' => Ok(Cuneiform::SignKad2),
            '𒆑' => Ok(Cuneiform::SignKad3),
            '𒆒' => Ok(Cuneiform::SignKad4),
            '𒆓' => Ok(Cuneiform::SignKad5),
            '𒆔' => Ok(Cuneiform::SignKad5OverKad5),
            '𒆕' => Ok(Cuneiform::SignKak),
            '𒆖' => Ok(Cuneiform::SignKakTimesIgiGunu),
            '𒆗' => Ok(Cuneiform::SignKal),
            '𒆘' => Ok(Cuneiform::SignKalTimesBad),
            '𒆙' => Ok(Cuneiform::SignKalCrossingKal),
            '𒆚' => Ok(Cuneiform::SignKam2),
            '𒆛' => Ok(Cuneiform::SignKam4),
            '𒆜' => Ok(Cuneiform::SignKaskal),
            '𒆝' => Ok(Cuneiform::SignKaskalLagabTimesUOverLagabTimesU),
            '𒆞' => Ok(Cuneiform::SignKaskalOverKaskalLagabTimesUOverLagabTimesU),
            '𒆟' => Ok(Cuneiform::SignKesh2),
            '𒆠' => Ok(Cuneiform::SignKi),
            '𒆡' => Ok(Cuneiform::SignKiTimesBad),
            '𒆢' => Ok(Cuneiform::SignKiTimesU),
            '𒆣' => Ok(Cuneiform::SignKiTimesUd),
            '𒆤' => Ok(Cuneiform::SignKid),
            '𒆥' => Ok(Cuneiform::SignKin),
            '𒆦' => Ok(Cuneiform::SignKisal),
            '𒆧' => Ok(Cuneiform::SignKish),
            '𒆨' => Ok(Cuneiform::SignKisim5),
            '𒆩' => Ok(Cuneiform::SignKisim5OverKisim5),
            '𒆪' => Ok(Cuneiform::SignKu),
            '𒆫' => Ok(Cuneiform::SignKuOverHiTimesAsh2KuOverHiTimesAsh2),
            '𒆬' => Ok(Cuneiform::SignKu3),
            '𒆭' => Ok(Cuneiform::SignKu4),
            '𒆮' => Ok(Cuneiform::SignKu4VariantForm),
            '𒆯' => Ok(Cuneiform::SignKu7),
            '𒆰' => Ok(Cuneiform::SignKul),
            '𒆱' => Ok(Cuneiform::SignKulGunu),
            '𒆲' => Ok(Cuneiform::SignKun),
            '𒆳' => Ok(Cuneiform::SignKur),
            '𒆴' => Ok(Cuneiform::SignKurOpposingKur),
            '𒆵' => Ok(Cuneiform::SignKushu2),
            '𒆶' => Ok(Cuneiform::SignKwu318),
            '𒆷' => Ok(Cuneiform::SignLa),
            '𒆸' => Ok(Cuneiform::SignLagab),
            '𒆹' => Ok(Cuneiform::SignLagabTimesA),
            '𒆺' => Ok(Cuneiform::SignLagabTimesAPlusDaPlusHa),
            '𒆻' => Ok(Cuneiform::SignLagabTimesAPlusGar),
            '𒆼' => Ok(Cuneiform::SignLagabTimesAPlusLal),
            '𒆽' => Ok(Cuneiform::SignLagabTimesAl),
            '𒆾' => Ok(Cuneiform::SignLagabTimesAn),
            '𒆿' => Ok(Cuneiform::SignLagabTimesAshZidaTenu),
            '𒇀' => Ok(Cuneiform::SignLagabTimesBad),
            '𒇁' => Ok(Cuneiform::SignLagabTimesBi),
            '𒇂' => Ok(Cuneiform::SignLagabTimesDar),
            '𒇃' => Ok(Cuneiform::SignLagabTimesEn),
            '𒇄' => Ok(Cuneiform::SignLagabTimesGa),
            '𒇅' => Ok(Cuneiform::SignLagabTimesGar),
            '𒇆' => Ok(Cuneiform::SignLagabTimesGud),
            '𒇇' => Ok(Cuneiform::SignLagabTimesGudPlusGud),
            '𒇈' => Ok(Cuneiform::SignLagabTimesHa),
            '𒇉' => Ok(Cuneiform::SignLagabTimesHal),
            '𒇊' => Ok(Cuneiform::SignLagabTimesHiTimesNun),
            '𒇋' => Ok(Cuneiform::SignLagabTimesIgiGunu),
            '𒇌' => Ok(Cuneiform::SignLagabTimesIm),
            '𒇍' => Ok(Cuneiform::SignLagabTimesImPlusHa),
            '𒇎' => Ok(Cuneiform::SignLagabTimesImPlusLu),
            '𒇏' => Ok(Cuneiform::SignLagabTimesKi),
            '𒇐' => Ok(Cuneiform::SignLagabTimesKin),
            '𒇑' => Ok(Cuneiform::SignLagabTimesKu3),
            '𒇒' => Ok(Cuneiform::SignLagabTimesKul),
            '𒇓' => Ok(Cuneiform::SignLagabTimesKulPlusHiPlusA),
            '𒇔' => Ok(Cuneiform::SignLagabTimesLagab),
            '𒇕' => Ok(Cuneiform::SignLagabTimesLish),
            '𒇖' => Ok(Cuneiform::SignLagabTimesLu),
            '𒇗' => Ok(Cuneiform::SignLagabTimesLul),
            '𒇘' => Ok(Cuneiform::SignLagabTimesMe),
            '𒇙' => Ok(Cuneiform::SignLagabTimesMePlusEn),
            '𒇚' => Ok(Cuneiform::SignLagabTimesMush),
            '𒇛' => Ok(Cuneiform::SignLagabTimesNe),
            '𒇜' => Ok(Cuneiform::SignLagabTimesShePlusSum),
            '𒇝' => Ok(Cuneiform::SignLagabTimesShitaPlusGishPlusErin2),
            '𒇞' => Ok(Cuneiform::SignLagabTimesShitaPlusGishTenu),
            '𒇟' => Ok(Cuneiform::SignLagabTimesShu2),
            '𒇠' => Ok(Cuneiform::SignLagabTimesShu2PlusShu2),
            '𒇡' => Ok(Cuneiform::SignLagabTimesSum),
            '𒇢' => Ok(Cuneiform::SignLagabTimesTag),
            '𒇣' => Ok(Cuneiform::SignLagabTimesTak4),
            '𒇤' => Ok(Cuneiform::SignLagabTimesTePlusAPlusSuPlusNa),
            '𒇥' => Ok(Cuneiform::SignLagabTimesU),
            '𒇦' => Ok(Cuneiform::SignLagabTimesUPlusA),
            '𒇧' => Ok(Cuneiform::SignLagabTimesUPlusUPlusU),
            '𒇨' => Ok(Cuneiform::SignLagabTimesU2PlusAsh),
            '𒇩' => Ok(Cuneiform::SignLagabTimesUd),
            '𒇪' => Ok(Cuneiform::SignLagabTimesUsh),
            '𒇫' => Ok(Cuneiform::SignLagabSquared),
            '𒇬' => Ok(Cuneiform::SignLagar),
            '𒇭' => Ok(Cuneiform::SignLagarTimesShe),
            '𒇮' => Ok(Cuneiform::SignLagarTimesShePlusSum),
            '𒇯' => Ok(Cuneiform::SignLagarGunu),
            '𒇰' => Ok(Cuneiform::SignLagarGunuOverLagarGunuShe),
            '𒇱' => Ok(Cuneiform::SignLahshu),
            '𒇲' => Ok(Cuneiform::SignLal),
            '𒇳' => Ok(Cuneiform::SignLalTimesLal),
            '𒇴' => Ok(Cuneiform::SignLam),
            '𒇵' => Ok(Cuneiform::SignLamTimesKur),
            '𒇶' => Ok(Cuneiform::SignLamTimesKurPlusRu),
            '𒇷' => Ok(Cuneiform::SignLi),
            '𒇸' => Ok(Cuneiform::SignLil),
            '𒇹' => Ok(Cuneiform::SignLimmu2),
            '𒇺' => Ok(Cuneiform::SignLish),
            '𒇻' => Ok(Cuneiform::SignLu),
            '𒇼' => Ok(Cuneiform::SignLuTimesBad),
            '𒇽' => Ok(Cuneiform::SignLu2),
            '𒇾' => Ok(Cuneiform::SignLu2TimesAl),
            '𒇿' => Ok(Cuneiform::SignLu2TimesBad),
            '𒈀' => Ok(Cuneiform::SignLu2TimesEsh2),
            '𒈁' => Ok(Cuneiform::SignLu2TimesEsh2Tenu),
            '𒈂' => Ok(Cuneiform::SignLu2TimesGan2Tenu),
            '𒈃' => Ok(Cuneiform::SignLu2TimesHiTimesBad),
            '𒈄' => Ok(Cuneiform::SignLu2TimesIm),
            '𒈅' => Ok(Cuneiform::SignLu2TimesKad2),
            '𒈆' => Ok(Cuneiform::SignLu2TimesKad3),
            '𒈇' => Ok(Cuneiform::SignLu2TimesKad3PlusAsh),
            '𒈈' => Ok(Cuneiform::SignLu2TimesKi),
            '𒈉' => Ok(Cuneiform::SignLu2TimesLaPlusAsh),
            '𒈊' => Ok(Cuneiform::SignLu2TimesLagab),
            '𒈋' => Ok(Cuneiform::SignLu2TimesMePlusEn),
            '𒈌' => Ok(Cuneiform::SignLu2TimesNe),
            '𒈍' => Ok(Cuneiform::SignLu2TimesNu),
            '𒈎' => Ok(Cuneiform::SignLu2TimesSiPlusAsh),
            '𒈏' => Ok(Cuneiform::SignLu2TimesSik2PlusBu),
            '𒈐' => Ok(Cuneiform::SignLu2TimesTug2),
            '𒈑' => Ok(Cuneiform::SignLu2Tenu),
            '𒈒' => Ok(Cuneiform::SignLu2CrossingLu2),
            '𒈓' => Ok(Cuneiform::SignLu2OpposingLu2),
            '𒈔' => Ok(Cuneiform::SignLu2Squared),
            '𒈕' => Ok(Cuneiform::SignLu2Sheshig),
            '𒈖' => Ok(Cuneiform::SignLu3),
            '𒈗' => Ok(Cuneiform::SignLugal),
            '𒈘' => Ok(Cuneiform::SignLugalOverLugal),
            '𒈙' => Ok(Cuneiform::SignLugalOpposingLugal),
            '𒈚' => Ok(Cuneiform::SignLugalSheshig),
            '𒈛' => Ok(Cuneiform::SignLuh),
            '𒈜' => Ok(Cuneiform::SignLul),
            '𒈝' => Ok(Cuneiform::SignLum),
            '𒈞' => Ok(Cuneiform::SignLumOverLum),
            '𒈟' => Ok(Cuneiform::SignLumOverLumGarOverGar),
            '𒈠' => Ok(Cuneiform::SignMa),
            '𒈡' => Ok(Cuneiform::SignMaTimesTak4),
            '𒈢' => Ok(Cuneiform::SignMaGunu),
            '𒈣' => Ok(Cuneiform::SignMa2),
            '𒈤' => Ok(Cuneiform::SignMah),
            '𒈥' => Ok(Cuneiform::SignMar),
            '𒈦' => Ok(Cuneiform::SignMash),
            '𒈧' => Ok(Cuneiform::SignMash2),
            '𒈨' => Ok(Cuneiform::SignMe),
            '𒈩' => Ok(Cuneiform::SignMes),
            '𒈪' => Ok(Cuneiform::SignMi),
            '𒈫' => Ok(Cuneiform::SignMin),
            '𒈬' => Ok(Cuneiform::SignMu),
            '𒈭' => Ok(Cuneiform::SignMuOverMu),
            '𒈮' => Ok(Cuneiform::SignMug),
            '𒈯' => Ok(Cuneiform::SignMugGunu),
            '𒈰' => Ok(Cuneiform::SignMunsub),
            '𒈱' => Ok(Cuneiform::SignMurgu2),
            '𒈲' => Ok(Cuneiform::SignMush),
            '𒈳' => Ok(Cuneiform::SignMushTimesA),
            '𒈴' => Ok(Cuneiform::SignMushTimesKur),
            '𒈵' => Ok(Cuneiform::SignMushTimesZa),
            '𒈶' => Ok(Cuneiform::SignMushOverMush),
            '𒈷' => Ok(Cuneiform::SignMushOverMushTimesAPlusNa),
            '𒈸' => Ok(Cuneiform::SignMushCrossingMush),
            '𒈹' => Ok(Cuneiform::SignMush3),
            '𒈺' => Ok(Cuneiform::SignMush3TimesA),
            '𒈻' => Ok(Cuneiform::SignMush3TimesAPlusDi),
            '𒈼' => Ok(Cuneiform::SignMush3TimesDi),
            '𒈽' => Ok(Cuneiform::SignMush3Gunu),
            '𒈾' => Ok(Cuneiform::SignNa),
            '𒈿' => Ok(Cuneiform::SignNa2),
            '𒉀' => Ok(Cuneiform::SignNaga),
            '𒉁' => Ok(Cuneiform::SignNagaInverted),
            '𒉂' => Ok(Cuneiform::SignNagaTimesShuTenu),
            '𒉃' => Ok(Cuneiform::SignNagaOpposingNaga),
            '𒉄' => Ok(Cuneiform::SignNagar),
            '𒉅' => Ok(Cuneiform::SignNamNutillu),
            '𒉆' => Ok(Cuneiform::SignNam),
            '𒉇' => Ok(Cuneiform::SignNam2),
            '𒉈' => Ok(Cuneiform::SignNe),
            '𒉉' => Ok(Cuneiform::SignNeTimesA),
            '𒉊' => Ok(Cuneiform::SignNeTimesUd),
            '𒉋' => Ok(Cuneiform::SignNeSheshig),
            '𒉌' => Ok(Cuneiform::SignNi),
            '𒉍' => Ok(Cuneiform::SignNiTimesE),
            '𒉎' => Ok(Cuneiform::SignNi2),
            '𒉏' => Ok(Cuneiform::SignNim),
            '𒉐' => Ok(Cuneiform::SignNimTimesGan2Tenu),
            '𒉑' => Ok(Cuneiform::SignNimTimesGarPlusGan2Tenu),
            '𒉒' => Ok(Cuneiform::SignNinda2),
            '𒉓' => Ok(Cuneiform::SignNinda2TimesAn),
            '𒉔' => Ok(Cuneiform::SignNinda2TimesAsh),
            '𒉕' => Ok(Cuneiform::SignNinda2TimesAshPlusAsh),
            '𒉖' => Ok(Cuneiform::SignNinda2TimesGud),
            '𒉗' => Ok(Cuneiform::SignNinda2TimesMePlusGan2Tenu),
            '𒉘' => Ok(Cuneiform::SignNinda2TimesNe),
            '𒉙' => Ok(Cuneiform::SignNinda2TimesNun),
            '𒉚' => Ok(Cuneiform::SignNinda2TimesShe),
            '𒉛' => Ok(Cuneiform::SignNinda2TimesShePlusAAn),
            '𒉜' => Ok(Cuneiform::SignNinda2TimesShePlusAsh),
            '𒉝' => Ok(Cuneiform::SignNinda2TimesShePlusAshPlusAsh),
            '𒉞' => Ok(Cuneiform::SignNinda2TimesU2PlusAsh),
            '𒉟' => Ok(Cuneiform::SignNinda2TimesUsh),
            '𒉠' => Ok(Cuneiform::SignNisag),
            '𒉡' => Ok(Cuneiform::SignNu),
            '𒉢' => Ok(Cuneiform::SignNu11),
            '𒉣' => Ok(Cuneiform::SignNun),
            '𒉤' => Ok(Cuneiform::SignNunLagarTimesGar),
            '𒉥' => Ok(Cuneiform::SignNunLagarTimesMash),
            '𒉦' => Ok(Cuneiform::SignNunLagarTimesSal),
            '𒉧' => Ok(Cuneiform::SignNunLagarTimesSalOverNunLagarTimesSal),
            '𒉨' => Ok(Cuneiform::SignNunLagarTimesUsh),
            '𒉩' => Ok(Cuneiform::SignNunTenu),
            '𒉪' => Ok(Cuneiform::SignNunOverNun),
            '𒉫' => Ok(Cuneiform::SignNunCrossingNun),
            '𒉬' => Ok(Cuneiform::SignNunCrossingNunLagarOverLagar),
            '𒉭' => Ok(Cuneiform::SignNunuz),
            '𒉮' => Ok(Cuneiform::SignNunuzAb2TimesAshgab),
            '𒉯' => Ok(Cuneiform::SignNunuzAb2TimesBi),
            '𒉰' => Ok(Cuneiform::SignNunuzAb2TimesDug),
            '𒉱' => Ok(Cuneiform::SignNunuzAb2TimesGud),
            '𒉲' => Ok(Cuneiform::SignNunuzAb2TimesIgiGunu),
            '𒉳' => Ok(Cuneiform::SignNunuzAb2TimesKad3),
            '𒉴' => Ok(Cuneiform::SignNunuzAb2TimesLa),
            '𒉵' => Ok(Cuneiform::SignNunuzAb2TimesNe),
            '𒉶' => Ok(Cuneiform::SignNunuzAb2TimesSila3),
            '𒉷' => Ok(Cuneiform::SignNunuzAb2TimesU2),
            '𒉸' => Ok(Cuneiform::SignNunuzKisim5TimesBi),
            '𒉹' => Ok(Cuneiform::SignNunuzKisim5TimesBiU),
            '𒉺' => Ok(Cuneiform::SignPa),
            '𒉻' => Ok(Cuneiform::SignPad),
            '𒉼' => Ok(Cuneiform::SignPan),
            '𒉽' => Ok(Cuneiform::SignPap),
            '𒉾' => Ok(Cuneiform::SignPesh2),
            '𒉿' => Ok(Cuneiform::SignPi),
            '𒊀' => Ok(Cuneiform::SignPiTimesA),
            '𒊁' => Ok(Cuneiform::SignPiTimesAb),
            '𒊂' => Ok(Cuneiform::SignPiTimesBi),
            '𒊃' => Ok(Cuneiform::SignPiTimesBu),
            '𒊄' => Ok(Cuneiform::SignPiTimesE),
            '𒊅' => Ok(Cuneiform::SignPiTimesI),
            '𒊆' => Ok(Cuneiform::SignPiTimesIb),
            '𒊇' => Ok(Cuneiform::SignPiTimesU),
            '𒊈' => Ok(Cuneiform::SignPiTimesU2),
            '𒊉' => Ok(Cuneiform::SignPiCrossingPi),
            '𒊊' => Ok(Cuneiform::SignPirig),
            '𒊋' => Ok(Cuneiform::SignPirigTimesKal),
            '𒊌' => Ok(Cuneiform::SignPirigTimesUd),
            '𒊍' => Ok(Cuneiform::SignPirigTimesZa),
            '𒊎' => Ok(Cuneiform::SignPirigOpposingPirig),
            '𒊏' => Ok(Cuneiform::SignRa),
            '𒊐' => Ok(Cuneiform::SignRab),
            '𒊑' => Ok(Cuneiform::SignRi),
            '𒊒' => Ok(Cuneiform::SignRu),
            '𒊓' => Ok(Cuneiform::SignSa),
            '𒊔' => Ok(Cuneiform::SignSagNutillu),
            '𒊕' => Ok(Cuneiform::SignSag),
            '𒊖' => Ok(Cuneiform::SignSagTimesA),
            '𒊗' => Ok(Cuneiform::SignSagTimesDu),
            '𒊘' => Ok(Cuneiform::SignSagTimesDub),
            '𒊙' => Ok(Cuneiform::SignSagTimesHa),
            '𒊚' => Ok(Cuneiform::SignSagTimesKak),
            '𒊛' => Ok(Cuneiform::SignSagTimesKur),
            '𒊜' => Ok(Cuneiform::SignSagTimesLum),
            '𒊝' => Ok(Cuneiform::SignSagTimesMi),
            '𒊞' => Ok(Cuneiform::SignSagTimesNun),
            '𒊟' => Ok(Cuneiform::SignSagTimesSal),
            '𒊠' => Ok(Cuneiform::SignSagTimesShid),
            '𒊡' => Ok(Cuneiform::SignSagTimesTab),
            '𒊢' => Ok(Cuneiform::SignSagTimesU2),
            '𒊣' => Ok(Cuneiform::SignSagTimesUb),
            '𒊤' => Ok(Cuneiform::SignSagTimesUm),
            '𒊥' => Ok(Cuneiform::SignSagTimesUr),
            '𒊦' => Ok(Cuneiform::SignSagTimesUsh),
            '𒊧' => Ok(Cuneiform::SignSagOverSag),
            '𒊨' => Ok(Cuneiform::SignSagGunu),
            '𒊩' => Ok(Cuneiform::SignSal),
            '𒊪' => Ok(Cuneiform::SignSalLagabTimesAsh2),
            '𒊫' => Ok(Cuneiform::SignSanga2),
            '𒊬' => Ok(Cuneiform::SignSar),
            '𒊭' => Ok(Cuneiform::SignSha),
            '𒊮' => Ok(Cuneiform::SignSha3),
            '𒊯' => Ok(Cuneiform::SignSha3TimesA),
            '𒊰' => Ok(Cuneiform::SignSha3TimesBad),
            '𒊱' => Ok(Cuneiform::SignSha3TimesGish),
            '𒊲' => Ok(Cuneiform::SignSha3TimesNe),
            '𒊳' => Ok(Cuneiform::SignSha3TimesShu2),
            '𒊴' => Ok(Cuneiform::SignSha3TimesTur),
            '𒊵' => Ok(Cuneiform::SignSha3TimesU),
            '𒊶' => Ok(Cuneiform::SignSha3TimesUPlusA),
            '𒊷' => Ok(Cuneiform::SignSha6),
            '𒊸' => Ok(Cuneiform::SignShab6),
            '𒊹' => Ok(Cuneiform::SignShar2),
            '𒊺' => Ok(Cuneiform::SignShe),
            '𒊻' => Ok(Cuneiform::SignSheHu),
            '𒊼' => Ok(Cuneiform::SignSheOverSheGadOverGadGarOverGar),
            '𒊽' => Ok(Cuneiform::SignSheOverSheTabOverTabGarOverGar),
            '𒊾' => Ok(Cuneiform::SignSheg9),
            '𒊿' => Ok(Cuneiform::SignShen),
            '𒋀' => Ok(Cuneiform::SignShesh),
            '𒋁' => Ok(Cuneiform::SignShesh2),
            '𒋂' => Ok(Cuneiform::SignSheshlam),
            '𒋃' => Ok(Cuneiform::SignShid),
            '𒋄' => Ok(Cuneiform::SignShidTimesA),
            '𒋅' => Ok(Cuneiform::SignShidTimesIm),
            '𒋆' => Ok(Cuneiform::SignShim),
            '𒋇' => Ok(Cuneiform::SignShimTimesA),
            '𒋈' => Ok(Cuneiform::SignShimTimesBal),
            '𒋉' => Ok(Cuneiform::SignShimTimesBulug),
            '𒋊' => Ok(Cuneiform::SignShimTimesDin),
            '𒋋' => Ok(Cuneiform::SignShimTimesGar),
            '𒋌' => Ok(Cuneiform::SignShimTimesIgi),
            '𒋍' => Ok(Cuneiform::SignShimTimesIgiGunu),
            '𒋎' => Ok(Cuneiform::SignShimTimesKushu2),
            '𒋏' => Ok(Cuneiform::SignShimTimesLul),
            '𒋐' => Ok(Cuneiform::SignShimTimesMug),
            '𒋑' => Ok(Cuneiform::SignShimTimesSal),
            '𒋒' => Ok(Cuneiform::SignShinig),
            '𒋓' => Ok(Cuneiform::SignShir),
            '𒋔' => Ok(Cuneiform::SignShirTenu),
            '𒋕' => Ok(Cuneiform::SignShirOverShirBurOverBur),
            '𒋖' => Ok(Cuneiform::SignShita),
            '𒋗' => Ok(Cuneiform::SignShu),
            '𒋘' => Ok(Cuneiform::SignShuOverInvertedShu),
            '𒋙' => Ok(Cuneiform::SignShu2),
            '𒋚' => Ok(Cuneiform::SignShubur),
            '𒋛' => Ok(Cuneiform::SignSi),
            '𒋜' => Ok(Cuneiform::SignSiGunu),
            '𒋝' => Ok(Cuneiform::SignSig),
            '𒋞' => Ok(Cuneiform::SignSig4),
            '𒋟' => Ok(Cuneiform::SignSig4OverSig4Shu2),
            '𒋠' => Ok(Cuneiform::SignSik2),
            '𒋡' => Ok(Cuneiform::SignSila3),
            '𒋢' => Ok(Cuneiform::SignSu),
            '𒋣' => Ok(Cuneiform::SignSuOverSu),
            '𒋤' => Ok(Cuneiform::SignSud),
            '𒋥' => Ok(Cuneiform::SignSud2),
            '𒋦' => Ok(Cuneiform::SignSuhur),
            '𒋧' => Ok(Cuneiform::SignSum),
            '𒋨' => Ok(Cuneiform::SignSumash),
            '𒋩' => Ok(Cuneiform::SignSur),
            '𒋪' => Ok(Cuneiform::SignSur9),
            '𒋫' => Ok(Cuneiform::SignTa),
            '𒋬' => Ok(Cuneiform::SignTaAsterisk),
            '𒋭' => Ok(Cuneiform::SignTaTimesHi),
            '𒋮' => Ok(Cuneiform::SignTaTimesMi),
            '𒋯' => Ok(Cuneiform::SignTaGunu),
            '𒋰' => Ok(Cuneiform::SignTab),
            '𒋱' => Ok(Cuneiform::SignTabOverTabNiOverNiDishOverDish),
            '𒋲' => Ok(Cuneiform::SignTabSquared),
            '𒋳' => Ok(Cuneiform::SignTag),
            '𒋴' => Ok(Cuneiform::SignTagTimesBi),
            '𒋵' => Ok(Cuneiform::SignTagTimesGud),
            '𒋶' => Ok(Cuneiform::SignTagTimesShe),
            '𒋷' => Ok(Cuneiform::SignTagTimesShu),
            '𒋸' => Ok(Cuneiform::SignTagTimesTug2),
            '𒋹' => Ok(Cuneiform::SignTagTimesUd),
            '𒋺' => Ok(Cuneiform::SignTak4),
            '𒋻' => Ok(Cuneiform::SignTar),
            '𒋼' => Ok(Cuneiform::SignTe),
            '𒋽' => Ok(Cuneiform::SignTeGunu),
            '𒋾' => Ok(Cuneiform::SignTi),
            '𒋿' => Ok(Cuneiform::SignTiTenu),
            '𒌀' => Ok(Cuneiform::SignTil),
            '𒌁' => Ok(Cuneiform::SignTir),
            '𒌂' => Ok(Cuneiform::SignTirTimesTak4),
            '𒌃' => Ok(Cuneiform::SignTirOverTir),
            '𒌄' => Ok(Cuneiform::SignTirOverTirGadOverGadGarOverGar),
            '𒌅' => Ok(Cuneiform::SignTu),
            '𒌆' => Ok(Cuneiform::SignTug2),
            '𒌇' => Ok(Cuneiform::SignTuk),
            '𒌈' => Ok(Cuneiform::SignTum),
            '𒌉' => Ok(Cuneiform::SignTur),
            '𒌊' => Ok(Cuneiform::SignTurOverTurZaOverZa),
            '𒌋' => Ok(Cuneiform::SignU),
            '𒌌' => Ok(Cuneiform::SignUGud),
            '𒌍' => Ok(Cuneiform::SignUUU),
            '𒌎' => Ok(Cuneiform::SignUOverUPaOverPaGarOverGar),
            '𒌏' => Ok(Cuneiform::SignUOverUSurOverSur),
            '𒌐' => Ok(Cuneiform::SignUOverUUReversedOverUReversed),
            '𒌑' => Ok(Cuneiform::SignU2),
            '𒌒' => Ok(Cuneiform::SignUb),
            '𒌓' => Ok(Cuneiform::SignUd),
            '𒌔' => Ok(Cuneiform::SignUdKushu2),
            '𒌕' => Ok(Cuneiform::SignUdTimesBad),
            '𒌖' => Ok(Cuneiform::SignUdTimesMi),
            '𒌗' => Ok(Cuneiform::SignUdTimesUPlusUPlusU),
            '𒌘' => Ok(Cuneiform::SignUdTimesUPlusUPlusUGunu),
            '𒌙' => Ok(Cuneiform::SignUdGunu),
            '𒌚' => Ok(Cuneiform::SignUdSheshig),
            '𒌛' => Ok(Cuneiform::SignUdSheshigTimesBad),
            '𒌜' => Ok(Cuneiform::SignUdug),
            '𒌝' => Ok(Cuneiform::SignUm),
            '𒌞' => Ok(Cuneiform::SignUmTimesLagab),
            '𒌟' => Ok(Cuneiform::SignUmTimesMePlusDa),
            '𒌠' => Ok(Cuneiform::SignUmTimesSha3),
            '𒌡' => Ok(Cuneiform::SignUmTimesU),
            '𒌢' => Ok(Cuneiform::SignUmbin),
            '𒌣' => Ok(Cuneiform::SignUmum),
            '𒌤' => Ok(Cuneiform::SignUmumTimesKaskal),
            '𒌥' => Ok(Cuneiform::SignUmumTimesPa),
            '𒌦' => Ok(Cuneiform::SignUn),
            '𒌧' => Ok(Cuneiform::SignUnGunu),
            '𒌨' => Ok(Cuneiform::SignUr),
            '𒌩' => Ok(Cuneiform::SignUrCrossingUr),
            '𒌪' => Ok(Cuneiform::SignUrSheshig),
            '𒌫' => Ok(Cuneiform::SignUr2),
            '𒌬' => Ok(Cuneiform::SignUr2TimesAPlusHa),
            '𒌭' => Ok(Cuneiform::SignUr2TimesAPlusNa),
            '𒌮' => Ok(Cuneiform::SignUr2TimesAl),
            '𒌯' => Ok(Cuneiform::SignUr2TimesHa),
            '𒌰' => Ok(Cuneiform::SignUr2TimesNun),
            '𒌱' => Ok(Cuneiform::SignUr2TimesU2),
            '𒌲' => Ok(Cuneiform::SignUr2TimesU2PlusAsh),
            '𒌳' => Ok(Cuneiform::SignUr2TimesU2PlusBi),
            '𒌴' => Ok(Cuneiform::SignUr4),
            '𒌵' => Ok(Cuneiform::SignUri),
            '𒌶' => Ok(Cuneiform::SignUri3),
            '𒌷' => Ok(Cuneiform::SignUru),
            '𒌸' => Ok(Cuneiform::SignUruTimesA),
            '𒌹' => Ok(Cuneiform::SignUruTimesAshgab),
            '𒌺' => Ok(Cuneiform::SignUruTimesBar),
            '𒌻' => Ok(Cuneiform::SignUruTimesDun),
            '𒌼' => Ok(Cuneiform::SignUruTimesGa),
            '𒌽' => Ok(Cuneiform::SignUruTimesGal),
            '𒌾' => Ok(Cuneiform::SignUruTimesGan2Tenu),
            '𒌿' => Ok(Cuneiform::SignUruTimesGar),
            '𒍀' => Ok(Cuneiform::SignUruTimesGu),
            '𒍁' => Ok(Cuneiform::SignUruTimesHa),
            '𒍂' => Ok(Cuneiform::SignUruTimesIgi),
            '𒍃' => Ok(Cuneiform::SignUruTimesIm),
            '𒍄' => Ok(Cuneiform::SignUruTimesIsh),
            '𒍅' => Ok(Cuneiform::SignUruTimesKi),
            '𒍆' => Ok(Cuneiform::SignUruTimesLum),
            '𒍇' => Ok(Cuneiform::SignUruTimesMin),
            '𒍈' => Ok(Cuneiform::SignUruTimesPa),
            '𒍉' => Ok(Cuneiform::SignUruTimesShe),
            '𒍊' => Ok(Cuneiform::SignUruTimesSig4),
            '𒍋' => Ok(Cuneiform::SignUruTimesTu),
            '𒍌' => Ok(Cuneiform::SignUruTimesUPlusGud),
            '𒍍' => Ok(Cuneiform::SignUruTimesUd),
            '𒍎' => Ok(Cuneiform::SignUruTimesUruda),
            '𒍏' => Ok(Cuneiform::SignUruda),
            '𒍐' => Ok(Cuneiform::SignUrudaTimesU),
            '𒍑' => Ok(Cuneiform::SignUsh),
            '𒍒' => Ok(Cuneiform::SignUshTimesA),
            '𒍓' => Ok(Cuneiform::SignUshTimesKu),
            '𒍔' => Ok(Cuneiform::SignUshTimesKur),
            '𒍕' => Ok(Cuneiform::SignUshTimesTak4),
            '𒍖' => Ok(Cuneiform::SignUshx),
            '𒍗' => Ok(Cuneiform::SignUsh2),
            '𒍘' => Ok(Cuneiform::SignUshumx),
            '𒍙' => Ok(Cuneiform::SignUtuki),
            '𒍚' => Ok(Cuneiform::SignUz3),
            '𒍛' => Ok(Cuneiform::SignUz3TimesKaskal),
            '𒍜' => Ok(Cuneiform::SignUzu),
            '𒍝' => Ok(Cuneiform::SignZa),
            '𒍞' => Ok(Cuneiform::SignZaTenu),
            '𒍟' => Ok(Cuneiform::SignZaSquaredTimesKur),
            '𒍠' => Ok(Cuneiform::SignZag),
            '𒍡' => Ok(Cuneiform::SignZamx),
            '𒍢' => Ok(Cuneiform::SignZe2),
            '𒍣' => Ok(Cuneiform::SignZi),
            '𒍤' => Ok(Cuneiform::SignZiOverZi),
            '𒍥' => Ok(Cuneiform::SignZi3),
            '𒍦' => Ok(Cuneiform::SignZib),
            '𒍧' => Ok(Cuneiform::SignZibKabaTenu),
            '𒍨' => Ok(Cuneiform::SignZig),
            '𒍩' => Ok(Cuneiform::SignZiz2),
            '𒍪' => Ok(Cuneiform::SignZu),
            '𒍫' => Ok(Cuneiform::SignZu5),
            '𒍬' => Ok(Cuneiform::SignZu5TimesA),
            '𒍭' => Ok(Cuneiform::SignZubur),
            '𒍮' => Ok(Cuneiform::SignZum),
            '𒍯' => Ok(Cuneiform::SignKapElamite),
            '𒍰' => Ok(Cuneiform::SignAbTimesNun),
            '𒍱' => Ok(Cuneiform::SignAb2TimesA),
            '𒍲' => Ok(Cuneiform::SignAmarTimesKug),
            '𒍳' => Ok(Cuneiform::SignDagKisim5TimesU2PlusMash),
            '𒍴' => Ok(Cuneiform::SignDag3),
            '𒍵' => Ok(Cuneiform::SignDishPlusShu),
            '𒍶' => Ok(Cuneiform::SignDubTimesShe),
            '𒍷' => Ok(Cuneiform::SignEzenTimesGud),
            '𒍸' => Ok(Cuneiform::SignEzenTimesShe),
            '𒍹' => Ok(Cuneiform::SignGa2TimesAnPlusKakPlusA),
            '𒍺' => Ok(Cuneiform::SignGa2TimesAsh2),
            '𒍻' => Ok(Cuneiform::SignGe22),
            '𒍼' => Ok(Cuneiform::SignGig),
            '𒍽' => Ok(Cuneiform::SignHush),
            '𒍾' => Ok(Cuneiform::SignKaTimesAnshe),
            '𒍿' => Ok(Cuneiform::SignKaTimesAsh3),
            '𒎀' => Ok(Cuneiform::SignKaTimesGish),
            '𒎁' => Ok(Cuneiform::SignKaTimesGud),
            '𒎂' => Ok(Cuneiform::SignKaTimesHiTimesAsh2),
            '𒎃' => Ok(Cuneiform::SignKaTimesLum),
            '𒎄' => Ok(Cuneiform::SignKaTimesPa),
            '𒎅' => Ok(Cuneiform::SignKaTimesShul),
            '𒎆' => Ok(Cuneiform::SignKaTimesTu),
            '𒎇' => Ok(Cuneiform::SignKaTimesUr2),
            '𒎈' => Ok(Cuneiform::SignLagabTimesGi),
            '𒎉' => Ok(Cuneiform::SignLu2SheshigTimesBad),
            '𒎊' => Ok(Cuneiform::SignLu2TimesEsh2PlusLal),
            '𒎋' => Ok(Cuneiform::SignLu2TimesShu),
            '𒎌' => Ok(Cuneiform::SignMesh),
            '𒎍' => Ok(Cuneiform::SignMush3TimesZa),
            '𒎎' => Ok(Cuneiform::SignNa4),
            '𒎏' => Ok(Cuneiform::SignNin),
            '𒎐' => Ok(Cuneiform::SignNin9),
            '𒎑' => Ok(Cuneiform::SignNinda2TimesBal),
            '𒎒' => Ok(Cuneiform::SignNinda2TimesGi),
            '𒎓' => Ok(Cuneiform::SignNu11RotatedNinetyDegrees),
            '𒎔' => Ok(Cuneiform::SignPesh2Asterisk),
            '𒎕' => Ok(Cuneiform::SignPir2),
            '𒎖' => Ok(Cuneiform::SignSagTimesIgiGunu),
            '𒎗' => Ok(Cuneiform::SignTi2),
            '𒎘' => Ok(Cuneiform::SignUmTimesMe),
            '𒎙' => Ok(Cuneiform::SignUU),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Cuneiform {
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

impl std::convert::TryFrom<u32> for Cuneiform {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Cuneiform {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Cuneiform {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Cuneiform::SignA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Cuneiform{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
