/// \u{1200} → \u{137f}\
///\
/// ሀ ሁ ሂ ሃ ሄ ህ ሆ ሇ ለ ሉ ሊ ላ ሌ ል ሎ ሏ\
/// ሐ ሑ ሒ ሓ ሔ ሕ ሖ ሗ መ ሙ ሚ ማ ሜ ም ሞ ሟ\
/// ሠ ሡ ሢ ሣ ሤ ሥ ሦ ሧ ረ ሩ ሪ ራ ሬ ር ሮ ሯ\
/// ሰ ሱ ሲ ሳ ሴ ስ ሶ ሷ ሸ ሹ ሺ ሻ ሼ ሽ ሾ ሿ\
/// ቀ ቁ ቂ ቃ ቄ ቅ ቆ ቇ ቈ ቊ ቋ ቌ ቍ ቐ ቑ ቒ\
/// ቓ ቔ ቕ ቖ ቘ ቚ ቛ ቜ ቝ በ ቡ ቢ ባ ቤ ብ ቦ\
/// ቧ ቨ ቩ ቪ ቫ ቬ ቭ ቮ ቯ ተ ቱ ቲ ታ ቴ ት ቶ\
/// ቷ ቸ ቹ ቺ ቻ ቼ ች ቾ ቿ ኀ ኁ ኂ ኃ ኄ ኅ ኆ\
/// ኇ ኈ ኊ ኋ ኌ ኍ ነ ኑ ኒ ና ኔ ን ኖ ኗ ኘ ኙ\
/// ኚ ኛ ኜ ኝ ኞ ኟ አ ኡ ኢ ኣ ኤ እ ኦ ኧ ከ ኩ\
/// ኪ ካ ኬ ክ ኮ ኯ ኰ ኲ ኳ ኴ ኵ ኸ ኹ ኺ ኻ ኼ\
/// ኽ ኾ ዀ ዂ ዃ ዄ ዅ ወ ዉ ዊ ዋ ዌ ው ዎ ዏ ዐ\
/// ዑ ዒ ዓ ዔ ዕ ዖ ዘ ዙ ዚ ዛ ዜ ዝ ዞ ዟ ዠ ዡ\
/// ዢ ዣ ዤ ዥ ዦ ዧ የ ዩ ዪ ያ ዬ ይ ዮ ዯ ደ ዱ\
/// ዲ ዳ ዴ ድ ዶ ዷ ዸ ዹ ዺ ዻ ዼ ዽ ዾ ዿ ጀ ጁ\
/// ጂ ጃ ጄ ጅ ጆ ጇ ገ ጉ ጊ ጋ ጌ ግ ጎ ጏ ጐ ጒ\
/// ጓ ጔ ጕ ጘ ጙ ጚ ጛ ጜ ጝ ጞ ጟ ጠ ጡ ጢ ጣ ጤ\
/// ጥ ጦ ጧ ጨ ጩ ጪ ጫ ጬ ጭ ጮ ጯ ጰ ጱ ጲ ጳ ጴ\
/// ጵ ጶ ጷ ጸ ጹ ጺ ጻ ጼ ጽ ጾ ጿ ፀ ፁ ፂ ፃ ፄ\
/// ፅ ፆ ፇ ፈ ፉ ፊ ፋ ፌ ፍ ፎ ፏ ፐ ፑ ፒ ፓ ፔ\
/// ፕ ፖ ፗ ፘ ፙ ፚ ፝ ፞ ፟ ፠ ፡ ። ፣ ፤ ፥ ፦\
/// ፧ ፨ ፩ ፪ ፫ ፬ ፭ ፮ ፯ ፰ ፱ ፲ ፳ ፴ ፵ ፶\
/// ፷ ፸ ፹ ፺ ፻ ፼\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1200}: 'ሀ'
    pub const SYLLABLE_HA: char = 'ሀ';
    /// \u{1201}: 'ሁ'
    pub const SYLLABLE_HU: char = 'ሁ';
    /// \u{1202}: 'ሂ'
    pub const SYLLABLE_HI: char = 'ሂ';
    /// \u{1203}: 'ሃ'
    pub const SYLLABLE_HAA: char = 'ሃ';
    /// \u{1204}: 'ሄ'
    pub const SYLLABLE_HEE: char = 'ሄ';
    /// \u{1205}: 'ህ'
    pub const SYLLABLE_HE: char = 'ህ';
    /// \u{1206}: 'ሆ'
    pub const SYLLABLE_HO: char = 'ሆ';
    /// \u{1207}: 'ሇ'
    pub const SYLLABLE_HOA: char = 'ሇ';
    /// \u{1208}: 'ለ'
    pub const SYLLABLE_LA: char = 'ለ';
    /// \u{1209}: 'ሉ'
    pub const SYLLABLE_LU: char = 'ሉ';
    /// \u{120a}: 'ሊ'
    pub const SYLLABLE_LI: char = 'ሊ';
    /// \u{120b}: 'ላ'
    pub const SYLLABLE_LAA: char = 'ላ';
    /// \u{120c}: 'ሌ'
    pub const SYLLABLE_LEE: char = 'ሌ';
    /// \u{120d}: 'ል'
    pub const SYLLABLE_LE: char = 'ል';
    /// \u{120e}: 'ሎ'
    pub const SYLLABLE_LO: char = 'ሎ';
    /// \u{120f}: 'ሏ'
    pub const SYLLABLE_LWA: char = 'ሏ';
    /// \u{1210}: 'ሐ'
    pub const SYLLABLE_HHA: char = 'ሐ';
    /// \u{1211}: 'ሑ'
    pub const SYLLABLE_HHU: char = 'ሑ';
    /// \u{1212}: 'ሒ'
    pub const SYLLABLE_HHI: char = 'ሒ';
    /// \u{1213}: 'ሓ'
    pub const SYLLABLE_HHAA: char = 'ሓ';
    /// \u{1214}: 'ሔ'
    pub const SYLLABLE_HHEE: char = 'ሔ';
    /// \u{1215}: 'ሕ'
    pub const SYLLABLE_HHE: char = 'ሕ';
    /// \u{1216}: 'ሖ'
    pub const SYLLABLE_HHO: char = 'ሖ';
    /// \u{1217}: 'ሗ'
    pub const SYLLABLE_HHWA: char = 'ሗ';
    /// \u{1218}: 'መ'
    pub const SYLLABLE_MA: char = 'መ';
    /// \u{1219}: 'ሙ'
    pub const SYLLABLE_MU: char = 'ሙ';
    /// \u{121a}: 'ሚ'
    pub const SYLLABLE_MI: char = 'ሚ';
    /// \u{121b}: 'ማ'
    pub const SYLLABLE_MAA: char = 'ማ';
    /// \u{121c}: 'ሜ'
    pub const SYLLABLE_MEE: char = 'ሜ';
    /// \u{121d}: 'ም'
    pub const SYLLABLE_ME: char = 'ም';
    /// \u{121e}: 'ሞ'
    pub const SYLLABLE_MO: char = 'ሞ';
    /// \u{121f}: 'ሟ'
    pub const SYLLABLE_MWA: char = 'ሟ';
    /// \u{1220}: 'ሠ'
    pub const SYLLABLE_SZA: char = 'ሠ';
    /// \u{1221}: 'ሡ'
    pub const SYLLABLE_SZU: char = 'ሡ';
    /// \u{1222}: 'ሢ'
    pub const SYLLABLE_SZI: char = 'ሢ';
    /// \u{1223}: 'ሣ'
    pub const SYLLABLE_SZAA: char = 'ሣ';
    /// \u{1224}: 'ሤ'
    pub const SYLLABLE_SZEE: char = 'ሤ';
    /// \u{1225}: 'ሥ'
    pub const SYLLABLE_SZE: char = 'ሥ';
    /// \u{1226}: 'ሦ'
    pub const SYLLABLE_SZO: char = 'ሦ';
    /// \u{1227}: 'ሧ'
    pub const SYLLABLE_SZWA: char = 'ሧ';
    /// \u{1228}: 'ረ'
    pub const SYLLABLE_RA: char = 'ረ';
    /// \u{1229}: 'ሩ'
    pub const SYLLABLE_RU: char = 'ሩ';
    /// \u{122a}: 'ሪ'
    pub const SYLLABLE_RI: char = 'ሪ';
    /// \u{122b}: 'ራ'
    pub const SYLLABLE_RAA: char = 'ራ';
    /// \u{122c}: 'ሬ'
    pub const SYLLABLE_REE: char = 'ሬ';
    /// \u{122d}: 'ር'
    pub const SYLLABLE_RE: char = 'ር';
    /// \u{122e}: 'ሮ'
    pub const SYLLABLE_RO: char = 'ሮ';
    /// \u{122f}: 'ሯ'
    pub const SYLLABLE_RWA: char = 'ሯ';
    /// \u{1230}: 'ሰ'
    pub const SYLLABLE_SA: char = 'ሰ';
    /// \u{1231}: 'ሱ'
    pub const SYLLABLE_SU: char = 'ሱ';
    /// \u{1232}: 'ሲ'
    pub const SYLLABLE_SI: char = 'ሲ';
    /// \u{1233}: 'ሳ'
    pub const SYLLABLE_SAA: char = 'ሳ';
    /// \u{1234}: 'ሴ'
    pub const SYLLABLE_SEE: char = 'ሴ';
    /// \u{1235}: 'ስ'
    pub const SYLLABLE_SE: char = 'ስ';
    /// \u{1236}: 'ሶ'
    pub const SYLLABLE_SO: char = 'ሶ';
    /// \u{1237}: 'ሷ'
    pub const SYLLABLE_SWA: char = 'ሷ';
    /// \u{1238}: 'ሸ'
    pub const SYLLABLE_SHA: char = 'ሸ';
    /// \u{1239}: 'ሹ'
    pub const SYLLABLE_SHU: char = 'ሹ';
    /// \u{123a}: 'ሺ'
    pub const SYLLABLE_SHI: char = 'ሺ';
    /// \u{123b}: 'ሻ'
    pub const SYLLABLE_SHAA: char = 'ሻ';
    /// \u{123c}: 'ሼ'
    pub const SYLLABLE_SHEE: char = 'ሼ';
    /// \u{123d}: 'ሽ'
    pub const SYLLABLE_SHE: char = 'ሽ';
    /// \u{123e}: 'ሾ'
    pub const SYLLABLE_SHO: char = 'ሾ';
    /// \u{123f}: 'ሿ'
    pub const SYLLABLE_SHWA: char = 'ሿ';
    /// \u{1240}: 'ቀ'
    pub const SYLLABLE_QA: char = 'ቀ';
    /// \u{1241}: 'ቁ'
    pub const SYLLABLE_QU: char = 'ቁ';
    /// \u{1242}: 'ቂ'
    pub const SYLLABLE_QI: char = 'ቂ';
    /// \u{1243}: 'ቃ'
    pub const SYLLABLE_QAA: char = 'ቃ';
    /// \u{1244}: 'ቄ'
    pub const SYLLABLE_QEE: char = 'ቄ';
    /// \u{1245}: 'ቅ'
    pub const SYLLABLE_QE: char = 'ቅ';
    /// \u{1246}: 'ቆ'
    pub const SYLLABLE_QO: char = 'ቆ';
    /// \u{1247}: 'ቇ'
    pub const SYLLABLE_QOA: char = 'ቇ';
    /// \u{1248}: 'ቈ'
    pub const SYLLABLE_QWA: char = 'ቈ';
    /// \u{124a}: 'ቊ'
    pub const SYLLABLE_QWI: char = 'ቊ';
    /// \u{124b}: 'ቋ'
    pub const SYLLABLE_QWAA: char = 'ቋ';
    /// \u{124c}: 'ቌ'
    pub const SYLLABLE_QWEE: char = 'ቌ';
    /// \u{124d}: 'ቍ'
    pub const SYLLABLE_QWE: char = 'ቍ';
    /// \u{1250}: 'ቐ'
    pub const SYLLABLE_QHA: char = 'ቐ';
    /// \u{1251}: 'ቑ'
    pub const SYLLABLE_QHU: char = 'ቑ';
    /// \u{1252}: 'ቒ'
    pub const SYLLABLE_QHI: char = 'ቒ';
    /// \u{1253}: 'ቓ'
    pub const SYLLABLE_QHAA: char = 'ቓ';
    /// \u{1254}: 'ቔ'
    pub const SYLLABLE_QHEE: char = 'ቔ';
    /// \u{1255}: 'ቕ'
    pub const SYLLABLE_QHE: char = 'ቕ';
    /// \u{1256}: 'ቖ'
    pub const SYLLABLE_QHO: char = 'ቖ';
    /// \u{1258}: 'ቘ'
    pub const SYLLABLE_QHWA: char = 'ቘ';
    /// \u{125a}: 'ቚ'
    pub const SYLLABLE_QHWI: char = 'ቚ';
    /// \u{125b}: 'ቛ'
    pub const SYLLABLE_QHWAA: char = 'ቛ';
    /// \u{125c}: 'ቜ'
    pub const SYLLABLE_QHWEE: char = 'ቜ';
    /// \u{125d}: 'ቝ'
    pub const SYLLABLE_QHWE: char = 'ቝ';
    /// \u{1260}: 'በ'
    pub const SYLLABLE_BA: char = 'በ';
    /// \u{1261}: 'ቡ'
    pub const SYLLABLE_BU: char = 'ቡ';
    /// \u{1262}: 'ቢ'
    pub const SYLLABLE_BI: char = 'ቢ';
    /// \u{1263}: 'ባ'
    pub const SYLLABLE_BAA: char = 'ባ';
    /// \u{1264}: 'ቤ'
    pub const SYLLABLE_BEE: char = 'ቤ';
    /// \u{1265}: 'ብ'
    pub const SYLLABLE_BE: char = 'ብ';
    /// \u{1266}: 'ቦ'
    pub const SYLLABLE_BO: char = 'ቦ';
    /// \u{1267}: 'ቧ'
    pub const SYLLABLE_BWA: char = 'ቧ';
    /// \u{1268}: 'ቨ'
    pub const SYLLABLE_VA: char = 'ቨ';
    /// \u{1269}: 'ቩ'
    pub const SYLLABLE_VU: char = 'ቩ';
    /// \u{126a}: 'ቪ'
    pub const SYLLABLE_VI: char = 'ቪ';
    /// \u{126b}: 'ቫ'
    pub const SYLLABLE_VAA: char = 'ቫ';
    /// \u{126c}: 'ቬ'
    pub const SYLLABLE_VEE: char = 'ቬ';
    /// \u{126d}: 'ቭ'
    pub const SYLLABLE_VE: char = 'ቭ';
    /// \u{126e}: 'ቮ'
    pub const SYLLABLE_VO: char = 'ቮ';
    /// \u{126f}: 'ቯ'
    pub const SYLLABLE_VWA: char = 'ቯ';
    /// \u{1270}: 'ተ'
    pub const SYLLABLE_TA: char = 'ተ';
    /// \u{1271}: 'ቱ'
    pub const SYLLABLE_TU: char = 'ቱ';
    /// \u{1272}: 'ቲ'
    pub const SYLLABLE_TI: char = 'ቲ';
    /// \u{1273}: 'ታ'
    pub const SYLLABLE_TAA: char = 'ታ';
    /// \u{1274}: 'ቴ'
    pub const SYLLABLE_TEE: char = 'ቴ';
    /// \u{1275}: 'ት'
    pub const SYLLABLE_TE: char = 'ት';
    /// \u{1276}: 'ቶ'
    pub const SYLLABLE_TO: char = 'ቶ';
    /// \u{1277}: 'ቷ'
    pub const SYLLABLE_TWA: char = 'ቷ';
    /// \u{1278}: 'ቸ'
    pub const SYLLABLE_CA: char = 'ቸ';
    /// \u{1279}: 'ቹ'
    pub const SYLLABLE_CU: char = 'ቹ';
    /// \u{127a}: 'ቺ'
    pub const SYLLABLE_CI: char = 'ቺ';
    /// \u{127b}: 'ቻ'
    pub const SYLLABLE_CAA: char = 'ቻ';
    /// \u{127c}: 'ቼ'
    pub const SYLLABLE_CEE: char = 'ቼ';
    /// \u{127d}: 'ች'
    pub const SYLLABLE_CE: char = 'ች';
    /// \u{127e}: 'ቾ'
    pub const SYLLABLE_CO: char = 'ቾ';
    /// \u{127f}: 'ቿ'
    pub const SYLLABLE_CWA: char = 'ቿ';
    /// \u{1280}: 'ኀ'
    pub const SYLLABLE_XA: char = 'ኀ';
    /// \u{1281}: 'ኁ'
    pub const SYLLABLE_XU: char = 'ኁ';
    /// \u{1282}: 'ኂ'
    pub const SYLLABLE_XI: char = 'ኂ';
    /// \u{1283}: 'ኃ'
    pub const SYLLABLE_XAA: char = 'ኃ';
    /// \u{1284}: 'ኄ'
    pub const SYLLABLE_XEE: char = 'ኄ';
    /// \u{1285}: 'ኅ'
    pub const SYLLABLE_XE: char = 'ኅ';
    /// \u{1286}: 'ኆ'
    pub const SYLLABLE_XO: char = 'ኆ';
    /// \u{1287}: 'ኇ'
    pub const SYLLABLE_XOA: char = 'ኇ';
    /// \u{1288}: 'ኈ'
    pub const SYLLABLE_XWA: char = 'ኈ';
    /// \u{128a}: 'ኊ'
    pub const SYLLABLE_XWI: char = 'ኊ';
    /// \u{128b}: 'ኋ'
    pub const SYLLABLE_XWAA: char = 'ኋ';
    /// \u{128c}: 'ኌ'
    pub const SYLLABLE_XWEE: char = 'ኌ';
    /// \u{128d}: 'ኍ'
    pub const SYLLABLE_XWE: char = 'ኍ';
    /// \u{1290}: 'ነ'
    pub const SYLLABLE_NA: char = 'ነ';
    /// \u{1291}: 'ኑ'
    pub const SYLLABLE_NU: char = 'ኑ';
    /// \u{1292}: 'ኒ'
    pub const SYLLABLE_NI: char = 'ኒ';
    /// \u{1293}: 'ና'
    pub const SYLLABLE_NAA: char = 'ና';
    /// \u{1294}: 'ኔ'
    pub const SYLLABLE_NEE: char = 'ኔ';
    /// \u{1295}: 'ን'
    pub const SYLLABLE_NE: char = 'ን';
    /// \u{1296}: 'ኖ'
    pub const SYLLABLE_NO: char = 'ኖ';
    /// \u{1297}: 'ኗ'
    pub const SYLLABLE_NWA: char = 'ኗ';
    /// \u{1298}: 'ኘ'
    pub const SYLLABLE_NYA: char = 'ኘ';
    /// \u{1299}: 'ኙ'
    pub const SYLLABLE_NYU: char = 'ኙ';
    /// \u{129a}: 'ኚ'
    pub const SYLLABLE_NYI: char = 'ኚ';
    /// \u{129b}: 'ኛ'
    pub const SYLLABLE_NYAA: char = 'ኛ';
    /// \u{129c}: 'ኜ'
    pub const SYLLABLE_NYEE: char = 'ኜ';
    /// \u{129d}: 'ኝ'
    pub const SYLLABLE_NYE: char = 'ኝ';
    /// \u{129e}: 'ኞ'
    pub const SYLLABLE_NYO: char = 'ኞ';
    /// \u{129f}: 'ኟ'
    pub const SYLLABLE_NYWA: char = 'ኟ';
    /// \u{12a0}: 'አ'
    pub const SYLLABLE_GLOTTAL_A: char = 'አ';
    /// \u{12a1}: 'ኡ'
    pub const SYLLABLE_GLOTTAL_U: char = 'ኡ';
    /// \u{12a2}: 'ኢ'
    pub const SYLLABLE_GLOTTAL_I: char = 'ኢ';
    /// \u{12a3}: 'ኣ'
    pub const SYLLABLE_GLOTTAL_AA: char = 'ኣ';
    /// \u{12a4}: 'ኤ'
    pub const SYLLABLE_GLOTTAL_EE: char = 'ኤ';
    /// \u{12a5}: 'እ'
    pub const SYLLABLE_GLOTTAL_E: char = 'እ';
    /// \u{12a6}: 'ኦ'
    pub const SYLLABLE_GLOTTAL_O: char = 'ኦ';
    /// \u{12a7}: 'ኧ'
    pub const SYLLABLE_GLOTTAL_WA: char = 'ኧ';
    /// \u{12a8}: 'ከ'
    pub const SYLLABLE_KA: char = 'ከ';
    /// \u{12a9}: 'ኩ'
    pub const SYLLABLE_KU: char = 'ኩ';
    /// \u{12aa}: 'ኪ'
    pub const SYLLABLE_KI: char = 'ኪ';
    /// \u{12ab}: 'ካ'
    pub const SYLLABLE_KAA: char = 'ካ';
    /// \u{12ac}: 'ኬ'
    pub const SYLLABLE_KEE: char = 'ኬ';
    /// \u{12ad}: 'ክ'
    pub const SYLLABLE_KE: char = 'ክ';
    /// \u{12ae}: 'ኮ'
    pub const SYLLABLE_KO: char = 'ኮ';
    /// \u{12af}: 'ኯ'
    pub const SYLLABLE_KOA: char = 'ኯ';
    /// \u{12b0}: 'ኰ'
    pub const SYLLABLE_KWA: char = 'ኰ';
    /// \u{12b2}: 'ኲ'
    pub const SYLLABLE_KWI: char = 'ኲ';
    /// \u{12b3}: 'ኳ'
    pub const SYLLABLE_KWAA: char = 'ኳ';
    /// \u{12b4}: 'ኴ'
    pub const SYLLABLE_KWEE: char = 'ኴ';
    /// \u{12b5}: 'ኵ'
    pub const SYLLABLE_KWE: char = 'ኵ';
    /// \u{12b8}: 'ኸ'
    pub const SYLLABLE_KXA: char = 'ኸ';
    /// \u{12b9}: 'ኹ'
    pub const SYLLABLE_KXU: char = 'ኹ';
    /// \u{12ba}: 'ኺ'
    pub const SYLLABLE_KXI: char = 'ኺ';
    /// \u{12bb}: 'ኻ'
    pub const SYLLABLE_KXAA: char = 'ኻ';
    /// \u{12bc}: 'ኼ'
    pub const SYLLABLE_KXEE: char = 'ኼ';
    /// \u{12bd}: 'ኽ'
    pub const SYLLABLE_KXE: char = 'ኽ';
    /// \u{12be}: 'ኾ'
    pub const SYLLABLE_KXO: char = 'ኾ';
    /// \u{12c0}: 'ዀ'
    pub const SYLLABLE_KXWA: char = 'ዀ';
    /// \u{12c2}: 'ዂ'
    pub const SYLLABLE_KXWI: char = 'ዂ';
    /// \u{12c3}: 'ዃ'
    pub const SYLLABLE_KXWAA: char = 'ዃ';
    /// \u{12c4}: 'ዄ'
    pub const SYLLABLE_KXWEE: char = 'ዄ';
    /// \u{12c5}: 'ዅ'
    pub const SYLLABLE_KXWE: char = 'ዅ';
    /// \u{12c8}: 'ወ'
    pub const SYLLABLE_WA: char = 'ወ';
    /// \u{12c9}: 'ዉ'
    pub const SYLLABLE_WU: char = 'ዉ';
    /// \u{12ca}: 'ዊ'
    pub const SYLLABLE_WI: char = 'ዊ';
    /// \u{12cb}: 'ዋ'
    pub const SYLLABLE_WAA: char = 'ዋ';
    /// \u{12cc}: 'ዌ'
    pub const SYLLABLE_WEE: char = 'ዌ';
    /// \u{12cd}: 'ው'
    pub const SYLLABLE_WE: char = 'ው';
    /// \u{12ce}: 'ዎ'
    pub const SYLLABLE_WO: char = 'ዎ';
    /// \u{12cf}: 'ዏ'
    pub const SYLLABLE_WOA: char = 'ዏ';
    /// \u{12d0}: 'ዐ'
    pub const SYLLABLE_PHARYNGEAL_A: char = 'ዐ';
    /// \u{12d1}: 'ዑ'
    pub const SYLLABLE_PHARYNGEAL_U: char = 'ዑ';
    /// \u{12d2}: 'ዒ'
    pub const SYLLABLE_PHARYNGEAL_I: char = 'ዒ';
    /// \u{12d3}: 'ዓ'
    pub const SYLLABLE_PHARYNGEAL_AA: char = 'ዓ';
    /// \u{12d4}: 'ዔ'
    pub const SYLLABLE_PHARYNGEAL_EE: char = 'ዔ';
    /// \u{12d5}: 'ዕ'
    pub const SYLLABLE_PHARYNGEAL_E: char = 'ዕ';
    /// \u{12d6}: 'ዖ'
    pub const SYLLABLE_PHARYNGEAL_O: char = 'ዖ';
    /// \u{12d8}: 'ዘ'
    pub const SYLLABLE_ZA: char = 'ዘ';
    /// \u{12d9}: 'ዙ'
    pub const SYLLABLE_ZU: char = 'ዙ';
    /// \u{12da}: 'ዚ'
    pub const SYLLABLE_ZI: char = 'ዚ';
    /// \u{12db}: 'ዛ'
    pub const SYLLABLE_ZAA: char = 'ዛ';
    /// \u{12dc}: 'ዜ'
    pub const SYLLABLE_ZEE: char = 'ዜ';
    /// \u{12dd}: 'ዝ'
    pub const SYLLABLE_ZE: char = 'ዝ';
    /// \u{12de}: 'ዞ'
    pub const SYLLABLE_ZO: char = 'ዞ';
    /// \u{12df}: 'ዟ'
    pub const SYLLABLE_ZWA: char = 'ዟ';
    /// \u{12e0}: 'ዠ'
    pub const SYLLABLE_ZHA: char = 'ዠ';
    /// \u{12e1}: 'ዡ'
    pub const SYLLABLE_ZHU: char = 'ዡ';
    /// \u{12e2}: 'ዢ'
    pub const SYLLABLE_ZHI: char = 'ዢ';
    /// \u{12e3}: 'ዣ'
    pub const SYLLABLE_ZHAA: char = 'ዣ';
    /// \u{12e4}: 'ዤ'
    pub const SYLLABLE_ZHEE: char = 'ዤ';
    /// \u{12e5}: 'ዥ'
    pub const SYLLABLE_ZHE: char = 'ዥ';
    /// \u{12e6}: 'ዦ'
    pub const SYLLABLE_ZHO: char = 'ዦ';
    /// \u{12e7}: 'ዧ'
    pub const SYLLABLE_ZHWA: char = 'ዧ';
    /// \u{12e8}: 'የ'
    pub const SYLLABLE_YA: char = 'የ';
    /// \u{12e9}: 'ዩ'
    pub const SYLLABLE_YU: char = 'ዩ';
    /// \u{12ea}: 'ዪ'
    pub const SYLLABLE_YI: char = 'ዪ';
    /// \u{12eb}: 'ያ'
    pub const SYLLABLE_YAA: char = 'ያ';
    /// \u{12ec}: 'ዬ'
    pub const SYLLABLE_YEE: char = 'ዬ';
    /// \u{12ed}: 'ይ'
    pub const SYLLABLE_YE: char = 'ይ';
    /// \u{12ee}: 'ዮ'
    pub const SYLLABLE_YO: char = 'ዮ';
    /// \u{12ef}: 'ዯ'
    pub const SYLLABLE_YOA: char = 'ዯ';
    /// \u{12f0}: 'ደ'
    pub const SYLLABLE_DA: char = 'ደ';
    /// \u{12f1}: 'ዱ'
    pub const SYLLABLE_DU: char = 'ዱ';
    /// \u{12f2}: 'ዲ'
    pub const SYLLABLE_DI: char = 'ዲ';
    /// \u{12f3}: 'ዳ'
    pub const SYLLABLE_DAA: char = 'ዳ';
    /// \u{12f4}: 'ዴ'
    pub const SYLLABLE_DEE: char = 'ዴ';
    /// \u{12f5}: 'ድ'
    pub const SYLLABLE_DE: char = 'ድ';
    /// \u{12f6}: 'ዶ'
    pub const SYLLABLE_DO: char = 'ዶ';
    /// \u{12f7}: 'ዷ'
    pub const SYLLABLE_DWA: char = 'ዷ';
    /// \u{12f8}: 'ዸ'
    pub const SYLLABLE_DDA: char = 'ዸ';
    /// \u{12f9}: 'ዹ'
    pub const SYLLABLE_DDU: char = 'ዹ';
    /// \u{12fa}: 'ዺ'
    pub const SYLLABLE_DDI: char = 'ዺ';
    /// \u{12fb}: 'ዻ'
    pub const SYLLABLE_DDAA: char = 'ዻ';
    /// \u{12fc}: 'ዼ'
    pub const SYLLABLE_DDEE: char = 'ዼ';
    /// \u{12fd}: 'ዽ'
    pub const SYLLABLE_DDE: char = 'ዽ';
    /// \u{12fe}: 'ዾ'
    pub const SYLLABLE_DDO: char = 'ዾ';
    /// \u{12ff}: 'ዿ'
    pub const SYLLABLE_DDWA: char = 'ዿ';
    /// \u{1300}: 'ጀ'
    pub const SYLLABLE_JA: char = 'ጀ';
    /// \u{1301}: 'ጁ'
    pub const SYLLABLE_JU: char = 'ጁ';
    /// \u{1302}: 'ጂ'
    pub const SYLLABLE_JI: char = 'ጂ';
    /// \u{1303}: 'ጃ'
    pub const SYLLABLE_JAA: char = 'ጃ';
    /// \u{1304}: 'ጄ'
    pub const SYLLABLE_JEE: char = 'ጄ';
    /// \u{1305}: 'ጅ'
    pub const SYLLABLE_JE: char = 'ጅ';
    /// \u{1306}: 'ጆ'
    pub const SYLLABLE_JO: char = 'ጆ';
    /// \u{1307}: 'ጇ'
    pub const SYLLABLE_JWA: char = 'ጇ';
    /// \u{1308}: 'ገ'
    pub const SYLLABLE_GA: char = 'ገ';
    /// \u{1309}: 'ጉ'
    pub const SYLLABLE_GU: char = 'ጉ';
    /// \u{130a}: 'ጊ'
    pub const SYLLABLE_GI: char = 'ጊ';
    /// \u{130b}: 'ጋ'
    pub const SYLLABLE_GAA: char = 'ጋ';
    /// \u{130c}: 'ጌ'
    pub const SYLLABLE_GEE: char = 'ጌ';
    /// \u{130d}: 'ግ'
    pub const SYLLABLE_GE: char = 'ግ';
    /// \u{130e}: 'ጎ'
    pub const SYLLABLE_GO: char = 'ጎ';
    /// \u{130f}: 'ጏ'
    pub const SYLLABLE_GOA: char = 'ጏ';
    /// \u{1310}: 'ጐ'
    pub const SYLLABLE_GWA: char = 'ጐ';
    /// \u{1312}: 'ጒ'
    pub const SYLLABLE_GWI: char = 'ጒ';
    /// \u{1313}: 'ጓ'
    pub const SYLLABLE_GWAA: char = 'ጓ';
    /// \u{1314}: 'ጔ'
    pub const SYLLABLE_GWEE: char = 'ጔ';
    /// \u{1315}: 'ጕ'
    pub const SYLLABLE_GWE: char = 'ጕ';
    /// \u{1318}: 'ጘ'
    pub const SYLLABLE_GGA: char = 'ጘ';
    /// \u{1319}: 'ጙ'
    pub const SYLLABLE_GGU: char = 'ጙ';
    /// \u{131a}: 'ጚ'
    pub const SYLLABLE_GGI: char = 'ጚ';
    /// \u{131b}: 'ጛ'
    pub const SYLLABLE_GGAA: char = 'ጛ';
    /// \u{131c}: 'ጜ'
    pub const SYLLABLE_GGEE: char = 'ጜ';
    /// \u{131d}: 'ጝ'
    pub const SYLLABLE_GGE: char = 'ጝ';
    /// \u{131e}: 'ጞ'
    pub const SYLLABLE_GGO: char = 'ጞ';
    /// \u{131f}: 'ጟ'
    pub const SYLLABLE_GGWAA: char = 'ጟ';
    /// \u{1320}: 'ጠ'
    pub const SYLLABLE_THA: char = 'ጠ';
    /// \u{1321}: 'ጡ'
    pub const SYLLABLE_THU: char = 'ጡ';
    /// \u{1322}: 'ጢ'
    pub const SYLLABLE_THI: char = 'ጢ';
    /// \u{1323}: 'ጣ'
    pub const SYLLABLE_THAA: char = 'ጣ';
    /// \u{1324}: 'ጤ'
    pub const SYLLABLE_THEE: char = 'ጤ';
    /// \u{1325}: 'ጥ'
    pub const SYLLABLE_THE: char = 'ጥ';
    /// \u{1326}: 'ጦ'
    pub const SYLLABLE_THO: char = 'ጦ';
    /// \u{1327}: 'ጧ'
    pub const SYLLABLE_THWA: char = 'ጧ';
    /// \u{1328}: 'ጨ'
    pub const SYLLABLE_CHA: char = 'ጨ';
    /// \u{1329}: 'ጩ'
    pub const SYLLABLE_CHU: char = 'ጩ';
    /// \u{132a}: 'ጪ'
    pub const SYLLABLE_CHI: char = 'ጪ';
    /// \u{132b}: 'ጫ'
    pub const SYLLABLE_CHAA: char = 'ጫ';
    /// \u{132c}: 'ጬ'
    pub const SYLLABLE_CHEE: char = 'ጬ';
    /// \u{132d}: 'ጭ'
    pub const SYLLABLE_CHE: char = 'ጭ';
    /// \u{132e}: 'ጮ'
    pub const SYLLABLE_CHO: char = 'ጮ';
    /// \u{132f}: 'ጯ'
    pub const SYLLABLE_CHWA: char = 'ጯ';
    /// \u{1330}: 'ጰ'
    pub const SYLLABLE_PHA: char = 'ጰ';
    /// \u{1331}: 'ጱ'
    pub const SYLLABLE_PHU: char = 'ጱ';
    /// \u{1332}: 'ጲ'
    pub const SYLLABLE_PHI: char = 'ጲ';
    /// \u{1333}: 'ጳ'
    pub const SYLLABLE_PHAA: char = 'ጳ';
    /// \u{1334}: 'ጴ'
    pub const SYLLABLE_PHEE: char = 'ጴ';
    /// \u{1335}: 'ጵ'
    pub const SYLLABLE_PHE: char = 'ጵ';
    /// \u{1336}: 'ጶ'
    pub const SYLLABLE_PHO: char = 'ጶ';
    /// \u{1337}: 'ጷ'
    pub const SYLLABLE_PHWA: char = 'ጷ';
    /// \u{1338}: 'ጸ'
    pub const SYLLABLE_TSA: char = 'ጸ';
    /// \u{1339}: 'ጹ'
    pub const SYLLABLE_TSU: char = 'ጹ';
    /// \u{133a}: 'ጺ'
    pub const SYLLABLE_TSI: char = 'ጺ';
    /// \u{133b}: 'ጻ'
    pub const SYLLABLE_TSAA: char = 'ጻ';
    /// \u{133c}: 'ጼ'
    pub const SYLLABLE_TSEE: char = 'ጼ';
    /// \u{133d}: 'ጽ'
    pub const SYLLABLE_TSE: char = 'ጽ';
    /// \u{133e}: 'ጾ'
    pub const SYLLABLE_TSO: char = 'ጾ';
    /// \u{133f}: 'ጿ'
    pub const SYLLABLE_TSWA: char = 'ጿ';
    /// \u{1340}: 'ፀ'
    pub const SYLLABLE_TZA: char = 'ፀ';
    /// \u{1341}: 'ፁ'
    pub const SYLLABLE_TZU: char = 'ፁ';
    /// \u{1342}: 'ፂ'
    pub const SYLLABLE_TZI: char = 'ፂ';
    /// \u{1343}: 'ፃ'
    pub const SYLLABLE_TZAA: char = 'ፃ';
    /// \u{1344}: 'ፄ'
    pub const SYLLABLE_TZEE: char = 'ፄ';
    /// \u{1345}: 'ፅ'
    pub const SYLLABLE_TZE: char = 'ፅ';
    /// \u{1346}: 'ፆ'
    pub const SYLLABLE_TZO: char = 'ፆ';
    /// \u{1347}: 'ፇ'
    pub const SYLLABLE_TZOA: char = 'ፇ';
    /// \u{1348}: 'ፈ'
    pub const SYLLABLE_FA: char = 'ፈ';
    /// \u{1349}: 'ፉ'
    pub const SYLLABLE_FU: char = 'ፉ';
    /// \u{134a}: 'ፊ'
    pub const SYLLABLE_FI: char = 'ፊ';
    /// \u{134b}: 'ፋ'
    pub const SYLLABLE_FAA: char = 'ፋ';
    /// \u{134c}: 'ፌ'
    pub const SYLLABLE_FEE: char = 'ፌ';
    /// \u{134d}: 'ፍ'
    pub const SYLLABLE_FE: char = 'ፍ';
    /// \u{134e}: 'ፎ'
    pub const SYLLABLE_FO: char = 'ፎ';
    /// \u{134f}: 'ፏ'
    pub const SYLLABLE_FWA: char = 'ፏ';
    /// \u{1350}: 'ፐ'
    pub const SYLLABLE_PA: char = 'ፐ';
    /// \u{1351}: 'ፑ'
    pub const SYLLABLE_PU: char = 'ፑ';
    /// \u{1352}: 'ፒ'
    pub const SYLLABLE_PI: char = 'ፒ';
    /// \u{1353}: 'ፓ'
    pub const SYLLABLE_PAA: char = 'ፓ';
    /// \u{1354}: 'ፔ'
    pub const SYLLABLE_PEE: char = 'ፔ';
    /// \u{1355}: 'ፕ'
    pub const SYLLABLE_PE: char = 'ፕ';
    /// \u{1356}: 'ፖ'
    pub const SYLLABLE_PO: char = 'ፖ';
    /// \u{1357}: 'ፗ'
    pub const SYLLABLE_PWA: char = 'ፗ';
    /// \u{1358}: 'ፘ'
    pub const SYLLABLE_RYA: char = 'ፘ';
    /// \u{1359}: 'ፙ'
    pub const SYLLABLE_MYA: char = 'ፙ';
    /// \u{135a}: 'ፚ'
    pub const SYLLABLE_FYA: char = 'ፚ';
    /// \u{135d}: '፝'
    pub const COMBINING_GEMINATION_AND_VOWEL_LENGTH_MARK: char = '፝';
    /// \u{135e}: '፞'
    pub const COMBINING_VOWEL_LENGTH_MARK: char = '፞';
    /// \u{135f}: '፟'
    pub const COMBINING_GEMINATION_MARK: char = '፟';
    /// \u{1360}: '፠'
    pub const SECTION_MARK: char = '፠';
    /// \u{1361}: '፡'
    pub const WORDSPACE: char = '፡';
    /// \u{1362}: '።'
    pub const FULL_STOP: char = '።';
    /// \u{1363}: '፣'
    pub const COMMA: char = '፣';
    /// \u{1364}: '፤'
    pub const SEMICOLON: char = '፤';
    /// \u{1365}: '፥'
    pub const COLON: char = '፥';
    /// \u{1366}: '፦'
    pub const PREFACE_COLON: char = '፦';
    /// \u{1367}: '፧'
    pub const QUESTION_MARK: char = '፧';
    /// \u{1368}: '፨'
    pub const PARAGRAPH_SEPARATOR: char = '፨';
    /// \u{1369}: '፩'
    pub const DIGIT_ONE: char = '፩';
    /// \u{136a}: '፪'
    pub const DIGIT_TWO: char = '፪';
    /// \u{136b}: '፫'
    pub const DIGIT_THREE: char = '፫';
    /// \u{136c}: '፬'
    pub const DIGIT_FOUR: char = '፬';
    /// \u{136d}: '፭'
    pub const DIGIT_FIVE: char = '፭';
    /// \u{136e}: '፮'
    pub const DIGIT_SIX: char = '፮';
    /// \u{136f}: '፯'
    pub const DIGIT_SEVEN: char = '፯';
    /// \u{1370}: '፰'
    pub const DIGIT_EIGHT: char = '፰';
    /// \u{1371}: '፱'
    pub const DIGIT_NINE: char = '፱';
    /// \u{1372}: '፲'
    pub const NUMBER_TEN: char = '፲';
    /// \u{1373}: '፳'
    pub const NUMBER_TWENTY: char = '፳';
    /// \u{1374}: '፴'
    pub const NUMBER_THIRTY: char = '፴';
    /// \u{1375}: '፵'
    pub const NUMBER_FORTY: char = '፵';
    /// \u{1376}: '፶'
    pub const NUMBER_FIFTY: char = '፶';
    /// \u{1377}: '፷'
    pub const NUMBER_SIXTY: char = '፷';
    /// \u{1378}: '፸'
    pub const NUMBER_SEVENTY: char = '፸';
    /// \u{1379}: '፹'
    pub const NUMBER_EIGHTY: char = '፹';
    /// \u{137a}: '፺'
    pub const NUMBER_NINETY: char = '፺';
    /// \u{137b}: '፻'
    pub const NUMBER_HUNDRED: char = '፻';
    /// \u{137c}: '፼'
    pub const NUMBER_TEN_THOUSAND: char = '፼';
}

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
        use constants::*;
        match self {
            Ethiopic::SyllableHa => SYLLABLE_HA,
            Ethiopic::SyllableHu => SYLLABLE_HU,
            Ethiopic::SyllableHi => SYLLABLE_HI,
            Ethiopic::SyllableHaa => SYLLABLE_HAA,
            Ethiopic::SyllableHee => SYLLABLE_HEE,
            Ethiopic::SyllableHe => SYLLABLE_HE,
            Ethiopic::SyllableHo => SYLLABLE_HO,
            Ethiopic::SyllableHoa => SYLLABLE_HOA,
            Ethiopic::SyllableLa => SYLLABLE_LA,
            Ethiopic::SyllableLu => SYLLABLE_LU,
            Ethiopic::SyllableLi => SYLLABLE_LI,
            Ethiopic::SyllableLaa => SYLLABLE_LAA,
            Ethiopic::SyllableLee => SYLLABLE_LEE,
            Ethiopic::SyllableLe => SYLLABLE_LE,
            Ethiopic::SyllableLo => SYLLABLE_LO,
            Ethiopic::SyllableLwa => SYLLABLE_LWA,
            Ethiopic::SyllableHha => SYLLABLE_HHA,
            Ethiopic::SyllableHhu => SYLLABLE_HHU,
            Ethiopic::SyllableHhi => SYLLABLE_HHI,
            Ethiopic::SyllableHhaa => SYLLABLE_HHAA,
            Ethiopic::SyllableHhee => SYLLABLE_HHEE,
            Ethiopic::SyllableHhe => SYLLABLE_HHE,
            Ethiopic::SyllableHho => SYLLABLE_HHO,
            Ethiopic::SyllableHhwa => SYLLABLE_HHWA,
            Ethiopic::SyllableMa => SYLLABLE_MA,
            Ethiopic::SyllableMu => SYLLABLE_MU,
            Ethiopic::SyllableMi => SYLLABLE_MI,
            Ethiopic::SyllableMaa => SYLLABLE_MAA,
            Ethiopic::SyllableMee => SYLLABLE_MEE,
            Ethiopic::SyllableMe => SYLLABLE_ME,
            Ethiopic::SyllableMo => SYLLABLE_MO,
            Ethiopic::SyllableMwa => SYLLABLE_MWA,
            Ethiopic::SyllableSza => SYLLABLE_SZA,
            Ethiopic::SyllableSzu => SYLLABLE_SZU,
            Ethiopic::SyllableSzi => SYLLABLE_SZI,
            Ethiopic::SyllableSzaa => SYLLABLE_SZAA,
            Ethiopic::SyllableSzee => SYLLABLE_SZEE,
            Ethiopic::SyllableSze => SYLLABLE_SZE,
            Ethiopic::SyllableSzo => SYLLABLE_SZO,
            Ethiopic::SyllableSzwa => SYLLABLE_SZWA,
            Ethiopic::SyllableRa => SYLLABLE_RA,
            Ethiopic::SyllableRu => SYLLABLE_RU,
            Ethiopic::SyllableRi => SYLLABLE_RI,
            Ethiopic::SyllableRaa => SYLLABLE_RAA,
            Ethiopic::SyllableRee => SYLLABLE_REE,
            Ethiopic::SyllableRe => SYLLABLE_RE,
            Ethiopic::SyllableRo => SYLLABLE_RO,
            Ethiopic::SyllableRwa => SYLLABLE_RWA,
            Ethiopic::SyllableSa => SYLLABLE_SA,
            Ethiopic::SyllableSu => SYLLABLE_SU,
            Ethiopic::SyllableSi => SYLLABLE_SI,
            Ethiopic::SyllableSaa => SYLLABLE_SAA,
            Ethiopic::SyllableSee => SYLLABLE_SEE,
            Ethiopic::SyllableSe => SYLLABLE_SE,
            Ethiopic::SyllableSo => SYLLABLE_SO,
            Ethiopic::SyllableSwa => SYLLABLE_SWA,
            Ethiopic::SyllableSha => SYLLABLE_SHA,
            Ethiopic::SyllableShu => SYLLABLE_SHU,
            Ethiopic::SyllableShi => SYLLABLE_SHI,
            Ethiopic::SyllableShaa => SYLLABLE_SHAA,
            Ethiopic::SyllableShee => SYLLABLE_SHEE,
            Ethiopic::SyllableShe => SYLLABLE_SHE,
            Ethiopic::SyllableSho => SYLLABLE_SHO,
            Ethiopic::SyllableShwa => SYLLABLE_SHWA,
            Ethiopic::SyllableQa => SYLLABLE_QA,
            Ethiopic::SyllableQu => SYLLABLE_QU,
            Ethiopic::SyllableQi => SYLLABLE_QI,
            Ethiopic::SyllableQaa => SYLLABLE_QAA,
            Ethiopic::SyllableQee => SYLLABLE_QEE,
            Ethiopic::SyllableQe => SYLLABLE_QE,
            Ethiopic::SyllableQo => SYLLABLE_QO,
            Ethiopic::SyllableQoa => SYLLABLE_QOA,
            Ethiopic::SyllableQwa => SYLLABLE_QWA,
            Ethiopic::SyllableQwi => SYLLABLE_QWI,
            Ethiopic::SyllableQwaa => SYLLABLE_QWAA,
            Ethiopic::SyllableQwee => SYLLABLE_QWEE,
            Ethiopic::SyllableQwe => SYLLABLE_QWE,
            Ethiopic::SyllableQha => SYLLABLE_QHA,
            Ethiopic::SyllableQhu => SYLLABLE_QHU,
            Ethiopic::SyllableQhi => SYLLABLE_QHI,
            Ethiopic::SyllableQhaa => SYLLABLE_QHAA,
            Ethiopic::SyllableQhee => SYLLABLE_QHEE,
            Ethiopic::SyllableQhe => SYLLABLE_QHE,
            Ethiopic::SyllableQho => SYLLABLE_QHO,
            Ethiopic::SyllableQhwa => SYLLABLE_QHWA,
            Ethiopic::SyllableQhwi => SYLLABLE_QHWI,
            Ethiopic::SyllableQhwaa => SYLLABLE_QHWAA,
            Ethiopic::SyllableQhwee => SYLLABLE_QHWEE,
            Ethiopic::SyllableQhwe => SYLLABLE_QHWE,
            Ethiopic::SyllableBa => SYLLABLE_BA,
            Ethiopic::SyllableBu => SYLLABLE_BU,
            Ethiopic::SyllableBi => SYLLABLE_BI,
            Ethiopic::SyllableBaa => SYLLABLE_BAA,
            Ethiopic::SyllableBee => SYLLABLE_BEE,
            Ethiopic::SyllableBe => SYLLABLE_BE,
            Ethiopic::SyllableBo => SYLLABLE_BO,
            Ethiopic::SyllableBwa => SYLLABLE_BWA,
            Ethiopic::SyllableVa => SYLLABLE_VA,
            Ethiopic::SyllableVu => SYLLABLE_VU,
            Ethiopic::SyllableVi => SYLLABLE_VI,
            Ethiopic::SyllableVaa => SYLLABLE_VAA,
            Ethiopic::SyllableVee => SYLLABLE_VEE,
            Ethiopic::SyllableVe => SYLLABLE_VE,
            Ethiopic::SyllableVo => SYLLABLE_VO,
            Ethiopic::SyllableVwa => SYLLABLE_VWA,
            Ethiopic::SyllableTa => SYLLABLE_TA,
            Ethiopic::SyllableTu => SYLLABLE_TU,
            Ethiopic::SyllableTi => SYLLABLE_TI,
            Ethiopic::SyllableTaa => SYLLABLE_TAA,
            Ethiopic::SyllableTee => SYLLABLE_TEE,
            Ethiopic::SyllableTe => SYLLABLE_TE,
            Ethiopic::SyllableTo => SYLLABLE_TO,
            Ethiopic::SyllableTwa => SYLLABLE_TWA,
            Ethiopic::SyllableCa => SYLLABLE_CA,
            Ethiopic::SyllableCu => SYLLABLE_CU,
            Ethiopic::SyllableCi => SYLLABLE_CI,
            Ethiopic::SyllableCaa => SYLLABLE_CAA,
            Ethiopic::SyllableCee => SYLLABLE_CEE,
            Ethiopic::SyllableCe => SYLLABLE_CE,
            Ethiopic::SyllableCo => SYLLABLE_CO,
            Ethiopic::SyllableCwa => SYLLABLE_CWA,
            Ethiopic::SyllableXa => SYLLABLE_XA,
            Ethiopic::SyllableXu => SYLLABLE_XU,
            Ethiopic::SyllableXi => SYLLABLE_XI,
            Ethiopic::SyllableXaa => SYLLABLE_XAA,
            Ethiopic::SyllableXee => SYLLABLE_XEE,
            Ethiopic::SyllableXe => SYLLABLE_XE,
            Ethiopic::SyllableXo => SYLLABLE_XO,
            Ethiopic::SyllableXoa => SYLLABLE_XOA,
            Ethiopic::SyllableXwa => SYLLABLE_XWA,
            Ethiopic::SyllableXwi => SYLLABLE_XWI,
            Ethiopic::SyllableXwaa => SYLLABLE_XWAA,
            Ethiopic::SyllableXwee => SYLLABLE_XWEE,
            Ethiopic::SyllableXwe => SYLLABLE_XWE,
            Ethiopic::SyllableNa => SYLLABLE_NA,
            Ethiopic::SyllableNu => SYLLABLE_NU,
            Ethiopic::SyllableNi => SYLLABLE_NI,
            Ethiopic::SyllableNaa => SYLLABLE_NAA,
            Ethiopic::SyllableNee => SYLLABLE_NEE,
            Ethiopic::SyllableNe => SYLLABLE_NE,
            Ethiopic::SyllableNo => SYLLABLE_NO,
            Ethiopic::SyllableNwa => SYLLABLE_NWA,
            Ethiopic::SyllableNya => SYLLABLE_NYA,
            Ethiopic::SyllableNyu => SYLLABLE_NYU,
            Ethiopic::SyllableNyi => SYLLABLE_NYI,
            Ethiopic::SyllableNyaa => SYLLABLE_NYAA,
            Ethiopic::SyllableNyee => SYLLABLE_NYEE,
            Ethiopic::SyllableNye => SYLLABLE_NYE,
            Ethiopic::SyllableNyo => SYLLABLE_NYO,
            Ethiopic::SyllableNywa => SYLLABLE_NYWA,
            Ethiopic::SyllableGlottalA => SYLLABLE_GLOTTAL_A,
            Ethiopic::SyllableGlottalU => SYLLABLE_GLOTTAL_U,
            Ethiopic::SyllableGlottalI => SYLLABLE_GLOTTAL_I,
            Ethiopic::SyllableGlottalAa => SYLLABLE_GLOTTAL_AA,
            Ethiopic::SyllableGlottalEe => SYLLABLE_GLOTTAL_EE,
            Ethiopic::SyllableGlottalE => SYLLABLE_GLOTTAL_E,
            Ethiopic::SyllableGlottalO => SYLLABLE_GLOTTAL_O,
            Ethiopic::SyllableGlottalWa => SYLLABLE_GLOTTAL_WA,
            Ethiopic::SyllableKa => SYLLABLE_KA,
            Ethiopic::SyllableKu => SYLLABLE_KU,
            Ethiopic::SyllableKi => SYLLABLE_KI,
            Ethiopic::SyllableKaa => SYLLABLE_KAA,
            Ethiopic::SyllableKee => SYLLABLE_KEE,
            Ethiopic::SyllableKe => SYLLABLE_KE,
            Ethiopic::SyllableKo => SYLLABLE_KO,
            Ethiopic::SyllableKoa => SYLLABLE_KOA,
            Ethiopic::SyllableKwa => SYLLABLE_KWA,
            Ethiopic::SyllableKwi => SYLLABLE_KWI,
            Ethiopic::SyllableKwaa => SYLLABLE_KWAA,
            Ethiopic::SyllableKwee => SYLLABLE_KWEE,
            Ethiopic::SyllableKwe => SYLLABLE_KWE,
            Ethiopic::SyllableKxa => SYLLABLE_KXA,
            Ethiopic::SyllableKxu => SYLLABLE_KXU,
            Ethiopic::SyllableKxi => SYLLABLE_KXI,
            Ethiopic::SyllableKxaa => SYLLABLE_KXAA,
            Ethiopic::SyllableKxee => SYLLABLE_KXEE,
            Ethiopic::SyllableKxe => SYLLABLE_KXE,
            Ethiopic::SyllableKxo => SYLLABLE_KXO,
            Ethiopic::SyllableKxwa => SYLLABLE_KXWA,
            Ethiopic::SyllableKxwi => SYLLABLE_KXWI,
            Ethiopic::SyllableKxwaa => SYLLABLE_KXWAA,
            Ethiopic::SyllableKxwee => SYLLABLE_KXWEE,
            Ethiopic::SyllableKxwe => SYLLABLE_KXWE,
            Ethiopic::SyllableWa => SYLLABLE_WA,
            Ethiopic::SyllableWu => SYLLABLE_WU,
            Ethiopic::SyllableWi => SYLLABLE_WI,
            Ethiopic::SyllableWaa => SYLLABLE_WAA,
            Ethiopic::SyllableWee => SYLLABLE_WEE,
            Ethiopic::SyllableWe => SYLLABLE_WE,
            Ethiopic::SyllableWo => SYLLABLE_WO,
            Ethiopic::SyllableWoa => SYLLABLE_WOA,
            Ethiopic::SyllablePharyngealA => SYLLABLE_PHARYNGEAL_A,
            Ethiopic::SyllablePharyngealU => SYLLABLE_PHARYNGEAL_U,
            Ethiopic::SyllablePharyngealI => SYLLABLE_PHARYNGEAL_I,
            Ethiopic::SyllablePharyngealAa => SYLLABLE_PHARYNGEAL_AA,
            Ethiopic::SyllablePharyngealEe => SYLLABLE_PHARYNGEAL_EE,
            Ethiopic::SyllablePharyngealE => SYLLABLE_PHARYNGEAL_E,
            Ethiopic::SyllablePharyngealO => SYLLABLE_PHARYNGEAL_O,
            Ethiopic::SyllableZa => SYLLABLE_ZA,
            Ethiopic::SyllableZu => SYLLABLE_ZU,
            Ethiopic::SyllableZi => SYLLABLE_ZI,
            Ethiopic::SyllableZaa => SYLLABLE_ZAA,
            Ethiopic::SyllableZee => SYLLABLE_ZEE,
            Ethiopic::SyllableZe => SYLLABLE_ZE,
            Ethiopic::SyllableZo => SYLLABLE_ZO,
            Ethiopic::SyllableZwa => SYLLABLE_ZWA,
            Ethiopic::SyllableZha => SYLLABLE_ZHA,
            Ethiopic::SyllableZhu => SYLLABLE_ZHU,
            Ethiopic::SyllableZhi => SYLLABLE_ZHI,
            Ethiopic::SyllableZhaa => SYLLABLE_ZHAA,
            Ethiopic::SyllableZhee => SYLLABLE_ZHEE,
            Ethiopic::SyllableZhe => SYLLABLE_ZHE,
            Ethiopic::SyllableZho => SYLLABLE_ZHO,
            Ethiopic::SyllableZhwa => SYLLABLE_ZHWA,
            Ethiopic::SyllableYa => SYLLABLE_YA,
            Ethiopic::SyllableYu => SYLLABLE_YU,
            Ethiopic::SyllableYi => SYLLABLE_YI,
            Ethiopic::SyllableYaa => SYLLABLE_YAA,
            Ethiopic::SyllableYee => SYLLABLE_YEE,
            Ethiopic::SyllableYe => SYLLABLE_YE,
            Ethiopic::SyllableYo => SYLLABLE_YO,
            Ethiopic::SyllableYoa => SYLLABLE_YOA,
            Ethiopic::SyllableDa => SYLLABLE_DA,
            Ethiopic::SyllableDu => SYLLABLE_DU,
            Ethiopic::SyllableDi => SYLLABLE_DI,
            Ethiopic::SyllableDaa => SYLLABLE_DAA,
            Ethiopic::SyllableDee => SYLLABLE_DEE,
            Ethiopic::SyllableDe => SYLLABLE_DE,
            Ethiopic::SyllableDo => SYLLABLE_DO,
            Ethiopic::SyllableDwa => SYLLABLE_DWA,
            Ethiopic::SyllableDda => SYLLABLE_DDA,
            Ethiopic::SyllableDdu => SYLLABLE_DDU,
            Ethiopic::SyllableDdi => SYLLABLE_DDI,
            Ethiopic::SyllableDdaa => SYLLABLE_DDAA,
            Ethiopic::SyllableDdee => SYLLABLE_DDEE,
            Ethiopic::SyllableDde => SYLLABLE_DDE,
            Ethiopic::SyllableDdo => SYLLABLE_DDO,
            Ethiopic::SyllableDdwa => SYLLABLE_DDWA,
            Ethiopic::SyllableJa => SYLLABLE_JA,
            Ethiopic::SyllableJu => SYLLABLE_JU,
            Ethiopic::SyllableJi => SYLLABLE_JI,
            Ethiopic::SyllableJaa => SYLLABLE_JAA,
            Ethiopic::SyllableJee => SYLLABLE_JEE,
            Ethiopic::SyllableJe => SYLLABLE_JE,
            Ethiopic::SyllableJo => SYLLABLE_JO,
            Ethiopic::SyllableJwa => SYLLABLE_JWA,
            Ethiopic::SyllableGa => SYLLABLE_GA,
            Ethiopic::SyllableGu => SYLLABLE_GU,
            Ethiopic::SyllableGi => SYLLABLE_GI,
            Ethiopic::SyllableGaa => SYLLABLE_GAA,
            Ethiopic::SyllableGee => SYLLABLE_GEE,
            Ethiopic::SyllableGe => SYLLABLE_GE,
            Ethiopic::SyllableGo => SYLLABLE_GO,
            Ethiopic::SyllableGoa => SYLLABLE_GOA,
            Ethiopic::SyllableGwa => SYLLABLE_GWA,
            Ethiopic::SyllableGwi => SYLLABLE_GWI,
            Ethiopic::SyllableGwaa => SYLLABLE_GWAA,
            Ethiopic::SyllableGwee => SYLLABLE_GWEE,
            Ethiopic::SyllableGwe => SYLLABLE_GWE,
            Ethiopic::SyllableGga => SYLLABLE_GGA,
            Ethiopic::SyllableGgu => SYLLABLE_GGU,
            Ethiopic::SyllableGgi => SYLLABLE_GGI,
            Ethiopic::SyllableGgaa => SYLLABLE_GGAA,
            Ethiopic::SyllableGgee => SYLLABLE_GGEE,
            Ethiopic::SyllableGge => SYLLABLE_GGE,
            Ethiopic::SyllableGgo => SYLLABLE_GGO,
            Ethiopic::SyllableGgwaa => SYLLABLE_GGWAA,
            Ethiopic::SyllableTha => SYLLABLE_THA,
            Ethiopic::SyllableThu => SYLLABLE_THU,
            Ethiopic::SyllableThi => SYLLABLE_THI,
            Ethiopic::SyllableThaa => SYLLABLE_THAA,
            Ethiopic::SyllableThee => SYLLABLE_THEE,
            Ethiopic::SyllableThe => SYLLABLE_THE,
            Ethiopic::SyllableTho => SYLLABLE_THO,
            Ethiopic::SyllableThwa => SYLLABLE_THWA,
            Ethiopic::SyllableCha => SYLLABLE_CHA,
            Ethiopic::SyllableChu => SYLLABLE_CHU,
            Ethiopic::SyllableChi => SYLLABLE_CHI,
            Ethiopic::SyllableChaa => SYLLABLE_CHAA,
            Ethiopic::SyllableChee => SYLLABLE_CHEE,
            Ethiopic::SyllableChe => SYLLABLE_CHE,
            Ethiopic::SyllableCho => SYLLABLE_CHO,
            Ethiopic::SyllableChwa => SYLLABLE_CHWA,
            Ethiopic::SyllablePha => SYLLABLE_PHA,
            Ethiopic::SyllablePhu => SYLLABLE_PHU,
            Ethiopic::SyllablePhi => SYLLABLE_PHI,
            Ethiopic::SyllablePhaa => SYLLABLE_PHAA,
            Ethiopic::SyllablePhee => SYLLABLE_PHEE,
            Ethiopic::SyllablePhe => SYLLABLE_PHE,
            Ethiopic::SyllablePho => SYLLABLE_PHO,
            Ethiopic::SyllablePhwa => SYLLABLE_PHWA,
            Ethiopic::SyllableTsa => SYLLABLE_TSA,
            Ethiopic::SyllableTsu => SYLLABLE_TSU,
            Ethiopic::SyllableTsi => SYLLABLE_TSI,
            Ethiopic::SyllableTsaa => SYLLABLE_TSAA,
            Ethiopic::SyllableTsee => SYLLABLE_TSEE,
            Ethiopic::SyllableTse => SYLLABLE_TSE,
            Ethiopic::SyllableTso => SYLLABLE_TSO,
            Ethiopic::SyllableTswa => SYLLABLE_TSWA,
            Ethiopic::SyllableTza => SYLLABLE_TZA,
            Ethiopic::SyllableTzu => SYLLABLE_TZU,
            Ethiopic::SyllableTzi => SYLLABLE_TZI,
            Ethiopic::SyllableTzaa => SYLLABLE_TZAA,
            Ethiopic::SyllableTzee => SYLLABLE_TZEE,
            Ethiopic::SyllableTze => SYLLABLE_TZE,
            Ethiopic::SyllableTzo => SYLLABLE_TZO,
            Ethiopic::SyllableTzoa => SYLLABLE_TZOA,
            Ethiopic::SyllableFa => SYLLABLE_FA,
            Ethiopic::SyllableFu => SYLLABLE_FU,
            Ethiopic::SyllableFi => SYLLABLE_FI,
            Ethiopic::SyllableFaa => SYLLABLE_FAA,
            Ethiopic::SyllableFee => SYLLABLE_FEE,
            Ethiopic::SyllableFe => SYLLABLE_FE,
            Ethiopic::SyllableFo => SYLLABLE_FO,
            Ethiopic::SyllableFwa => SYLLABLE_FWA,
            Ethiopic::SyllablePa => SYLLABLE_PA,
            Ethiopic::SyllablePu => SYLLABLE_PU,
            Ethiopic::SyllablePi => SYLLABLE_PI,
            Ethiopic::SyllablePaa => SYLLABLE_PAA,
            Ethiopic::SyllablePee => SYLLABLE_PEE,
            Ethiopic::SyllablePe => SYLLABLE_PE,
            Ethiopic::SyllablePo => SYLLABLE_PO,
            Ethiopic::SyllablePwa => SYLLABLE_PWA,
            Ethiopic::SyllableRya => SYLLABLE_RYA,
            Ethiopic::SyllableMya => SYLLABLE_MYA,
            Ethiopic::SyllableFya => SYLLABLE_FYA,
            Ethiopic::CombiningGeminationAndVowelLengthMark => COMBINING_GEMINATION_AND_VOWEL_LENGTH_MARK,
            Ethiopic::CombiningVowelLengthMark => COMBINING_VOWEL_LENGTH_MARK,
            Ethiopic::CombiningGeminationMark => COMBINING_GEMINATION_MARK,
            Ethiopic::SectionMark => SECTION_MARK,
            Ethiopic::Wordspace => WORDSPACE,
            Ethiopic::FullStop => FULL_STOP,
            Ethiopic::Comma => COMMA,
            Ethiopic::Semicolon => SEMICOLON,
            Ethiopic::Colon => COLON,
            Ethiopic::PrefaceColon => PREFACE_COLON,
            Ethiopic::QuestionMark => QUESTION_MARK,
            Ethiopic::ParagraphSeparator => PARAGRAPH_SEPARATOR,
            Ethiopic::DigitOne => DIGIT_ONE,
            Ethiopic::DigitTwo => DIGIT_TWO,
            Ethiopic::DigitThree => DIGIT_THREE,
            Ethiopic::DigitFour => DIGIT_FOUR,
            Ethiopic::DigitFive => DIGIT_FIVE,
            Ethiopic::DigitSix => DIGIT_SIX,
            Ethiopic::DigitSeven => DIGIT_SEVEN,
            Ethiopic::DigitEight => DIGIT_EIGHT,
            Ethiopic::DigitNine => DIGIT_NINE,
            Ethiopic::NumberTen => NUMBER_TEN,
            Ethiopic::NumberTwenty => NUMBER_TWENTY,
            Ethiopic::NumberThirty => NUMBER_THIRTY,
            Ethiopic::NumberForty => NUMBER_FORTY,
            Ethiopic::NumberFifty => NUMBER_FIFTY,
            Ethiopic::NumberSixty => NUMBER_SIXTY,
            Ethiopic::NumberSeventy => NUMBER_SEVENTY,
            Ethiopic::NumberEighty => NUMBER_EIGHTY,
            Ethiopic::NumberNinety => NUMBER_NINETY,
            Ethiopic::NumberHundred => NUMBER_HUNDRED,
            Ethiopic::NumberTenThousand => NUMBER_TEN_THOUSAND,
        }
    }
}

impl std::convert::TryFrom<char> for Ethiopic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SYLLABLE_HA => Ok(Ethiopic::SyllableHa),
            SYLLABLE_HU => Ok(Ethiopic::SyllableHu),
            SYLLABLE_HI => Ok(Ethiopic::SyllableHi),
            SYLLABLE_HAA => Ok(Ethiopic::SyllableHaa),
            SYLLABLE_HEE => Ok(Ethiopic::SyllableHee),
            SYLLABLE_HE => Ok(Ethiopic::SyllableHe),
            SYLLABLE_HO => Ok(Ethiopic::SyllableHo),
            SYLLABLE_HOA => Ok(Ethiopic::SyllableHoa),
            SYLLABLE_LA => Ok(Ethiopic::SyllableLa),
            SYLLABLE_LU => Ok(Ethiopic::SyllableLu),
            SYLLABLE_LI => Ok(Ethiopic::SyllableLi),
            SYLLABLE_LAA => Ok(Ethiopic::SyllableLaa),
            SYLLABLE_LEE => Ok(Ethiopic::SyllableLee),
            SYLLABLE_LE => Ok(Ethiopic::SyllableLe),
            SYLLABLE_LO => Ok(Ethiopic::SyllableLo),
            SYLLABLE_LWA => Ok(Ethiopic::SyllableLwa),
            SYLLABLE_HHA => Ok(Ethiopic::SyllableHha),
            SYLLABLE_HHU => Ok(Ethiopic::SyllableHhu),
            SYLLABLE_HHI => Ok(Ethiopic::SyllableHhi),
            SYLLABLE_HHAA => Ok(Ethiopic::SyllableHhaa),
            SYLLABLE_HHEE => Ok(Ethiopic::SyllableHhee),
            SYLLABLE_HHE => Ok(Ethiopic::SyllableHhe),
            SYLLABLE_HHO => Ok(Ethiopic::SyllableHho),
            SYLLABLE_HHWA => Ok(Ethiopic::SyllableHhwa),
            SYLLABLE_MA => Ok(Ethiopic::SyllableMa),
            SYLLABLE_MU => Ok(Ethiopic::SyllableMu),
            SYLLABLE_MI => Ok(Ethiopic::SyllableMi),
            SYLLABLE_MAA => Ok(Ethiopic::SyllableMaa),
            SYLLABLE_MEE => Ok(Ethiopic::SyllableMee),
            SYLLABLE_ME => Ok(Ethiopic::SyllableMe),
            SYLLABLE_MO => Ok(Ethiopic::SyllableMo),
            SYLLABLE_MWA => Ok(Ethiopic::SyllableMwa),
            SYLLABLE_SZA => Ok(Ethiopic::SyllableSza),
            SYLLABLE_SZU => Ok(Ethiopic::SyllableSzu),
            SYLLABLE_SZI => Ok(Ethiopic::SyllableSzi),
            SYLLABLE_SZAA => Ok(Ethiopic::SyllableSzaa),
            SYLLABLE_SZEE => Ok(Ethiopic::SyllableSzee),
            SYLLABLE_SZE => Ok(Ethiopic::SyllableSze),
            SYLLABLE_SZO => Ok(Ethiopic::SyllableSzo),
            SYLLABLE_SZWA => Ok(Ethiopic::SyllableSzwa),
            SYLLABLE_RA => Ok(Ethiopic::SyllableRa),
            SYLLABLE_RU => Ok(Ethiopic::SyllableRu),
            SYLLABLE_RI => Ok(Ethiopic::SyllableRi),
            SYLLABLE_RAA => Ok(Ethiopic::SyllableRaa),
            SYLLABLE_REE => Ok(Ethiopic::SyllableRee),
            SYLLABLE_RE => Ok(Ethiopic::SyllableRe),
            SYLLABLE_RO => Ok(Ethiopic::SyllableRo),
            SYLLABLE_RWA => Ok(Ethiopic::SyllableRwa),
            SYLLABLE_SA => Ok(Ethiopic::SyllableSa),
            SYLLABLE_SU => Ok(Ethiopic::SyllableSu),
            SYLLABLE_SI => Ok(Ethiopic::SyllableSi),
            SYLLABLE_SAA => Ok(Ethiopic::SyllableSaa),
            SYLLABLE_SEE => Ok(Ethiopic::SyllableSee),
            SYLLABLE_SE => Ok(Ethiopic::SyllableSe),
            SYLLABLE_SO => Ok(Ethiopic::SyllableSo),
            SYLLABLE_SWA => Ok(Ethiopic::SyllableSwa),
            SYLLABLE_SHA => Ok(Ethiopic::SyllableSha),
            SYLLABLE_SHU => Ok(Ethiopic::SyllableShu),
            SYLLABLE_SHI => Ok(Ethiopic::SyllableShi),
            SYLLABLE_SHAA => Ok(Ethiopic::SyllableShaa),
            SYLLABLE_SHEE => Ok(Ethiopic::SyllableShee),
            SYLLABLE_SHE => Ok(Ethiopic::SyllableShe),
            SYLLABLE_SHO => Ok(Ethiopic::SyllableSho),
            SYLLABLE_SHWA => Ok(Ethiopic::SyllableShwa),
            SYLLABLE_QA => Ok(Ethiopic::SyllableQa),
            SYLLABLE_QU => Ok(Ethiopic::SyllableQu),
            SYLLABLE_QI => Ok(Ethiopic::SyllableQi),
            SYLLABLE_QAA => Ok(Ethiopic::SyllableQaa),
            SYLLABLE_QEE => Ok(Ethiopic::SyllableQee),
            SYLLABLE_QE => Ok(Ethiopic::SyllableQe),
            SYLLABLE_QO => Ok(Ethiopic::SyllableQo),
            SYLLABLE_QOA => Ok(Ethiopic::SyllableQoa),
            SYLLABLE_QWA => Ok(Ethiopic::SyllableQwa),
            SYLLABLE_QWI => Ok(Ethiopic::SyllableQwi),
            SYLLABLE_QWAA => Ok(Ethiopic::SyllableQwaa),
            SYLLABLE_QWEE => Ok(Ethiopic::SyllableQwee),
            SYLLABLE_QWE => Ok(Ethiopic::SyllableQwe),
            SYLLABLE_QHA => Ok(Ethiopic::SyllableQha),
            SYLLABLE_QHU => Ok(Ethiopic::SyllableQhu),
            SYLLABLE_QHI => Ok(Ethiopic::SyllableQhi),
            SYLLABLE_QHAA => Ok(Ethiopic::SyllableQhaa),
            SYLLABLE_QHEE => Ok(Ethiopic::SyllableQhee),
            SYLLABLE_QHE => Ok(Ethiopic::SyllableQhe),
            SYLLABLE_QHO => Ok(Ethiopic::SyllableQho),
            SYLLABLE_QHWA => Ok(Ethiopic::SyllableQhwa),
            SYLLABLE_QHWI => Ok(Ethiopic::SyllableQhwi),
            SYLLABLE_QHWAA => Ok(Ethiopic::SyllableQhwaa),
            SYLLABLE_QHWEE => Ok(Ethiopic::SyllableQhwee),
            SYLLABLE_QHWE => Ok(Ethiopic::SyllableQhwe),
            SYLLABLE_BA => Ok(Ethiopic::SyllableBa),
            SYLLABLE_BU => Ok(Ethiopic::SyllableBu),
            SYLLABLE_BI => Ok(Ethiopic::SyllableBi),
            SYLLABLE_BAA => Ok(Ethiopic::SyllableBaa),
            SYLLABLE_BEE => Ok(Ethiopic::SyllableBee),
            SYLLABLE_BE => Ok(Ethiopic::SyllableBe),
            SYLLABLE_BO => Ok(Ethiopic::SyllableBo),
            SYLLABLE_BWA => Ok(Ethiopic::SyllableBwa),
            SYLLABLE_VA => Ok(Ethiopic::SyllableVa),
            SYLLABLE_VU => Ok(Ethiopic::SyllableVu),
            SYLLABLE_VI => Ok(Ethiopic::SyllableVi),
            SYLLABLE_VAA => Ok(Ethiopic::SyllableVaa),
            SYLLABLE_VEE => Ok(Ethiopic::SyllableVee),
            SYLLABLE_VE => Ok(Ethiopic::SyllableVe),
            SYLLABLE_VO => Ok(Ethiopic::SyllableVo),
            SYLLABLE_VWA => Ok(Ethiopic::SyllableVwa),
            SYLLABLE_TA => Ok(Ethiopic::SyllableTa),
            SYLLABLE_TU => Ok(Ethiopic::SyllableTu),
            SYLLABLE_TI => Ok(Ethiopic::SyllableTi),
            SYLLABLE_TAA => Ok(Ethiopic::SyllableTaa),
            SYLLABLE_TEE => Ok(Ethiopic::SyllableTee),
            SYLLABLE_TE => Ok(Ethiopic::SyllableTe),
            SYLLABLE_TO => Ok(Ethiopic::SyllableTo),
            SYLLABLE_TWA => Ok(Ethiopic::SyllableTwa),
            SYLLABLE_CA => Ok(Ethiopic::SyllableCa),
            SYLLABLE_CU => Ok(Ethiopic::SyllableCu),
            SYLLABLE_CI => Ok(Ethiopic::SyllableCi),
            SYLLABLE_CAA => Ok(Ethiopic::SyllableCaa),
            SYLLABLE_CEE => Ok(Ethiopic::SyllableCee),
            SYLLABLE_CE => Ok(Ethiopic::SyllableCe),
            SYLLABLE_CO => Ok(Ethiopic::SyllableCo),
            SYLLABLE_CWA => Ok(Ethiopic::SyllableCwa),
            SYLLABLE_XA => Ok(Ethiopic::SyllableXa),
            SYLLABLE_XU => Ok(Ethiopic::SyllableXu),
            SYLLABLE_XI => Ok(Ethiopic::SyllableXi),
            SYLLABLE_XAA => Ok(Ethiopic::SyllableXaa),
            SYLLABLE_XEE => Ok(Ethiopic::SyllableXee),
            SYLLABLE_XE => Ok(Ethiopic::SyllableXe),
            SYLLABLE_XO => Ok(Ethiopic::SyllableXo),
            SYLLABLE_XOA => Ok(Ethiopic::SyllableXoa),
            SYLLABLE_XWA => Ok(Ethiopic::SyllableXwa),
            SYLLABLE_XWI => Ok(Ethiopic::SyllableXwi),
            SYLLABLE_XWAA => Ok(Ethiopic::SyllableXwaa),
            SYLLABLE_XWEE => Ok(Ethiopic::SyllableXwee),
            SYLLABLE_XWE => Ok(Ethiopic::SyllableXwe),
            SYLLABLE_NA => Ok(Ethiopic::SyllableNa),
            SYLLABLE_NU => Ok(Ethiopic::SyllableNu),
            SYLLABLE_NI => Ok(Ethiopic::SyllableNi),
            SYLLABLE_NAA => Ok(Ethiopic::SyllableNaa),
            SYLLABLE_NEE => Ok(Ethiopic::SyllableNee),
            SYLLABLE_NE => Ok(Ethiopic::SyllableNe),
            SYLLABLE_NO => Ok(Ethiopic::SyllableNo),
            SYLLABLE_NWA => Ok(Ethiopic::SyllableNwa),
            SYLLABLE_NYA => Ok(Ethiopic::SyllableNya),
            SYLLABLE_NYU => Ok(Ethiopic::SyllableNyu),
            SYLLABLE_NYI => Ok(Ethiopic::SyllableNyi),
            SYLLABLE_NYAA => Ok(Ethiopic::SyllableNyaa),
            SYLLABLE_NYEE => Ok(Ethiopic::SyllableNyee),
            SYLLABLE_NYE => Ok(Ethiopic::SyllableNye),
            SYLLABLE_NYO => Ok(Ethiopic::SyllableNyo),
            SYLLABLE_NYWA => Ok(Ethiopic::SyllableNywa),
            SYLLABLE_GLOTTAL_A => Ok(Ethiopic::SyllableGlottalA),
            SYLLABLE_GLOTTAL_U => Ok(Ethiopic::SyllableGlottalU),
            SYLLABLE_GLOTTAL_I => Ok(Ethiopic::SyllableGlottalI),
            SYLLABLE_GLOTTAL_AA => Ok(Ethiopic::SyllableGlottalAa),
            SYLLABLE_GLOTTAL_EE => Ok(Ethiopic::SyllableGlottalEe),
            SYLLABLE_GLOTTAL_E => Ok(Ethiopic::SyllableGlottalE),
            SYLLABLE_GLOTTAL_O => Ok(Ethiopic::SyllableGlottalO),
            SYLLABLE_GLOTTAL_WA => Ok(Ethiopic::SyllableGlottalWa),
            SYLLABLE_KA => Ok(Ethiopic::SyllableKa),
            SYLLABLE_KU => Ok(Ethiopic::SyllableKu),
            SYLLABLE_KI => Ok(Ethiopic::SyllableKi),
            SYLLABLE_KAA => Ok(Ethiopic::SyllableKaa),
            SYLLABLE_KEE => Ok(Ethiopic::SyllableKee),
            SYLLABLE_KE => Ok(Ethiopic::SyllableKe),
            SYLLABLE_KO => Ok(Ethiopic::SyllableKo),
            SYLLABLE_KOA => Ok(Ethiopic::SyllableKoa),
            SYLLABLE_KWA => Ok(Ethiopic::SyllableKwa),
            SYLLABLE_KWI => Ok(Ethiopic::SyllableKwi),
            SYLLABLE_KWAA => Ok(Ethiopic::SyllableKwaa),
            SYLLABLE_KWEE => Ok(Ethiopic::SyllableKwee),
            SYLLABLE_KWE => Ok(Ethiopic::SyllableKwe),
            SYLLABLE_KXA => Ok(Ethiopic::SyllableKxa),
            SYLLABLE_KXU => Ok(Ethiopic::SyllableKxu),
            SYLLABLE_KXI => Ok(Ethiopic::SyllableKxi),
            SYLLABLE_KXAA => Ok(Ethiopic::SyllableKxaa),
            SYLLABLE_KXEE => Ok(Ethiopic::SyllableKxee),
            SYLLABLE_KXE => Ok(Ethiopic::SyllableKxe),
            SYLLABLE_KXO => Ok(Ethiopic::SyllableKxo),
            SYLLABLE_KXWA => Ok(Ethiopic::SyllableKxwa),
            SYLLABLE_KXWI => Ok(Ethiopic::SyllableKxwi),
            SYLLABLE_KXWAA => Ok(Ethiopic::SyllableKxwaa),
            SYLLABLE_KXWEE => Ok(Ethiopic::SyllableKxwee),
            SYLLABLE_KXWE => Ok(Ethiopic::SyllableKxwe),
            SYLLABLE_WA => Ok(Ethiopic::SyllableWa),
            SYLLABLE_WU => Ok(Ethiopic::SyllableWu),
            SYLLABLE_WI => Ok(Ethiopic::SyllableWi),
            SYLLABLE_WAA => Ok(Ethiopic::SyllableWaa),
            SYLLABLE_WEE => Ok(Ethiopic::SyllableWee),
            SYLLABLE_WE => Ok(Ethiopic::SyllableWe),
            SYLLABLE_WO => Ok(Ethiopic::SyllableWo),
            SYLLABLE_WOA => Ok(Ethiopic::SyllableWoa),
            SYLLABLE_PHARYNGEAL_A => Ok(Ethiopic::SyllablePharyngealA),
            SYLLABLE_PHARYNGEAL_U => Ok(Ethiopic::SyllablePharyngealU),
            SYLLABLE_PHARYNGEAL_I => Ok(Ethiopic::SyllablePharyngealI),
            SYLLABLE_PHARYNGEAL_AA => Ok(Ethiopic::SyllablePharyngealAa),
            SYLLABLE_PHARYNGEAL_EE => Ok(Ethiopic::SyllablePharyngealEe),
            SYLLABLE_PHARYNGEAL_E => Ok(Ethiopic::SyllablePharyngealE),
            SYLLABLE_PHARYNGEAL_O => Ok(Ethiopic::SyllablePharyngealO),
            SYLLABLE_ZA => Ok(Ethiopic::SyllableZa),
            SYLLABLE_ZU => Ok(Ethiopic::SyllableZu),
            SYLLABLE_ZI => Ok(Ethiopic::SyllableZi),
            SYLLABLE_ZAA => Ok(Ethiopic::SyllableZaa),
            SYLLABLE_ZEE => Ok(Ethiopic::SyllableZee),
            SYLLABLE_ZE => Ok(Ethiopic::SyllableZe),
            SYLLABLE_ZO => Ok(Ethiopic::SyllableZo),
            SYLLABLE_ZWA => Ok(Ethiopic::SyllableZwa),
            SYLLABLE_ZHA => Ok(Ethiopic::SyllableZha),
            SYLLABLE_ZHU => Ok(Ethiopic::SyllableZhu),
            SYLLABLE_ZHI => Ok(Ethiopic::SyllableZhi),
            SYLLABLE_ZHAA => Ok(Ethiopic::SyllableZhaa),
            SYLLABLE_ZHEE => Ok(Ethiopic::SyllableZhee),
            SYLLABLE_ZHE => Ok(Ethiopic::SyllableZhe),
            SYLLABLE_ZHO => Ok(Ethiopic::SyllableZho),
            SYLLABLE_ZHWA => Ok(Ethiopic::SyllableZhwa),
            SYLLABLE_YA => Ok(Ethiopic::SyllableYa),
            SYLLABLE_YU => Ok(Ethiopic::SyllableYu),
            SYLLABLE_YI => Ok(Ethiopic::SyllableYi),
            SYLLABLE_YAA => Ok(Ethiopic::SyllableYaa),
            SYLLABLE_YEE => Ok(Ethiopic::SyllableYee),
            SYLLABLE_YE => Ok(Ethiopic::SyllableYe),
            SYLLABLE_YO => Ok(Ethiopic::SyllableYo),
            SYLLABLE_YOA => Ok(Ethiopic::SyllableYoa),
            SYLLABLE_DA => Ok(Ethiopic::SyllableDa),
            SYLLABLE_DU => Ok(Ethiopic::SyllableDu),
            SYLLABLE_DI => Ok(Ethiopic::SyllableDi),
            SYLLABLE_DAA => Ok(Ethiopic::SyllableDaa),
            SYLLABLE_DEE => Ok(Ethiopic::SyllableDee),
            SYLLABLE_DE => Ok(Ethiopic::SyllableDe),
            SYLLABLE_DO => Ok(Ethiopic::SyllableDo),
            SYLLABLE_DWA => Ok(Ethiopic::SyllableDwa),
            SYLLABLE_DDA => Ok(Ethiopic::SyllableDda),
            SYLLABLE_DDU => Ok(Ethiopic::SyllableDdu),
            SYLLABLE_DDI => Ok(Ethiopic::SyllableDdi),
            SYLLABLE_DDAA => Ok(Ethiopic::SyllableDdaa),
            SYLLABLE_DDEE => Ok(Ethiopic::SyllableDdee),
            SYLLABLE_DDE => Ok(Ethiopic::SyllableDde),
            SYLLABLE_DDO => Ok(Ethiopic::SyllableDdo),
            SYLLABLE_DDWA => Ok(Ethiopic::SyllableDdwa),
            SYLLABLE_JA => Ok(Ethiopic::SyllableJa),
            SYLLABLE_JU => Ok(Ethiopic::SyllableJu),
            SYLLABLE_JI => Ok(Ethiopic::SyllableJi),
            SYLLABLE_JAA => Ok(Ethiopic::SyllableJaa),
            SYLLABLE_JEE => Ok(Ethiopic::SyllableJee),
            SYLLABLE_JE => Ok(Ethiopic::SyllableJe),
            SYLLABLE_JO => Ok(Ethiopic::SyllableJo),
            SYLLABLE_JWA => Ok(Ethiopic::SyllableJwa),
            SYLLABLE_GA => Ok(Ethiopic::SyllableGa),
            SYLLABLE_GU => Ok(Ethiopic::SyllableGu),
            SYLLABLE_GI => Ok(Ethiopic::SyllableGi),
            SYLLABLE_GAA => Ok(Ethiopic::SyllableGaa),
            SYLLABLE_GEE => Ok(Ethiopic::SyllableGee),
            SYLLABLE_GE => Ok(Ethiopic::SyllableGe),
            SYLLABLE_GO => Ok(Ethiopic::SyllableGo),
            SYLLABLE_GOA => Ok(Ethiopic::SyllableGoa),
            SYLLABLE_GWA => Ok(Ethiopic::SyllableGwa),
            SYLLABLE_GWI => Ok(Ethiopic::SyllableGwi),
            SYLLABLE_GWAA => Ok(Ethiopic::SyllableGwaa),
            SYLLABLE_GWEE => Ok(Ethiopic::SyllableGwee),
            SYLLABLE_GWE => Ok(Ethiopic::SyllableGwe),
            SYLLABLE_GGA => Ok(Ethiopic::SyllableGga),
            SYLLABLE_GGU => Ok(Ethiopic::SyllableGgu),
            SYLLABLE_GGI => Ok(Ethiopic::SyllableGgi),
            SYLLABLE_GGAA => Ok(Ethiopic::SyllableGgaa),
            SYLLABLE_GGEE => Ok(Ethiopic::SyllableGgee),
            SYLLABLE_GGE => Ok(Ethiopic::SyllableGge),
            SYLLABLE_GGO => Ok(Ethiopic::SyllableGgo),
            SYLLABLE_GGWAA => Ok(Ethiopic::SyllableGgwaa),
            SYLLABLE_THA => Ok(Ethiopic::SyllableTha),
            SYLLABLE_THU => Ok(Ethiopic::SyllableThu),
            SYLLABLE_THI => Ok(Ethiopic::SyllableThi),
            SYLLABLE_THAA => Ok(Ethiopic::SyllableThaa),
            SYLLABLE_THEE => Ok(Ethiopic::SyllableThee),
            SYLLABLE_THE => Ok(Ethiopic::SyllableThe),
            SYLLABLE_THO => Ok(Ethiopic::SyllableTho),
            SYLLABLE_THWA => Ok(Ethiopic::SyllableThwa),
            SYLLABLE_CHA => Ok(Ethiopic::SyllableCha),
            SYLLABLE_CHU => Ok(Ethiopic::SyllableChu),
            SYLLABLE_CHI => Ok(Ethiopic::SyllableChi),
            SYLLABLE_CHAA => Ok(Ethiopic::SyllableChaa),
            SYLLABLE_CHEE => Ok(Ethiopic::SyllableChee),
            SYLLABLE_CHE => Ok(Ethiopic::SyllableChe),
            SYLLABLE_CHO => Ok(Ethiopic::SyllableCho),
            SYLLABLE_CHWA => Ok(Ethiopic::SyllableChwa),
            SYLLABLE_PHA => Ok(Ethiopic::SyllablePha),
            SYLLABLE_PHU => Ok(Ethiopic::SyllablePhu),
            SYLLABLE_PHI => Ok(Ethiopic::SyllablePhi),
            SYLLABLE_PHAA => Ok(Ethiopic::SyllablePhaa),
            SYLLABLE_PHEE => Ok(Ethiopic::SyllablePhee),
            SYLLABLE_PHE => Ok(Ethiopic::SyllablePhe),
            SYLLABLE_PHO => Ok(Ethiopic::SyllablePho),
            SYLLABLE_PHWA => Ok(Ethiopic::SyllablePhwa),
            SYLLABLE_TSA => Ok(Ethiopic::SyllableTsa),
            SYLLABLE_TSU => Ok(Ethiopic::SyllableTsu),
            SYLLABLE_TSI => Ok(Ethiopic::SyllableTsi),
            SYLLABLE_TSAA => Ok(Ethiopic::SyllableTsaa),
            SYLLABLE_TSEE => Ok(Ethiopic::SyllableTsee),
            SYLLABLE_TSE => Ok(Ethiopic::SyllableTse),
            SYLLABLE_TSO => Ok(Ethiopic::SyllableTso),
            SYLLABLE_TSWA => Ok(Ethiopic::SyllableTswa),
            SYLLABLE_TZA => Ok(Ethiopic::SyllableTza),
            SYLLABLE_TZU => Ok(Ethiopic::SyllableTzu),
            SYLLABLE_TZI => Ok(Ethiopic::SyllableTzi),
            SYLLABLE_TZAA => Ok(Ethiopic::SyllableTzaa),
            SYLLABLE_TZEE => Ok(Ethiopic::SyllableTzee),
            SYLLABLE_TZE => Ok(Ethiopic::SyllableTze),
            SYLLABLE_TZO => Ok(Ethiopic::SyllableTzo),
            SYLLABLE_TZOA => Ok(Ethiopic::SyllableTzoa),
            SYLLABLE_FA => Ok(Ethiopic::SyllableFa),
            SYLLABLE_FU => Ok(Ethiopic::SyllableFu),
            SYLLABLE_FI => Ok(Ethiopic::SyllableFi),
            SYLLABLE_FAA => Ok(Ethiopic::SyllableFaa),
            SYLLABLE_FEE => Ok(Ethiopic::SyllableFee),
            SYLLABLE_FE => Ok(Ethiopic::SyllableFe),
            SYLLABLE_FO => Ok(Ethiopic::SyllableFo),
            SYLLABLE_FWA => Ok(Ethiopic::SyllableFwa),
            SYLLABLE_PA => Ok(Ethiopic::SyllablePa),
            SYLLABLE_PU => Ok(Ethiopic::SyllablePu),
            SYLLABLE_PI => Ok(Ethiopic::SyllablePi),
            SYLLABLE_PAA => Ok(Ethiopic::SyllablePaa),
            SYLLABLE_PEE => Ok(Ethiopic::SyllablePee),
            SYLLABLE_PE => Ok(Ethiopic::SyllablePe),
            SYLLABLE_PO => Ok(Ethiopic::SyllablePo),
            SYLLABLE_PWA => Ok(Ethiopic::SyllablePwa),
            SYLLABLE_RYA => Ok(Ethiopic::SyllableRya),
            SYLLABLE_MYA => Ok(Ethiopic::SyllableMya),
            SYLLABLE_FYA => Ok(Ethiopic::SyllableFya),
            COMBINING_GEMINATION_AND_VOWEL_LENGTH_MARK => Ok(Ethiopic::CombiningGeminationAndVowelLengthMark),
            COMBINING_VOWEL_LENGTH_MARK => Ok(Ethiopic::CombiningVowelLengthMark),
            COMBINING_GEMINATION_MARK => Ok(Ethiopic::CombiningGeminationMark),
            SECTION_MARK => Ok(Ethiopic::SectionMark),
            WORDSPACE => Ok(Ethiopic::Wordspace),
            FULL_STOP => Ok(Ethiopic::FullStop),
            COMMA => Ok(Ethiopic::Comma),
            SEMICOLON => Ok(Ethiopic::Semicolon),
            COLON => Ok(Ethiopic::Colon),
            PREFACE_COLON => Ok(Ethiopic::PrefaceColon),
            QUESTION_MARK => Ok(Ethiopic::QuestionMark),
            PARAGRAPH_SEPARATOR => Ok(Ethiopic::ParagraphSeparator),
            DIGIT_ONE => Ok(Ethiopic::DigitOne),
            DIGIT_TWO => Ok(Ethiopic::DigitTwo),
            DIGIT_THREE => Ok(Ethiopic::DigitThree),
            DIGIT_FOUR => Ok(Ethiopic::DigitFour),
            DIGIT_FIVE => Ok(Ethiopic::DigitFive),
            DIGIT_SIX => Ok(Ethiopic::DigitSix),
            DIGIT_SEVEN => Ok(Ethiopic::DigitSeven),
            DIGIT_EIGHT => Ok(Ethiopic::DigitEight),
            DIGIT_NINE => Ok(Ethiopic::DigitNine),
            NUMBER_TEN => Ok(Ethiopic::NumberTen),
            NUMBER_TWENTY => Ok(Ethiopic::NumberTwenty),
            NUMBER_THIRTY => Ok(Ethiopic::NumberThirty),
            NUMBER_FORTY => Ok(Ethiopic::NumberForty),
            NUMBER_FIFTY => Ok(Ethiopic::NumberFifty),
            NUMBER_SIXTY => Ok(Ethiopic::NumberSixty),
            NUMBER_SEVENTY => Ok(Ethiopic::NumberSeventy),
            NUMBER_EIGHTY => Ok(Ethiopic::NumberEighty),
            NUMBER_NINETY => Ok(Ethiopic::NumberNinety),
            NUMBER_HUNDRED => Ok(Ethiopic::NumberHundred),
            NUMBER_TEN_THOUSAND => Ok(Ethiopic::NumberTenThousand),
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
