
/// An enum to represent all characters in the UnifiedCanadianAboriginalSyllabics block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UnifiedCanadianAboriginalSyllabics {
    /// \u{1400}: '᐀'
    CanadianSyllabicsHyphen,
    /// \u{1401}: 'ᐁ'
    CanadianSyllabicsE,
    /// \u{1402}: 'ᐂ'
    CanadianSyllabicsAai,
    /// \u{1403}: 'ᐃ'
    CanadianSyllabicsI,
    /// \u{1404}: 'ᐄ'
    CanadianSyllabicsIi,
    /// \u{1405}: 'ᐅ'
    CanadianSyllabicsO,
    /// \u{1406}: 'ᐆ'
    CanadianSyllabicsOo,
    /// \u{1407}: 'ᐇ'
    CanadianSyllabicsYDashCreeOo,
    /// \u{1408}: 'ᐈ'
    CanadianSyllabicsCarrierEe,
    /// \u{1409}: 'ᐉ'
    CanadianSyllabicsCarrierI,
    /// \u{140a}: 'ᐊ'
    CanadianSyllabicsA,
    /// \u{140b}: 'ᐋ'
    CanadianSyllabicsAa,
    /// \u{140c}: 'ᐌ'
    CanadianSyllabicsWe,
    /// \u{140d}: 'ᐍ'
    CanadianSyllabicsWestDashCreeWe,
    /// \u{140e}: 'ᐎ'
    CanadianSyllabicsWi,
    /// \u{140f}: 'ᐏ'
    CanadianSyllabicsWestDashCreeWi,
    /// \u{1410}: 'ᐐ'
    CanadianSyllabicsWii,
    /// \u{1411}: 'ᐑ'
    CanadianSyllabicsWestDashCreeWii,
    /// \u{1412}: 'ᐒ'
    CanadianSyllabicsWo,
    /// \u{1413}: 'ᐓ'
    CanadianSyllabicsWestDashCreeWo,
    /// \u{1414}: 'ᐔ'
    CanadianSyllabicsWoo,
    /// \u{1415}: 'ᐕ'
    CanadianSyllabicsWestDashCreeWoo,
    /// \u{1416}: 'ᐖ'
    CanadianSyllabicsNaskapiWoo,
    /// \u{1417}: 'ᐗ'
    CanadianSyllabicsWa,
    /// \u{1418}: 'ᐘ'
    CanadianSyllabicsWestDashCreeWa,
    /// \u{1419}: 'ᐙ'
    CanadianSyllabicsWaa,
    /// \u{141a}: 'ᐚ'
    CanadianSyllabicsWestDashCreeWaa,
    /// \u{141b}: 'ᐛ'
    CanadianSyllabicsNaskapiWaa,
    /// \u{141c}: 'ᐜ'
    CanadianSyllabicsAi,
    /// \u{141d}: 'ᐝ'
    CanadianSyllabicsYDashCreeW,
    /// \u{141e}: 'ᐞ'
    CanadianSyllabicsGlottalStop,
    /// \u{141f}: 'ᐟ'
    CanadianSyllabicsFinalAcute,
    /// \u{1420}: 'ᐠ'
    CanadianSyllabicsFinalGrave,
    /// \u{1421}: 'ᐡ'
    CanadianSyllabicsFinalBottomHalfRing,
    /// \u{1422}: 'ᐢ'
    CanadianSyllabicsFinalTopHalfRing,
    /// \u{1423}: 'ᐣ'
    CanadianSyllabicsFinalRightHalfRing,
    /// \u{1424}: 'ᐤ'
    CanadianSyllabicsFinalRing,
    /// \u{1425}: 'ᐥ'
    CanadianSyllabicsFinalDoubleAcute,
    /// \u{1426}: 'ᐦ'
    CanadianSyllabicsFinalDoubleShortVerticalStrokes,
    /// \u{1427}: 'ᐧ'
    CanadianSyllabicsFinalMiddleDot,
    /// \u{1428}: 'ᐨ'
    CanadianSyllabicsFinalShortHorizontalStroke,
    /// \u{1429}: 'ᐩ'
    CanadianSyllabicsFinalPlus,
    /// \u{142a}: 'ᐪ'
    CanadianSyllabicsFinalDownTack,
    /// \u{142b}: 'ᐫ'
    CanadianSyllabicsEn,
    /// \u{142c}: 'ᐬ'
    CanadianSyllabicsIn,
    /// \u{142d}: 'ᐭ'
    CanadianSyllabicsOn,
    /// \u{142e}: 'ᐮ'
    CanadianSyllabicsAn,
    /// \u{142f}: 'ᐯ'
    CanadianSyllabicsPe,
    /// \u{1430}: 'ᐰ'
    CanadianSyllabicsPaai,
    /// \u{1431}: 'ᐱ'
    CanadianSyllabicsPi,
    /// \u{1432}: 'ᐲ'
    CanadianSyllabicsPii,
    /// \u{1433}: 'ᐳ'
    CanadianSyllabicsPo,
    /// \u{1434}: 'ᐴ'
    CanadianSyllabicsPoo,
    /// \u{1435}: 'ᐵ'
    CanadianSyllabicsYDashCreePoo,
    /// \u{1436}: 'ᐶ'
    CanadianSyllabicsCarrierHee,
    /// \u{1437}: 'ᐷ'
    CanadianSyllabicsCarrierHi,
    /// \u{1438}: 'ᐸ'
    CanadianSyllabicsPa,
    /// \u{1439}: 'ᐹ'
    CanadianSyllabicsPaa,
    /// \u{143a}: 'ᐺ'
    CanadianSyllabicsPwe,
    /// \u{143b}: 'ᐻ'
    CanadianSyllabicsWestDashCreePwe,
    /// \u{143c}: 'ᐼ'
    CanadianSyllabicsPwi,
    /// \u{143d}: 'ᐽ'
    CanadianSyllabicsWestDashCreePwi,
    /// \u{143e}: 'ᐾ'
    CanadianSyllabicsPwii,
    /// \u{143f}: 'ᐿ'
    CanadianSyllabicsWestDashCreePwii,
    /// \u{1440}: 'ᑀ'
    CanadianSyllabicsPwo,
    /// \u{1441}: 'ᑁ'
    CanadianSyllabicsWestDashCreePwo,
    /// \u{1442}: 'ᑂ'
    CanadianSyllabicsPwoo,
    /// \u{1443}: 'ᑃ'
    CanadianSyllabicsWestDashCreePwoo,
    /// \u{1444}: 'ᑄ'
    CanadianSyllabicsPwa,
    /// \u{1445}: 'ᑅ'
    CanadianSyllabicsWestDashCreePwa,
    /// \u{1446}: 'ᑆ'
    CanadianSyllabicsPwaa,
    /// \u{1447}: 'ᑇ'
    CanadianSyllabicsWestDashCreePwaa,
    /// \u{1448}: 'ᑈ'
    CanadianSyllabicsYDashCreePwaa,
    /// \u{1449}: 'ᑉ'
    CanadianSyllabicsP,
    /// \u{144a}: 'ᑊ'
    CanadianSyllabicsWestDashCreeP,
    /// \u{144b}: 'ᑋ'
    CanadianSyllabicsCarrierH,
    /// \u{144c}: 'ᑌ'
    CanadianSyllabicsTe,
    /// \u{144d}: 'ᑍ'
    CanadianSyllabicsTaai,
    /// \u{144e}: 'ᑎ'
    CanadianSyllabicsTi,
    /// \u{144f}: 'ᑏ'
    CanadianSyllabicsTii,
    /// \u{1450}: 'ᑐ'
    CanadianSyllabicsTo,
    /// \u{1451}: 'ᑑ'
    CanadianSyllabicsToo,
    /// \u{1452}: 'ᑒ'
    CanadianSyllabicsYDashCreeToo,
    /// \u{1453}: 'ᑓ'
    CanadianSyllabicsCarrierDee,
    /// \u{1454}: 'ᑔ'
    CanadianSyllabicsCarrierDi,
    /// \u{1455}: 'ᑕ'
    CanadianSyllabicsTa,
    /// \u{1456}: 'ᑖ'
    CanadianSyllabicsTaa,
    /// \u{1457}: 'ᑗ'
    CanadianSyllabicsTwe,
    /// \u{1458}: 'ᑘ'
    CanadianSyllabicsWestDashCreeTwe,
    /// \u{1459}: 'ᑙ'
    CanadianSyllabicsTwi,
    /// \u{145a}: 'ᑚ'
    CanadianSyllabicsWestDashCreeTwi,
    /// \u{145b}: 'ᑛ'
    CanadianSyllabicsTwii,
    /// \u{145c}: 'ᑜ'
    CanadianSyllabicsWestDashCreeTwii,
    /// \u{145d}: 'ᑝ'
    CanadianSyllabicsTwo,
    /// \u{145e}: 'ᑞ'
    CanadianSyllabicsWestDashCreeTwo,
    /// \u{145f}: 'ᑟ'
    CanadianSyllabicsTwoo,
    /// \u{1460}: 'ᑠ'
    CanadianSyllabicsWestDashCreeTwoo,
    /// \u{1461}: 'ᑡ'
    CanadianSyllabicsTwa,
    /// \u{1462}: 'ᑢ'
    CanadianSyllabicsWestDashCreeTwa,
    /// \u{1463}: 'ᑣ'
    CanadianSyllabicsTwaa,
    /// \u{1464}: 'ᑤ'
    CanadianSyllabicsWestDashCreeTwaa,
    /// \u{1465}: 'ᑥ'
    CanadianSyllabicsNaskapiTwaa,
    /// \u{1466}: 'ᑦ'
    CanadianSyllabicsT,
    /// \u{1467}: 'ᑧ'
    CanadianSyllabicsTte,
    /// \u{1468}: 'ᑨ'
    CanadianSyllabicsTti,
    /// \u{1469}: 'ᑩ'
    CanadianSyllabicsTto,
    /// \u{146a}: 'ᑪ'
    CanadianSyllabicsTta,
    /// \u{146b}: 'ᑫ'
    CanadianSyllabicsKe,
    /// \u{146c}: 'ᑬ'
    CanadianSyllabicsKaai,
    /// \u{146d}: 'ᑭ'
    CanadianSyllabicsKi,
    /// \u{146e}: 'ᑮ'
    CanadianSyllabicsKii,
    /// \u{146f}: 'ᑯ'
    CanadianSyllabicsKo,
    /// \u{1470}: 'ᑰ'
    CanadianSyllabicsKoo,
    /// \u{1471}: 'ᑱ'
    CanadianSyllabicsYDashCreeKoo,
    /// \u{1472}: 'ᑲ'
    CanadianSyllabicsKa,
    /// \u{1473}: 'ᑳ'
    CanadianSyllabicsKaa,
    /// \u{1474}: 'ᑴ'
    CanadianSyllabicsKwe,
    /// \u{1475}: 'ᑵ'
    CanadianSyllabicsWestDashCreeKwe,
    /// \u{1476}: 'ᑶ'
    CanadianSyllabicsKwi,
    /// \u{1477}: 'ᑷ'
    CanadianSyllabicsWestDashCreeKwi,
    /// \u{1478}: 'ᑸ'
    CanadianSyllabicsKwii,
    /// \u{1479}: 'ᑹ'
    CanadianSyllabicsWestDashCreeKwii,
    /// \u{147a}: 'ᑺ'
    CanadianSyllabicsKwo,
    /// \u{147b}: 'ᑻ'
    CanadianSyllabicsWestDashCreeKwo,
    /// \u{147c}: 'ᑼ'
    CanadianSyllabicsKwoo,
    /// \u{147d}: 'ᑽ'
    CanadianSyllabicsWestDashCreeKwoo,
    /// \u{147e}: 'ᑾ'
    CanadianSyllabicsKwa,
    /// \u{147f}: 'ᑿ'
    CanadianSyllabicsWestDashCreeKwa,
    /// \u{1480}: 'ᒀ'
    CanadianSyllabicsKwaa,
    /// \u{1481}: 'ᒁ'
    CanadianSyllabicsWestDashCreeKwaa,
    /// \u{1482}: 'ᒂ'
    CanadianSyllabicsNaskapiKwaa,
    /// \u{1483}: 'ᒃ'
    CanadianSyllabicsK,
    /// \u{1484}: 'ᒄ'
    CanadianSyllabicsKw,
    /// \u{1485}: 'ᒅ'
    CanadianSyllabicsSouthDashSlaveyKeh,
    /// \u{1486}: 'ᒆ'
    CanadianSyllabicsSouthDashSlaveyKih,
    /// \u{1487}: 'ᒇ'
    CanadianSyllabicsSouthDashSlaveyKoh,
    /// \u{1488}: 'ᒈ'
    CanadianSyllabicsSouthDashSlaveyKah,
    /// \u{1489}: 'ᒉ'
    CanadianSyllabicsCe,
    /// \u{148a}: 'ᒊ'
    CanadianSyllabicsCaai,
    /// \u{148b}: 'ᒋ'
    CanadianSyllabicsCi,
    /// \u{148c}: 'ᒌ'
    CanadianSyllabicsCii,
    /// \u{148d}: 'ᒍ'
    CanadianSyllabicsCo,
    /// \u{148e}: 'ᒎ'
    CanadianSyllabicsCoo,
    /// \u{148f}: 'ᒏ'
    CanadianSyllabicsYDashCreeCoo,
    /// \u{1490}: 'ᒐ'
    CanadianSyllabicsCa,
    /// \u{1491}: 'ᒑ'
    CanadianSyllabicsCaa,
    /// \u{1492}: 'ᒒ'
    CanadianSyllabicsCwe,
    /// \u{1493}: 'ᒓ'
    CanadianSyllabicsWestDashCreeCwe,
    /// \u{1494}: 'ᒔ'
    CanadianSyllabicsCwi,
    /// \u{1495}: 'ᒕ'
    CanadianSyllabicsWestDashCreeCwi,
    /// \u{1496}: 'ᒖ'
    CanadianSyllabicsCwii,
    /// \u{1497}: 'ᒗ'
    CanadianSyllabicsWestDashCreeCwii,
    /// \u{1498}: 'ᒘ'
    CanadianSyllabicsCwo,
    /// \u{1499}: 'ᒙ'
    CanadianSyllabicsWestDashCreeCwo,
    /// \u{149a}: 'ᒚ'
    CanadianSyllabicsCwoo,
    /// \u{149b}: 'ᒛ'
    CanadianSyllabicsWestDashCreeCwoo,
    /// \u{149c}: 'ᒜ'
    CanadianSyllabicsCwa,
    /// \u{149d}: 'ᒝ'
    CanadianSyllabicsWestDashCreeCwa,
    /// \u{149e}: 'ᒞ'
    CanadianSyllabicsCwaa,
    /// \u{149f}: 'ᒟ'
    CanadianSyllabicsWestDashCreeCwaa,
    /// \u{14a0}: 'ᒠ'
    CanadianSyllabicsNaskapiCwaa,
    /// \u{14a1}: 'ᒡ'
    CanadianSyllabicsC,
    /// \u{14a2}: 'ᒢ'
    CanadianSyllabicsSayisiTh,
    /// \u{14a3}: 'ᒣ'
    CanadianSyllabicsMe,
    /// \u{14a4}: 'ᒤ'
    CanadianSyllabicsMaai,
    /// \u{14a5}: 'ᒥ'
    CanadianSyllabicsMi,
    /// \u{14a6}: 'ᒦ'
    CanadianSyllabicsMii,
    /// \u{14a7}: 'ᒧ'
    CanadianSyllabicsMo,
    /// \u{14a8}: 'ᒨ'
    CanadianSyllabicsMoo,
    /// \u{14a9}: 'ᒩ'
    CanadianSyllabicsYDashCreeMoo,
    /// \u{14aa}: 'ᒪ'
    CanadianSyllabicsMa,
    /// \u{14ab}: 'ᒫ'
    CanadianSyllabicsMaa,
    /// \u{14ac}: 'ᒬ'
    CanadianSyllabicsMwe,
    /// \u{14ad}: 'ᒭ'
    CanadianSyllabicsWestDashCreeMwe,
    /// \u{14ae}: 'ᒮ'
    CanadianSyllabicsMwi,
    /// \u{14af}: 'ᒯ'
    CanadianSyllabicsWestDashCreeMwi,
    /// \u{14b0}: 'ᒰ'
    CanadianSyllabicsMwii,
    /// \u{14b1}: 'ᒱ'
    CanadianSyllabicsWestDashCreeMwii,
    /// \u{14b2}: 'ᒲ'
    CanadianSyllabicsMwo,
    /// \u{14b3}: 'ᒳ'
    CanadianSyllabicsWestDashCreeMwo,
    /// \u{14b4}: 'ᒴ'
    CanadianSyllabicsMwoo,
    /// \u{14b5}: 'ᒵ'
    CanadianSyllabicsWestDashCreeMwoo,
    /// \u{14b6}: 'ᒶ'
    CanadianSyllabicsMwa,
    /// \u{14b7}: 'ᒷ'
    CanadianSyllabicsWestDashCreeMwa,
    /// \u{14b8}: 'ᒸ'
    CanadianSyllabicsMwaa,
    /// \u{14b9}: 'ᒹ'
    CanadianSyllabicsWestDashCreeMwaa,
    /// \u{14ba}: 'ᒺ'
    CanadianSyllabicsNaskapiMwaa,
    /// \u{14bb}: 'ᒻ'
    CanadianSyllabicsM,
    /// \u{14bc}: 'ᒼ'
    CanadianSyllabicsWestDashCreeM,
    /// \u{14bd}: 'ᒽ'
    CanadianSyllabicsMh,
    /// \u{14be}: 'ᒾ'
    CanadianSyllabicsAthapascanM,
    /// \u{14bf}: 'ᒿ'
    CanadianSyllabicsSayisiM,
    /// \u{14c0}: 'ᓀ'
    CanadianSyllabicsNe,
    /// \u{14c1}: 'ᓁ'
    CanadianSyllabicsNaai,
    /// \u{14c2}: 'ᓂ'
    CanadianSyllabicsNi,
    /// \u{14c3}: 'ᓃ'
    CanadianSyllabicsNii,
    /// \u{14c4}: 'ᓄ'
    CanadianSyllabicsNo,
    /// \u{14c5}: 'ᓅ'
    CanadianSyllabicsNoo,
    /// \u{14c6}: 'ᓆ'
    CanadianSyllabicsYDashCreeNoo,
    /// \u{14c7}: 'ᓇ'
    CanadianSyllabicsNa,
    /// \u{14c8}: 'ᓈ'
    CanadianSyllabicsNaa,
    /// \u{14c9}: 'ᓉ'
    CanadianSyllabicsNwe,
    /// \u{14ca}: 'ᓊ'
    CanadianSyllabicsWestDashCreeNwe,
    /// \u{14cb}: 'ᓋ'
    CanadianSyllabicsNwa,
    /// \u{14cc}: 'ᓌ'
    CanadianSyllabicsWestDashCreeNwa,
    /// \u{14cd}: 'ᓍ'
    CanadianSyllabicsNwaa,
    /// \u{14ce}: 'ᓎ'
    CanadianSyllabicsWestDashCreeNwaa,
    /// \u{14cf}: 'ᓏ'
    CanadianSyllabicsNaskapiNwaa,
    /// \u{14d0}: 'ᓐ'
    CanadianSyllabicsN,
    /// \u{14d1}: 'ᓑ'
    CanadianSyllabicsCarrierNg,
    /// \u{14d2}: 'ᓒ'
    CanadianSyllabicsNh,
    /// \u{14d3}: 'ᓓ'
    CanadianSyllabicsLe,
    /// \u{14d4}: 'ᓔ'
    CanadianSyllabicsLaai,
    /// \u{14d5}: 'ᓕ'
    CanadianSyllabicsLi,
    /// \u{14d6}: 'ᓖ'
    CanadianSyllabicsLii,
    /// \u{14d7}: 'ᓗ'
    CanadianSyllabicsLo,
    /// \u{14d8}: 'ᓘ'
    CanadianSyllabicsLoo,
    /// \u{14d9}: 'ᓙ'
    CanadianSyllabicsYDashCreeLoo,
    /// \u{14da}: 'ᓚ'
    CanadianSyllabicsLa,
    /// \u{14db}: 'ᓛ'
    CanadianSyllabicsLaa,
    /// \u{14dc}: 'ᓜ'
    CanadianSyllabicsLwe,
    /// \u{14dd}: 'ᓝ'
    CanadianSyllabicsWestDashCreeLwe,
    /// \u{14de}: 'ᓞ'
    CanadianSyllabicsLwi,
    /// \u{14df}: 'ᓟ'
    CanadianSyllabicsWestDashCreeLwi,
    /// \u{14e0}: 'ᓠ'
    CanadianSyllabicsLwii,
    /// \u{14e1}: 'ᓡ'
    CanadianSyllabicsWestDashCreeLwii,
    /// \u{14e2}: 'ᓢ'
    CanadianSyllabicsLwo,
    /// \u{14e3}: 'ᓣ'
    CanadianSyllabicsWestDashCreeLwo,
    /// \u{14e4}: 'ᓤ'
    CanadianSyllabicsLwoo,
    /// \u{14e5}: 'ᓥ'
    CanadianSyllabicsWestDashCreeLwoo,
    /// \u{14e6}: 'ᓦ'
    CanadianSyllabicsLwa,
    /// \u{14e7}: 'ᓧ'
    CanadianSyllabicsWestDashCreeLwa,
    /// \u{14e8}: 'ᓨ'
    CanadianSyllabicsLwaa,
    /// \u{14e9}: 'ᓩ'
    CanadianSyllabicsWestDashCreeLwaa,
    /// \u{14ea}: 'ᓪ'
    CanadianSyllabicsL,
    /// \u{14eb}: 'ᓫ'
    CanadianSyllabicsWestDashCreeL,
    /// \u{14ec}: 'ᓬ'
    CanadianSyllabicsMedialL,
    /// \u{14ed}: 'ᓭ'
    CanadianSyllabicsSe,
    /// \u{14ee}: 'ᓮ'
    CanadianSyllabicsSaai,
    /// \u{14ef}: 'ᓯ'
    CanadianSyllabicsSi,
    /// \u{14f0}: 'ᓰ'
    CanadianSyllabicsSii,
    /// \u{14f1}: 'ᓱ'
    CanadianSyllabicsSo,
    /// \u{14f2}: 'ᓲ'
    CanadianSyllabicsSoo,
    /// \u{14f3}: 'ᓳ'
    CanadianSyllabicsYDashCreeSoo,
    /// \u{14f4}: 'ᓴ'
    CanadianSyllabicsSa,
    /// \u{14f5}: 'ᓵ'
    CanadianSyllabicsSaa,
    /// \u{14f6}: 'ᓶ'
    CanadianSyllabicsSwe,
    /// \u{14f7}: 'ᓷ'
    CanadianSyllabicsWestDashCreeSwe,
    /// \u{14f8}: 'ᓸ'
    CanadianSyllabicsSwi,
    /// \u{14f9}: 'ᓹ'
    CanadianSyllabicsWestDashCreeSwi,
    /// \u{14fa}: 'ᓺ'
    CanadianSyllabicsSwii,
    /// \u{14fb}: 'ᓻ'
    CanadianSyllabicsWestDashCreeSwii,
    /// \u{14fc}: 'ᓼ'
    CanadianSyllabicsSwo,
    /// \u{14fd}: 'ᓽ'
    CanadianSyllabicsWestDashCreeSwo,
    /// \u{14fe}: 'ᓾ'
    CanadianSyllabicsSwoo,
    /// \u{14ff}: 'ᓿ'
    CanadianSyllabicsWestDashCreeSwoo,
    /// \u{1500}: 'ᔀ'
    CanadianSyllabicsSwa,
    /// \u{1501}: 'ᔁ'
    CanadianSyllabicsWestDashCreeSwa,
    /// \u{1502}: 'ᔂ'
    CanadianSyllabicsSwaa,
    /// \u{1503}: 'ᔃ'
    CanadianSyllabicsWestDashCreeSwaa,
    /// \u{1504}: 'ᔄ'
    CanadianSyllabicsNaskapiSwaa,
    /// \u{1505}: 'ᔅ'
    CanadianSyllabicsS,
    /// \u{1506}: 'ᔆ'
    CanadianSyllabicsAthapascanS,
    /// \u{1507}: 'ᔇ'
    CanadianSyllabicsSw,
    /// \u{1508}: 'ᔈ'
    CanadianSyllabicsBlackfootS,
    /// \u{1509}: 'ᔉ'
    CanadianSyllabicsMooseDashCreeSk,
    /// \u{150a}: 'ᔊ'
    CanadianSyllabicsNaskapiSkw,
    /// \u{150b}: 'ᔋ'
    CanadianSyllabicsNaskapiSDashW,
    /// \u{150c}: 'ᔌ'
    CanadianSyllabicsNaskapiSpwa,
    /// \u{150d}: 'ᔍ'
    CanadianSyllabicsNaskapiStwa,
    /// \u{150e}: 'ᔎ'
    CanadianSyllabicsNaskapiSkwa,
    /// \u{150f}: 'ᔏ'
    CanadianSyllabicsNaskapiScwa,
    /// \u{1510}: 'ᔐ'
    CanadianSyllabicsShe,
    /// \u{1511}: 'ᔑ'
    CanadianSyllabicsShi,
    /// \u{1512}: 'ᔒ'
    CanadianSyllabicsShii,
    /// \u{1513}: 'ᔓ'
    CanadianSyllabicsSho,
    /// \u{1514}: 'ᔔ'
    CanadianSyllabicsShoo,
    /// \u{1515}: 'ᔕ'
    CanadianSyllabicsSha,
    /// \u{1516}: 'ᔖ'
    CanadianSyllabicsShaa,
    /// \u{1517}: 'ᔗ'
    CanadianSyllabicsShwe,
    /// \u{1518}: 'ᔘ'
    CanadianSyllabicsWestDashCreeShwe,
    /// \u{1519}: 'ᔙ'
    CanadianSyllabicsShwi,
    /// \u{151a}: 'ᔚ'
    CanadianSyllabicsWestDashCreeShwi,
    /// \u{151b}: 'ᔛ'
    CanadianSyllabicsShwii,
    /// \u{151c}: 'ᔜ'
    CanadianSyllabicsWestDashCreeShwii,
    /// \u{151d}: 'ᔝ'
    CanadianSyllabicsShwo,
    /// \u{151e}: 'ᔞ'
    CanadianSyllabicsWestDashCreeShwo,
    /// \u{151f}: 'ᔟ'
    CanadianSyllabicsShwoo,
    /// \u{1520}: 'ᔠ'
    CanadianSyllabicsWestDashCreeShwoo,
    /// \u{1521}: 'ᔡ'
    CanadianSyllabicsShwa,
    /// \u{1522}: 'ᔢ'
    CanadianSyllabicsWestDashCreeShwa,
    /// \u{1523}: 'ᔣ'
    CanadianSyllabicsShwaa,
    /// \u{1524}: 'ᔤ'
    CanadianSyllabicsWestDashCreeShwaa,
    /// \u{1525}: 'ᔥ'
    CanadianSyllabicsSh,
    /// \u{1526}: 'ᔦ'
    CanadianSyllabicsYe,
    /// \u{1527}: 'ᔧ'
    CanadianSyllabicsYaai,
    /// \u{1528}: 'ᔨ'
    CanadianSyllabicsYi,
    /// \u{1529}: 'ᔩ'
    CanadianSyllabicsYii,
    /// \u{152a}: 'ᔪ'
    CanadianSyllabicsYo,
    /// \u{152b}: 'ᔫ'
    CanadianSyllabicsYoo,
    /// \u{152c}: 'ᔬ'
    CanadianSyllabicsYDashCreeYoo,
    /// \u{152d}: 'ᔭ'
    CanadianSyllabicsYa,
    /// \u{152e}: 'ᔮ'
    CanadianSyllabicsYaa,
    /// \u{152f}: 'ᔯ'
    CanadianSyllabicsYwe,
    /// \u{1530}: 'ᔰ'
    CanadianSyllabicsWestDashCreeYwe,
    /// \u{1531}: 'ᔱ'
    CanadianSyllabicsYwi,
    /// \u{1532}: 'ᔲ'
    CanadianSyllabicsWestDashCreeYwi,
    /// \u{1533}: 'ᔳ'
    CanadianSyllabicsYwii,
    /// \u{1534}: 'ᔴ'
    CanadianSyllabicsWestDashCreeYwii,
    /// \u{1535}: 'ᔵ'
    CanadianSyllabicsYwo,
    /// \u{1536}: 'ᔶ'
    CanadianSyllabicsWestDashCreeYwo,
    /// \u{1537}: 'ᔷ'
    CanadianSyllabicsYwoo,
    /// \u{1538}: 'ᔸ'
    CanadianSyllabicsWestDashCreeYwoo,
    /// \u{1539}: 'ᔹ'
    CanadianSyllabicsYwa,
    /// \u{153a}: 'ᔺ'
    CanadianSyllabicsWestDashCreeYwa,
    /// \u{153b}: 'ᔻ'
    CanadianSyllabicsYwaa,
    /// \u{153c}: 'ᔼ'
    CanadianSyllabicsWestDashCreeYwaa,
    /// \u{153d}: 'ᔽ'
    CanadianSyllabicsNaskapiYwaa,
    /// \u{153e}: 'ᔾ'
    CanadianSyllabicsY,
    /// \u{153f}: 'ᔿ'
    CanadianSyllabicsBibleDashCreeY,
    /// \u{1540}: 'ᕀ'
    CanadianSyllabicsWestDashCreeY,
    /// \u{1541}: 'ᕁ'
    CanadianSyllabicsSayisiYi,
    /// \u{1542}: 'ᕂ'
    CanadianSyllabicsRe,
    /// \u{1543}: 'ᕃ'
    CanadianSyllabicsRDashCreeRe,
    /// \u{1544}: 'ᕄ'
    CanadianSyllabicsWestDashCreeLe,
    /// \u{1545}: 'ᕅ'
    CanadianSyllabicsRaai,
    /// \u{1546}: 'ᕆ'
    CanadianSyllabicsRi,
    /// \u{1547}: 'ᕇ'
    CanadianSyllabicsRii,
    /// \u{1548}: 'ᕈ'
    CanadianSyllabicsRo,
    /// \u{1549}: 'ᕉ'
    CanadianSyllabicsRoo,
    /// \u{154a}: 'ᕊ'
    CanadianSyllabicsWestDashCreeLo,
    /// \u{154b}: 'ᕋ'
    CanadianSyllabicsRa,
    /// \u{154c}: 'ᕌ'
    CanadianSyllabicsRaa,
    /// \u{154d}: 'ᕍ'
    CanadianSyllabicsWestDashCreeLa,
    /// \u{154e}: 'ᕎ'
    CanadianSyllabicsRwaa,
    /// \u{154f}: 'ᕏ'
    CanadianSyllabicsWestDashCreeRwaa,
    /// \u{1550}: 'ᕐ'
    CanadianSyllabicsR,
    /// \u{1551}: 'ᕑ'
    CanadianSyllabicsWestDashCreeR,
    /// \u{1552}: 'ᕒ'
    CanadianSyllabicsMedialR,
    /// \u{1553}: 'ᕓ'
    CanadianSyllabicsFe,
    /// \u{1554}: 'ᕔ'
    CanadianSyllabicsFaai,
    /// \u{1555}: 'ᕕ'
    CanadianSyllabicsFi,
    /// \u{1556}: 'ᕖ'
    CanadianSyllabicsFii,
    /// \u{1557}: 'ᕗ'
    CanadianSyllabicsFo,
    /// \u{1558}: 'ᕘ'
    CanadianSyllabicsFoo,
    /// \u{1559}: 'ᕙ'
    CanadianSyllabicsFa,
    /// \u{155a}: 'ᕚ'
    CanadianSyllabicsFaa,
    /// \u{155b}: 'ᕛ'
    CanadianSyllabicsFwaa,
    /// \u{155c}: 'ᕜ'
    CanadianSyllabicsWestDashCreeFwaa,
    /// \u{155d}: 'ᕝ'
    CanadianSyllabicsF,
    /// \u{155e}: 'ᕞ'
    CanadianSyllabicsThe,
    /// \u{155f}: 'ᕟ'
    CanadianSyllabicsNDashCreeThe,
    /// \u{1560}: 'ᕠ'
    CanadianSyllabicsThi,
    /// \u{1561}: 'ᕡ'
    CanadianSyllabicsNDashCreeThi,
    /// \u{1562}: 'ᕢ'
    CanadianSyllabicsThii,
    /// \u{1563}: 'ᕣ'
    CanadianSyllabicsNDashCreeThii,
    /// \u{1564}: 'ᕤ'
    CanadianSyllabicsTho,
    /// \u{1565}: 'ᕥ'
    CanadianSyllabicsThoo,
    /// \u{1566}: 'ᕦ'
    CanadianSyllabicsTha,
    /// \u{1567}: 'ᕧ'
    CanadianSyllabicsThaa,
    /// \u{1568}: 'ᕨ'
    CanadianSyllabicsThwaa,
    /// \u{1569}: 'ᕩ'
    CanadianSyllabicsWestDashCreeThwaa,
    /// \u{156a}: 'ᕪ'
    CanadianSyllabicsTh,
    /// \u{156b}: 'ᕫ'
    CanadianSyllabicsTthe,
    /// \u{156c}: 'ᕬ'
    CanadianSyllabicsTthi,
    /// \u{156d}: 'ᕭ'
    CanadianSyllabicsTtho,
    /// \u{156e}: 'ᕮ'
    CanadianSyllabicsTtha,
    /// \u{156f}: 'ᕯ'
    CanadianSyllabicsTth,
    /// \u{1570}: 'ᕰ'
    CanadianSyllabicsTye,
    /// \u{1571}: 'ᕱ'
    CanadianSyllabicsTyi,
    /// \u{1572}: 'ᕲ'
    CanadianSyllabicsTyo,
    /// \u{1573}: 'ᕳ'
    CanadianSyllabicsTya,
    /// \u{1574}: 'ᕴ'
    CanadianSyllabicsNunavikHe,
    /// \u{1575}: 'ᕵ'
    CanadianSyllabicsNunavikHi,
    /// \u{1576}: 'ᕶ'
    CanadianSyllabicsNunavikHii,
    /// \u{1577}: 'ᕷ'
    CanadianSyllabicsNunavikHo,
    /// \u{1578}: 'ᕸ'
    CanadianSyllabicsNunavikHoo,
    /// \u{1579}: 'ᕹ'
    CanadianSyllabicsNunavikHa,
    /// \u{157a}: 'ᕺ'
    CanadianSyllabicsNunavikHaa,
    /// \u{157b}: 'ᕻ'
    CanadianSyllabicsNunavikH,
    /// \u{157c}: 'ᕼ'
    CanadianSyllabicsNunavutH,
    /// \u{157d}: 'ᕽ'
    CanadianSyllabicsHk,
    /// \u{157e}: 'ᕾ'
    CanadianSyllabicsQaai,
    /// \u{157f}: 'ᕿ'
    CanadianSyllabicsQi,
    /// \u{1580}: 'ᖀ'
    CanadianSyllabicsQii,
    /// \u{1581}: 'ᖁ'
    CanadianSyllabicsQo,
    /// \u{1582}: 'ᖂ'
    CanadianSyllabicsQoo,
    /// \u{1583}: 'ᖃ'
    CanadianSyllabicsQa,
    /// \u{1584}: 'ᖄ'
    CanadianSyllabicsQaa,
    /// \u{1585}: 'ᖅ'
    CanadianSyllabicsQ,
    /// \u{1586}: 'ᖆ'
    CanadianSyllabicsTlhe,
    /// \u{1587}: 'ᖇ'
    CanadianSyllabicsTlhi,
    /// \u{1588}: 'ᖈ'
    CanadianSyllabicsTlho,
    /// \u{1589}: 'ᖉ'
    CanadianSyllabicsTlha,
    /// \u{158a}: 'ᖊ'
    CanadianSyllabicsWestDashCreeRe,
    /// \u{158b}: 'ᖋ'
    CanadianSyllabicsWestDashCreeRi,
    /// \u{158c}: 'ᖌ'
    CanadianSyllabicsWestDashCreeRo,
    /// \u{158d}: 'ᖍ'
    CanadianSyllabicsWestDashCreeRa,
    /// \u{158e}: 'ᖎ'
    CanadianSyllabicsNgaai,
    /// \u{158f}: 'ᖏ'
    CanadianSyllabicsNgi,
    /// \u{1590}: 'ᖐ'
    CanadianSyllabicsNgii,
    /// \u{1591}: 'ᖑ'
    CanadianSyllabicsNgo,
    /// \u{1592}: 'ᖒ'
    CanadianSyllabicsNgoo,
    /// \u{1593}: 'ᖓ'
    CanadianSyllabicsNga,
    /// \u{1594}: 'ᖔ'
    CanadianSyllabicsNgaa,
    /// \u{1595}: 'ᖕ'
    CanadianSyllabicsNg,
    /// \u{1596}: 'ᖖ'
    CanadianSyllabicsNng,
    /// \u{1597}: 'ᖗ'
    CanadianSyllabicsSayisiShe,
    /// \u{1598}: 'ᖘ'
    CanadianSyllabicsSayisiShi,
    /// \u{1599}: 'ᖙ'
    CanadianSyllabicsSayisiSho,
    /// \u{159a}: 'ᖚ'
    CanadianSyllabicsSayisiSha,
    /// \u{159b}: 'ᖛ'
    CanadianSyllabicsWoodsDashCreeThe,
    /// \u{159c}: 'ᖜ'
    CanadianSyllabicsWoodsDashCreeThi,
    /// \u{159d}: 'ᖝ'
    CanadianSyllabicsWoodsDashCreeTho,
    /// \u{159e}: 'ᖞ'
    CanadianSyllabicsWoodsDashCreeTha,
    /// \u{159f}: 'ᖟ'
    CanadianSyllabicsWoodsDashCreeTh,
    /// \u{15a0}: 'ᖠ'
    CanadianSyllabicsLhi,
    /// \u{15a1}: 'ᖡ'
    CanadianSyllabicsLhii,
    /// \u{15a2}: 'ᖢ'
    CanadianSyllabicsLho,
    /// \u{15a3}: 'ᖣ'
    CanadianSyllabicsLhoo,
    /// \u{15a4}: 'ᖤ'
    CanadianSyllabicsLha,
    /// \u{15a5}: 'ᖥ'
    CanadianSyllabicsLhaa,
    /// \u{15a6}: 'ᖦ'
    CanadianSyllabicsLh,
    /// \u{15a7}: 'ᖧ'
    CanadianSyllabicsThDashCreeThe,
    /// \u{15a8}: 'ᖨ'
    CanadianSyllabicsThDashCreeThi,
    /// \u{15a9}: 'ᖩ'
    CanadianSyllabicsThDashCreeThii,
    /// \u{15aa}: 'ᖪ'
    CanadianSyllabicsThDashCreeTho,
    /// \u{15ab}: 'ᖫ'
    CanadianSyllabicsThDashCreeThoo,
    /// \u{15ac}: 'ᖬ'
    CanadianSyllabicsThDashCreeTha,
    /// \u{15ad}: 'ᖭ'
    CanadianSyllabicsThDashCreeThaa,
    /// \u{15ae}: 'ᖮ'
    CanadianSyllabicsThDashCreeTh,
    /// \u{15af}: 'ᖯ'
    CanadianSyllabicsAivilikB,
    /// \u{15b0}: 'ᖰ'
    CanadianSyllabicsBlackfootE,
    /// \u{15b1}: 'ᖱ'
    CanadianSyllabicsBlackfootI,
    /// \u{15b2}: 'ᖲ'
    CanadianSyllabicsBlackfootO,
    /// \u{15b3}: 'ᖳ'
    CanadianSyllabicsBlackfootA,
    /// \u{15b4}: 'ᖴ'
    CanadianSyllabicsBlackfootWe,
    /// \u{15b5}: 'ᖵ'
    CanadianSyllabicsBlackfootWi,
    /// \u{15b6}: 'ᖶ'
    CanadianSyllabicsBlackfootWo,
    /// \u{15b7}: 'ᖷ'
    CanadianSyllabicsBlackfootWa,
    /// \u{15b8}: 'ᖸ'
    CanadianSyllabicsBlackfootNe,
    /// \u{15b9}: 'ᖹ'
    CanadianSyllabicsBlackfootNi,
    /// \u{15ba}: 'ᖺ'
    CanadianSyllabicsBlackfootNo,
    /// \u{15bb}: 'ᖻ'
    CanadianSyllabicsBlackfootNa,
    /// \u{15bc}: 'ᖼ'
    CanadianSyllabicsBlackfootKe,
    /// \u{15bd}: 'ᖽ'
    CanadianSyllabicsBlackfootKi,
    /// \u{15be}: 'ᖾ'
    CanadianSyllabicsBlackfootKo,
    /// \u{15bf}: 'ᖿ'
    CanadianSyllabicsBlackfootKa,
    /// \u{15c0}: 'ᗀ'
    CanadianSyllabicsSayisiHe,
    /// \u{15c1}: 'ᗁ'
    CanadianSyllabicsSayisiHi,
    /// \u{15c2}: 'ᗂ'
    CanadianSyllabicsSayisiHo,
    /// \u{15c3}: 'ᗃ'
    CanadianSyllabicsSayisiHa,
    /// \u{15c4}: 'ᗄ'
    CanadianSyllabicsCarrierGhu,
    /// \u{15c5}: 'ᗅ'
    CanadianSyllabicsCarrierGho,
    /// \u{15c6}: 'ᗆ'
    CanadianSyllabicsCarrierGhe,
    /// \u{15c7}: 'ᗇ'
    CanadianSyllabicsCarrierGhee,
    /// \u{15c8}: 'ᗈ'
    CanadianSyllabicsCarrierGhi,
    /// \u{15c9}: 'ᗉ'
    CanadianSyllabicsCarrierGha,
    /// \u{15ca}: 'ᗊ'
    CanadianSyllabicsCarrierRu,
    /// \u{15cb}: 'ᗋ'
    CanadianSyllabicsCarrierRo,
    /// \u{15cc}: 'ᗌ'
    CanadianSyllabicsCarrierRe,
    /// \u{15cd}: 'ᗍ'
    CanadianSyllabicsCarrierRee,
    /// \u{15ce}: 'ᗎ'
    CanadianSyllabicsCarrierRi,
    /// \u{15cf}: 'ᗏ'
    CanadianSyllabicsCarrierRa,
    /// \u{15d0}: 'ᗐ'
    CanadianSyllabicsCarrierWu,
    /// \u{15d1}: 'ᗑ'
    CanadianSyllabicsCarrierWo,
    /// \u{15d2}: 'ᗒ'
    CanadianSyllabicsCarrierWe,
    /// \u{15d3}: 'ᗓ'
    CanadianSyllabicsCarrierWee,
    /// \u{15d4}: 'ᗔ'
    CanadianSyllabicsCarrierWi,
    /// \u{15d5}: 'ᗕ'
    CanadianSyllabicsCarrierWa,
    /// \u{15d6}: 'ᗖ'
    CanadianSyllabicsCarrierHwu,
    /// \u{15d7}: 'ᗗ'
    CanadianSyllabicsCarrierHwo,
    /// \u{15d8}: 'ᗘ'
    CanadianSyllabicsCarrierHwe,
    /// \u{15d9}: 'ᗙ'
    CanadianSyllabicsCarrierHwee,
    /// \u{15da}: 'ᗚ'
    CanadianSyllabicsCarrierHwi,
    /// \u{15db}: 'ᗛ'
    CanadianSyllabicsCarrierHwa,
    /// \u{15dc}: 'ᗜ'
    CanadianSyllabicsCarrierThu,
    /// \u{15dd}: 'ᗝ'
    CanadianSyllabicsCarrierTho,
    /// \u{15de}: 'ᗞ'
    CanadianSyllabicsCarrierThe,
    /// \u{15df}: 'ᗟ'
    CanadianSyllabicsCarrierThee,
    /// \u{15e0}: 'ᗠ'
    CanadianSyllabicsCarrierThi,
    /// \u{15e1}: 'ᗡ'
    CanadianSyllabicsCarrierTha,
    /// \u{15e2}: 'ᗢ'
    CanadianSyllabicsCarrierTtu,
    /// \u{15e3}: 'ᗣ'
    CanadianSyllabicsCarrierTto,
    /// \u{15e4}: 'ᗤ'
    CanadianSyllabicsCarrierTte,
    /// \u{15e5}: 'ᗥ'
    CanadianSyllabicsCarrierTtee,
    /// \u{15e6}: 'ᗦ'
    CanadianSyllabicsCarrierTti,
    /// \u{15e7}: 'ᗧ'
    CanadianSyllabicsCarrierTta,
    /// \u{15e8}: 'ᗨ'
    CanadianSyllabicsCarrierPu,
    /// \u{15e9}: 'ᗩ'
    CanadianSyllabicsCarrierPo,
    /// \u{15ea}: 'ᗪ'
    CanadianSyllabicsCarrierPe,
    /// \u{15eb}: 'ᗫ'
    CanadianSyllabicsCarrierPee,
    /// \u{15ec}: 'ᗬ'
    CanadianSyllabicsCarrierPi,
    /// \u{15ed}: 'ᗭ'
    CanadianSyllabicsCarrierPa,
    /// \u{15ee}: 'ᗮ'
    CanadianSyllabicsCarrierP,
    /// \u{15ef}: 'ᗯ'
    CanadianSyllabicsCarrierGu,
    /// \u{15f0}: 'ᗰ'
    CanadianSyllabicsCarrierGo,
    /// \u{15f1}: 'ᗱ'
    CanadianSyllabicsCarrierGe,
    /// \u{15f2}: 'ᗲ'
    CanadianSyllabicsCarrierGee,
    /// \u{15f3}: 'ᗳ'
    CanadianSyllabicsCarrierGi,
    /// \u{15f4}: 'ᗴ'
    CanadianSyllabicsCarrierGa,
    /// \u{15f5}: 'ᗵ'
    CanadianSyllabicsCarrierKhu,
    /// \u{15f6}: 'ᗶ'
    CanadianSyllabicsCarrierKho,
    /// \u{15f7}: 'ᗷ'
    CanadianSyllabicsCarrierKhe,
    /// \u{15f8}: 'ᗸ'
    CanadianSyllabicsCarrierKhee,
    /// \u{15f9}: 'ᗹ'
    CanadianSyllabicsCarrierKhi,
    /// \u{15fa}: 'ᗺ'
    CanadianSyllabicsCarrierKha,
    /// \u{15fb}: 'ᗻ'
    CanadianSyllabicsCarrierKku,
    /// \u{15fc}: 'ᗼ'
    CanadianSyllabicsCarrierKko,
    /// \u{15fd}: 'ᗽ'
    CanadianSyllabicsCarrierKke,
    /// \u{15fe}: 'ᗾ'
    CanadianSyllabicsCarrierKkee,
    /// \u{15ff}: 'ᗿ'
    CanadianSyllabicsCarrierKki,
    /// \u{1600}: 'ᘀ'
    CanadianSyllabicsCarrierKka,
    /// \u{1601}: 'ᘁ'
    CanadianSyllabicsCarrierKk,
    /// \u{1602}: 'ᘂ'
    CanadianSyllabicsCarrierNu,
    /// \u{1603}: 'ᘃ'
    CanadianSyllabicsCarrierNo,
    /// \u{1604}: 'ᘄ'
    CanadianSyllabicsCarrierNe,
    /// \u{1605}: 'ᘅ'
    CanadianSyllabicsCarrierNee,
    /// \u{1606}: 'ᘆ'
    CanadianSyllabicsCarrierNi,
    /// \u{1607}: 'ᘇ'
    CanadianSyllabicsCarrierNa,
    /// \u{1608}: 'ᘈ'
    CanadianSyllabicsCarrierMu,
    /// \u{1609}: 'ᘉ'
    CanadianSyllabicsCarrierMo,
    /// \u{160a}: 'ᘊ'
    CanadianSyllabicsCarrierMe,
    /// \u{160b}: 'ᘋ'
    CanadianSyllabicsCarrierMee,
    /// \u{160c}: 'ᘌ'
    CanadianSyllabicsCarrierMi,
    /// \u{160d}: 'ᘍ'
    CanadianSyllabicsCarrierMa,
    /// \u{160e}: 'ᘎ'
    CanadianSyllabicsCarrierYu,
    /// \u{160f}: 'ᘏ'
    CanadianSyllabicsCarrierYo,
    /// \u{1610}: 'ᘐ'
    CanadianSyllabicsCarrierYe,
    /// \u{1611}: 'ᘑ'
    CanadianSyllabicsCarrierYee,
    /// \u{1612}: 'ᘒ'
    CanadianSyllabicsCarrierYi,
    /// \u{1613}: 'ᘓ'
    CanadianSyllabicsCarrierYa,
    /// \u{1614}: 'ᘔ'
    CanadianSyllabicsCarrierJu,
    /// \u{1615}: 'ᘕ'
    CanadianSyllabicsSayisiJu,
    /// \u{1616}: 'ᘖ'
    CanadianSyllabicsCarrierJo,
    /// \u{1617}: 'ᘗ'
    CanadianSyllabicsCarrierJe,
    /// \u{1618}: 'ᘘ'
    CanadianSyllabicsCarrierJee,
    /// \u{1619}: 'ᘙ'
    CanadianSyllabicsCarrierJi,
    /// \u{161a}: 'ᘚ'
    CanadianSyllabicsSayisiJi,
    /// \u{161b}: 'ᘛ'
    CanadianSyllabicsCarrierJa,
    /// \u{161c}: 'ᘜ'
    CanadianSyllabicsCarrierJju,
    /// \u{161d}: 'ᘝ'
    CanadianSyllabicsCarrierJjo,
    /// \u{161e}: 'ᘞ'
    CanadianSyllabicsCarrierJje,
    /// \u{161f}: 'ᘟ'
    CanadianSyllabicsCarrierJjee,
    /// \u{1620}: 'ᘠ'
    CanadianSyllabicsCarrierJji,
    /// \u{1621}: 'ᘡ'
    CanadianSyllabicsCarrierJja,
    /// \u{1622}: 'ᘢ'
    CanadianSyllabicsCarrierLu,
    /// \u{1623}: 'ᘣ'
    CanadianSyllabicsCarrierLo,
    /// \u{1624}: 'ᘤ'
    CanadianSyllabicsCarrierLe,
    /// \u{1625}: 'ᘥ'
    CanadianSyllabicsCarrierLee,
    /// \u{1626}: 'ᘦ'
    CanadianSyllabicsCarrierLi,
    /// \u{1627}: 'ᘧ'
    CanadianSyllabicsCarrierLa,
    /// \u{1628}: 'ᘨ'
    CanadianSyllabicsCarrierDlu,
    /// \u{1629}: 'ᘩ'
    CanadianSyllabicsCarrierDlo,
    /// \u{162a}: 'ᘪ'
    CanadianSyllabicsCarrierDle,
    /// \u{162b}: 'ᘫ'
    CanadianSyllabicsCarrierDlee,
    /// \u{162c}: 'ᘬ'
    CanadianSyllabicsCarrierDli,
    /// \u{162d}: 'ᘭ'
    CanadianSyllabicsCarrierDla,
    /// \u{162e}: 'ᘮ'
    CanadianSyllabicsCarrierLhu,
    /// \u{162f}: 'ᘯ'
    CanadianSyllabicsCarrierLho,
    /// \u{1630}: 'ᘰ'
    CanadianSyllabicsCarrierLhe,
    /// \u{1631}: 'ᘱ'
    CanadianSyllabicsCarrierLhee,
    /// \u{1632}: 'ᘲ'
    CanadianSyllabicsCarrierLhi,
    /// \u{1633}: 'ᘳ'
    CanadianSyllabicsCarrierLha,
    /// \u{1634}: 'ᘴ'
    CanadianSyllabicsCarrierTlhu,
    /// \u{1635}: 'ᘵ'
    CanadianSyllabicsCarrierTlho,
    /// \u{1636}: 'ᘶ'
    CanadianSyllabicsCarrierTlhe,
    /// \u{1637}: 'ᘷ'
    CanadianSyllabicsCarrierTlhee,
    /// \u{1638}: 'ᘸ'
    CanadianSyllabicsCarrierTlhi,
    /// \u{1639}: 'ᘹ'
    CanadianSyllabicsCarrierTlha,
    /// \u{163a}: 'ᘺ'
    CanadianSyllabicsCarrierTlu,
    /// \u{163b}: 'ᘻ'
    CanadianSyllabicsCarrierTlo,
    /// \u{163c}: 'ᘼ'
    CanadianSyllabicsCarrierTle,
    /// \u{163d}: 'ᘽ'
    CanadianSyllabicsCarrierTlee,
    /// \u{163e}: 'ᘾ'
    CanadianSyllabicsCarrierTli,
    /// \u{163f}: 'ᘿ'
    CanadianSyllabicsCarrierTla,
    /// \u{1640}: 'ᙀ'
    CanadianSyllabicsCarrierZu,
    /// \u{1641}: 'ᙁ'
    CanadianSyllabicsCarrierZo,
    /// \u{1642}: 'ᙂ'
    CanadianSyllabicsCarrierZe,
    /// \u{1643}: 'ᙃ'
    CanadianSyllabicsCarrierZee,
    /// \u{1644}: 'ᙄ'
    CanadianSyllabicsCarrierZi,
    /// \u{1645}: 'ᙅ'
    CanadianSyllabicsCarrierZa,
    /// \u{1646}: 'ᙆ'
    CanadianSyllabicsCarrierZ,
    /// \u{1647}: 'ᙇ'
    CanadianSyllabicsCarrierInitialZ,
    /// \u{1648}: 'ᙈ'
    CanadianSyllabicsCarrierDzu,
    /// \u{1649}: 'ᙉ'
    CanadianSyllabicsCarrierDzo,
    /// \u{164a}: 'ᙊ'
    CanadianSyllabicsCarrierDze,
    /// \u{164b}: 'ᙋ'
    CanadianSyllabicsCarrierDzee,
    /// \u{164c}: 'ᙌ'
    CanadianSyllabicsCarrierDzi,
    /// \u{164d}: 'ᙍ'
    CanadianSyllabicsCarrierDza,
    /// \u{164e}: 'ᙎ'
    CanadianSyllabicsCarrierSu,
    /// \u{164f}: 'ᙏ'
    CanadianSyllabicsCarrierSo,
    /// \u{1650}: 'ᙐ'
    CanadianSyllabicsCarrierSe,
    /// \u{1651}: 'ᙑ'
    CanadianSyllabicsCarrierSee,
    /// \u{1652}: 'ᙒ'
    CanadianSyllabicsCarrierSi,
    /// \u{1653}: 'ᙓ'
    CanadianSyllabicsCarrierSa,
    /// \u{1654}: 'ᙔ'
    CanadianSyllabicsCarrierShu,
    /// \u{1655}: 'ᙕ'
    CanadianSyllabicsCarrierSho,
    /// \u{1656}: 'ᙖ'
    CanadianSyllabicsCarrierShe,
    /// \u{1657}: 'ᙗ'
    CanadianSyllabicsCarrierShee,
    /// \u{1658}: 'ᙘ'
    CanadianSyllabicsCarrierShi,
    /// \u{1659}: 'ᙙ'
    CanadianSyllabicsCarrierSha,
    /// \u{165a}: 'ᙚ'
    CanadianSyllabicsCarrierSh,
    /// \u{165b}: 'ᙛ'
    CanadianSyllabicsCarrierTsu,
    /// \u{165c}: 'ᙜ'
    CanadianSyllabicsCarrierTso,
    /// \u{165d}: 'ᙝ'
    CanadianSyllabicsCarrierTse,
    /// \u{165e}: 'ᙞ'
    CanadianSyllabicsCarrierTsee,
    /// \u{165f}: 'ᙟ'
    CanadianSyllabicsCarrierTsi,
    /// \u{1660}: 'ᙠ'
    CanadianSyllabicsCarrierTsa,
    /// \u{1661}: 'ᙡ'
    CanadianSyllabicsCarrierChu,
    /// \u{1662}: 'ᙢ'
    CanadianSyllabicsCarrierCho,
    /// \u{1663}: 'ᙣ'
    CanadianSyllabicsCarrierChe,
    /// \u{1664}: 'ᙤ'
    CanadianSyllabicsCarrierChee,
    /// \u{1665}: 'ᙥ'
    CanadianSyllabicsCarrierChi,
    /// \u{1666}: 'ᙦ'
    CanadianSyllabicsCarrierCha,
    /// \u{1667}: 'ᙧ'
    CanadianSyllabicsCarrierTtsu,
    /// \u{1668}: 'ᙨ'
    CanadianSyllabicsCarrierTtso,
    /// \u{1669}: 'ᙩ'
    CanadianSyllabicsCarrierTtse,
    /// \u{166a}: 'ᙪ'
    CanadianSyllabicsCarrierTtsee,
    /// \u{166b}: 'ᙫ'
    CanadianSyllabicsCarrierTtsi,
    /// \u{166c}: 'ᙬ'
    CanadianSyllabicsCarrierTtsa,
    /// \u{166d}: '᙭'
    CanadianSyllabicsChiSign,
    /// \u{166e}: '᙮'
    CanadianSyllabicsFullStop,
    /// \u{166f}: 'ᙯ'
    CanadianSyllabicsQai,
    /// \u{1670}: 'ᙰ'
    CanadianSyllabicsNgai,
    /// \u{1671}: 'ᙱ'
    CanadianSyllabicsNngi,
    /// \u{1672}: 'ᙲ'
    CanadianSyllabicsNngii,
    /// \u{1673}: 'ᙳ'
    CanadianSyllabicsNngo,
    /// \u{1674}: 'ᙴ'
    CanadianSyllabicsNngoo,
    /// \u{1675}: 'ᙵ'
    CanadianSyllabicsNnga,
    /// \u{1676}: 'ᙶ'
    CanadianSyllabicsNngaa,
    /// \u{1677}: 'ᙷ'
    CanadianSyllabicsWoodsDashCreeThwee,
    /// \u{1678}: 'ᙸ'
    CanadianSyllabicsWoodsDashCreeThwi,
    /// \u{1679}: 'ᙹ'
    CanadianSyllabicsWoodsDashCreeThwii,
    /// \u{167a}: 'ᙺ'
    CanadianSyllabicsWoodsDashCreeThwo,
    /// \u{167b}: 'ᙻ'
    CanadianSyllabicsWoodsDashCreeThwoo,
    /// \u{167c}: 'ᙼ'
    CanadianSyllabicsWoodsDashCreeThwa,
    /// \u{167d}: 'ᙽ'
    CanadianSyllabicsWoodsDashCreeThwaa,
    /// \u{167e}: 'ᙾ'
    CanadianSyllabicsWoodsDashCreeFinalTh,
}

impl Into<char> for UnifiedCanadianAboriginalSyllabics {
    fn into(self) -> char {
        match self {
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsHyphen => '᐀',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsE => 'ᐁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAai => 'ᐂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsI => 'ᐃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsIi => 'ᐄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsO => 'ᐅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsOo => 'ᐆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeOo => 'ᐇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierEe => 'ᐈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierI => 'ᐉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsA => 'ᐊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAa => 'ᐋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWe => 'ᐌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWe => 'ᐍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWi => 'ᐎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWi => 'ᐏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWii => 'ᐐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWii => 'ᐑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWo => 'ᐒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWo => 'ᐓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoo => 'ᐔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWoo => 'ᐕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiWoo => 'ᐖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWa => 'ᐗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWa => 'ᐘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWaa => 'ᐙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWaa => 'ᐚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiWaa => 'ᐛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAi => 'ᐜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeW => 'ᐝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsGlottalStop => 'ᐞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalAcute => 'ᐟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalGrave => 'ᐠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalBottomHalfRing => 'ᐡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalTopHalfRing => 'ᐢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalRightHalfRing => 'ᐣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalRing => 'ᐤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDoubleAcute => 'ᐥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDoubleShortVerticalStrokes => 'ᐦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalMiddleDot => 'ᐧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalShortHorizontalStroke => 'ᐨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalPlus => 'ᐩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDownTack => 'ᐪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsEn => 'ᐫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsIn => 'ᐬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsOn => 'ᐭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAn => 'ᐮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPe => 'ᐯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPaai => 'ᐰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPi => 'ᐱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPii => 'ᐲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPo => 'ᐳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPoo => 'ᐴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreePoo => 'ᐵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHee => 'ᐶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHi => 'ᐷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPa => 'ᐸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPaa => 'ᐹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwe => 'ᐺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwe => 'ᐻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwi => 'ᐼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwi => 'ᐽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwii => 'ᐾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwii => 'ᐿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwo => 'ᑀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwo => 'ᑁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwoo => 'ᑂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwoo => 'ᑃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwa => 'ᑄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwa => 'ᑅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwaa => 'ᑆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwaa => 'ᑇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreePwaa => 'ᑈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsP => 'ᑉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeP => 'ᑊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierH => 'ᑋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTe => 'ᑌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTaai => 'ᑍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTi => 'ᑎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTii => 'ᑏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTo => 'ᑐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsToo => 'ᑑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeToo => 'ᑒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDee => 'ᑓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDi => 'ᑔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTa => 'ᑕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTaa => 'ᑖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwe => 'ᑗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwe => 'ᑘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwi => 'ᑙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwi => 'ᑚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwii => 'ᑛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwii => 'ᑜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwo => 'ᑝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwo => 'ᑞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwoo => 'ᑟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwoo => 'ᑠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwa => 'ᑡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwa => 'ᑢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwaa => 'ᑣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwaa => 'ᑤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiTwaa => 'ᑥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsT => 'ᑦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTte => 'ᑧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTti => 'ᑨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTto => 'ᑩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTta => 'ᑪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKe => 'ᑫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKaai => 'ᑬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKi => 'ᑭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKii => 'ᑮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKo => 'ᑯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKoo => 'ᑰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeKoo => 'ᑱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKa => 'ᑲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKaa => 'ᑳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwe => 'ᑴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwe => 'ᑵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwi => 'ᑶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwi => 'ᑷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwii => 'ᑸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwii => 'ᑹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwo => 'ᑺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwo => 'ᑻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwoo => 'ᑼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwoo => 'ᑽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwa => 'ᑾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwa => 'ᑿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwaa => 'ᒀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwaa => 'ᒁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiKwaa => 'ᒂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsK => 'ᒃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKw => 'ᒄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKeh => 'ᒅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKih => 'ᒆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKoh => 'ᒇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKah => 'ᒈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCe => 'ᒉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCaai => 'ᒊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCi => 'ᒋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCii => 'ᒌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCo => 'ᒍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCoo => 'ᒎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeCoo => 'ᒏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCa => 'ᒐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCaa => 'ᒑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwe => 'ᒒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwe => 'ᒓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwi => 'ᒔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwi => 'ᒕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwii => 'ᒖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwii => 'ᒗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwo => 'ᒘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwo => 'ᒙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwoo => 'ᒚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwoo => 'ᒛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwa => 'ᒜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwa => 'ᒝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwaa => 'ᒞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwaa => 'ᒟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiCwaa => 'ᒠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsC => 'ᒡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiTh => 'ᒢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMe => 'ᒣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMaai => 'ᒤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMi => 'ᒥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMii => 'ᒦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMo => 'ᒧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMoo => 'ᒨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeMoo => 'ᒩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMa => 'ᒪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMaa => 'ᒫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwe => 'ᒬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwe => 'ᒭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwi => 'ᒮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwi => 'ᒯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwii => 'ᒰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwii => 'ᒱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwo => 'ᒲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwo => 'ᒳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwoo => 'ᒴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwoo => 'ᒵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwa => 'ᒶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwa => 'ᒷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwaa => 'ᒸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwaa => 'ᒹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiMwaa => 'ᒺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsM => 'ᒻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeM => 'ᒼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMh => 'ᒽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAthapascanM => 'ᒾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiM => 'ᒿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNe => 'ᓀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaai => 'ᓁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNi => 'ᓂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNii => 'ᓃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNo => 'ᓄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNoo => 'ᓅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeNoo => 'ᓆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNa => 'ᓇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaa => 'ᓈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwe => 'ᓉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwe => 'ᓊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwa => 'ᓋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwa => 'ᓌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwaa => 'ᓍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwaa => 'ᓎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiNwaa => 'ᓏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsN => 'ᓐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNg => 'ᓑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNh => 'ᓒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLe => 'ᓓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLaai => 'ᓔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLi => 'ᓕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLii => 'ᓖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLo => 'ᓗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLoo => 'ᓘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeLoo => 'ᓙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLa => 'ᓚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLaa => 'ᓛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwe => 'ᓜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwe => 'ᓝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwi => 'ᓞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwi => 'ᓟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwii => 'ᓠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwii => 'ᓡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwo => 'ᓢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwo => 'ᓣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwoo => 'ᓤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwoo => 'ᓥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwa => 'ᓦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwa => 'ᓧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwaa => 'ᓨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwaa => 'ᓩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsL => 'ᓪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeL => 'ᓫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMedialL => 'ᓬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSe => 'ᓭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSaai => 'ᓮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSi => 'ᓯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSii => 'ᓰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSo => 'ᓱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSoo => 'ᓲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeSoo => 'ᓳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSa => 'ᓴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSaa => 'ᓵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwe => 'ᓶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwe => 'ᓷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwi => 'ᓸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwi => 'ᓹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwii => 'ᓺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwii => 'ᓻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwo => 'ᓼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwo => 'ᓽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwoo => 'ᓾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwoo => 'ᓿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwa => 'ᔀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwa => 'ᔁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwaa => 'ᔂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwaa => 'ᔃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSwaa => 'ᔄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsS => 'ᔅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAthapascanS => 'ᔆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSw => 'ᔇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootS => 'ᔈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMooseDashCreeSk => 'ᔉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSkw => 'ᔊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSDashW => 'ᔋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSpwa => 'ᔌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiStwa => 'ᔍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSkwa => 'ᔎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiScwa => 'ᔏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShe => 'ᔐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShi => 'ᔑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShii => 'ᔒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSho => 'ᔓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShoo => 'ᔔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSha => 'ᔕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShaa => 'ᔖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwe => 'ᔗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwe => 'ᔘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwi => 'ᔙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwi => 'ᔚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwii => 'ᔛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwii => 'ᔜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwo => 'ᔝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwo => 'ᔞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwoo => 'ᔟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwoo => 'ᔠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwa => 'ᔡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwa => 'ᔢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwaa => 'ᔣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwaa => 'ᔤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSh => 'ᔥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYe => 'ᔦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYaai => 'ᔧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYi => 'ᔨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYii => 'ᔩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYo => 'ᔪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYoo => 'ᔫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeYoo => 'ᔬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYa => 'ᔭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYaa => 'ᔮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwe => 'ᔯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwe => 'ᔰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwi => 'ᔱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwi => 'ᔲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwii => 'ᔳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwii => 'ᔴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwo => 'ᔵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwo => 'ᔶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwoo => 'ᔷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwoo => 'ᔸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwa => 'ᔹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwa => 'ᔺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwaa => 'ᔻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwaa => 'ᔼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiYwaa => 'ᔽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsY => 'ᔾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBibleDashCreeY => 'ᔿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeY => 'ᕀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiYi => 'ᕁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRe => 'ᕂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRDashCreeRe => 'ᕃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLe => 'ᕄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRaai => 'ᕅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRi => 'ᕆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRii => 'ᕇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRo => 'ᕈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRoo => 'ᕉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLo => 'ᕊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRa => 'ᕋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRaa => 'ᕌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLa => 'ᕍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRwaa => 'ᕎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRwaa => 'ᕏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsR => 'ᕐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeR => 'ᕑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMedialR => 'ᕒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFe => 'ᕓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFaai => 'ᕔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFi => 'ᕕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFii => 'ᕖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFo => 'ᕗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFoo => 'ᕘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFa => 'ᕙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFaa => 'ᕚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFwaa => 'ᕛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeFwaa => 'ᕜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsF => 'ᕝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThe => 'ᕞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThe => 'ᕟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThi => 'ᕠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThi => 'ᕡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThii => 'ᕢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThii => 'ᕣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTho => 'ᕤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThoo => 'ᕥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTha => 'ᕦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThaa => 'ᕧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThwaa => 'ᕨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeThwaa => 'ᕩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTh => 'ᕪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTthe => 'ᕫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTthi => 'ᕬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTtho => 'ᕭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTtha => 'ᕮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTth => 'ᕯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTye => 'ᕰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTyi => 'ᕱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTyo => 'ᕲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTya => 'ᕳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHe => 'ᕴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHi => 'ᕵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHii => 'ᕶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHo => 'ᕷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHoo => 'ᕸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHa => 'ᕹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHaa => 'ᕺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikH => 'ᕻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavutH => 'ᕼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsHk => 'ᕽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQaai => 'ᕾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQi => 'ᕿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQii => 'ᖀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQo => 'ᖁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQoo => 'ᖂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQa => 'ᖃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQaa => 'ᖄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQ => 'ᖅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlhe => 'ᖆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlhi => 'ᖇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlho => 'ᖈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlha => 'ᖉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRe => 'ᖊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRi => 'ᖋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRo => 'ᖌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRa => 'ᖍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgaai => 'ᖎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgi => 'ᖏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgii => 'ᖐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgo => 'ᖑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgoo => 'ᖒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNga => 'ᖓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgaa => 'ᖔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNg => 'ᖕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNng => 'ᖖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiShe => 'ᖗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiShi => 'ᖘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiSho => 'ᖙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiSha => 'ᖚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThe => 'ᖛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThi => 'ᖜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTho => 'ᖝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTha => 'ᖞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTh => 'ᖟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhi => 'ᖠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhii => 'ᖡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLho => 'ᖢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhoo => 'ᖣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLha => 'ᖤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhaa => 'ᖥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLh => 'ᖦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThe => 'ᖧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThi => 'ᖨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThii => 'ᖩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTho => 'ᖪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThoo => 'ᖫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTha => 'ᖬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThaa => 'ᖭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTh => 'ᖮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAivilikB => 'ᖯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootE => 'ᖰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootI => 'ᖱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootO => 'ᖲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootA => 'ᖳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWe => 'ᖴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWi => 'ᖵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWo => 'ᖶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWa => 'ᖷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNe => 'ᖸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNi => 'ᖹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNo => 'ᖺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNa => 'ᖻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKe => 'ᖼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKi => 'ᖽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKo => 'ᖾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKa => 'ᖿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHe => 'ᗀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHi => 'ᗁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHo => 'ᗂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHa => 'ᗃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhu => 'ᗄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGho => 'ᗅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhe => 'ᗆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhee => 'ᗇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhi => 'ᗈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGha => 'ᗉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRu => 'ᗊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRo => 'ᗋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRe => 'ᗌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRee => 'ᗍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRi => 'ᗎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRa => 'ᗏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWu => 'ᗐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWo => 'ᗑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWe => 'ᗒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWee => 'ᗓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWi => 'ᗔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWa => 'ᗕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwu => 'ᗖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwo => 'ᗗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwe => 'ᗘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwee => 'ᗙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwi => 'ᗚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwa => 'ᗛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThu => 'ᗜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTho => 'ᗝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThe => 'ᗞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThee => 'ᗟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThi => 'ᗠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTha => 'ᗡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtu => 'ᗢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTto => 'ᗣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTte => 'ᗤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtee => 'ᗥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTti => 'ᗦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTta => 'ᗧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPu => 'ᗨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPo => 'ᗩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPe => 'ᗪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPee => 'ᗫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPi => 'ᗬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPa => 'ᗭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierP => 'ᗮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGu => 'ᗯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGo => 'ᗰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGe => 'ᗱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGee => 'ᗲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGi => 'ᗳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGa => 'ᗴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhu => 'ᗵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKho => 'ᗶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhe => 'ᗷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhee => 'ᗸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhi => 'ᗹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKha => 'ᗺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKku => 'ᗻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKko => 'ᗼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKke => 'ᗽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKkee => 'ᗾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKki => 'ᗿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKka => 'ᘀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKk => 'ᘁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNu => 'ᘂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNo => 'ᘃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNe => 'ᘄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNee => 'ᘅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNi => 'ᘆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNa => 'ᘇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMu => 'ᘈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMo => 'ᘉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMe => 'ᘊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMee => 'ᘋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMi => 'ᘌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMa => 'ᘍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYu => 'ᘎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYo => 'ᘏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYe => 'ᘐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYee => 'ᘑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYi => 'ᘒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYa => 'ᘓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJu => 'ᘔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiJu => 'ᘕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJo => 'ᘖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJe => 'ᘗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJee => 'ᘘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJi => 'ᘙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiJi => 'ᘚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJa => 'ᘛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJju => 'ᘜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJjo => 'ᘝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJje => 'ᘞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJjee => 'ᘟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJji => 'ᘠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJja => 'ᘡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLu => 'ᘢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLo => 'ᘣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLe => 'ᘤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLee => 'ᘥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLi => 'ᘦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLa => 'ᘧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlu => 'ᘨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlo => 'ᘩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDle => 'ᘪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlee => 'ᘫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDli => 'ᘬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDla => 'ᘭ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhu => 'ᘮ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLho => 'ᘯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhe => 'ᘰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhee => 'ᘱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhi => 'ᘲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLha => 'ᘳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhu => 'ᘴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlho => 'ᘵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhe => 'ᘶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhee => 'ᘷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhi => 'ᘸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlha => 'ᘹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlu => 'ᘺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlo => 'ᘻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTle => 'ᘼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlee => 'ᘽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTli => 'ᘾ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTla => 'ᘿ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZu => 'ᙀ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZo => 'ᙁ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZe => 'ᙂ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZee => 'ᙃ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZi => 'ᙄ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZa => 'ᙅ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZ => 'ᙆ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierInitialZ => 'ᙇ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzu => 'ᙈ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzo => 'ᙉ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDze => 'ᙊ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzee => 'ᙋ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzi => 'ᙌ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDza => 'ᙍ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSu => 'ᙎ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSo => 'ᙏ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSe => 'ᙐ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSee => 'ᙑ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSi => 'ᙒ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSa => 'ᙓ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShu => 'ᙔ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSho => 'ᙕ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShe => 'ᙖ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShee => 'ᙗ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShi => 'ᙘ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSha => 'ᙙ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSh => 'ᙚ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsu => 'ᙛ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTso => 'ᙜ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTse => 'ᙝ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsee => 'ᙞ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsi => 'ᙟ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsa => 'ᙠ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChu => 'ᙡ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierCho => 'ᙢ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChe => 'ᙣ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChee => 'ᙤ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChi => 'ᙥ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierCha => 'ᙦ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsu => 'ᙧ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtso => 'ᙨ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtse => 'ᙩ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsee => 'ᙪ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsi => 'ᙫ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsa => 'ᙬ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsChiSign => '᙭',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFullStop => '᙮',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQai => 'ᙯ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgai => 'ᙰ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngi => 'ᙱ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngii => 'ᙲ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngo => 'ᙳ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngoo => 'ᙴ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNnga => 'ᙵ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngaa => 'ᙶ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwee => 'ᙷ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwi => 'ᙸ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwii => 'ᙹ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwo => 'ᙺ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwoo => 'ᙻ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwa => 'ᙼ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwaa => 'ᙽ',
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeFinalTh => 'ᙾ',
        }
    }
}

impl std::convert::TryFrom<char> for UnifiedCanadianAboriginalSyllabics {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '᐀' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsHyphen),
            'ᐁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsE),
            'ᐂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAai),
            'ᐃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsI),
            'ᐄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsIi),
            'ᐅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsO),
            'ᐆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsOo),
            'ᐇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeOo),
            'ᐈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierEe),
            'ᐉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierI),
            'ᐊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsA),
            'ᐋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAa),
            'ᐌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWe),
            'ᐍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWe),
            'ᐎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWi),
            'ᐏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWi),
            'ᐐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWii),
            'ᐑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWii),
            'ᐒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWo),
            'ᐓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWo),
            'ᐔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoo),
            'ᐕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWoo),
            'ᐖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiWoo),
            'ᐗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWa),
            'ᐘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWa),
            'ᐙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWaa),
            'ᐚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWaa),
            'ᐛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiWaa),
            'ᐜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAi),
            'ᐝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeW),
            'ᐞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsGlottalStop),
            'ᐟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalAcute),
            'ᐠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalGrave),
            'ᐡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalBottomHalfRing),
            'ᐢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalTopHalfRing),
            'ᐣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalRightHalfRing),
            'ᐤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalRing),
            'ᐥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDoubleAcute),
            'ᐦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDoubleShortVerticalStrokes),
            'ᐧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalMiddleDot),
            'ᐨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalShortHorizontalStroke),
            'ᐩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalPlus),
            'ᐪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDownTack),
            'ᐫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsEn),
            'ᐬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsIn),
            'ᐭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsOn),
            'ᐮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAn),
            'ᐯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPe),
            'ᐰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPaai),
            'ᐱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPi),
            'ᐲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPii),
            'ᐳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPo),
            'ᐴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPoo),
            'ᐵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreePoo),
            'ᐶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHee),
            'ᐷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHi),
            'ᐸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPa),
            'ᐹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPaa),
            'ᐺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwe),
            'ᐻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwe),
            'ᐼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwi),
            'ᐽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwi),
            'ᐾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwii),
            'ᐿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwii),
            'ᑀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwo),
            'ᑁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwo),
            'ᑂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwoo),
            'ᑃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwoo),
            'ᑄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwa),
            'ᑅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwa),
            'ᑆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwaa),
            'ᑇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwaa),
            'ᑈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreePwaa),
            'ᑉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsP),
            'ᑊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeP),
            'ᑋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierH),
            'ᑌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTe),
            'ᑍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTaai),
            'ᑎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTi),
            'ᑏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTii),
            'ᑐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTo),
            'ᑑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsToo),
            'ᑒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeToo),
            'ᑓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDee),
            'ᑔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDi),
            'ᑕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTa),
            'ᑖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTaa),
            'ᑗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwe),
            'ᑘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwe),
            'ᑙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwi),
            'ᑚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwi),
            'ᑛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwii),
            'ᑜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwii),
            'ᑝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwo),
            'ᑞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwo),
            'ᑟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwoo),
            'ᑠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwoo),
            'ᑡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwa),
            'ᑢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwa),
            'ᑣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwaa),
            'ᑤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwaa),
            'ᑥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiTwaa),
            'ᑦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsT),
            'ᑧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTte),
            'ᑨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTti),
            'ᑩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTto),
            'ᑪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTta),
            'ᑫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKe),
            'ᑬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKaai),
            'ᑭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKi),
            'ᑮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKii),
            'ᑯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKo),
            'ᑰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKoo),
            'ᑱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeKoo),
            'ᑲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKa),
            'ᑳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKaa),
            'ᑴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwe),
            'ᑵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwe),
            'ᑶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwi),
            'ᑷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwi),
            'ᑸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwii),
            'ᑹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwii),
            'ᑺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwo),
            'ᑻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwo),
            'ᑼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwoo),
            'ᑽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwoo),
            'ᑾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwa),
            'ᑿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwa),
            'ᒀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwaa),
            'ᒁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwaa),
            'ᒂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiKwaa),
            'ᒃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsK),
            'ᒄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKw),
            'ᒅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKeh),
            'ᒆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKih),
            'ᒇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKoh),
            'ᒈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKah),
            'ᒉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCe),
            'ᒊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCaai),
            'ᒋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCi),
            'ᒌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCii),
            'ᒍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCo),
            'ᒎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCoo),
            'ᒏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeCoo),
            'ᒐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCa),
            'ᒑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCaa),
            'ᒒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwe),
            'ᒓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwe),
            'ᒔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwi),
            'ᒕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwi),
            'ᒖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwii),
            'ᒗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwii),
            'ᒘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwo),
            'ᒙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwo),
            'ᒚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwoo),
            'ᒛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwoo),
            'ᒜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwa),
            'ᒝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwa),
            'ᒞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwaa),
            'ᒟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwaa),
            'ᒠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiCwaa),
            'ᒡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsC),
            'ᒢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiTh),
            'ᒣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMe),
            'ᒤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMaai),
            'ᒥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMi),
            'ᒦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMii),
            'ᒧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMo),
            'ᒨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMoo),
            'ᒩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeMoo),
            'ᒪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMa),
            'ᒫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMaa),
            'ᒬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwe),
            'ᒭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwe),
            'ᒮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwi),
            'ᒯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwi),
            'ᒰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwii),
            'ᒱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwii),
            'ᒲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwo),
            'ᒳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwo),
            'ᒴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwoo),
            'ᒵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwoo),
            'ᒶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwa),
            'ᒷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwa),
            'ᒸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwaa),
            'ᒹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwaa),
            'ᒺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiMwaa),
            'ᒻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsM),
            'ᒼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeM),
            'ᒽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMh),
            'ᒾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAthapascanM),
            'ᒿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiM),
            'ᓀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNe),
            'ᓁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaai),
            'ᓂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNi),
            'ᓃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNii),
            'ᓄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNo),
            'ᓅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNoo),
            'ᓆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeNoo),
            'ᓇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNa),
            'ᓈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaa),
            'ᓉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwe),
            'ᓊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwe),
            'ᓋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwa),
            'ᓌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwa),
            'ᓍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwaa),
            'ᓎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwaa),
            'ᓏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiNwaa),
            'ᓐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsN),
            'ᓑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNg),
            'ᓒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNh),
            'ᓓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLe),
            'ᓔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLaai),
            'ᓕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLi),
            'ᓖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLii),
            'ᓗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLo),
            'ᓘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLoo),
            'ᓙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeLoo),
            'ᓚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLa),
            'ᓛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLaa),
            'ᓜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwe),
            'ᓝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwe),
            'ᓞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwi),
            'ᓟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwi),
            'ᓠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwii),
            'ᓡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwii),
            'ᓢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwo),
            'ᓣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwo),
            'ᓤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwoo),
            'ᓥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwoo),
            'ᓦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwa),
            'ᓧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwa),
            'ᓨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwaa),
            'ᓩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwaa),
            'ᓪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsL),
            'ᓫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeL),
            'ᓬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMedialL),
            'ᓭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSe),
            'ᓮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSaai),
            'ᓯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSi),
            'ᓰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSii),
            'ᓱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSo),
            'ᓲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSoo),
            'ᓳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeSoo),
            'ᓴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSa),
            'ᓵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSaa),
            'ᓶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwe),
            'ᓷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwe),
            'ᓸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwi),
            'ᓹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwi),
            'ᓺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwii),
            'ᓻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwii),
            'ᓼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwo),
            'ᓽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwo),
            'ᓾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwoo),
            'ᓿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwoo),
            'ᔀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwa),
            'ᔁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwa),
            'ᔂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwaa),
            'ᔃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwaa),
            'ᔄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSwaa),
            'ᔅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsS),
            'ᔆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAthapascanS),
            'ᔇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSw),
            'ᔈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootS),
            'ᔉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMooseDashCreeSk),
            'ᔊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSkw),
            'ᔋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSDashW),
            'ᔌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSpwa),
            'ᔍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiStwa),
            'ᔎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSkwa),
            'ᔏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiScwa),
            'ᔐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShe),
            'ᔑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShi),
            'ᔒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShii),
            'ᔓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSho),
            'ᔔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShoo),
            'ᔕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSha),
            'ᔖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShaa),
            'ᔗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwe),
            'ᔘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwe),
            'ᔙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwi),
            'ᔚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwi),
            'ᔛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwii),
            'ᔜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwii),
            'ᔝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwo),
            'ᔞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwo),
            'ᔟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwoo),
            'ᔠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwoo),
            'ᔡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwa),
            'ᔢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwa),
            'ᔣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwaa),
            'ᔤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwaa),
            'ᔥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSh),
            'ᔦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYe),
            'ᔧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYaai),
            'ᔨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYi),
            'ᔩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYii),
            'ᔪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYo),
            'ᔫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYoo),
            'ᔬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeYoo),
            'ᔭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYa),
            'ᔮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYaa),
            'ᔯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwe),
            'ᔰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwe),
            'ᔱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwi),
            'ᔲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwi),
            'ᔳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwii),
            'ᔴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwii),
            'ᔵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwo),
            'ᔶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwo),
            'ᔷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwoo),
            'ᔸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwoo),
            'ᔹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwa),
            'ᔺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwa),
            'ᔻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwaa),
            'ᔼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwaa),
            'ᔽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiYwaa),
            'ᔾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsY),
            'ᔿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBibleDashCreeY),
            'ᕀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeY),
            'ᕁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiYi),
            'ᕂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRe),
            'ᕃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRDashCreeRe),
            'ᕄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLe),
            'ᕅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRaai),
            'ᕆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRi),
            'ᕇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRii),
            'ᕈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRo),
            'ᕉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRoo),
            'ᕊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLo),
            'ᕋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRa),
            'ᕌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRaa),
            'ᕍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLa),
            'ᕎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRwaa),
            'ᕏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRwaa),
            'ᕐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsR),
            'ᕑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeR),
            'ᕒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMedialR),
            'ᕓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFe),
            'ᕔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFaai),
            'ᕕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFi),
            'ᕖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFii),
            'ᕗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFo),
            'ᕘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFoo),
            'ᕙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFa),
            'ᕚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFaa),
            'ᕛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFwaa),
            'ᕜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeFwaa),
            'ᕝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsF),
            'ᕞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThe),
            'ᕟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThe),
            'ᕠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThi),
            'ᕡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThi),
            'ᕢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThii),
            'ᕣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThii),
            'ᕤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTho),
            'ᕥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThoo),
            'ᕦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTha),
            'ᕧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThaa),
            'ᕨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThwaa),
            'ᕩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeThwaa),
            'ᕪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTh),
            'ᕫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTthe),
            'ᕬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTthi),
            'ᕭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTtho),
            'ᕮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTtha),
            'ᕯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTth),
            'ᕰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTye),
            'ᕱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTyi),
            'ᕲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTyo),
            'ᕳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTya),
            'ᕴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHe),
            'ᕵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHi),
            'ᕶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHii),
            'ᕷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHo),
            'ᕸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHoo),
            'ᕹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHa),
            'ᕺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHaa),
            'ᕻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikH),
            'ᕼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavutH),
            'ᕽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsHk),
            'ᕾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQaai),
            'ᕿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQi),
            'ᖀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQii),
            'ᖁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQo),
            'ᖂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQoo),
            'ᖃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQa),
            'ᖄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQaa),
            'ᖅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQ),
            'ᖆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlhe),
            'ᖇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlhi),
            'ᖈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlho),
            'ᖉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlha),
            'ᖊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRe),
            'ᖋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRi),
            'ᖌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRo),
            'ᖍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRa),
            'ᖎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgaai),
            'ᖏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgi),
            'ᖐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgii),
            'ᖑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgo),
            'ᖒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgoo),
            'ᖓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNga),
            'ᖔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgaa),
            'ᖕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNg),
            'ᖖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNng),
            'ᖗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiShe),
            'ᖘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiShi),
            'ᖙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiSho),
            'ᖚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiSha),
            'ᖛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThe),
            'ᖜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThi),
            'ᖝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTho),
            'ᖞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTha),
            'ᖟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTh),
            'ᖠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhi),
            'ᖡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhii),
            'ᖢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLho),
            'ᖣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhoo),
            'ᖤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLha),
            'ᖥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhaa),
            'ᖦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLh),
            'ᖧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThe),
            'ᖨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThi),
            'ᖩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThii),
            'ᖪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTho),
            'ᖫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThoo),
            'ᖬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTha),
            'ᖭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThaa),
            'ᖮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTh),
            'ᖯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAivilikB),
            'ᖰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootE),
            'ᖱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootI),
            'ᖲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootO),
            'ᖳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootA),
            'ᖴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWe),
            'ᖵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWi),
            'ᖶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWo),
            'ᖷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWa),
            'ᖸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNe),
            'ᖹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNi),
            'ᖺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNo),
            'ᖻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNa),
            'ᖼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKe),
            'ᖽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKi),
            'ᖾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKo),
            'ᖿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKa),
            'ᗀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHe),
            'ᗁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHi),
            'ᗂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHo),
            'ᗃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHa),
            'ᗄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhu),
            'ᗅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGho),
            'ᗆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhe),
            'ᗇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhee),
            'ᗈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhi),
            'ᗉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGha),
            'ᗊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRu),
            'ᗋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRo),
            'ᗌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRe),
            'ᗍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRee),
            'ᗎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRi),
            'ᗏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRa),
            'ᗐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWu),
            'ᗑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWo),
            'ᗒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWe),
            'ᗓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWee),
            'ᗔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWi),
            'ᗕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWa),
            'ᗖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwu),
            'ᗗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwo),
            'ᗘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwe),
            'ᗙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwee),
            'ᗚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwi),
            'ᗛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwa),
            'ᗜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThu),
            'ᗝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTho),
            'ᗞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThe),
            'ᗟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThee),
            'ᗠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThi),
            'ᗡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTha),
            'ᗢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtu),
            'ᗣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTto),
            'ᗤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTte),
            'ᗥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtee),
            'ᗦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTti),
            'ᗧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTta),
            'ᗨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPu),
            'ᗩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPo),
            'ᗪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPe),
            'ᗫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPee),
            'ᗬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPi),
            'ᗭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPa),
            'ᗮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierP),
            'ᗯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGu),
            'ᗰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGo),
            'ᗱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGe),
            'ᗲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGee),
            'ᗳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGi),
            'ᗴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGa),
            'ᗵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhu),
            'ᗶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKho),
            'ᗷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhe),
            'ᗸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhee),
            'ᗹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhi),
            'ᗺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKha),
            'ᗻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKku),
            'ᗼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKko),
            'ᗽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKke),
            'ᗾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKkee),
            'ᗿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKki),
            'ᘀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKka),
            'ᘁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKk),
            'ᘂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNu),
            'ᘃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNo),
            'ᘄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNe),
            'ᘅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNee),
            'ᘆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNi),
            'ᘇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNa),
            'ᘈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMu),
            'ᘉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMo),
            'ᘊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMe),
            'ᘋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMee),
            'ᘌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMi),
            'ᘍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMa),
            'ᘎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYu),
            'ᘏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYo),
            'ᘐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYe),
            'ᘑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYee),
            'ᘒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYi),
            'ᘓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYa),
            'ᘔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJu),
            'ᘕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiJu),
            'ᘖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJo),
            'ᘗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJe),
            'ᘘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJee),
            'ᘙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJi),
            'ᘚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiJi),
            'ᘛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJa),
            'ᘜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJju),
            'ᘝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJjo),
            'ᘞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJje),
            'ᘟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJjee),
            'ᘠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJji),
            'ᘡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJja),
            'ᘢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLu),
            'ᘣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLo),
            'ᘤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLe),
            'ᘥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLee),
            'ᘦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLi),
            'ᘧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLa),
            'ᘨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlu),
            'ᘩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlo),
            'ᘪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDle),
            'ᘫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlee),
            'ᘬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDli),
            'ᘭ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDla),
            'ᘮ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhu),
            'ᘯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLho),
            'ᘰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhe),
            'ᘱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhee),
            'ᘲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhi),
            'ᘳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLha),
            'ᘴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhu),
            'ᘵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlho),
            'ᘶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhe),
            'ᘷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhee),
            'ᘸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhi),
            'ᘹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlha),
            'ᘺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlu),
            'ᘻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlo),
            'ᘼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTle),
            'ᘽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlee),
            'ᘾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTli),
            'ᘿ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTla),
            'ᙀ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZu),
            'ᙁ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZo),
            'ᙂ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZe),
            'ᙃ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZee),
            'ᙄ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZi),
            'ᙅ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZa),
            'ᙆ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZ),
            'ᙇ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierInitialZ),
            'ᙈ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzu),
            'ᙉ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzo),
            'ᙊ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDze),
            'ᙋ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzee),
            'ᙌ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzi),
            'ᙍ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDza),
            'ᙎ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSu),
            'ᙏ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSo),
            'ᙐ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSe),
            'ᙑ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSee),
            'ᙒ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSi),
            'ᙓ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSa),
            'ᙔ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShu),
            'ᙕ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSho),
            'ᙖ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShe),
            'ᙗ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShee),
            'ᙘ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShi),
            'ᙙ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSha),
            'ᙚ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSh),
            'ᙛ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsu),
            'ᙜ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTso),
            'ᙝ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTse),
            'ᙞ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsee),
            'ᙟ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsi),
            'ᙠ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsa),
            'ᙡ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChu),
            'ᙢ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierCho),
            'ᙣ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChe),
            'ᙤ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChee),
            'ᙥ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChi),
            'ᙦ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierCha),
            'ᙧ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsu),
            'ᙨ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtso),
            'ᙩ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtse),
            'ᙪ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsee),
            'ᙫ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsi),
            'ᙬ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsa),
            '᙭' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsChiSign),
            '᙮' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFullStop),
            'ᙯ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQai),
            'ᙰ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgai),
            'ᙱ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngi),
            'ᙲ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngii),
            'ᙳ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngo),
            'ᙴ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngoo),
            'ᙵ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNnga),
            'ᙶ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngaa),
            'ᙷ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwee),
            'ᙸ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwi),
            'ᙹ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwii),
            'ᙺ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwo),
            'ᙻ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwoo),
            'ᙼ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwa),
            'ᙽ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwaa),
            'ᙾ' => Ok(UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeFinalTh),
            _ => Err(()),
        }
    }
}

impl Into<u32> for UnifiedCanadianAboriginalSyllabics {
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

impl std::convert::TryFrom<u32> for UnifiedCanadianAboriginalSyllabics {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for UnifiedCanadianAboriginalSyllabics {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl UnifiedCanadianAboriginalSyllabics {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsHyphen
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsHyphen => "canadian syllabics hyphen",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsE => "canadian syllabics e",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAai => "canadian syllabics aai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsI => "canadian syllabics i",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsIi => "canadian syllabics ii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsO => "canadian syllabics o",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsOo => "canadian syllabics oo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeOo => "canadian syllabics y-cree oo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierEe => "canadian syllabics carrier ee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierI => "canadian syllabics carrier i",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsA => "canadian syllabics a",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAa => "canadian syllabics aa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWe => "canadian syllabics we",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWe => "canadian syllabics west-cree we",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWi => "canadian syllabics wi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWi => "canadian syllabics west-cree wi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWii => "canadian syllabics wii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWii => "canadian syllabics west-cree wii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWo => "canadian syllabics wo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWo => "canadian syllabics west-cree wo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoo => "canadian syllabics woo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWoo => "canadian syllabics west-cree woo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiWoo => "canadian syllabics naskapi woo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWa => "canadian syllabics wa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWa => "canadian syllabics west-cree wa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWaa => "canadian syllabics waa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeWaa => "canadian syllabics west-cree waa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiWaa => "canadian syllabics naskapi waa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAi => "canadian syllabics ai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeW => "canadian syllabics y-cree w",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsGlottalStop => "canadian syllabics glottal stop",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalAcute => "canadian syllabics final acute",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalGrave => "canadian syllabics final grave",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalBottomHalfRing => "canadian syllabics final bottom half ring",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalTopHalfRing => "canadian syllabics final top half ring",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalRightHalfRing => "canadian syllabics final right half ring",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalRing => "canadian syllabics final ring",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDoubleAcute => "canadian syllabics final double acute",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDoubleShortVerticalStrokes => "canadian syllabics final double short vertical strokes",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalMiddleDot => "canadian syllabics final middle dot",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalShortHorizontalStroke => "canadian syllabics final short horizontal stroke",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalPlus => "canadian syllabics final plus",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFinalDownTack => "canadian syllabics final down tack",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsEn => "canadian syllabics en",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsIn => "canadian syllabics in",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsOn => "canadian syllabics on",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAn => "canadian syllabics an",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPe => "canadian syllabics pe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPaai => "canadian syllabics paai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPi => "canadian syllabics pi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPii => "canadian syllabics pii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPo => "canadian syllabics po",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPoo => "canadian syllabics poo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreePoo => "canadian syllabics y-cree poo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHee => "canadian syllabics carrier hee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHi => "canadian syllabics carrier hi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPa => "canadian syllabics pa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPaa => "canadian syllabics paa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwe => "canadian syllabics pwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwe => "canadian syllabics west-cree pwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwi => "canadian syllabics pwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwi => "canadian syllabics west-cree pwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwii => "canadian syllabics pwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwii => "canadian syllabics west-cree pwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwo => "canadian syllabics pwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwo => "canadian syllabics west-cree pwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwoo => "canadian syllabics pwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwoo => "canadian syllabics west-cree pwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwa => "canadian syllabics pwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwa => "canadian syllabics west-cree pwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsPwaa => "canadian syllabics pwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreePwaa => "canadian syllabics west-cree pwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreePwaa => "canadian syllabics y-cree pwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsP => "canadian syllabics p",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeP => "canadian syllabics west-cree p",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierH => "canadian syllabics carrier h",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTe => "canadian syllabics te",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTaai => "canadian syllabics taai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTi => "canadian syllabics ti",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTii => "canadian syllabics tii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTo => "canadian syllabics to",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsToo => "canadian syllabics too",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeToo => "canadian syllabics y-cree too",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDee => "canadian syllabics carrier dee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDi => "canadian syllabics carrier di",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTa => "canadian syllabics ta",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTaa => "canadian syllabics taa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwe => "canadian syllabics twe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwe => "canadian syllabics west-cree twe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwi => "canadian syllabics twi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwi => "canadian syllabics west-cree twi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwii => "canadian syllabics twii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwii => "canadian syllabics west-cree twii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwo => "canadian syllabics two",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwo => "canadian syllabics west-cree two",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwoo => "canadian syllabics twoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwoo => "canadian syllabics west-cree twoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwa => "canadian syllabics twa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwa => "canadian syllabics west-cree twa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTwaa => "canadian syllabics twaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeTwaa => "canadian syllabics west-cree twaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiTwaa => "canadian syllabics naskapi twaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsT => "canadian syllabics t",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTte => "canadian syllabics tte",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTti => "canadian syllabics tti",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTto => "canadian syllabics tto",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTta => "canadian syllabics tta",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKe => "canadian syllabics ke",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKaai => "canadian syllabics kaai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKi => "canadian syllabics ki",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKii => "canadian syllabics kii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKo => "canadian syllabics ko",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKoo => "canadian syllabics koo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeKoo => "canadian syllabics y-cree koo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKa => "canadian syllabics ka",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKaa => "canadian syllabics kaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwe => "canadian syllabics kwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwe => "canadian syllabics west-cree kwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwi => "canadian syllabics kwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwi => "canadian syllabics west-cree kwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwii => "canadian syllabics kwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwii => "canadian syllabics west-cree kwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwo => "canadian syllabics kwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwo => "canadian syllabics west-cree kwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwoo => "canadian syllabics kwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwoo => "canadian syllabics west-cree kwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwa => "canadian syllabics kwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwa => "canadian syllabics west-cree kwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKwaa => "canadian syllabics kwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeKwaa => "canadian syllabics west-cree kwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiKwaa => "canadian syllabics naskapi kwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsK => "canadian syllabics k",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsKw => "canadian syllabics kw",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKeh => "canadian syllabics south-slavey keh",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKih => "canadian syllabics south-slavey kih",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKoh => "canadian syllabics south-slavey koh",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSouthDashSlaveyKah => "canadian syllabics south-slavey kah",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCe => "canadian syllabics ce",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCaai => "canadian syllabics caai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCi => "canadian syllabics ci",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCii => "canadian syllabics cii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCo => "canadian syllabics co",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCoo => "canadian syllabics coo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeCoo => "canadian syllabics y-cree coo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCa => "canadian syllabics ca",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCaa => "canadian syllabics caa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwe => "canadian syllabics cwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwe => "canadian syllabics west-cree cwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwi => "canadian syllabics cwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwi => "canadian syllabics west-cree cwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwii => "canadian syllabics cwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwii => "canadian syllabics west-cree cwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwo => "canadian syllabics cwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwo => "canadian syllabics west-cree cwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwoo => "canadian syllabics cwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwoo => "canadian syllabics west-cree cwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwa => "canadian syllabics cwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwa => "canadian syllabics west-cree cwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCwaa => "canadian syllabics cwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeCwaa => "canadian syllabics west-cree cwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiCwaa => "canadian syllabics naskapi cwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsC => "canadian syllabics c",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiTh => "canadian syllabics sayisi th",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMe => "canadian syllabics me",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMaai => "canadian syllabics maai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMi => "canadian syllabics mi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMii => "canadian syllabics mii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMo => "canadian syllabics mo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMoo => "canadian syllabics moo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeMoo => "canadian syllabics y-cree moo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMa => "canadian syllabics ma",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMaa => "canadian syllabics maa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwe => "canadian syllabics mwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwe => "canadian syllabics west-cree mwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwi => "canadian syllabics mwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwi => "canadian syllabics west-cree mwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwii => "canadian syllabics mwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwii => "canadian syllabics west-cree mwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwo => "canadian syllabics mwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwo => "canadian syllabics west-cree mwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwoo => "canadian syllabics mwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwoo => "canadian syllabics west-cree mwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwa => "canadian syllabics mwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwa => "canadian syllabics west-cree mwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMwaa => "canadian syllabics mwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeMwaa => "canadian syllabics west-cree mwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiMwaa => "canadian syllabics naskapi mwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsM => "canadian syllabics m",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeM => "canadian syllabics west-cree m",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMh => "canadian syllabics mh",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAthapascanM => "canadian syllabics athapascan m",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiM => "canadian syllabics sayisi m",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNe => "canadian syllabics ne",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaai => "canadian syllabics naai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNi => "canadian syllabics ni",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNii => "canadian syllabics nii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNo => "canadian syllabics no",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNoo => "canadian syllabics noo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeNoo => "canadian syllabics y-cree noo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNa => "canadian syllabics na",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaa => "canadian syllabics naa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwe => "canadian syllabics nwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwe => "canadian syllabics west-cree nwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwa => "canadian syllabics nwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwa => "canadian syllabics west-cree nwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNwaa => "canadian syllabics nwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeNwaa => "canadian syllabics west-cree nwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiNwaa => "canadian syllabics naskapi nwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsN => "canadian syllabics n",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNg => "canadian syllabics carrier ng",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNh => "canadian syllabics nh",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLe => "canadian syllabics le",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLaai => "canadian syllabics laai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLi => "canadian syllabics li",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLii => "canadian syllabics lii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLo => "canadian syllabics lo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLoo => "canadian syllabics loo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeLoo => "canadian syllabics y-cree loo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLa => "canadian syllabics la",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLaa => "canadian syllabics laa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwe => "canadian syllabics lwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwe => "canadian syllabics west-cree lwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwi => "canadian syllabics lwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwi => "canadian syllabics west-cree lwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwii => "canadian syllabics lwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwii => "canadian syllabics west-cree lwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwo => "canadian syllabics lwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwo => "canadian syllabics west-cree lwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwoo => "canadian syllabics lwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwoo => "canadian syllabics west-cree lwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwa => "canadian syllabics lwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwa => "canadian syllabics west-cree lwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLwaa => "canadian syllabics lwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLwaa => "canadian syllabics west-cree lwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsL => "canadian syllabics l",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeL => "canadian syllabics west-cree l",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMedialL => "canadian syllabics medial l",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSe => "canadian syllabics se",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSaai => "canadian syllabics saai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSi => "canadian syllabics si",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSii => "canadian syllabics sii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSo => "canadian syllabics so",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSoo => "canadian syllabics soo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeSoo => "canadian syllabics y-cree soo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSa => "canadian syllabics sa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSaa => "canadian syllabics saa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwe => "canadian syllabics swe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwe => "canadian syllabics west-cree swe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwi => "canadian syllabics swi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwi => "canadian syllabics west-cree swi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwii => "canadian syllabics swii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwii => "canadian syllabics west-cree swii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwo => "canadian syllabics swo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwo => "canadian syllabics west-cree swo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwoo => "canadian syllabics swoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwoo => "canadian syllabics west-cree swoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwa => "canadian syllabics swa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwa => "canadian syllabics west-cree swa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSwaa => "canadian syllabics swaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeSwaa => "canadian syllabics west-cree swaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSwaa => "canadian syllabics naskapi swaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsS => "canadian syllabics s",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAthapascanS => "canadian syllabics athapascan s",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSw => "canadian syllabics sw",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootS => "canadian syllabics blackfoot s",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMooseDashCreeSk => "canadian syllabics moose-cree sk",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSkw => "canadian syllabics naskapi skw",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSDashW => "canadian syllabics naskapi s-w",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSpwa => "canadian syllabics naskapi spwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiStwa => "canadian syllabics naskapi stwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiSkwa => "canadian syllabics naskapi skwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiScwa => "canadian syllabics naskapi scwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShe => "canadian syllabics she",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShi => "canadian syllabics shi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShii => "canadian syllabics shii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSho => "canadian syllabics sho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShoo => "canadian syllabics shoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSha => "canadian syllabics sha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShaa => "canadian syllabics shaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwe => "canadian syllabics shwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwe => "canadian syllabics west-cree shwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwi => "canadian syllabics shwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwi => "canadian syllabics west-cree shwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwii => "canadian syllabics shwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwii => "canadian syllabics west-cree shwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwo => "canadian syllabics shwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwo => "canadian syllabics west-cree shwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwoo => "canadian syllabics shwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwoo => "canadian syllabics west-cree shwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwa => "canadian syllabics shwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwa => "canadian syllabics west-cree shwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsShwaa => "canadian syllabics shwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeShwaa => "canadian syllabics west-cree shwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSh => "canadian syllabics sh",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYe => "canadian syllabics ye",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYaai => "canadian syllabics yaai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYi => "canadian syllabics yi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYii => "canadian syllabics yii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYo => "canadian syllabics yo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYoo => "canadian syllabics yoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYDashCreeYoo => "canadian syllabics y-cree yoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYa => "canadian syllabics ya",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYaa => "canadian syllabics yaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwe => "canadian syllabics ywe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwe => "canadian syllabics west-cree ywe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwi => "canadian syllabics ywi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwi => "canadian syllabics west-cree ywi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwii => "canadian syllabics ywii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwii => "canadian syllabics west-cree ywii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwo => "canadian syllabics ywo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwo => "canadian syllabics west-cree ywo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwoo => "canadian syllabics ywoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwoo => "canadian syllabics west-cree ywoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwa => "canadian syllabics ywa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwa => "canadian syllabics west-cree ywa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsYwaa => "canadian syllabics ywaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeYwaa => "canadian syllabics west-cree ywaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNaskapiYwaa => "canadian syllabics naskapi ywaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsY => "canadian syllabics y",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBibleDashCreeY => "canadian syllabics bible-cree y",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeY => "canadian syllabics west-cree y",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiYi => "canadian syllabics sayisi yi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRe => "canadian syllabics re",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRDashCreeRe => "canadian syllabics r-cree re",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLe => "canadian syllabics west-cree le",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRaai => "canadian syllabics raai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRi => "canadian syllabics ri",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRii => "canadian syllabics rii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRo => "canadian syllabics ro",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRoo => "canadian syllabics roo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLo => "canadian syllabics west-cree lo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRa => "canadian syllabics ra",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRaa => "canadian syllabics raa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeLa => "canadian syllabics west-cree la",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsRwaa => "canadian syllabics rwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRwaa => "canadian syllabics west-cree rwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsR => "canadian syllabics r",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeR => "canadian syllabics west-cree r",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsMedialR => "canadian syllabics medial r",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFe => "canadian syllabics fe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFaai => "canadian syllabics faai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFi => "canadian syllabics fi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFii => "canadian syllabics fii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFo => "canadian syllabics fo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFoo => "canadian syllabics foo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFa => "canadian syllabics fa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFaa => "canadian syllabics faa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFwaa => "canadian syllabics fwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeFwaa => "canadian syllabics west-cree fwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsF => "canadian syllabics f",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThe => "canadian syllabics the",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThe => "canadian syllabics n-cree the",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThi => "canadian syllabics thi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThi => "canadian syllabics n-cree thi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThii => "canadian syllabics thii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNDashCreeThii => "canadian syllabics n-cree thii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTho => "canadian syllabics tho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThoo => "canadian syllabics thoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTha => "canadian syllabics tha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThaa => "canadian syllabics thaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThwaa => "canadian syllabics thwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeThwaa => "canadian syllabics west-cree thwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTh => "canadian syllabics th",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTthe => "canadian syllabics tthe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTthi => "canadian syllabics tthi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTtho => "canadian syllabics ttho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTtha => "canadian syllabics ttha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTth => "canadian syllabics tth",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTye => "canadian syllabics tye",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTyi => "canadian syllabics tyi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTyo => "canadian syllabics tyo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTya => "canadian syllabics tya",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHe => "canadian syllabics nunavik he",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHi => "canadian syllabics nunavik hi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHii => "canadian syllabics nunavik hii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHo => "canadian syllabics nunavik ho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHoo => "canadian syllabics nunavik hoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHa => "canadian syllabics nunavik ha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikHaa => "canadian syllabics nunavik haa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavikH => "canadian syllabics nunavik h",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNunavutH => "canadian syllabics nunavut h",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsHk => "canadian syllabics hk",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQaai => "canadian syllabics qaai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQi => "canadian syllabics qi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQii => "canadian syllabics qii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQo => "canadian syllabics qo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQoo => "canadian syllabics qoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQa => "canadian syllabics qa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQaa => "canadian syllabics qaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQ => "canadian syllabics q",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlhe => "canadian syllabics tlhe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlhi => "canadian syllabics tlhi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlho => "canadian syllabics tlho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsTlha => "canadian syllabics tlha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRe => "canadian syllabics west-cree re",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRi => "canadian syllabics west-cree ri",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRo => "canadian syllabics west-cree ro",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWestDashCreeRa => "canadian syllabics west-cree ra",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgaai => "canadian syllabics ngaai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgi => "canadian syllabics ngi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgii => "canadian syllabics ngii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgo => "canadian syllabics ngo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgoo => "canadian syllabics ngoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNga => "canadian syllabics nga",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgaa => "canadian syllabics ngaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNg => "canadian syllabics ng",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNng => "canadian syllabics nng",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiShe => "canadian syllabics sayisi she",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiShi => "canadian syllabics sayisi shi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiSho => "canadian syllabics sayisi sho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiSha => "canadian syllabics sayisi sha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThe => "canadian syllabics woods-cree the",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThi => "canadian syllabics woods-cree thi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTho => "canadian syllabics woods-cree tho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTha => "canadian syllabics woods-cree tha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeTh => "canadian syllabics woods-cree th",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhi => "canadian syllabics lhi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhii => "canadian syllabics lhii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLho => "canadian syllabics lho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhoo => "canadian syllabics lhoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLha => "canadian syllabics lha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLhaa => "canadian syllabics lhaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsLh => "canadian syllabics lh",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThe => "canadian syllabics th-cree the",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThi => "canadian syllabics th-cree thi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThii => "canadian syllabics th-cree thii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTho => "canadian syllabics th-cree tho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThoo => "canadian syllabics th-cree thoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTha => "canadian syllabics th-cree tha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeThaa => "canadian syllabics th-cree thaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsThDashCreeTh => "canadian syllabics th-cree th",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsAivilikB => "canadian syllabics aivilik b",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootE => "canadian syllabics blackfoot e",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootI => "canadian syllabics blackfoot i",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootO => "canadian syllabics blackfoot o",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootA => "canadian syllabics blackfoot a",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWe => "canadian syllabics blackfoot we",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWi => "canadian syllabics blackfoot wi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWo => "canadian syllabics blackfoot wo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootWa => "canadian syllabics blackfoot wa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNe => "canadian syllabics blackfoot ne",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNi => "canadian syllabics blackfoot ni",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNo => "canadian syllabics blackfoot no",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootNa => "canadian syllabics blackfoot na",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKe => "canadian syllabics blackfoot ke",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKi => "canadian syllabics blackfoot ki",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKo => "canadian syllabics blackfoot ko",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsBlackfootKa => "canadian syllabics blackfoot ka",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHe => "canadian syllabics sayisi he",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHi => "canadian syllabics sayisi hi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHo => "canadian syllabics sayisi ho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiHa => "canadian syllabics sayisi ha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhu => "canadian syllabics carrier ghu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGho => "canadian syllabics carrier gho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhe => "canadian syllabics carrier ghe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhee => "canadian syllabics carrier ghee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGhi => "canadian syllabics carrier ghi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGha => "canadian syllabics carrier gha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRu => "canadian syllabics carrier ru",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRo => "canadian syllabics carrier ro",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRe => "canadian syllabics carrier re",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRee => "canadian syllabics carrier ree",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRi => "canadian syllabics carrier ri",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierRa => "canadian syllabics carrier ra",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWu => "canadian syllabics carrier wu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWo => "canadian syllabics carrier wo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWe => "canadian syllabics carrier we",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWee => "canadian syllabics carrier wee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWi => "canadian syllabics carrier wi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierWa => "canadian syllabics carrier wa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwu => "canadian syllabics carrier hwu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwo => "canadian syllabics carrier hwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwe => "canadian syllabics carrier hwe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwee => "canadian syllabics carrier hwee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwi => "canadian syllabics carrier hwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierHwa => "canadian syllabics carrier hwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThu => "canadian syllabics carrier thu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTho => "canadian syllabics carrier tho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThe => "canadian syllabics carrier the",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThee => "canadian syllabics carrier thee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierThi => "canadian syllabics carrier thi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTha => "canadian syllabics carrier tha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtu => "canadian syllabics carrier ttu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTto => "canadian syllabics carrier tto",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTte => "canadian syllabics carrier tte",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtee => "canadian syllabics carrier ttee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTti => "canadian syllabics carrier tti",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTta => "canadian syllabics carrier tta",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPu => "canadian syllabics carrier pu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPo => "canadian syllabics carrier po",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPe => "canadian syllabics carrier pe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPee => "canadian syllabics carrier pee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPi => "canadian syllabics carrier pi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierPa => "canadian syllabics carrier pa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierP => "canadian syllabics carrier p",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGu => "canadian syllabics carrier gu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGo => "canadian syllabics carrier go",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGe => "canadian syllabics carrier ge",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGee => "canadian syllabics carrier gee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGi => "canadian syllabics carrier gi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierGa => "canadian syllabics carrier ga",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhu => "canadian syllabics carrier khu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKho => "canadian syllabics carrier kho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhe => "canadian syllabics carrier khe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhee => "canadian syllabics carrier khee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKhi => "canadian syllabics carrier khi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKha => "canadian syllabics carrier kha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKku => "canadian syllabics carrier kku",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKko => "canadian syllabics carrier kko",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKke => "canadian syllabics carrier kke",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKkee => "canadian syllabics carrier kkee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKki => "canadian syllabics carrier kki",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKka => "canadian syllabics carrier kka",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierKk => "canadian syllabics carrier kk",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNu => "canadian syllabics carrier nu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNo => "canadian syllabics carrier no",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNe => "canadian syllabics carrier ne",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNee => "canadian syllabics carrier nee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNi => "canadian syllabics carrier ni",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierNa => "canadian syllabics carrier na",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMu => "canadian syllabics carrier mu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMo => "canadian syllabics carrier mo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMe => "canadian syllabics carrier me",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMee => "canadian syllabics carrier mee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMi => "canadian syllabics carrier mi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierMa => "canadian syllabics carrier ma",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYu => "canadian syllabics carrier yu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYo => "canadian syllabics carrier yo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYe => "canadian syllabics carrier ye",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYee => "canadian syllabics carrier yee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYi => "canadian syllabics carrier yi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierYa => "canadian syllabics carrier ya",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJu => "canadian syllabics carrier ju",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiJu => "canadian syllabics sayisi ju",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJo => "canadian syllabics carrier jo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJe => "canadian syllabics carrier je",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJee => "canadian syllabics carrier jee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJi => "canadian syllabics carrier ji",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsSayisiJi => "canadian syllabics sayisi ji",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJa => "canadian syllabics carrier ja",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJju => "canadian syllabics carrier jju",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJjo => "canadian syllabics carrier jjo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJje => "canadian syllabics carrier jje",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJjee => "canadian syllabics carrier jjee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJji => "canadian syllabics carrier jji",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierJja => "canadian syllabics carrier jja",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLu => "canadian syllabics carrier lu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLo => "canadian syllabics carrier lo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLe => "canadian syllabics carrier le",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLee => "canadian syllabics carrier lee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLi => "canadian syllabics carrier li",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLa => "canadian syllabics carrier la",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlu => "canadian syllabics carrier dlu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlo => "canadian syllabics carrier dlo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDle => "canadian syllabics carrier dle",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDlee => "canadian syllabics carrier dlee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDli => "canadian syllabics carrier dli",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDla => "canadian syllabics carrier dla",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhu => "canadian syllabics carrier lhu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLho => "canadian syllabics carrier lho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhe => "canadian syllabics carrier lhe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhee => "canadian syllabics carrier lhee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLhi => "canadian syllabics carrier lhi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierLha => "canadian syllabics carrier lha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhu => "canadian syllabics carrier tlhu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlho => "canadian syllabics carrier tlho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhe => "canadian syllabics carrier tlhe",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhee => "canadian syllabics carrier tlhee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlhi => "canadian syllabics carrier tlhi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlha => "canadian syllabics carrier tlha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlu => "canadian syllabics carrier tlu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlo => "canadian syllabics carrier tlo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTle => "canadian syllabics carrier tle",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTlee => "canadian syllabics carrier tlee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTli => "canadian syllabics carrier tli",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTla => "canadian syllabics carrier tla",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZu => "canadian syllabics carrier zu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZo => "canadian syllabics carrier zo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZe => "canadian syllabics carrier ze",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZee => "canadian syllabics carrier zee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZi => "canadian syllabics carrier zi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZa => "canadian syllabics carrier za",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierZ => "canadian syllabics carrier z",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierInitialZ => "canadian syllabics carrier initial z",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzu => "canadian syllabics carrier dzu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzo => "canadian syllabics carrier dzo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDze => "canadian syllabics carrier dze",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzee => "canadian syllabics carrier dzee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDzi => "canadian syllabics carrier dzi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierDza => "canadian syllabics carrier dza",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSu => "canadian syllabics carrier su",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSo => "canadian syllabics carrier so",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSe => "canadian syllabics carrier se",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSee => "canadian syllabics carrier see",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSi => "canadian syllabics carrier si",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSa => "canadian syllabics carrier sa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShu => "canadian syllabics carrier shu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSho => "canadian syllabics carrier sho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShe => "canadian syllabics carrier she",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShee => "canadian syllabics carrier shee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierShi => "canadian syllabics carrier shi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSha => "canadian syllabics carrier sha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierSh => "canadian syllabics carrier sh",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsu => "canadian syllabics carrier tsu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTso => "canadian syllabics carrier tso",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTse => "canadian syllabics carrier tse",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsee => "canadian syllabics carrier tsee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsi => "canadian syllabics carrier tsi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTsa => "canadian syllabics carrier tsa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChu => "canadian syllabics carrier chu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierCho => "canadian syllabics carrier cho",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChe => "canadian syllabics carrier che",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChee => "canadian syllabics carrier chee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierChi => "canadian syllabics carrier chi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierCha => "canadian syllabics carrier cha",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsu => "canadian syllabics carrier ttsu",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtso => "canadian syllabics carrier ttso",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtse => "canadian syllabics carrier ttse",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsee => "canadian syllabics carrier ttsee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsi => "canadian syllabics carrier ttsi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsCarrierTtsa => "canadian syllabics carrier ttsa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsChiSign => "canadian syllabics chi sign",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsFullStop => "canadian syllabics full stop",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsQai => "canadian syllabics qai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNgai => "canadian syllabics ngai",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngi => "canadian syllabics nngi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngii => "canadian syllabics nngii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngo => "canadian syllabics nngo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngoo => "canadian syllabics nngoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNnga => "canadian syllabics nnga",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsNngaa => "canadian syllabics nngaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwee => "canadian syllabics woods-cree thwee",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwi => "canadian syllabics woods-cree thwi",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwii => "canadian syllabics woods-cree thwii",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwo => "canadian syllabics woods-cree thwo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwoo => "canadian syllabics woods-cree thwoo",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwa => "canadian syllabics woods-cree thwa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeThwaa => "canadian syllabics woods-cree thwaa",
            UnifiedCanadianAboriginalSyllabics::CanadianSyllabicsWoodsDashCreeFinalTh => "canadian syllabics woods-cree final th",
        }
    }
}
