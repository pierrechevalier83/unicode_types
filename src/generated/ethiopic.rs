
/// An enum to represent all characters in the Ethiopic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ethiopic {
    /// \u{1200}: 'ሀ'
    SyllableHa,
    /// \u{1201}: 'ሁ'
    SyllableHu,
    /// \u{1202}: 'ሂ'
    SyllableHi,
    /// \u{1203}: 'ሃ'
    SyllableHaa,
    /// \u{1204}: 'ሄ'
    SyllableHee,
    /// \u{1205}: 'ህ'
    SyllableHe,
    /// \u{1206}: 'ሆ'
    SyllableHo,
    /// \u{1207}: 'ሇ'
    SyllableHoa,
    /// \u{1208}: 'ለ'
    SyllableLa,
    /// \u{1209}: 'ሉ'
    SyllableLu,
    /// \u{120a}: 'ሊ'
    SyllableLi,
    /// \u{120b}: 'ላ'
    SyllableLaa,
    /// \u{120c}: 'ሌ'
    SyllableLee,
    /// \u{120d}: 'ል'
    SyllableLe,
    /// \u{120e}: 'ሎ'
    SyllableLo,
    /// \u{120f}: 'ሏ'
    SyllableLwa,
    /// \u{1210}: 'ሐ'
    SyllableHha,
    /// \u{1211}: 'ሑ'
    SyllableHhu,
    /// \u{1212}: 'ሒ'
    SyllableHhi,
    /// \u{1213}: 'ሓ'
    SyllableHhaa,
    /// \u{1214}: 'ሔ'
    SyllableHhee,
    /// \u{1215}: 'ሕ'
    SyllableHhe,
    /// \u{1216}: 'ሖ'
    SyllableHho,
    /// \u{1217}: 'ሗ'
    SyllableHhwa,
    /// \u{1218}: 'መ'
    SyllableMa,
    /// \u{1219}: 'ሙ'
    SyllableMu,
    /// \u{121a}: 'ሚ'
    SyllableMi,
    /// \u{121b}: 'ማ'
    SyllableMaa,
    /// \u{121c}: 'ሜ'
    SyllableMee,
    /// \u{121d}: 'ም'
    SyllableMe,
    /// \u{121e}: 'ሞ'
    SyllableMo,
    /// \u{121f}: 'ሟ'
    SyllableMwa,
    /// \u{1220}: 'ሠ'
    SyllableSza,
    /// \u{1221}: 'ሡ'
    SyllableSzu,
    /// \u{1222}: 'ሢ'
    SyllableSzi,
    /// \u{1223}: 'ሣ'
    SyllableSzaa,
    /// \u{1224}: 'ሤ'
    SyllableSzee,
    /// \u{1225}: 'ሥ'
    SyllableSze,
    /// \u{1226}: 'ሦ'
    SyllableSzo,
    /// \u{1227}: 'ሧ'
    SyllableSzwa,
    /// \u{1228}: 'ረ'
    SyllableRa,
    /// \u{1229}: 'ሩ'
    SyllableRu,
    /// \u{122a}: 'ሪ'
    SyllableRi,
    /// \u{122b}: 'ራ'
    SyllableRaa,
    /// \u{122c}: 'ሬ'
    SyllableRee,
    /// \u{122d}: 'ር'
    SyllableRe,
    /// \u{122e}: 'ሮ'
    SyllableRo,
    /// \u{122f}: 'ሯ'
    SyllableRwa,
    /// \u{1230}: 'ሰ'
    SyllableSa,
    /// \u{1231}: 'ሱ'
    SyllableSu,
    /// \u{1232}: 'ሲ'
    SyllableSi,
    /// \u{1233}: 'ሳ'
    SyllableSaa,
    /// \u{1234}: 'ሴ'
    SyllableSee,
    /// \u{1235}: 'ስ'
    SyllableSe,
    /// \u{1236}: 'ሶ'
    SyllableSo,
    /// \u{1237}: 'ሷ'
    SyllableSwa,
    /// \u{1238}: 'ሸ'
    SyllableSha,
    /// \u{1239}: 'ሹ'
    SyllableShu,
    /// \u{123a}: 'ሺ'
    SyllableShi,
    /// \u{123b}: 'ሻ'
    SyllableShaa,
    /// \u{123c}: 'ሼ'
    SyllableShee,
    /// \u{123d}: 'ሽ'
    SyllableShe,
    /// \u{123e}: 'ሾ'
    SyllableSho,
    /// \u{123f}: 'ሿ'
    SyllableShwa,
    /// \u{1240}: 'ቀ'
    SyllableQa,
    /// \u{1241}: 'ቁ'
    SyllableQu,
    /// \u{1242}: 'ቂ'
    SyllableQi,
    /// \u{1243}: 'ቃ'
    SyllableQaa,
    /// \u{1244}: 'ቄ'
    SyllableQee,
    /// \u{1245}: 'ቅ'
    SyllableQe,
    /// \u{1246}: 'ቆ'
    SyllableQo,
    /// \u{1247}: 'ቇ'
    SyllableQoa,
    /// \u{1248}: 'ቈ'
    SyllableQwa,
    /// \u{124a}: 'ቊ'
    SyllableQwi,
    /// \u{124b}: 'ቋ'
    SyllableQwaa,
    /// \u{124c}: 'ቌ'
    SyllableQwee,
    /// \u{124d}: 'ቍ'
    SyllableQwe,
    /// \u{1250}: 'ቐ'
    SyllableQha,
    /// \u{1251}: 'ቑ'
    SyllableQhu,
    /// \u{1252}: 'ቒ'
    SyllableQhi,
    /// \u{1253}: 'ቓ'
    SyllableQhaa,
    /// \u{1254}: 'ቔ'
    SyllableQhee,
    /// \u{1255}: 'ቕ'
    SyllableQhe,
    /// \u{1256}: 'ቖ'
    SyllableQho,
    /// \u{1258}: 'ቘ'
    SyllableQhwa,
    /// \u{125a}: 'ቚ'
    SyllableQhwi,
    /// \u{125b}: 'ቛ'
    SyllableQhwaa,
    /// \u{125c}: 'ቜ'
    SyllableQhwee,
    /// \u{125d}: 'ቝ'
    SyllableQhwe,
    /// \u{1260}: 'በ'
    SyllableBa,
    /// \u{1261}: 'ቡ'
    SyllableBu,
    /// \u{1262}: 'ቢ'
    SyllableBi,
    /// \u{1263}: 'ባ'
    SyllableBaa,
    /// \u{1264}: 'ቤ'
    SyllableBee,
    /// \u{1265}: 'ብ'
    SyllableBe,
    /// \u{1266}: 'ቦ'
    SyllableBo,
    /// \u{1267}: 'ቧ'
    SyllableBwa,
    /// \u{1268}: 'ቨ'
    SyllableVa,
    /// \u{1269}: 'ቩ'
    SyllableVu,
    /// \u{126a}: 'ቪ'
    SyllableVi,
    /// \u{126b}: 'ቫ'
    SyllableVaa,
    /// \u{126c}: 'ቬ'
    SyllableVee,
    /// \u{126d}: 'ቭ'
    SyllableVe,
    /// \u{126e}: 'ቮ'
    SyllableVo,
    /// \u{126f}: 'ቯ'
    SyllableVwa,
    /// \u{1270}: 'ተ'
    SyllableTa,
    /// \u{1271}: 'ቱ'
    SyllableTu,
    /// \u{1272}: 'ቲ'
    SyllableTi,
    /// \u{1273}: 'ታ'
    SyllableTaa,
    /// \u{1274}: 'ቴ'
    SyllableTee,
    /// \u{1275}: 'ት'
    SyllableTe,
    /// \u{1276}: 'ቶ'
    SyllableTo,
    /// \u{1277}: 'ቷ'
    SyllableTwa,
    /// \u{1278}: 'ቸ'
    SyllableCa,
    /// \u{1279}: 'ቹ'
    SyllableCu,
    /// \u{127a}: 'ቺ'
    SyllableCi,
    /// \u{127b}: 'ቻ'
    SyllableCaa,
    /// \u{127c}: 'ቼ'
    SyllableCee,
    /// \u{127d}: 'ች'
    SyllableCe,
    /// \u{127e}: 'ቾ'
    SyllableCo,
    /// \u{127f}: 'ቿ'
    SyllableCwa,
    /// \u{1280}: 'ኀ'
    SyllableXa,
    /// \u{1281}: 'ኁ'
    SyllableXu,
    /// \u{1282}: 'ኂ'
    SyllableXi,
    /// \u{1283}: 'ኃ'
    SyllableXaa,
    /// \u{1284}: 'ኄ'
    SyllableXee,
    /// \u{1285}: 'ኅ'
    SyllableXe,
    /// \u{1286}: 'ኆ'
    SyllableXo,
    /// \u{1287}: 'ኇ'
    SyllableXoa,
    /// \u{1288}: 'ኈ'
    SyllableXwa,
    /// \u{128a}: 'ኊ'
    SyllableXwi,
    /// \u{128b}: 'ኋ'
    SyllableXwaa,
    /// \u{128c}: 'ኌ'
    SyllableXwee,
    /// \u{128d}: 'ኍ'
    SyllableXwe,
    /// \u{1290}: 'ነ'
    SyllableNa,
    /// \u{1291}: 'ኑ'
    SyllableNu,
    /// \u{1292}: 'ኒ'
    SyllableNi,
    /// \u{1293}: 'ና'
    SyllableNaa,
    /// \u{1294}: 'ኔ'
    SyllableNee,
    /// \u{1295}: 'ን'
    SyllableNe,
    /// \u{1296}: 'ኖ'
    SyllableNo,
    /// \u{1297}: 'ኗ'
    SyllableNwa,
    /// \u{1298}: 'ኘ'
    SyllableNya,
    /// \u{1299}: 'ኙ'
    SyllableNyu,
    /// \u{129a}: 'ኚ'
    SyllableNyi,
    /// \u{129b}: 'ኛ'
    SyllableNyaa,
    /// \u{129c}: 'ኜ'
    SyllableNyee,
    /// \u{129d}: 'ኝ'
    SyllableNye,
    /// \u{129e}: 'ኞ'
    SyllableNyo,
    /// \u{129f}: 'ኟ'
    SyllableNywa,
    /// \u{12a0}: 'አ'
    SyllableGlottalA,
    /// \u{12a1}: 'ኡ'
    SyllableGlottalU,
    /// \u{12a2}: 'ኢ'
    SyllableGlottalI,
    /// \u{12a3}: 'ኣ'
    SyllableGlottalAa,
    /// \u{12a4}: 'ኤ'
    SyllableGlottalEe,
    /// \u{12a5}: 'እ'
    SyllableGlottalE,
    /// \u{12a6}: 'ኦ'
    SyllableGlottalO,
    /// \u{12a7}: 'ኧ'
    SyllableGlottalWa,
    /// \u{12a8}: 'ከ'
    SyllableKa,
    /// \u{12a9}: 'ኩ'
    SyllableKu,
    /// \u{12aa}: 'ኪ'
    SyllableKi,
    /// \u{12ab}: 'ካ'
    SyllableKaa,
    /// \u{12ac}: 'ኬ'
    SyllableKee,
    /// \u{12ad}: 'ክ'
    SyllableKe,
    /// \u{12ae}: 'ኮ'
    SyllableKo,
    /// \u{12af}: 'ኯ'
    SyllableKoa,
    /// \u{12b0}: 'ኰ'
    SyllableKwa,
    /// \u{12b2}: 'ኲ'
    SyllableKwi,
    /// \u{12b3}: 'ኳ'
    SyllableKwaa,
    /// \u{12b4}: 'ኴ'
    SyllableKwee,
    /// \u{12b5}: 'ኵ'
    SyllableKwe,
    /// \u{12b8}: 'ኸ'
    SyllableKxa,
    /// \u{12b9}: 'ኹ'
    SyllableKxu,
    /// \u{12ba}: 'ኺ'
    SyllableKxi,
    /// \u{12bb}: 'ኻ'
    SyllableKxaa,
    /// \u{12bc}: 'ኼ'
    SyllableKxee,
    /// \u{12bd}: 'ኽ'
    SyllableKxe,
    /// \u{12be}: 'ኾ'
    SyllableKxo,
    /// \u{12c0}: 'ዀ'
    SyllableKxwa,
    /// \u{12c2}: 'ዂ'
    SyllableKxwi,
    /// \u{12c3}: 'ዃ'
    SyllableKxwaa,
    /// \u{12c4}: 'ዄ'
    SyllableKxwee,
    /// \u{12c5}: 'ዅ'
    SyllableKxwe,
    /// \u{12c8}: 'ወ'
    SyllableWa,
    /// \u{12c9}: 'ዉ'
    SyllableWu,
    /// \u{12ca}: 'ዊ'
    SyllableWi,
    /// \u{12cb}: 'ዋ'
    SyllableWaa,
    /// \u{12cc}: 'ዌ'
    SyllableWee,
    /// \u{12cd}: 'ው'
    SyllableWe,
    /// \u{12ce}: 'ዎ'
    SyllableWo,
    /// \u{12cf}: 'ዏ'
    SyllableWoa,
    /// \u{12d0}: 'ዐ'
    SyllablePharyngealA,
    /// \u{12d1}: 'ዑ'
    SyllablePharyngealU,
    /// \u{12d2}: 'ዒ'
    SyllablePharyngealI,
    /// \u{12d3}: 'ዓ'
    SyllablePharyngealAa,
    /// \u{12d4}: 'ዔ'
    SyllablePharyngealEe,
    /// \u{12d5}: 'ዕ'
    SyllablePharyngealE,
    /// \u{12d6}: 'ዖ'
    SyllablePharyngealO,
    /// \u{12d8}: 'ዘ'
    SyllableZa,
    /// \u{12d9}: 'ዙ'
    SyllableZu,
    /// \u{12da}: 'ዚ'
    SyllableZi,
    /// \u{12db}: 'ዛ'
    SyllableZaa,
    /// \u{12dc}: 'ዜ'
    SyllableZee,
    /// \u{12dd}: 'ዝ'
    SyllableZe,
    /// \u{12de}: 'ዞ'
    SyllableZo,
    /// \u{12df}: 'ዟ'
    SyllableZwa,
    /// \u{12e0}: 'ዠ'
    SyllableZha,
    /// \u{12e1}: 'ዡ'
    SyllableZhu,
    /// \u{12e2}: 'ዢ'
    SyllableZhi,
    /// \u{12e3}: 'ዣ'
    SyllableZhaa,
    /// \u{12e4}: 'ዤ'
    SyllableZhee,
    /// \u{12e5}: 'ዥ'
    SyllableZhe,
    /// \u{12e6}: 'ዦ'
    SyllableZho,
    /// \u{12e7}: 'ዧ'
    SyllableZhwa,
    /// \u{12e8}: 'የ'
    SyllableYa,
    /// \u{12e9}: 'ዩ'
    SyllableYu,
    /// \u{12ea}: 'ዪ'
    SyllableYi,
    /// \u{12eb}: 'ያ'
    SyllableYaa,
    /// \u{12ec}: 'ዬ'
    SyllableYee,
    /// \u{12ed}: 'ይ'
    SyllableYe,
    /// \u{12ee}: 'ዮ'
    SyllableYo,
    /// \u{12ef}: 'ዯ'
    SyllableYoa,
    /// \u{12f0}: 'ደ'
    SyllableDa,
    /// \u{12f1}: 'ዱ'
    SyllableDu,
    /// \u{12f2}: 'ዲ'
    SyllableDi,
    /// \u{12f3}: 'ዳ'
    SyllableDaa,
    /// \u{12f4}: 'ዴ'
    SyllableDee,
    /// \u{12f5}: 'ድ'
    SyllableDe,
    /// \u{12f6}: 'ዶ'
    SyllableDo,
    /// \u{12f7}: 'ዷ'
    SyllableDwa,
    /// \u{12f8}: 'ዸ'
    SyllableDda,
    /// \u{12f9}: 'ዹ'
    SyllableDdu,
    /// \u{12fa}: 'ዺ'
    SyllableDdi,
    /// \u{12fb}: 'ዻ'
    SyllableDdaa,
    /// \u{12fc}: 'ዼ'
    SyllableDdee,
    /// \u{12fd}: 'ዽ'
    SyllableDde,
    /// \u{12fe}: 'ዾ'
    SyllableDdo,
    /// \u{12ff}: 'ዿ'
    SyllableDdwa,
    /// \u{1300}: 'ጀ'
    SyllableJa,
    /// \u{1301}: 'ጁ'
    SyllableJu,
    /// \u{1302}: 'ጂ'
    SyllableJi,
    /// \u{1303}: 'ጃ'
    SyllableJaa,
    /// \u{1304}: 'ጄ'
    SyllableJee,
    /// \u{1305}: 'ጅ'
    SyllableJe,
    /// \u{1306}: 'ጆ'
    SyllableJo,
    /// \u{1307}: 'ጇ'
    SyllableJwa,
    /// \u{1308}: 'ገ'
    SyllableGa,
    /// \u{1309}: 'ጉ'
    SyllableGu,
    /// \u{130a}: 'ጊ'
    SyllableGi,
    /// \u{130b}: 'ጋ'
    SyllableGaa,
    /// \u{130c}: 'ጌ'
    SyllableGee,
    /// \u{130d}: 'ግ'
    SyllableGe,
    /// \u{130e}: 'ጎ'
    SyllableGo,
    /// \u{130f}: 'ጏ'
    SyllableGoa,
    /// \u{1310}: 'ጐ'
    SyllableGwa,
    /// \u{1312}: 'ጒ'
    SyllableGwi,
    /// \u{1313}: 'ጓ'
    SyllableGwaa,
    /// \u{1314}: 'ጔ'
    SyllableGwee,
    /// \u{1315}: 'ጕ'
    SyllableGwe,
    /// \u{1318}: 'ጘ'
    SyllableGga,
    /// \u{1319}: 'ጙ'
    SyllableGgu,
    /// \u{131a}: 'ጚ'
    SyllableGgi,
    /// \u{131b}: 'ጛ'
    SyllableGgaa,
    /// \u{131c}: 'ጜ'
    SyllableGgee,
    /// \u{131d}: 'ጝ'
    SyllableGge,
    /// \u{131e}: 'ጞ'
    SyllableGgo,
    /// \u{131f}: 'ጟ'
    SyllableGgwaa,
    /// \u{1320}: 'ጠ'
    SyllableTha,
    /// \u{1321}: 'ጡ'
    SyllableThu,
    /// \u{1322}: 'ጢ'
    SyllableThi,
    /// \u{1323}: 'ጣ'
    SyllableThaa,
    /// \u{1324}: 'ጤ'
    SyllableThee,
    /// \u{1325}: 'ጥ'
    SyllableThe,
    /// \u{1326}: 'ጦ'
    SyllableTho,
    /// \u{1327}: 'ጧ'
    SyllableThwa,
    /// \u{1328}: 'ጨ'
    SyllableCha,
    /// \u{1329}: 'ጩ'
    SyllableChu,
    /// \u{132a}: 'ጪ'
    SyllableChi,
    /// \u{132b}: 'ጫ'
    SyllableChaa,
    /// \u{132c}: 'ጬ'
    SyllableChee,
    /// \u{132d}: 'ጭ'
    SyllableChe,
    /// \u{132e}: 'ጮ'
    SyllableCho,
    /// \u{132f}: 'ጯ'
    SyllableChwa,
    /// \u{1330}: 'ጰ'
    SyllablePha,
    /// \u{1331}: 'ጱ'
    SyllablePhu,
    /// \u{1332}: 'ጲ'
    SyllablePhi,
    /// \u{1333}: 'ጳ'
    SyllablePhaa,
    /// \u{1334}: 'ጴ'
    SyllablePhee,
    /// \u{1335}: 'ጵ'
    SyllablePhe,
    /// \u{1336}: 'ጶ'
    SyllablePho,
    /// \u{1337}: 'ጷ'
    SyllablePhwa,
    /// \u{1338}: 'ጸ'
    SyllableTsa,
    /// \u{1339}: 'ጹ'
    SyllableTsu,
    /// \u{133a}: 'ጺ'
    SyllableTsi,
    /// \u{133b}: 'ጻ'
    SyllableTsaa,
    /// \u{133c}: 'ጼ'
    SyllableTsee,
    /// \u{133d}: 'ጽ'
    SyllableTse,
    /// \u{133e}: 'ጾ'
    SyllableTso,
    /// \u{133f}: 'ጿ'
    SyllableTswa,
    /// \u{1340}: 'ፀ'
    SyllableTza,
    /// \u{1341}: 'ፁ'
    SyllableTzu,
    /// \u{1342}: 'ፂ'
    SyllableTzi,
    /// \u{1343}: 'ፃ'
    SyllableTzaa,
    /// \u{1344}: 'ፄ'
    SyllableTzee,
    /// \u{1345}: 'ፅ'
    SyllableTze,
    /// \u{1346}: 'ፆ'
    SyllableTzo,
    /// \u{1347}: 'ፇ'
    SyllableTzoa,
    /// \u{1348}: 'ፈ'
    SyllableFa,
    /// \u{1349}: 'ፉ'
    SyllableFu,
    /// \u{134a}: 'ፊ'
    SyllableFi,
    /// \u{134b}: 'ፋ'
    SyllableFaa,
    /// \u{134c}: 'ፌ'
    SyllableFee,
    /// \u{134d}: 'ፍ'
    SyllableFe,
    /// \u{134e}: 'ፎ'
    SyllableFo,
    /// \u{134f}: 'ፏ'
    SyllableFwa,
    /// \u{1350}: 'ፐ'
    SyllablePa,
    /// \u{1351}: 'ፑ'
    SyllablePu,
    /// \u{1352}: 'ፒ'
    SyllablePi,
    /// \u{1353}: 'ፓ'
    SyllablePaa,
    /// \u{1354}: 'ፔ'
    SyllablePee,
    /// \u{1355}: 'ፕ'
    SyllablePe,
    /// \u{1356}: 'ፖ'
    SyllablePo,
    /// \u{1357}: 'ፗ'
    SyllablePwa,
    /// \u{1358}: 'ፘ'
    SyllableRya,
    /// \u{1359}: 'ፙ'
    SyllableMya,
    /// \u{135a}: 'ፚ'
    SyllableFya,
    /// \u{135d}: '፝'
    CombiningGeminationAndVowelLengthMark,
    /// \u{135e}: '፞'
    CombiningVowelLengthMark,
    /// \u{135f}: '፟'
    CombiningGeminationMark,
    /// \u{1360}: '፠'
    SectionMark,
    /// \u{1361}: '፡'
    Wordspace,
    /// \u{1362}: '።'
    FullStop,
    /// \u{1363}: '፣'
    Comma,
    /// \u{1364}: '፤'
    Semicolon,
    /// \u{1365}: '፥'
    Colon,
    /// \u{1366}: '፦'
    PrefaceColon,
    /// \u{1367}: '፧'
    QuestionMark,
    /// \u{1368}: '፨'
    ParagraphSeparator,
    /// \u{1369}: '፩'
    DigitOne,
    /// \u{136a}: '፪'
    DigitTwo,
    /// \u{136b}: '፫'
    DigitThree,
    /// \u{136c}: '፬'
    DigitFour,
    /// \u{136d}: '፭'
    DigitFive,
    /// \u{136e}: '፮'
    DigitSix,
    /// \u{136f}: '፯'
    DigitSeven,
    /// \u{1370}: '፰'
    DigitEight,
    /// \u{1371}: '፱'
    DigitNine,
    /// \u{1372}: '፲'
    NumberTen,
    /// \u{1373}: '፳'
    NumberTwenty,
    /// \u{1374}: '፴'
    NumberThirty,
    /// \u{1375}: '፵'
    NumberForty,
    /// \u{1376}: '፶'
    NumberFifty,
    /// \u{1377}: '፷'
    NumberSixty,
    /// \u{1378}: '፸'
    NumberSeventy,
    /// \u{1379}: '፹'
    NumberEighty,
    /// \u{137a}: '፺'
    NumberNinety,
    /// \u{137b}: '፻'
    NumberHundred,
    /// \u{137c}: '፼'
    NumberTenThousand,
}

impl Into<char> for Ethiopic {
    fn into(self) -> char {
        match self {
            Ethiopic::SyllableHa => 'ሀ',
            Ethiopic::SyllableHu => 'ሁ',
            Ethiopic::SyllableHi => 'ሂ',
            Ethiopic::SyllableHaa => 'ሃ',
            Ethiopic::SyllableHee => 'ሄ',
            Ethiopic::SyllableHe => 'ህ',
            Ethiopic::SyllableHo => 'ሆ',
            Ethiopic::SyllableHoa => 'ሇ',
            Ethiopic::SyllableLa => 'ለ',
            Ethiopic::SyllableLu => 'ሉ',
            Ethiopic::SyllableLi => 'ሊ',
            Ethiopic::SyllableLaa => 'ላ',
            Ethiopic::SyllableLee => 'ሌ',
            Ethiopic::SyllableLe => 'ል',
            Ethiopic::SyllableLo => 'ሎ',
            Ethiopic::SyllableLwa => 'ሏ',
            Ethiopic::SyllableHha => 'ሐ',
            Ethiopic::SyllableHhu => 'ሑ',
            Ethiopic::SyllableHhi => 'ሒ',
            Ethiopic::SyllableHhaa => 'ሓ',
            Ethiopic::SyllableHhee => 'ሔ',
            Ethiopic::SyllableHhe => 'ሕ',
            Ethiopic::SyllableHho => 'ሖ',
            Ethiopic::SyllableHhwa => 'ሗ',
            Ethiopic::SyllableMa => 'መ',
            Ethiopic::SyllableMu => 'ሙ',
            Ethiopic::SyllableMi => 'ሚ',
            Ethiopic::SyllableMaa => 'ማ',
            Ethiopic::SyllableMee => 'ሜ',
            Ethiopic::SyllableMe => 'ም',
            Ethiopic::SyllableMo => 'ሞ',
            Ethiopic::SyllableMwa => 'ሟ',
            Ethiopic::SyllableSza => 'ሠ',
            Ethiopic::SyllableSzu => 'ሡ',
            Ethiopic::SyllableSzi => 'ሢ',
            Ethiopic::SyllableSzaa => 'ሣ',
            Ethiopic::SyllableSzee => 'ሤ',
            Ethiopic::SyllableSze => 'ሥ',
            Ethiopic::SyllableSzo => 'ሦ',
            Ethiopic::SyllableSzwa => 'ሧ',
            Ethiopic::SyllableRa => 'ረ',
            Ethiopic::SyllableRu => 'ሩ',
            Ethiopic::SyllableRi => 'ሪ',
            Ethiopic::SyllableRaa => 'ራ',
            Ethiopic::SyllableRee => 'ሬ',
            Ethiopic::SyllableRe => 'ር',
            Ethiopic::SyllableRo => 'ሮ',
            Ethiopic::SyllableRwa => 'ሯ',
            Ethiopic::SyllableSa => 'ሰ',
            Ethiopic::SyllableSu => 'ሱ',
            Ethiopic::SyllableSi => 'ሲ',
            Ethiopic::SyllableSaa => 'ሳ',
            Ethiopic::SyllableSee => 'ሴ',
            Ethiopic::SyllableSe => 'ስ',
            Ethiopic::SyllableSo => 'ሶ',
            Ethiopic::SyllableSwa => 'ሷ',
            Ethiopic::SyllableSha => 'ሸ',
            Ethiopic::SyllableShu => 'ሹ',
            Ethiopic::SyllableShi => 'ሺ',
            Ethiopic::SyllableShaa => 'ሻ',
            Ethiopic::SyllableShee => 'ሼ',
            Ethiopic::SyllableShe => 'ሽ',
            Ethiopic::SyllableSho => 'ሾ',
            Ethiopic::SyllableShwa => 'ሿ',
            Ethiopic::SyllableQa => 'ቀ',
            Ethiopic::SyllableQu => 'ቁ',
            Ethiopic::SyllableQi => 'ቂ',
            Ethiopic::SyllableQaa => 'ቃ',
            Ethiopic::SyllableQee => 'ቄ',
            Ethiopic::SyllableQe => 'ቅ',
            Ethiopic::SyllableQo => 'ቆ',
            Ethiopic::SyllableQoa => 'ቇ',
            Ethiopic::SyllableQwa => 'ቈ',
            Ethiopic::SyllableQwi => 'ቊ',
            Ethiopic::SyllableQwaa => 'ቋ',
            Ethiopic::SyllableQwee => 'ቌ',
            Ethiopic::SyllableQwe => 'ቍ',
            Ethiopic::SyllableQha => 'ቐ',
            Ethiopic::SyllableQhu => 'ቑ',
            Ethiopic::SyllableQhi => 'ቒ',
            Ethiopic::SyllableQhaa => 'ቓ',
            Ethiopic::SyllableQhee => 'ቔ',
            Ethiopic::SyllableQhe => 'ቕ',
            Ethiopic::SyllableQho => 'ቖ',
            Ethiopic::SyllableQhwa => 'ቘ',
            Ethiopic::SyllableQhwi => 'ቚ',
            Ethiopic::SyllableQhwaa => 'ቛ',
            Ethiopic::SyllableQhwee => 'ቜ',
            Ethiopic::SyllableQhwe => 'ቝ',
            Ethiopic::SyllableBa => 'በ',
            Ethiopic::SyllableBu => 'ቡ',
            Ethiopic::SyllableBi => 'ቢ',
            Ethiopic::SyllableBaa => 'ባ',
            Ethiopic::SyllableBee => 'ቤ',
            Ethiopic::SyllableBe => 'ብ',
            Ethiopic::SyllableBo => 'ቦ',
            Ethiopic::SyllableBwa => 'ቧ',
            Ethiopic::SyllableVa => 'ቨ',
            Ethiopic::SyllableVu => 'ቩ',
            Ethiopic::SyllableVi => 'ቪ',
            Ethiopic::SyllableVaa => 'ቫ',
            Ethiopic::SyllableVee => 'ቬ',
            Ethiopic::SyllableVe => 'ቭ',
            Ethiopic::SyllableVo => 'ቮ',
            Ethiopic::SyllableVwa => 'ቯ',
            Ethiopic::SyllableTa => 'ተ',
            Ethiopic::SyllableTu => 'ቱ',
            Ethiopic::SyllableTi => 'ቲ',
            Ethiopic::SyllableTaa => 'ታ',
            Ethiopic::SyllableTee => 'ቴ',
            Ethiopic::SyllableTe => 'ት',
            Ethiopic::SyllableTo => 'ቶ',
            Ethiopic::SyllableTwa => 'ቷ',
            Ethiopic::SyllableCa => 'ቸ',
            Ethiopic::SyllableCu => 'ቹ',
            Ethiopic::SyllableCi => 'ቺ',
            Ethiopic::SyllableCaa => 'ቻ',
            Ethiopic::SyllableCee => 'ቼ',
            Ethiopic::SyllableCe => 'ች',
            Ethiopic::SyllableCo => 'ቾ',
            Ethiopic::SyllableCwa => 'ቿ',
            Ethiopic::SyllableXa => 'ኀ',
            Ethiopic::SyllableXu => 'ኁ',
            Ethiopic::SyllableXi => 'ኂ',
            Ethiopic::SyllableXaa => 'ኃ',
            Ethiopic::SyllableXee => 'ኄ',
            Ethiopic::SyllableXe => 'ኅ',
            Ethiopic::SyllableXo => 'ኆ',
            Ethiopic::SyllableXoa => 'ኇ',
            Ethiopic::SyllableXwa => 'ኈ',
            Ethiopic::SyllableXwi => 'ኊ',
            Ethiopic::SyllableXwaa => 'ኋ',
            Ethiopic::SyllableXwee => 'ኌ',
            Ethiopic::SyllableXwe => 'ኍ',
            Ethiopic::SyllableNa => 'ነ',
            Ethiopic::SyllableNu => 'ኑ',
            Ethiopic::SyllableNi => 'ኒ',
            Ethiopic::SyllableNaa => 'ና',
            Ethiopic::SyllableNee => 'ኔ',
            Ethiopic::SyllableNe => 'ን',
            Ethiopic::SyllableNo => 'ኖ',
            Ethiopic::SyllableNwa => 'ኗ',
            Ethiopic::SyllableNya => 'ኘ',
            Ethiopic::SyllableNyu => 'ኙ',
            Ethiopic::SyllableNyi => 'ኚ',
            Ethiopic::SyllableNyaa => 'ኛ',
            Ethiopic::SyllableNyee => 'ኜ',
            Ethiopic::SyllableNye => 'ኝ',
            Ethiopic::SyllableNyo => 'ኞ',
            Ethiopic::SyllableNywa => 'ኟ',
            Ethiopic::SyllableGlottalA => 'አ',
            Ethiopic::SyllableGlottalU => 'ኡ',
            Ethiopic::SyllableGlottalI => 'ኢ',
            Ethiopic::SyllableGlottalAa => 'ኣ',
            Ethiopic::SyllableGlottalEe => 'ኤ',
            Ethiopic::SyllableGlottalE => 'እ',
            Ethiopic::SyllableGlottalO => 'ኦ',
            Ethiopic::SyllableGlottalWa => 'ኧ',
            Ethiopic::SyllableKa => 'ከ',
            Ethiopic::SyllableKu => 'ኩ',
            Ethiopic::SyllableKi => 'ኪ',
            Ethiopic::SyllableKaa => 'ካ',
            Ethiopic::SyllableKee => 'ኬ',
            Ethiopic::SyllableKe => 'ክ',
            Ethiopic::SyllableKo => 'ኮ',
            Ethiopic::SyllableKoa => 'ኯ',
            Ethiopic::SyllableKwa => 'ኰ',
            Ethiopic::SyllableKwi => 'ኲ',
            Ethiopic::SyllableKwaa => 'ኳ',
            Ethiopic::SyllableKwee => 'ኴ',
            Ethiopic::SyllableKwe => 'ኵ',
            Ethiopic::SyllableKxa => 'ኸ',
            Ethiopic::SyllableKxu => 'ኹ',
            Ethiopic::SyllableKxi => 'ኺ',
            Ethiopic::SyllableKxaa => 'ኻ',
            Ethiopic::SyllableKxee => 'ኼ',
            Ethiopic::SyllableKxe => 'ኽ',
            Ethiopic::SyllableKxo => 'ኾ',
            Ethiopic::SyllableKxwa => 'ዀ',
            Ethiopic::SyllableKxwi => 'ዂ',
            Ethiopic::SyllableKxwaa => 'ዃ',
            Ethiopic::SyllableKxwee => 'ዄ',
            Ethiopic::SyllableKxwe => 'ዅ',
            Ethiopic::SyllableWa => 'ወ',
            Ethiopic::SyllableWu => 'ዉ',
            Ethiopic::SyllableWi => 'ዊ',
            Ethiopic::SyllableWaa => 'ዋ',
            Ethiopic::SyllableWee => 'ዌ',
            Ethiopic::SyllableWe => 'ው',
            Ethiopic::SyllableWo => 'ዎ',
            Ethiopic::SyllableWoa => 'ዏ',
            Ethiopic::SyllablePharyngealA => 'ዐ',
            Ethiopic::SyllablePharyngealU => 'ዑ',
            Ethiopic::SyllablePharyngealI => 'ዒ',
            Ethiopic::SyllablePharyngealAa => 'ዓ',
            Ethiopic::SyllablePharyngealEe => 'ዔ',
            Ethiopic::SyllablePharyngealE => 'ዕ',
            Ethiopic::SyllablePharyngealO => 'ዖ',
            Ethiopic::SyllableZa => 'ዘ',
            Ethiopic::SyllableZu => 'ዙ',
            Ethiopic::SyllableZi => 'ዚ',
            Ethiopic::SyllableZaa => 'ዛ',
            Ethiopic::SyllableZee => 'ዜ',
            Ethiopic::SyllableZe => 'ዝ',
            Ethiopic::SyllableZo => 'ዞ',
            Ethiopic::SyllableZwa => 'ዟ',
            Ethiopic::SyllableZha => 'ዠ',
            Ethiopic::SyllableZhu => 'ዡ',
            Ethiopic::SyllableZhi => 'ዢ',
            Ethiopic::SyllableZhaa => 'ዣ',
            Ethiopic::SyllableZhee => 'ዤ',
            Ethiopic::SyllableZhe => 'ዥ',
            Ethiopic::SyllableZho => 'ዦ',
            Ethiopic::SyllableZhwa => 'ዧ',
            Ethiopic::SyllableYa => 'የ',
            Ethiopic::SyllableYu => 'ዩ',
            Ethiopic::SyllableYi => 'ዪ',
            Ethiopic::SyllableYaa => 'ያ',
            Ethiopic::SyllableYee => 'ዬ',
            Ethiopic::SyllableYe => 'ይ',
            Ethiopic::SyllableYo => 'ዮ',
            Ethiopic::SyllableYoa => 'ዯ',
            Ethiopic::SyllableDa => 'ደ',
            Ethiopic::SyllableDu => 'ዱ',
            Ethiopic::SyllableDi => 'ዲ',
            Ethiopic::SyllableDaa => 'ዳ',
            Ethiopic::SyllableDee => 'ዴ',
            Ethiopic::SyllableDe => 'ድ',
            Ethiopic::SyllableDo => 'ዶ',
            Ethiopic::SyllableDwa => 'ዷ',
            Ethiopic::SyllableDda => 'ዸ',
            Ethiopic::SyllableDdu => 'ዹ',
            Ethiopic::SyllableDdi => 'ዺ',
            Ethiopic::SyllableDdaa => 'ዻ',
            Ethiopic::SyllableDdee => 'ዼ',
            Ethiopic::SyllableDde => 'ዽ',
            Ethiopic::SyllableDdo => 'ዾ',
            Ethiopic::SyllableDdwa => 'ዿ',
            Ethiopic::SyllableJa => 'ጀ',
            Ethiopic::SyllableJu => 'ጁ',
            Ethiopic::SyllableJi => 'ጂ',
            Ethiopic::SyllableJaa => 'ጃ',
            Ethiopic::SyllableJee => 'ጄ',
            Ethiopic::SyllableJe => 'ጅ',
            Ethiopic::SyllableJo => 'ጆ',
            Ethiopic::SyllableJwa => 'ጇ',
            Ethiopic::SyllableGa => 'ገ',
            Ethiopic::SyllableGu => 'ጉ',
            Ethiopic::SyllableGi => 'ጊ',
            Ethiopic::SyllableGaa => 'ጋ',
            Ethiopic::SyllableGee => 'ጌ',
            Ethiopic::SyllableGe => 'ግ',
            Ethiopic::SyllableGo => 'ጎ',
            Ethiopic::SyllableGoa => 'ጏ',
            Ethiopic::SyllableGwa => 'ጐ',
            Ethiopic::SyllableGwi => 'ጒ',
            Ethiopic::SyllableGwaa => 'ጓ',
            Ethiopic::SyllableGwee => 'ጔ',
            Ethiopic::SyllableGwe => 'ጕ',
            Ethiopic::SyllableGga => 'ጘ',
            Ethiopic::SyllableGgu => 'ጙ',
            Ethiopic::SyllableGgi => 'ጚ',
            Ethiopic::SyllableGgaa => 'ጛ',
            Ethiopic::SyllableGgee => 'ጜ',
            Ethiopic::SyllableGge => 'ጝ',
            Ethiopic::SyllableGgo => 'ጞ',
            Ethiopic::SyllableGgwaa => 'ጟ',
            Ethiopic::SyllableTha => 'ጠ',
            Ethiopic::SyllableThu => 'ጡ',
            Ethiopic::SyllableThi => 'ጢ',
            Ethiopic::SyllableThaa => 'ጣ',
            Ethiopic::SyllableThee => 'ጤ',
            Ethiopic::SyllableThe => 'ጥ',
            Ethiopic::SyllableTho => 'ጦ',
            Ethiopic::SyllableThwa => 'ጧ',
            Ethiopic::SyllableCha => 'ጨ',
            Ethiopic::SyllableChu => 'ጩ',
            Ethiopic::SyllableChi => 'ጪ',
            Ethiopic::SyllableChaa => 'ጫ',
            Ethiopic::SyllableChee => 'ጬ',
            Ethiopic::SyllableChe => 'ጭ',
            Ethiopic::SyllableCho => 'ጮ',
            Ethiopic::SyllableChwa => 'ጯ',
            Ethiopic::SyllablePha => 'ጰ',
            Ethiopic::SyllablePhu => 'ጱ',
            Ethiopic::SyllablePhi => 'ጲ',
            Ethiopic::SyllablePhaa => 'ጳ',
            Ethiopic::SyllablePhee => 'ጴ',
            Ethiopic::SyllablePhe => 'ጵ',
            Ethiopic::SyllablePho => 'ጶ',
            Ethiopic::SyllablePhwa => 'ጷ',
            Ethiopic::SyllableTsa => 'ጸ',
            Ethiopic::SyllableTsu => 'ጹ',
            Ethiopic::SyllableTsi => 'ጺ',
            Ethiopic::SyllableTsaa => 'ጻ',
            Ethiopic::SyllableTsee => 'ጼ',
            Ethiopic::SyllableTse => 'ጽ',
            Ethiopic::SyllableTso => 'ጾ',
            Ethiopic::SyllableTswa => 'ጿ',
            Ethiopic::SyllableTza => 'ፀ',
            Ethiopic::SyllableTzu => 'ፁ',
            Ethiopic::SyllableTzi => 'ፂ',
            Ethiopic::SyllableTzaa => 'ፃ',
            Ethiopic::SyllableTzee => 'ፄ',
            Ethiopic::SyllableTze => 'ፅ',
            Ethiopic::SyllableTzo => 'ፆ',
            Ethiopic::SyllableTzoa => 'ፇ',
            Ethiopic::SyllableFa => 'ፈ',
            Ethiopic::SyllableFu => 'ፉ',
            Ethiopic::SyllableFi => 'ፊ',
            Ethiopic::SyllableFaa => 'ፋ',
            Ethiopic::SyllableFee => 'ፌ',
            Ethiopic::SyllableFe => 'ፍ',
            Ethiopic::SyllableFo => 'ፎ',
            Ethiopic::SyllableFwa => 'ፏ',
            Ethiopic::SyllablePa => 'ፐ',
            Ethiopic::SyllablePu => 'ፑ',
            Ethiopic::SyllablePi => 'ፒ',
            Ethiopic::SyllablePaa => 'ፓ',
            Ethiopic::SyllablePee => 'ፔ',
            Ethiopic::SyllablePe => 'ፕ',
            Ethiopic::SyllablePo => 'ፖ',
            Ethiopic::SyllablePwa => 'ፗ',
            Ethiopic::SyllableRya => 'ፘ',
            Ethiopic::SyllableMya => 'ፙ',
            Ethiopic::SyllableFya => 'ፚ',
            Ethiopic::CombiningGeminationAndVowelLengthMark => '፝',
            Ethiopic::CombiningVowelLengthMark => '፞',
            Ethiopic::CombiningGeminationMark => '፟',
            Ethiopic::SectionMark => '፠',
            Ethiopic::Wordspace => '፡',
            Ethiopic::FullStop => '።',
            Ethiopic::Comma => '፣',
            Ethiopic::Semicolon => '፤',
            Ethiopic::Colon => '፥',
            Ethiopic::PrefaceColon => '፦',
            Ethiopic::QuestionMark => '፧',
            Ethiopic::ParagraphSeparator => '፨',
            Ethiopic::DigitOne => '፩',
            Ethiopic::DigitTwo => '፪',
            Ethiopic::DigitThree => '፫',
            Ethiopic::DigitFour => '፬',
            Ethiopic::DigitFive => '፭',
            Ethiopic::DigitSix => '፮',
            Ethiopic::DigitSeven => '፯',
            Ethiopic::DigitEight => '፰',
            Ethiopic::DigitNine => '፱',
            Ethiopic::NumberTen => '፲',
            Ethiopic::NumberTwenty => '፳',
            Ethiopic::NumberThirty => '፴',
            Ethiopic::NumberForty => '፵',
            Ethiopic::NumberFifty => '፶',
            Ethiopic::NumberSixty => '፷',
            Ethiopic::NumberSeventy => '፸',
            Ethiopic::NumberEighty => '፹',
            Ethiopic::NumberNinety => '፺',
            Ethiopic::NumberHundred => '፻',
            Ethiopic::NumberTenThousand => '፼',
        }
    }
}

impl std::convert::TryFrom<char> for Ethiopic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ሀ' => Ok(Ethiopic::SyllableHa),
            'ሁ' => Ok(Ethiopic::SyllableHu),
            'ሂ' => Ok(Ethiopic::SyllableHi),
            'ሃ' => Ok(Ethiopic::SyllableHaa),
            'ሄ' => Ok(Ethiopic::SyllableHee),
            'ህ' => Ok(Ethiopic::SyllableHe),
            'ሆ' => Ok(Ethiopic::SyllableHo),
            'ሇ' => Ok(Ethiopic::SyllableHoa),
            'ለ' => Ok(Ethiopic::SyllableLa),
            'ሉ' => Ok(Ethiopic::SyllableLu),
            'ሊ' => Ok(Ethiopic::SyllableLi),
            'ላ' => Ok(Ethiopic::SyllableLaa),
            'ሌ' => Ok(Ethiopic::SyllableLee),
            'ል' => Ok(Ethiopic::SyllableLe),
            'ሎ' => Ok(Ethiopic::SyllableLo),
            'ሏ' => Ok(Ethiopic::SyllableLwa),
            'ሐ' => Ok(Ethiopic::SyllableHha),
            'ሑ' => Ok(Ethiopic::SyllableHhu),
            'ሒ' => Ok(Ethiopic::SyllableHhi),
            'ሓ' => Ok(Ethiopic::SyllableHhaa),
            'ሔ' => Ok(Ethiopic::SyllableHhee),
            'ሕ' => Ok(Ethiopic::SyllableHhe),
            'ሖ' => Ok(Ethiopic::SyllableHho),
            'ሗ' => Ok(Ethiopic::SyllableHhwa),
            'መ' => Ok(Ethiopic::SyllableMa),
            'ሙ' => Ok(Ethiopic::SyllableMu),
            'ሚ' => Ok(Ethiopic::SyllableMi),
            'ማ' => Ok(Ethiopic::SyllableMaa),
            'ሜ' => Ok(Ethiopic::SyllableMee),
            'ም' => Ok(Ethiopic::SyllableMe),
            'ሞ' => Ok(Ethiopic::SyllableMo),
            'ሟ' => Ok(Ethiopic::SyllableMwa),
            'ሠ' => Ok(Ethiopic::SyllableSza),
            'ሡ' => Ok(Ethiopic::SyllableSzu),
            'ሢ' => Ok(Ethiopic::SyllableSzi),
            'ሣ' => Ok(Ethiopic::SyllableSzaa),
            'ሤ' => Ok(Ethiopic::SyllableSzee),
            'ሥ' => Ok(Ethiopic::SyllableSze),
            'ሦ' => Ok(Ethiopic::SyllableSzo),
            'ሧ' => Ok(Ethiopic::SyllableSzwa),
            'ረ' => Ok(Ethiopic::SyllableRa),
            'ሩ' => Ok(Ethiopic::SyllableRu),
            'ሪ' => Ok(Ethiopic::SyllableRi),
            'ራ' => Ok(Ethiopic::SyllableRaa),
            'ሬ' => Ok(Ethiopic::SyllableRee),
            'ር' => Ok(Ethiopic::SyllableRe),
            'ሮ' => Ok(Ethiopic::SyllableRo),
            'ሯ' => Ok(Ethiopic::SyllableRwa),
            'ሰ' => Ok(Ethiopic::SyllableSa),
            'ሱ' => Ok(Ethiopic::SyllableSu),
            'ሲ' => Ok(Ethiopic::SyllableSi),
            'ሳ' => Ok(Ethiopic::SyllableSaa),
            'ሴ' => Ok(Ethiopic::SyllableSee),
            'ስ' => Ok(Ethiopic::SyllableSe),
            'ሶ' => Ok(Ethiopic::SyllableSo),
            'ሷ' => Ok(Ethiopic::SyllableSwa),
            'ሸ' => Ok(Ethiopic::SyllableSha),
            'ሹ' => Ok(Ethiopic::SyllableShu),
            'ሺ' => Ok(Ethiopic::SyllableShi),
            'ሻ' => Ok(Ethiopic::SyllableShaa),
            'ሼ' => Ok(Ethiopic::SyllableShee),
            'ሽ' => Ok(Ethiopic::SyllableShe),
            'ሾ' => Ok(Ethiopic::SyllableSho),
            'ሿ' => Ok(Ethiopic::SyllableShwa),
            'ቀ' => Ok(Ethiopic::SyllableQa),
            'ቁ' => Ok(Ethiopic::SyllableQu),
            'ቂ' => Ok(Ethiopic::SyllableQi),
            'ቃ' => Ok(Ethiopic::SyllableQaa),
            'ቄ' => Ok(Ethiopic::SyllableQee),
            'ቅ' => Ok(Ethiopic::SyllableQe),
            'ቆ' => Ok(Ethiopic::SyllableQo),
            'ቇ' => Ok(Ethiopic::SyllableQoa),
            'ቈ' => Ok(Ethiopic::SyllableQwa),
            'ቊ' => Ok(Ethiopic::SyllableQwi),
            'ቋ' => Ok(Ethiopic::SyllableQwaa),
            'ቌ' => Ok(Ethiopic::SyllableQwee),
            'ቍ' => Ok(Ethiopic::SyllableQwe),
            'ቐ' => Ok(Ethiopic::SyllableQha),
            'ቑ' => Ok(Ethiopic::SyllableQhu),
            'ቒ' => Ok(Ethiopic::SyllableQhi),
            'ቓ' => Ok(Ethiopic::SyllableQhaa),
            'ቔ' => Ok(Ethiopic::SyllableQhee),
            'ቕ' => Ok(Ethiopic::SyllableQhe),
            'ቖ' => Ok(Ethiopic::SyllableQho),
            'ቘ' => Ok(Ethiopic::SyllableQhwa),
            'ቚ' => Ok(Ethiopic::SyllableQhwi),
            'ቛ' => Ok(Ethiopic::SyllableQhwaa),
            'ቜ' => Ok(Ethiopic::SyllableQhwee),
            'ቝ' => Ok(Ethiopic::SyllableQhwe),
            'በ' => Ok(Ethiopic::SyllableBa),
            'ቡ' => Ok(Ethiopic::SyllableBu),
            'ቢ' => Ok(Ethiopic::SyllableBi),
            'ባ' => Ok(Ethiopic::SyllableBaa),
            'ቤ' => Ok(Ethiopic::SyllableBee),
            'ብ' => Ok(Ethiopic::SyllableBe),
            'ቦ' => Ok(Ethiopic::SyllableBo),
            'ቧ' => Ok(Ethiopic::SyllableBwa),
            'ቨ' => Ok(Ethiopic::SyllableVa),
            'ቩ' => Ok(Ethiopic::SyllableVu),
            'ቪ' => Ok(Ethiopic::SyllableVi),
            'ቫ' => Ok(Ethiopic::SyllableVaa),
            'ቬ' => Ok(Ethiopic::SyllableVee),
            'ቭ' => Ok(Ethiopic::SyllableVe),
            'ቮ' => Ok(Ethiopic::SyllableVo),
            'ቯ' => Ok(Ethiopic::SyllableVwa),
            'ተ' => Ok(Ethiopic::SyllableTa),
            'ቱ' => Ok(Ethiopic::SyllableTu),
            'ቲ' => Ok(Ethiopic::SyllableTi),
            'ታ' => Ok(Ethiopic::SyllableTaa),
            'ቴ' => Ok(Ethiopic::SyllableTee),
            'ት' => Ok(Ethiopic::SyllableTe),
            'ቶ' => Ok(Ethiopic::SyllableTo),
            'ቷ' => Ok(Ethiopic::SyllableTwa),
            'ቸ' => Ok(Ethiopic::SyllableCa),
            'ቹ' => Ok(Ethiopic::SyllableCu),
            'ቺ' => Ok(Ethiopic::SyllableCi),
            'ቻ' => Ok(Ethiopic::SyllableCaa),
            'ቼ' => Ok(Ethiopic::SyllableCee),
            'ች' => Ok(Ethiopic::SyllableCe),
            'ቾ' => Ok(Ethiopic::SyllableCo),
            'ቿ' => Ok(Ethiopic::SyllableCwa),
            'ኀ' => Ok(Ethiopic::SyllableXa),
            'ኁ' => Ok(Ethiopic::SyllableXu),
            'ኂ' => Ok(Ethiopic::SyllableXi),
            'ኃ' => Ok(Ethiopic::SyllableXaa),
            'ኄ' => Ok(Ethiopic::SyllableXee),
            'ኅ' => Ok(Ethiopic::SyllableXe),
            'ኆ' => Ok(Ethiopic::SyllableXo),
            'ኇ' => Ok(Ethiopic::SyllableXoa),
            'ኈ' => Ok(Ethiopic::SyllableXwa),
            'ኊ' => Ok(Ethiopic::SyllableXwi),
            'ኋ' => Ok(Ethiopic::SyllableXwaa),
            'ኌ' => Ok(Ethiopic::SyllableXwee),
            'ኍ' => Ok(Ethiopic::SyllableXwe),
            'ነ' => Ok(Ethiopic::SyllableNa),
            'ኑ' => Ok(Ethiopic::SyllableNu),
            'ኒ' => Ok(Ethiopic::SyllableNi),
            'ና' => Ok(Ethiopic::SyllableNaa),
            'ኔ' => Ok(Ethiopic::SyllableNee),
            'ን' => Ok(Ethiopic::SyllableNe),
            'ኖ' => Ok(Ethiopic::SyllableNo),
            'ኗ' => Ok(Ethiopic::SyllableNwa),
            'ኘ' => Ok(Ethiopic::SyllableNya),
            'ኙ' => Ok(Ethiopic::SyllableNyu),
            'ኚ' => Ok(Ethiopic::SyllableNyi),
            'ኛ' => Ok(Ethiopic::SyllableNyaa),
            'ኜ' => Ok(Ethiopic::SyllableNyee),
            'ኝ' => Ok(Ethiopic::SyllableNye),
            'ኞ' => Ok(Ethiopic::SyllableNyo),
            'ኟ' => Ok(Ethiopic::SyllableNywa),
            'አ' => Ok(Ethiopic::SyllableGlottalA),
            'ኡ' => Ok(Ethiopic::SyllableGlottalU),
            'ኢ' => Ok(Ethiopic::SyllableGlottalI),
            'ኣ' => Ok(Ethiopic::SyllableGlottalAa),
            'ኤ' => Ok(Ethiopic::SyllableGlottalEe),
            'እ' => Ok(Ethiopic::SyllableGlottalE),
            'ኦ' => Ok(Ethiopic::SyllableGlottalO),
            'ኧ' => Ok(Ethiopic::SyllableGlottalWa),
            'ከ' => Ok(Ethiopic::SyllableKa),
            'ኩ' => Ok(Ethiopic::SyllableKu),
            'ኪ' => Ok(Ethiopic::SyllableKi),
            'ካ' => Ok(Ethiopic::SyllableKaa),
            'ኬ' => Ok(Ethiopic::SyllableKee),
            'ክ' => Ok(Ethiopic::SyllableKe),
            'ኮ' => Ok(Ethiopic::SyllableKo),
            'ኯ' => Ok(Ethiopic::SyllableKoa),
            'ኰ' => Ok(Ethiopic::SyllableKwa),
            'ኲ' => Ok(Ethiopic::SyllableKwi),
            'ኳ' => Ok(Ethiopic::SyllableKwaa),
            'ኴ' => Ok(Ethiopic::SyllableKwee),
            'ኵ' => Ok(Ethiopic::SyllableKwe),
            'ኸ' => Ok(Ethiopic::SyllableKxa),
            'ኹ' => Ok(Ethiopic::SyllableKxu),
            'ኺ' => Ok(Ethiopic::SyllableKxi),
            'ኻ' => Ok(Ethiopic::SyllableKxaa),
            'ኼ' => Ok(Ethiopic::SyllableKxee),
            'ኽ' => Ok(Ethiopic::SyllableKxe),
            'ኾ' => Ok(Ethiopic::SyllableKxo),
            'ዀ' => Ok(Ethiopic::SyllableKxwa),
            'ዂ' => Ok(Ethiopic::SyllableKxwi),
            'ዃ' => Ok(Ethiopic::SyllableKxwaa),
            'ዄ' => Ok(Ethiopic::SyllableKxwee),
            'ዅ' => Ok(Ethiopic::SyllableKxwe),
            'ወ' => Ok(Ethiopic::SyllableWa),
            'ዉ' => Ok(Ethiopic::SyllableWu),
            'ዊ' => Ok(Ethiopic::SyllableWi),
            'ዋ' => Ok(Ethiopic::SyllableWaa),
            'ዌ' => Ok(Ethiopic::SyllableWee),
            'ው' => Ok(Ethiopic::SyllableWe),
            'ዎ' => Ok(Ethiopic::SyllableWo),
            'ዏ' => Ok(Ethiopic::SyllableWoa),
            'ዐ' => Ok(Ethiopic::SyllablePharyngealA),
            'ዑ' => Ok(Ethiopic::SyllablePharyngealU),
            'ዒ' => Ok(Ethiopic::SyllablePharyngealI),
            'ዓ' => Ok(Ethiopic::SyllablePharyngealAa),
            'ዔ' => Ok(Ethiopic::SyllablePharyngealEe),
            'ዕ' => Ok(Ethiopic::SyllablePharyngealE),
            'ዖ' => Ok(Ethiopic::SyllablePharyngealO),
            'ዘ' => Ok(Ethiopic::SyllableZa),
            'ዙ' => Ok(Ethiopic::SyllableZu),
            'ዚ' => Ok(Ethiopic::SyllableZi),
            'ዛ' => Ok(Ethiopic::SyllableZaa),
            'ዜ' => Ok(Ethiopic::SyllableZee),
            'ዝ' => Ok(Ethiopic::SyllableZe),
            'ዞ' => Ok(Ethiopic::SyllableZo),
            'ዟ' => Ok(Ethiopic::SyllableZwa),
            'ዠ' => Ok(Ethiopic::SyllableZha),
            'ዡ' => Ok(Ethiopic::SyllableZhu),
            'ዢ' => Ok(Ethiopic::SyllableZhi),
            'ዣ' => Ok(Ethiopic::SyllableZhaa),
            'ዤ' => Ok(Ethiopic::SyllableZhee),
            'ዥ' => Ok(Ethiopic::SyllableZhe),
            'ዦ' => Ok(Ethiopic::SyllableZho),
            'ዧ' => Ok(Ethiopic::SyllableZhwa),
            'የ' => Ok(Ethiopic::SyllableYa),
            'ዩ' => Ok(Ethiopic::SyllableYu),
            'ዪ' => Ok(Ethiopic::SyllableYi),
            'ያ' => Ok(Ethiopic::SyllableYaa),
            'ዬ' => Ok(Ethiopic::SyllableYee),
            'ይ' => Ok(Ethiopic::SyllableYe),
            'ዮ' => Ok(Ethiopic::SyllableYo),
            'ዯ' => Ok(Ethiopic::SyllableYoa),
            'ደ' => Ok(Ethiopic::SyllableDa),
            'ዱ' => Ok(Ethiopic::SyllableDu),
            'ዲ' => Ok(Ethiopic::SyllableDi),
            'ዳ' => Ok(Ethiopic::SyllableDaa),
            'ዴ' => Ok(Ethiopic::SyllableDee),
            'ድ' => Ok(Ethiopic::SyllableDe),
            'ዶ' => Ok(Ethiopic::SyllableDo),
            'ዷ' => Ok(Ethiopic::SyllableDwa),
            'ዸ' => Ok(Ethiopic::SyllableDda),
            'ዹ' => Ok(Ethiopic::SyllableDdu),
            'ዺ' => Ok(Ethiopic::SyllableDdi),
            'ዻ' => Ok(Ethiopic::SyllableDdaa),
            'ዼ' => Ok(Ethiopic::SyllableDdee),
            'ዽ' => Ok(Ethiopic::SyllableDde),
            'ዾ' => Ok(Ethiopic::SyllableDdo),
            'ዿ' => Ok(Ethiopic::SyllableDdwa),
            'ጀ' => Ok(Ethiopic::SyllableJa),
            'ጁ' => Ok(Ethiopic::SyllableJu),
            'ጂ' => Ok(Ethiopic::SyllableJi),
            'ጃ' => Ok(Ethiopic::SyllableJaa),
            'ጄ' => Ok(Ethiopic::SyllableJee),
            'ጅ' => Ok(Ethiopic::SyllableJe),
            'ጆ' => Ok(Ethiopic::SyllableJo),
            'ጇ' => Ok(Ethiopic::SyllableJwa),
            'ገ' => Ok(Ethiopic::SyllableGa),
            'ጉ' => Ok(Ethiopic::SyllableGu),
            'ጊ' => Ok(Ethiopic::SyllableGi),
            'ጋ' => Ok(Ethiopic::SyllableGaa),
            'ጌ' => Ok(Ethiopic::SyllableGee),
            'ግ' => Ok(Ethiopic::SyllableGe),
            'ጎ' => Ok(Ethiopic::SyllableGo),
            'ጏ' => Ok(Ethiopic::SyllableGoa),
            'ጐ' => Ok(Ethiopic::SyllableGwa),
            'ጒ' => Ok(Ethiopic::SyllableGwi),
            'ጓ' => Ok(Ethiopic::SyllableGwaa),
            'ጔ' => Ok(Ethiopic::SyllableGwee),
            'ጕ' => Ok(Ethiopic::SyllableGwe),
            'ጘ' => Ok(Ethiopic::SyllableGga),
            'ጙ' => Ok(Ethiopic::SyllableGgu),
            'ጚ' => Ok(Ethiopic::SyllableGgi),
            'ጛ' => Ok(Ethiopic::SyllableGgaa),
            'ጜ' => Ok(Ethiopic::SyllableGgee),
            'ጝ' => Ok(Ethiopic::SyllableGge),
            'ጞ' => Ok(Ethiopic::SyllableGgo),
            'ጟ' => Ok(Ethiopic::SyllableGgwaa),
            'ጠ' => Ok(Ethiopic::SyllableTha),
            'ጡ' => Ok(Ethiopic::SyllableThu),
            'ጢ' => Ok(Ethiopic::SyllableThi),
            'ጣ' => Ok(Ethiopic::SyllableThaa),
            'ጤ' => Ok(Ethiopic::SyllableThee),
            'ጥ' => Ok(Ethiopic::SyllableThe),
            'ጦ' => Ok(Ethiopic::SyllableTho),
            'ጧ' => Ok(Ethiopic::SyllableThwa),
            'ጨ' => Ok(Ethiopic::SyllableCha),
            'ጩ' => Ok(Ethiopic::SyllableChu),
            'ጪ' => Ok(Ethiopic::SyllableChi),
            'ጫ' => Ok(Ethiopic::SyllableChaa),
            'ጬ' => Ok(Ethiopic::SyllableChee),
            'ጭ' => Ok(Ethiopic::SyllableChe),
            'ጮ' => Ok(Ethiopic::SyllableCho),
            'ጯ' => Ok(Ethiopic::SyllableChwa),
            'ጰ' => Ok(Ethiopic::SyllablePha),
            'ጱ' => Ok(Ethiopic::SyllablePhu),
            'ጲ' => Ok(Ethiopic::SyllablePhi),
            'ጳ' => Ok(Ethiopic::SyllablePhaa),
            'ጴ' => Ok(Ethiopic::SyllablePhee),
            'ጵ' => Ok(Ethiopic::SyllablePhe),
            'ጶ' => Ok(Ethiopic::SyllablePho),
            'ጷ' => Ok(Ethiopic::SyllablePhwa),
            'ጸ' => Ok(Ethiopic::SyllableTsa),
            'ጹ' => Ok(Ethiopic::SyllableTsu),
            'ጺ' => Ok(Ethiopic::SyllableTsi),
            'ጻ' => Ok(Ethiopic::SyllableTsaa),
            'ጼ' => Ok(Ethiopic::SyllableTsee),
            'ጽ' => Ok(Ethiopic::SyllableTse),
            'ጾ' => Ok(Ethiopic::SyllableTso),
            'ጿ' => Ok(Ethiopic::SyllableTswa),
            'ፀ' => Ok(Ethiopic::SyllableTza),
            'ፁ' => Ok(Ethiopic::SyllableTzu),
            'ፂ' => Ok(Ethiopic::SyllableTzi),
            'ፃ' => Ok(Ethiopic::SyllableTzaa),
            'ፄ' => Ok(Ethiopic::SyllableTzee),
            'ፅ' => Ok(Ethiopic::SyllableTze),
            'ፆ' => Ok(Ethiopic::SyllableTzo),
            'ፇ' => Ok(Ethiopic::SyllableTzoa),
            'ፈ' => Ok(Ethiopic::SyllableFa),
            'ፉ' => Ok(Ethiopic::SyllableFu),
            'ፊ' => Ok(Ethiopic::SyllableFi),
            'ፋ' => Ok(Ethiopic::SyllableFaa),
            'ፌ' => Ok(Ethiopic::SyllableFee),
            'ፍ' => Ok(Ethiopic::SyllableFe),
            'ፎ' => Ok(Ethiopic::SyllableFo),
            'ፏ' => Ok(Ethiopic::SyllableFwa),
            'ፐ' => Ok(Ethiopic::SyllablePa),
            'ፑ' => Ok(Ethiopic::SyllablePu),
            'ፒ' => Ok(Ethiopic::SyllablePi),
            'ፓ' => Ok(Ethiopic::SyllablePaa),
            'ፔ' => Ok(Ethiopic::SyllablePee),
            'ፕ' => Ok(Ethiopic::SyllablePe),
            'ፖ' => Ok(Ethiopic::SyllablePo),
            'ፗ' => Ok(Ethiopic::SyllablePwa),
            'ፘ' => Ok(Ethiopic::SyllableRya),
            'ፙ' => Ok(Ethiopic::SyllableMya),
            'ፚ' => Ok(Ethiopic::SyllableFya),
            '፝' => Ok(Ethiopic::CombiningGeminationAndVowelLengthMark),
            '፞' => Ok(Ethiopic::CombiningVowelLengthMark),
            '፟' => Ok(Ethiopic::CombiningGeminationMark),
            '፠' => Ok(Ethiopic::SectionMark),
            '፡' => Ok(Ethiopic::Wordspace),
            '።' => Ok(Ethiopic::FullStop),
            '፣' => Ok(Ethiopic::Comma),
            '፤' => Ok(Ethiopic::Semicolon),
            '፥' => Ok(Ethiopic::Colon),
            '፦' => Ok(Ethiopic::PrefaceColon),
            '፧' => Ok(Ethiopic::QuestionMark),
            '፨' => Ok(Ethiopic::ParagraphSeparator),
            '፩' => Ok(Ethiopic::DigitOne),
            '፪' => Ok(Ethiopic::DigitTwo),
            '፫' => Ok(Ethiopic::DigitThree),
            '፬' => Ok(Ethiopic::DigitFour),
            '፭' => Ok(Ethiopic::DigitFive),
            '፮' => Ok(Ethiopic::DigitSix),
            '፯' => Ok(Ethiopic::DigitSeven),
            '፰' => Ok(Ethiopic::DigitEight),
            '፱' => Ok(Ethiopic::DigitNine),
            '፲' => Ok(Ethiopic::NumberTen),
            '፳' => Ok(Ethiopic::NumberTwenty),
            '፴' => Ok(Ethiopic::NumberThirty),
            '፵' => Ok(Ethiopic::NumberForty),
            '፶' => Ok(Ethiopic::NumberFifty),
            '፷' => Ok(Ethiopic::NumberSixty),
            '፸' => Ok(Ethiopic::NumberSeventy),
            '፹' => Ok(Ethiopic::NumberEighty),
            '፺' => Ok(Ethiopic::NumberNinety),
            '፻' => Ok(Ethiopic::NumberHundred),
            '፼' => Ok(Ethiopic::NumberTenThousand),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Ethiopic {
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

impl std::convert::TryFrom<u32> for Ethiopic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Ethiopic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Ethiopic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Ethiopic::SyllableHa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Ethiopic::SyllableHa => "ethiopic syllable ha",
            Ethiopic::SyllableHu => "ethiopic syllable hu",
            Ethiopic::SyllableHi => "ethiopic syllable hi",
            Ethiopic::SyllableHaa => "ethiopic syllable haa",
            Ethiopic::SyllableHee => "ethiopic syllable hee",
            Ethiopic::SyllableHe => "ethiopic syllable he",
            Ethiopic::SyllableHo => "ethiopic syllable ho",
            Ethiopic::SyllableHoa => "ethiopic syllable hoa",
            Ethiopic::SyllableLa => "ethiopic syllable la",
            Ethiopic::SyllableLu => "ethiopic syllable lu",
            Ethiopic::SyllableLi => "ethiopic syllable li",
            Ethiopic::SyllableLaa => "ethiopic syllable laa",
            Ethiopic::SyllableLee => "ethiopic syllable lee",
            Ethiopic::SyllableLe => "ethiopic syllable le",
            Ethiopic::SyllableLo => "ethiopic syllable lo",
            Ethiopic::SyllableLwa => "ethiopic syllable lwa",
            Ethiopic::SyllableHha => "ethiopic syllable hha",
            Ethiopic::SyllableHhu => "ethiopic syllable hhu",
            Ethiopic::SyllableHhi => "ethiopic syllable hhi",
            Ethiopic::SyllableHhaa => "ethiopic syllable hhaa",
            Ethiopic::SyllableHhee => "ethiopic syllable hhee",
            Ethiopic::SyllableHhe => "ethiopic syllable hhe",
            Ethiopic::SyllableHho => "ethiopic syllable hho",
            Ethiopic::SyllableHhwa => "ethiopic syllable hhwa",
            Ethiopic::SyllableMa => "ethiopic syllable ma",
            Ethiopic::SyllableMu => "ethiopic syllable mu",
            Ethiopic::SyllableMi => "ethiopic syllable mi",
            Ethiopic::SyllableMaa => "ethiopic syllable maa",
            Ethiopic::SyllableMee => "ethiopic syllable mee",
            Ethiopic::SyllableMe => "ethiopic syllable me",
            Ethiopic::SyllableMo => "ethiopic syllable mo",
            Ethiopic::SyllableMwa => "ethiopic syllable mwa",
            Ethiopic::SyllableSza => "ethiopic syllable sza",
            Ethiopic::SyllableSzu => "ethiopic syllable szu",
            Ethiopic::SyllableSzi => "ethiopic syllable szi",
            Ethiopic::SyllableSzaa => "ethiopic syllable szaa",
            Ethiopic::SyllableSzee => "ethiopic syllable szee",
            Ethiopic::SyllableSze => "ethiopic syllable sze",
            Ethiopic::SyllableSzo => "ethiopic syllable szo",
            Ethiopic::SyllableSzwa => "ethiopic syllable szwa",
            Ethiopic::SyllableRa => "ethiopic syllable ra",
            Ethiopic::SyllableRu => "ethiopic syllable ru",
            Ethiopic::SyllableRi => "ethiopic syllable ri",
            Ethiopic::SyllableRaa => "ethiopic syllable raa",
            Ethiopic::SyllableRee => "ethiopic syllable ree",
            Ethiopic::SyllableRe => "ethiopic syllable re",
            Ethiopic::SyllableRo => "ethiopic syllable ro",
            Ethiopic::SyllableRwa => "ethiopic syllable rwa",
            Ethiopic::SyllableSa => "ethiopic syllable sa",
            Ethiopic::SyllableSu => "ethiopic syllable su",
            Ethiopic::SyllableSi => "ethiopic syllable si",
            Ethiopic::SyllableSaa => "ethiopic syllable saa",
            Ethiopic::SyllableSee => "ethiopic syllable see",
            Ethiopic::SyllableSe => "ethiopic syllable se",
            Ethiopic::SyllableSo => "ethiopic syllable so",
            Ethiopic::SyllableSwa => "ethiopic syllable swa",
            Ethiopic::SyllableSha => "ethiopic syllable sha",
            Ethiopic::SyllableShu => "ethiopic syllable shu",
            Ethiopic::SyllableShi => "ethiopic syllable shi",
            Ethiopic::SyllableShaa => "ethiopic syllable shaa",
            Ethiopic::SyllableShee => "ethiopic syllable shee",
            Ethiopic::SyllableShe => "ethiopic syllable she",
            Ethiopic::SyllableSho => "ethiopic syllable sho",
            Ethiopic::SyllableShwa => "ethiopic syllable shwa",
            Ethiopic::SyllableQa => "ethiopic syllable qa",
            Ethiopic::SyllableQu => "ethiopic syllable qu",
            Ethiopic::SyllableQi => "ethiopic syllable qi",
            Ethiopic::SyllableQaa => "ethiopic syllable qaa",
            Ethiopic::SyllableQee => "ethiopic syllable qee",
            Ethiopic::SyllableQe => "ethiopic syllable qe",
            Ethiopic::SyllableQo => "ethiopic syllable qo",
            Ethiopic::SyllableQoa => "ethiopic syllable qoa",
            Ethiopic::SyllableQwa => "ethiopic syllable qwa",
            Ethiopic::SyllableQwi => "ethiopic syllable qwi",
            Ethiopic::SyllableQwaa => "ethiopic syllable qwaa",
            Ethiopic::SyllableQwee => "ethiopic syllable qwee",
            Ethiopic::SyllableQwe => "ethiopic syllable qwe",
            Ethiopic::SyllableQha => "ethiopic syllable qha",
            Ethiopic::SyllableQhu => "ethiopic syllable qhu",
            Ethiopic::SyllableQhi => "ethiopic syllable qhi",
            Ethiopic::SyllableQhaa => "ethiopic syllable qhaa",
            Ethiopic::SyllableQhee => "ethiopic syllable qhee",
            Ethiopic::SyllableQhe => "ethiopic syllable qhe",
            Ethiopic::SyllableQho => "ethiopic syllable qho",
            Ethiopic::SyllableQhwa => "ethiopic syllable qhwa",
            Ethiopic::SyllableQhwi => "ethiopic syllable qhwi",
            Ethiopic::SyllableQhwaa => "ethiopic syllable qhwaa",
            Ethiopic::SyllableQhwee => "ethiopic syllable qhwee",
            Ethiopic::SyllableQhwe => "ethiopic syllable qhwe",
            Ethiopic::SyllableBa => "ethiopic syllable ba",
            Ethiopic::SyllableBu => "ethiopic syllable bu",
            Ethiopic::SyllableBi => "ethiopic syllable bi",
            Ethiopic::SyllableBaa => "ethiopic syllable baa",
            Ethiopic::SyllableBee => "ethiopic syllable bee",
            Ethiopic::SyllableBe => "ethiopic syllable be",
            Ethiopic::SyllableBo => "ethiopic syllable bo",
            Ethiopic::SyllableBwa => "ethiopic syllable bwa",
            Ethiopic::SyllableVa => "ethiopic syllable va",
            Ethiopic::SyllableVu => "ethiopic syllable vu",
            Ethiopic::SyllableVi => "ethiopic syllable vi",
            Ethiopic::SyllableVaa => "ethiopic syllable vaa",
            Ethiopic::SyllableVee => "ethiopic syllable vee",
            Ethiopic::SyllableVe => "ethiopic syllable ve",
            Ethiopic::SyllableVo => "ethiopic syllable vo",
            Ethiopic::SyllableVwa => "ethiopic syllable vwa",
            Ethiopic::SyllableTa => "ethiopic syllable ta",
            Ethiopic::SyllableTu => "ethiopic syllable tu",
            Ethiopic::SyllableTi => "ethiopic syllable ti",
            Ethiopic::SyllableTaa => "ethiopic syllable taa",
            Ethiopic::SyllableTee => "ethiopic syllable tee",
            Ethiopic::SyllableTe => "ethiopic syllable te",
            Ethiopic::SyllableTo => "ethiopic syllable to",
            Ethiopic::SyllableTwa => "ethiopic syllable twa",
            Ethiopic::SyllableCa => "ethiopic syllable ca",
            Ethiopic::SyllableCu => "ethiopic syllable cu",
            Ethiopic::SyllableCi => "ethiopic syllable ci",
            Ethiopic::SyllableCaa => "ethiopic syllable caa",
            Ethiopic::SyllableCee => "ethiopic syllable cee",
            Ethiopic::SyllableCe => "ethiopic syllable ce",
            Ethiopic::SyllableCo => "ethiopic syllable co",
            Ethiopic::SyllableCwa => "ethiopic syllable cwa",
            Ethiopic::SyllableXa => "ethiopic syllable xa",
            Ethiopic::SyllableXu => "ethiopic syllable xu",
            Ethiopic::SyllableXi => "ethiopic syllable xi",
            Ethiopic::SyllableXaa => "ethiopic syllable xaa",
            Ethiopic::SyllableXee => "ethiopic syllable xee",
            Ethiopic::SyllableXe => "ethiopic syllable xe",
            Ethiopic::SyllableXo => "ethiopic syllable xo",
            Ethiopic::SyllableXoa => "ethiopic syllable xoa",
            Ethiopic::SyllableXwa => "ethiopic syllable xwa",
            Ethiopic::SyllableXwi => "ethiopic syllable xwi",
            Ethiopic::SyllableXwaa => "ethiopic syllable xwaa",
            Ethiopic::SyllableXwee => "ethiopic syllable xwee",
            Ethiopic::SyllableXwe => "ethiopic syllable xwe",
            Ethiopic::SyllableNa => "ethiopic syllable na",
            Ethiopic::SyllableNu => "ethiopic syllable nu",
            Ethiopic::SyllableNi => "ethiopic syllable ni",
            Ethiopic::SyllableNaa => "ethiopic syllable naa",
            Ethiopic::SyllableNee => "ethiopic syllable nee",
            Ethiopic::SyllableNe => "ethiopic syllable ne",
            Ethiopic::SyllableNo => "ethiopic syllable no",
            Ethiopic::SyllableNwa => "ethiopic syllable nwa",
            Ethiopic::SyllableNya => "ethiopic syllable nya",
            Ethiopic::SyllableNyu => "ethiopic syllable nyu",
            Ethiopic::SyllableNyi => "ethiopic syllable nyi",
            Ethiopic::SyllableNyaa => "ethiopic syllable nyaa",
            Ethiopic::SyllableNyee => "ethiopic syllable nyee",
            Ethiopic::SyllableNye => "ethiopic syllable nye",
            Ethiopic::SyllableNyo => "ethiopic syllable nyo",
            Ethiopic::SyllableNywa => "ethiopic syllable nywa",
            Ethiopic::SyllableGlottalA => "ethiopic syllable glottal a",
            Ethiopic::SyllableGlottalU => "ethiopic syllable glottal u",
            Ethiopic::SyllableGlottalI => "ethiopic syllable glottal i",
            Ethiopic::SyllableGlottalAa => "ethiopic syllable glottal aa",
            Ethiopic::SyllableGlottalEe => "ethiopic syllable glottal ee",
            Ethiopic::SyllableGlottalE => "ethiopic syllable glottal e",
            Ethiopic::SyllableGlottalO => "ethiopic syllable glottal o",
            Ethiopic::SyllableGlottalWa => "ethiopic syllable glottal wa",
            Ethiopic::SyllableKa => "ethiopic syllable ka",
            Ethiopic::SyllableKu => "ethiopic syllable ku",
            Ethiopic::SyllableKi => "ethiopic syllable ki",
            Ethiopic::SyllableKaa => "ethiopic syllable kaa",
            Ethiopic::SyllableKee => "ethiopic syllable kee",
            Ethiopic::SyllableKe => "ethiopic syllable ke",
            Ethiopic::SyllableKo => "ethiopic syllable ko",
            Ethiopic::SyllableKoa => "ethiopic syllable koa",
            Ethiopic::SyllableKwa => "ethiopic syllable kwa",
            Ethiopic::SyllableKwi => "ethiopic syllable kwi",
            Ethiopic::SyllableKwaa => "ethiopic syllable kwaa",
            Ethiopic::SyllableKwee => "ethiopic syllable kwee",
            Ethiopic::SyllableKwe => "ethiopic syllable kwe",
            Ethiopic::SyllableKxa => "ethiopic syllable kxa",
            Ethiopic::SyllableKxu => "ethiopic syllable kxu",
            Ethiopic::SyllableKxi => "ethiopic syllable kxi",
            Ethiopic::SyllableKxaa => "ethiopic syllable kxaa",
            Ethiopic::SyllableKxee => "ethiopic syllable kxee",
            Ethiopic::SyllableKxe => "ethiopic syllable kxe",
            Ethiopic::SyllableKxo => "ethiopic syllable kxo",
            Ethiopic::SyllableKxwa => "ethiopic syllable kxwa",
            Ethiopic::SyllableKxwi => "ethiopic syllable kxwi",
            Ethiopic::SyllableKxwaa => "ethiopic syllable kxwaa",
            Ethiopic::SyllableKxwee => "ethiopic syllable kxwee",
            Ethiopic::SyllableKxwe => "ethiopic syllable kxwe",
            Ethiopic::SyllableWa => "ethiopic syllable wa",
            Ethiopic::SyllableWu => "ethiopic syllable wu",
            Ethiopic::SyllableWi => "ethiopic syllable wi",
            Ethiopic::SyllableWaa => "ethiopic syllable waa",
            Ethiopic::SyllableWee => "ethiopic syllable wee",
            Ethiopic::SyllableWe => "ethiopic syllable we",
            Ethiopic::SyllableWo => "ethiopic syllable wo",
            Ethiopic::SyllableWoa => "ethiopic syllable woa",
            Ethiopic::SyllablePharyngealA => "ethiopic syllable pharyngeal a",
            Ethiopic::SyllablePharyngealU => "ethiopic syllable pharyngeal u",
            Ethiopic::SyllablePharyngealI => "ethiopic syllable pharyngeal i",
            Ethiopic::SyllablePharyngealAa => "ethiopic syllable pharyngeal aa",
            Ethiopic::SyllablePharyngealEe => "ethiopic syllable pharyngeal ee",
            Ethiopic::SyllablePharyngealE => "ethiopic syllable pharyngeal e",
            Ethiopic::SyllablePharyngealO => "ethiopic syllable pharyngeal o",
            Ethiopic::SyllableZa => "ethiopic syllable za",
            Ethiopic::SyllableZu => "ethiopic syllable zu",
            Ethiopic::SyllableZi => "ethiopic syllable zi",
            Ethiopic::SyllableZaa => "ethiopic syllable zaa",
            Ethiopic::SyllableZee => "ethiopic syllable zee",
            Ethiopic::SyllableZe => "ethiopic syllable ze",
            Ethiopic::SyllableZo => "ethiopic syllable zo",
            Ethiopic::SyllableZwa => "ethiopic syllable zwa",
            Ethiopic::SyllableZha => "ethiopic syllable zha",
            Ethiopic::SyllableZhu => "ethiopic syllable zhu",
            Ethiopic::SyllableZhi => "ethiopic syllable zhi",
            Ethiopic::SyllableZhaa => "ethiopic syllable zhaa",
            Ethiopic::SyllableZhee => "ethiopic syllable zhee",
            Ethiopic::SyllableZhe => "ethiopic syllable zhe",
            Ethiopic::SyllableZho => "ethiopic syllable zho",
            Ethiopic::SyllableZhwa => "ethiopic syllable zhwa",
            Ethiopic::SyllableYa => "ethiopic syllable ya",
            Ethiopic::SyllableYu => "ethiopic syllable yu",
            Ethiopic::SyllableYi => "ethiopic syllable yi",
            Ethiopic::SyllableYaa => "ethiopic syllable yaa",
            Ethiopic::SyllableYee => "ethiopic syllable yee",
            Ethiopic::SyllableYe => "ethiopic syllable ye",
            Ethiopic::SyllableYo => "ethiopic syllable yo",
            Ethiopic::SyllableYoa => "ethiopic syllable yoa",
            Ethiopic::SyllableDa => "ethiopic syllable da",
            Ethiopic::SyllableDu => "ethiopic syllable du",
            Ethiopic::SyllableDi => "ethiopic syllable di",
            Ethiopic::SyllableDaa => "ethiopic syllable daa",
            Ethiopic::SyllableDee => "ethiopic syllable dee",
            Ethiopic::SyllableDe => "ethiopic syllable de",
            Ethiopic::SyllableDo => "ethiopic syllable do",
            Ethiopic::SyllableDwa => "ethiopic syllable dwa",
            Ethiopic::SyllableDda => "ethiopic syllable dda",
            Ethiopic::SyllableDdu => "ethiopic syllable ddu",
            Ethiopic::SyllableDdi => "ethiopic syllable ddi",
            Ethiopic::SyllableDdaa => "ethiopic syllable ddaa",
            Ethiopic::SyllableDdee => "ethiopic syllable ddee",
            Ethiopic::SyllableDde => "ethiopic syllable dde",
            Ethiopic::SyllableDdo => "ethiopic syllable ddo",
            Ethiopic::SyllableDdwa => "ethiopic syllable ddwa",
            Ethiopic::SyllableJa => "ethiopic syllable ja",
            Ethiopic::SyllableJu => "ethiopic syllable ju",
            Ethiopic::SyllableJi => "ethiopic syllable ji",
            Ethiopic::SyllableJaa => "ethiopic syllable jaa",
            Ethiopic::SyllableJee => "ethiopic syllable jee",
            Ethiopic::SyllableJe => "ethiopic syllable je",
            Ethiopic::SyllableJo => "ethiopic syllable jo",
            Ethiopic::SyllableJwa => "ethiopic syllable jwa",
            Ethiopic::SyllableGa => "ethiopic syllable ga",
            Ethiopic::SyllableGu => "ethiopic syllable gu",
            Ethiopic::SyllableGi => "ethiopic syllable gi",
            Ethiopic::SyllableGaa => "ethiopic syllable gaa",
            Ethiopic::SyllableGee => "ethiopic syllable gee",
            Ethiopic::SyllableGe => "ethiopic syllable ge",
            Ethiopic::SyllableGo => "ethiopic syllable go",
            Ethiopic::SyllableGoa => "ethiopic syllable goa",
            Ethiopic::SyllableGwa => "ethiopic syllable gwa",
            Ethiopic::SyllableGwi => "ethiopic syllable gwi",
            Ethiopic::SyllableGwaa => "ethiopic syllable gwaa",
            Ethiopic::SyllableGwee => "ethiopic syllable gwee",
            Ethiopic::SyllableGwe => "ethiopic syllable gwe",
            Ethiopic::SyllableGga => "ethiopic syllable gga",
            Ethiopic::SyllableGgu => "ethiopic syllable ggu",
            Ethiopic::SyllableGgi => "ethiopic syllable ggi",
            Ethiopic::SyllableGgaa => "ethiopic syllable ggaa",
            Ethiopic::SyllableGgee => "ethiopic syllable ggee",
            Ethiopic::SyllableGge => "ethiopic syllable gge",
            Ethiopic::SyllableGgo => "ethiopic syllable ggo",
            Ethiopic::SyllableGgwaa => "ethiopic syllable ggwaa",
            Ethiopic::SyllableTha => "ethiopic syllable tha",
            Ethiopic::SyllableThu => "ethiopic syllable thu",
            Ethiopic::SyllableThi => "ethiopic syllable thi",
            Ethiopic::SyllableThaa => "ethiopic syllable thaa",
            Ethiopic::SyllableThee => "ethiopic syllable thee",
            Ethiopic::SyllableThe => "ethiopic syllable the",
            Ethiopic::SyllableTho => "ethiopic syllable tho",
            Ethiopic::SyllableThwa => "ethiopic syllable thwa",
            Ethiopic::SyllableCha => "ethiopic syllable cha",
            Ethiopic::SyllableChu => "ethiopic syllable chu",
            Ethiopic::SyllableChi => "ethiopic syllable chi",
            Ethiopic::SyllableChaa => "ethiopic syllable chaa",
            Ethiopic::SyllableChee => "ethiopic syllable chee",
            Ethiopic::SyllableChe => "ethiopic syllable che",
            Ethiopic::SyllableCho => "ethiopic syllable cho",
            Ethiopic::SyllableChwa => "ethiopic syllable chwa",
            Ethiopic::SyllablePha => "ethiopic syllable pha",
            Ethiopic::SyllablePhu => "ethiopic syllable phu",
            Ethiopic::SyllablePhi => "ethiopic syllable phi",
            Ethiopic::SyllablePhaa => "ethiopic syllable phaa",
            Ethiopic::SyllablePhee => "ethiopic syllable phee",
            Ethiopic::SyllablePhe => "ethiopic syllable phe",
            Ethiopic::SyllablePho => "ethiopic syllable pho",
            Ethiopic::SyllablePhwa => "ethiopic syllable phwa",
            Ethiopic::SyllableTsa => "ethiopic syllable tsa",
            Ethiopic::SyllableTsu => "ethiopic syllable tsu",
            Ethiopic::SyllableTsi => "ethiopic syllable tsi",
            Ethiopic::SyllableTsaa => "ethiopic syllable tsaa",
            Ethiopic::SyllableTsee => "ethiopic syllable tsee",
            Ethiopic::SyllableTse => "ethiopic syllable tse",
            Ethiopic::SyllableTso => "ethiopic syllable tso",
            Ethiopic::SyllableTswa => "ethiopic syllable tswa",
            Ethiopic::SyllableTza => "ethiopic syllable tza",
            Ethiopic::SyllableTzu => "ethiopic syllable tzu",
            Ethiopic::SyllableTzi => "ethiopic syllable tzi",
            Ethiopic::SyllableTzaa => "ethiopic syllable tzaa",
            Ethiopic::SyllableTzee => "ethiopic syllable tzee",
            Ethiopic::SyllableTze => "ethiopic syllable tze",
            Ethiopic::SyllableTzo => "ethiopic syllable tzo",
            Ethiopic::SyllableTzoa => "ethiopic syllable tzoa",
            Ethiopic::SyllableFa => "ethiopic syllable fa",
            Ethiopic::SyllableFu => "ethiopic syllable fu",
            Ethiopic::SyllableFi => "ethiopic syllable fi",
            Ethiopic::SyllableFaa => "ethiopic syllable faa",
            Ethiopic::SyllableFee => "ethiopic syllable fee",
            Ethiopic::SyllableFe => "ethiopic syllable fe",
            Ethiopic::SyllableFo => "ethiopic syllable fo",
            Ethiopic::SyllableFwa => "ethiopic syllable fwa",
            Ethiopic::SyllablePa => "ethiopic syllable pa",
            Ethiopic::SyllablePu => "ethiopic syllable pu",
            Ethiopic::SyllablePi => "ethiopic syllable pi",
            Ethiopic::SyllablePaa => "ethiopic syllable paa",
            Ethiopic::SyllablePee => "ethiopic syllable pee",
            Ethiopic::SyllablePe => "ethiopic syllable pe",
            Ethiopic::SyllablePo => "ethiopic syllable po",
            Ethiopic::SyllablePwa => "ethiopic syllable pwa",
            Ethiopic::SyllableRya => "ethiopic syllable rya",
            Ethiopic::SyllableMya => "ethiopic syllable mya",
            Ethiopic::SyllableFya => "ethiopic syllable fya",
            Ethiopic::CombiningGeminationAndVowelLengthMark => "ethiopic combining gemination and vowel length mark",
            Ethiopic::CombiningVowelLengthMark => "ethiopic combining vowel length mark",
            Ethiopic::CombiningGeminationMark => "ethiopic combining gemination mark",
            Ethiopic::SectionMark => "ethiopic section mark",
            Ethiopic::Wordspace => "ethiopic wordspace",
            Ethiopic::FullStop => "ethiopic full stop",
            Ethiopic::Comma => "ethiopic comma",
            Ethiopic::Semicolon => "ethiopic semicolon",
            Ethiopic::Colon => "ethiopic colon",
            Ethiopic::PrefaceColon => "ethiopic preface colon",
            Ethiopic::QuestionMark => "ethiopic question mark",
            Ethiopic::ParagraphSeparator => "ethiopic paragraph separator",
            Ethiopic::DigitOne => "ethiopic digit one",
            Ethiopic::DigitTwo => "ethiopic digit two",
            Ethiopic::DigitThree => "ethiopic digit three",
            Ethiopic::DigitFour => "ethiopic digit four",
            Ethiopic::DigitFive => "ethiopic digit five",
            Ethiopic::DigitSix => "ethiopic digit six",
            Ethiopic::DigitSeven => "ethiopic digit seven",
            Ethiopic::DigitEight => "ethiopic digit eight",
            Ethiopic::DigitNine => "ethiopic digit nine",
            Ethiopic::NumberTen => "ethiopic number ten",
            Ethiopic::NumberTwenty => "ethiopic number twenty",
            Ethiopic::NumberThirty => "ethiopic number thirty",
            Ethiopic::NumberForty => "ethiopic number forty",
            Ethiopic::NumberFifty => "ethiopic number fifty",
            Ethiopic::NumberSixty => "ethiopic number sixty",
            Ethiopic::NumberSeventy => "ethiopic number seventy",
            Ethiopic::NumberEighty => "ethiopic number eighty",
            Ethiopic::NumberNinety => "ethiopic number ninety",
            Ethiopic::NumberHundred => "ethiopic number hundred",
            Ethiopic::NumberTenThousand => "ethiopic number ten thousand",
        }
    }
}
