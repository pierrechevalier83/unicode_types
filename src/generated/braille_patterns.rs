/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{2800}: '⠀'
    pub const BRAILLE_PATTERN_BLANK: char = '⠀';
    /// \u{2801}: '⠁'
    pub const BRAILLE_PATTERN_DOTS_DASH_1: char = '⠁';
    /// \u{2802}: '⠂'
    pub const BRAILLE_PATTERN_DOTS_DASH_2: char = '⠂';
    /// \u{2803}: '⠃'
    pub const BRAILLE_PATTERN_DOTS_DASH_12: char = '⠃';
    /// \u{2804}: '⠄'
    pub const BRAILLE_PATTERN_DOTS_DASH_3: char = '⠄';
    /// \u{2805}: '⠅'
    pub const BRAILLE_PATTERN_DOTS_DASH_13: char = '⠅';
    /// \u{2806}: '⠆'
    pub const BRAILLE_PATTERN_DOTS_DASH_23: char = '⠆';
    /// \u{2807}: '⠇'
    pub const BRAILLE_PATTERN_DOTS_DASH_123: char = '⠇';
    /// \u{2808}: '⠈'
    pub const BRAILLE_PATTERN_DOTS_DASH_4: char = '⠈';
    /// \u{2809}: '⠉'
    pub const BRAILLE_PATTERN_DOTS_DASH_14: char = '⠉';
    /// \u{280a}: '⠊'
    pub const BRAILLE_PATTERN_DOTS_DASH_24: char = '⠊';
    /// \u{280b}: '⠋'
    pub const BRAILLE_PATTERN_DOTS_DASH_124: char = '⠋';
    /// \u{280c}: '⠌'
    pub const BRAILLE_PATTERN_DOTS_DASH_34: char = '⠌';
    /// \u{280d}: '⠍'
    pub const BRAILLE_PATTERN_DOTS_DASH_134: char = '⠍';
    /// \u{280e}: '⠎'
    pub const BRAILLE_PATTERN_DOTS_DASH_234: char = '⠎';
    /// \u{280f}: '⠏'
    pub const BRAILLE_PATTERN_DOTS_DASH_1234: char = '⠏';
    /// \u{2810}: '⠐'
    pub const BRAILLE_PATTERN_DOTS_DASH_5: char = '⠐';
    /// \u{2811}: '⠑'
    pub const BRAILLE_PATTERN_DOTS_DASH_15: char = '⠑';
    /// \u{2812}: '⠒'
    pub const BRAILLE_PATTERN_DOTS_DASH_25: char = '⠒';
    /// \u{2813}: '⠓'
    pub const BRAILLE_PATTERN_DOTS_DASH_125: char = '⠓';
    /// \u{2814}: '⠔'
    pub const BRAILLE_PATTERN_DOTS_DASH_35: char = '⠔';
    /// \u{2815}: '⠕'
    pub const BRAILLE_PATTERN_DOTS_DASH_135: char = '⠕';
    /// \u{2816}: '⠖'
    pub const BRAILLE_PATTERN_DOTS_DASH_235: char = '⠖';
    /// \u{2817}: '⠗'
    pub const BRAILLE_PATTERN_DOTS_DASH_1235: char = '⠗';
    /// \u{2818}: '⠘'
    pub const BRAILLE_PATTERN_DOTS_DASH_45: char = '⠘';
    /// \u{2819}: '⠙'
    pub const BRAILLE_PATTERN_DOTS_DASH_145: char = '⠙';
    /// \u{281a}: '⠚'
    pub const BRAILLE_PATTERN_DOTS_DASH_245: char = '⠚';
    /// \u{281b}: '⠛'
    pub const BRAILLE_PATTERN_DOTS_DASH_1245: char = '⠛';
    /// \u{281c}: '⠜'
    pub const BRAILLE_PATTERN_DOTS_DASH_345: char = '⠜';
    /// \u{281d}: '⠝'
    pub const BRAILLE_PATTERN_DOTS_DASH_1345: char = '⠝';
    /// \u{281e}: '⠞'
    pub const BRAILLE_PATTERN_DOTS_DASH_2345: char = '⠞';
    /// \u{281f}: '⠟'
    pub const BRAILLE_PATTERN_DOTS_DASH_12345: char = '⠟';
    /// \u{2820}: '⠠'
    pub const BRAILLE_PATTERN_DOTS_DASH_6: char = '⠠';
    /// \u{2821}: '⠡'
    pub const BRAILLE_PATTERN_DOTS_DASH_16: char = '⠡';
    /// \u{2822}: '⠢'
    pub const BRAILLE_PATTERN_DOTS_DASH_26: char = '⠢';
    /// \u{2823}: '⠣'
    pub const BRAILLE_PATTERN_DOTS_DASH_126: char = '⠣';
    /// \u{2824}: '⠤'
    pub const BRAILLE_PATTERN_DOTS_DASH_36: char = '⠤';
    /// \u{2825}: '⠥'
    pub const BRAILLE_PATTERN_DOTS_DASH_136: char = '⠥';
    /// \u{2826}: '⠦'
    pub const BRAILLE_PATTERN_DOTS_DASH_236: char = '⠦';
    /// \u{2827}: '⠧'
    pub const BRAILLE_PATTERN_DOTS_DASH_1236: char = '⠧';
    /// \u{2828}: '⠨'
    pub const BRAILLE_PATTERN_DOTS_DASH_46: char = '⠨';
    /// \u{2829}: '⠩'
    pub const BRAILLE_PATTERN_DOTS_DASH_146: char = '⠩';
    /// \u{282a}: '⠪'
    pub const BRAILLE_PATTERN_DOTS_DASH_246: char = '⠪';
    /// \u{282b}: '⠫'
    pub const BRAILLE_PATTERN_DOTS_DASH_1246: char = '⠫';
    /// \u{282c}: '⠬'
    pub const BRAILLE_PATTERN_DOTS_DASH_346: char = '⠬';
    /// \u{282d}: '⠭'
    pub const BRAILLE_PATTERN_DOTS_DASH_1346: char = '⠭';
    /// \u{282e}: '⠮'
    pub const BRAILLE_PATTERN_DOTS_DASH_2346: char = '⠮';
    /// \u{282f}: '⠯'
    pub const BRAILLE_PATTERN_DOTS_DASH_12346: char = '⠯';
    /// \u{2830}: '⠰'
    pub const BRAILLE_PATTERN_DOTS_DASH_56: char = '⠰';
    /// \u{2831}: '⠱'
    pub const BRAILLE_PATTERN_DOTS_DASH_156: char = '⠱';
    /// \u{2832}: '⠲'
    pub const BRAILLE_PATTERN_DOTS_DASH_256: char = '⠲';
    /// \u{2833}: '⠳'
    pub const BRAILLE_PATTERN_DOTS_DASH_1256: char = '⠳';
    /// \u{2834}: '⠴'
    pub const BRAILLE_PATTERN_DOTS_DASH_356: char = '⠴';
    /// \u{2835}: '⠵'
    pub const BRAILLE_PATTERN_DOTS_DASH_1356: char = '⠵';
    /// \u{2836}: '⠶'
    pub const BRAILLE_PATTERN_DOTS_DASH_2356: char = '⠶';
    /// \u{2837}: '⠷'
    pub const BRAILLE_PATTERN_DOTS_DASH_12356: char = '⠷';
    /// \u{2838}: '⠸'
    pub const BRAILLE_PATTERN_DOTS_DASH_456: char = '⠸';
    /// \u{2839}: '⠹'
    pub const BRAILLE_PATTERN_DOTS_DASH_1456: char = '⠹';
    /// \u{283a}: '⠺'
    pub const BRAILLE_PATTERN_DOTS_DASH_2456: char = '⠺';
    /// \u{283b}: '⠻'
    pub const BRAILLE_PATTERN_DOTS_DASH_12456: char = '⠻';
    /// \u{283c}: '⠼'
    pub const BRAILLE_PATTERN_DOTS_DASH_3456: char = '⠼';
    /// \u{283d}: '⠽'
    pub const BRAILLE_PATTERN_DOTS_DASH_13456: char = '⠽';
    /// \u{283e}: '⠾'
    pub const BRAILLE_PATTERN_DOTS_DASH_23456: char = '⠾';
    /// \u{283f}: '⠿'
    pub const BRAILLE_PATTERN_DOTS_DASH_123456: char = '⠿';
    /// \u{2840}: '⡀'
    pub const BRAILLE_PATTERN_DOTS_DASH_7: char = '⡀';
    /// \u{2841}: '⡁'
    pub const BRAILLE_PATTERN_DOTS_DASH_17: char = '⡁';
    /// \u{2842}: '⡂'
    pub const BRAILLE_PATTERN_DOTS_DASH_27: char = '⡂';
    /// \u{2843}: '⡃'
    pub const BRAILLE_PATTERN_DOTS_DASH_127: char = '⡃';
    /// \u{2844}: '⡄'
    pub const BRAILLE_PATTERN_DOTS_DASH_37: char = '⡄';
    /// \u{2845}: '⡅'
    pub const BRAILLE_PATTERN_DOTS_DASH_137: char = '⡅';
    /// \u{2846}: '⡆'
    pub const BRAILLE_PATTERN_DOTS_DASH_237: char = '⡆';
    /// \u{2847}: '⡇'
    pub const BRAILLE_PATTERN_DOTS_DASH_1237: char = '⡇';
    /// \u{2848}: '⡈'
    pub const BRAILLE_PATTERN_DOTS_DASH_47: char = '⡈';
    /// \u{2849}: '⡉'
    pub const BRAILLE_PATTERN_DOTS_DASH_147: char = '⡉';
    /// \u{284a}: '⡊'
    pub const BRAILLE_PATTERN_DOTS_DASH_247: char = '⡊';
    /// \u{284b}: '⡋'
    pub const BRAILLE_PATTERN_DOTS_DASH_1247: char = '⡋';
    /// \u{284c}: '⡌'
    pub const BRAILLE_PATTERN_DOTS_DASH_347: char = '⡌';
    /// \u{284d}: '⡍'
    pub const BRAILLE_PATTERN_DOTS_DASH_1347: char = '⡍';
    /// \u{284e}: '⡎'
    pub const BRAILLE_PATTERN_DOTS_DASH_2347: char = '⡎';
    /// \u{284f}: '⡏'
    pub const BRAILLE_PATTERN_DOTS_DASH_12347: char = '⡏';
    /// \u{2850}: '⡐'
    pub const BRAILLE_PATTERN_DOTS_DASH_57: char = '⡐';
    /// \u{2851}: '⡑'
    pub const BRAILLE_PATTERN_DOTS_DASH_157: char = '⡑';
    /// \u{2852}: '⡒'
    pub const BRAILLE_PATTERN_DOTS_DASH_257: char = '⡒';
    /// \u{2853}: '⡓'
    pub const BRAILLE_PATTERN_DOTS_DASH_1257: char = '⡓';
    /// \u{2854}: '⡔'
    pub const BRAILLE_PATTERN_DOTS_DASH_357: char = '⡔';
    /// \u{2855}: '⡕'
    pub const BRAILLE_PATTERN_DOTS_DASH_1357: char = '⡕';
    /// \u{2856}: '⡖'
    pub const BRAILLE_PATTERN_DOTS_DASH_2357: char = '⡖';
    /// \u{2857}: '⡗'
    pub const BRAILLE_PATTERN_DOTS_DASH_12357: char = '⡗';
    /// \u{2858}: '⡘'
    pub const BRAILLE_PATTERN_DOTS_DASH_457: char = '⡘';
    /// \u{2859}: '⡙'
    pub const BRAILLE_PATTERN_DOTS_DASH_1457: char = '⡙';
    /// \u{285a}: '⡚'
    pub const BRAILLE_PATTERN_DOTS_DASH_2457: char = '⡚';
    /// \u{285b}: '⡛'
    pub const BRAILLE_PATTERN_DOTS_DASH_12457: char = '⡛';
    /// \u{285c}: '⡜'
    pub const BRAILLE_PATTERN_DOTS_DASH_3457: char = '⡜';
    /// \u{285d}: '⡝'
    pub const BRAILLE_PATTERN_DOTS_DASH_13457: char = '⡝';
    /// \u{285e}: '⡞'
    pub const BRAILLE_PATTERN_DOTS_DASH_23457: char = '⡞';
    /// \u{285f}: '⡟'
    pub const BRAILLE_PATTERN_DOTS_DASH_123457: char = '⡟';
    /// \u{2860}: '⡠'
    pub const BRAILLE_PATTERN_DOTS_DASH_67: char = '⡠';
    /// \u{2861}: '⡡'
    pub const BRAILLE_PATTERN_DOTS_DASH_167: char = '⡡';
    /// \u{2862}: '⡢'
    pub const BRAILLE_PATTERN_DOTS_DASH_267: char = '⡢';
    /// \u{2863}: '⡣'
    pub const BRAILLE_PATTERN_DOTS_DASH_1267: char = '⡣';
    /// \u{2864}: '⡤'
    pub const BRAILLE_PATTERN_DOTS_DASH_367: char = '⡤';
    /// \u{2865}: '⡥'
    pub const BRAILLE_PATTERN_DOTS_DASH_1367: char = '⡥';
    /// \u{2866}: '⡦'
    pub const BRAILLE_PATTERN_DOTS_DASH_2367: char = '⡦';
    /// \u{2867}: '⡧'
    pub const BRAILLE_PATTERN_DOTS_DASH_12367: char = '⡧';
    /// \u{2868}: '⡨'
    pub const BRAILLE_PATTERN_DOTS_DASH_467: char = '⡨';
    /// \u{2869}: '⡩'
    pub const BRAILLE_PATTERN_DOTS_DASH_1467: char = '⡩';
    /// \u{286a}: '⡪'
    pub const BRAILLE_PATTERN_DOTS_DASH_2467: char = '⡪';
    /// \u{286b}: '⡫'
    pub const BRAILLE_PATTERN_DOTS_DASH_12467: char = '⡫';
    /// \u{286c}: '⡬'
    pub const BRAILLE_PATTERN_DOTS_DASH_3467: char = '⡬';
    /// \u{286d}: '⡭'
    pub const BRAILLE_PATTERN_DOTS_DASH_13467: char = '⡭';
    /// \u{286e}: '⡮'
    pub const BRAILLE_PATTERN_DOTS_DASH_23467: char = '⡮';
    /// \u{286f}: '⡯'
    pub const BRAILLE_PATTERN_DOTS_DASH_123467: char = '⡯';
    /// \u{2870}: '⡰'
    pub const BRAILLE_PATTERN_DOTS_DASH_567: char = '⡰';
    /// \u{2871}: '⡱'
    pub const BRAILLE_PATTERN_DOTS_DASH_1567: char = '⡱';
    /// \u{2872}: '⡲'
    pub const BRAILLE_PATTERN_DOTS_DASH_2567: char = '⡲';
    /// \u{2873}: '⡳'
    pub const BRAILLE_PATTERN_DOTS_DASH_12567: char = '⡳';
    /// \u{2874}: '⡴'
    pub const BRAILLE_PATTERN_DOTS_DASH_3567: char = '⡴';
    /// \u{2875}: '⡵'
    pub const BRAILLE_PATTERN_DOTS_DASH_13567: char = '⡵';
    /// \u{2876}: '⡶'
    pub const BRAILLE_PATTERN_DOTS_DASH_23567: char = '⡶';
    /// \u{2877}: '⡷'
    pub const BRAILLE_PATTERN_DOTS_DASH_123567: char = '⡷';
    /// \u{2878}: '⡸'
    pub const BRAILLE_PATTERN_DOTS_DASH_4567: char = '⡸';
    /// \u{2879}: '⡹'
    pub const BRAILLE_PATTERN_DOTS_DASH_14567: char = '⡹';
    /// \u{287a}: '⡺'
    pub const BRAILLE_PATTERN_DOTS_DASH_24567: char = '⡺';
    /// \u{287b}: '⡻'
    pub const BRAILLE_PATTERN_DOTS_DASH_124567: char = '⡻';
    /// \u{287c}: '⡼'
    pub const BRAILLE_PATTERN_DOTS_DASH_34567: char = '⡼';
    /// \u{287d}: '⡽'
    pub const BRAILLE_PATTERN_DOTS_DASH_134567: char = '⡽';
    /// \u{287e}: '⡾'
    pub const BRAILLE_PATTERN_DOTS_DASH_234567: char = '⡾';
    /// \u{287f}: '⡿'
    pub const BRAILLE_PATTERN_DOTS_DASH_1234567: char = '⡿';
    /// \u{2880}: '⢀'
    pub const BRAILLE_PATTERN_DOTS_DASH_8: char = '⢀';
    /// \u{2881}: '⢁'
    pub const BRAILLE_PATTERN_DOTS_DASH_18: char = '⢁';
    /// \u{2882}: '⢂'
    pub const BRAILLE_PATTERN_DOTS_DASH_28: char = '⢂';
    /// \u{2883}: '⢃'
    pub const BRAILLE_PATTERN_DOTS_DASH_128: char = '⢃';
    /// \u{2884}: '⢄'
    pub const BRAILLE_PATTERN_DOTS_DASH_38: char = '⢄';
    /// \u{2885}: '⢅'
    pub const BRAILLE_PATTERN_DOTS_DASH_138: char = '⢅';
    /// \u{2886}: '⢆'
    pub const BRAILLE_PATTERN_DOTS_DASH_238: char = '⢆';
    /// \u{2887}: '⢇'
    pub const BRAILLE_PATTERN_DOTS_DASH_1238: char = '⢇';
    /// \u{2888}: '⢈'
    pub const BRAILLE_PATTERN_DOTS_DASH_48: char = '⢈';
    /// \u{2889}: '⢉'
    pub const BRAILLE_PATTERN_DOTS_DASH_148: char = '⢉';
    /// \u{288a}: '⢊'
    pub const BRAILLE_PATTERN_DOTS_DASH_248: char = '⢊';
    /// \u{288b}: '⢋'
    pub const BRAILLE_PATTERN_DOTS_DASH_1248: char = '⢋';
    /// \u{288c}: '⢌'
    pub const BRAILLE_PATTERN_DOTS_DASH_348: char = '⢌';
    /// \u{288d}: '⢍'
    pub const BRAILLE_PATTERN_DOTS_DASH_1348: char = '⢍';
    /// \u{288e}: '⢎'
    pub const BRAILLE_PATTERN_DOTS_DASH_2348: char = '⢎';
    /// \u{288f}: '⢏'
    pub const BRAILLE_PATTERN_DOTS_DASH_12348: char = '⢏';
    /// \u{2890}: '⢐'
    pub const BRAILLE_PATTERN_DOTS_DASH_58: char = '⢐';
    /// \u{2891}: '⢑'
    pub const BRAILLE_PATTERN_DOTS_DASH_158: char = '⢑';
    /// \u{2892}: '⢒'
    pub const BRAILLE_PATTERN_DOTS_DASH_258: char = '⢒';
    /// \u{2893}: '⢓'
    pub const BRAILLE_PATTERN_DOTS_DASH_1258: char = '⢓';
    /// \u{2894}: '⢔'
    pub const BRAILLE_PATTERN_DOTS_DASH_358: char = '⢔';
    /// \u{2895}: '⢕'
    pub const BRAILLE_PATTERN_DOTS_DASH_1358: char = '⢕';
    /// \u{2896}: '⢖'
    pub const BRAILLE_PATTERN_DOTS_DASH_2358: char = '⢖';
    /// \u{2897}: '⢗'
    pub const BRAILLE_PATTERN_DOTS_DASH_12358: char = '⢗';
    /// \u{2898}: '⢘'
    pub const BRAILLE_PATTERN_DOTS_DASH_458: char = '⢘';
    /// \u{2899}: '⢙'
    pub const BRAILLE_PATTERN_DOTS_DASH_1458: char = '⢙';
    /// \u{289a}: '⢚'
    pub const BRAILLE_PATTERN_DOTS_DASH_2458: char = '⢚';
    /// \u{289b}: '⢛'
    pub const BRAILLE_PATTERN_DOTS_DASH_12458: char = '⢛';
    /// \u{289c}: '⢜'
    pub const BRAILLE_PATTERN_DOTS_DASH_3458: char = '⢜';
    /// \u{289d}: '⢝'
    pub const BRAILLE_PATTERN_DOTS_DASH_13458: char = '⢝';
    /// \u{289e}: '⢞'
    pub const BRAILLE_PATTERN_DOTS_DASH_23458: char = '⢞';
    /// \u{289f}: '⢟'
    pub const BRAILLE_PATTERN_DOTS_DASH_123458: char = '⢟';
    /// \u{28a0}: '⢠'
    pub const BRAILLE_PATTERN_DOTS_DASH_68: char = '⢠';
    /// \u{28a1}: '⢡'
    pub const BRAILLE_PATTERN_DOTS_DASH_168: char = '⢡';
    /// \u{28a2}: '⢢'
    pub const BRAILLE_PATTERN_DOTS_DASH_268: char = '⢢';
    /// \u{28a3}: '⢣'
    pub const BRAILLE_PATTERN_DOTS_DASH_1268: char = '⢣';
    /// \u{28a4}: '⢤'
    pub const BRAILLE_PATTERN_DOTS_DASH_368: char = '⢤';
    /// \u{28a5}: '⢥'
    pub const BRAILLE_PATTERN_DOTS_DASH_1368: char = '⢥';
    /// \u{28a6}: '⢦'
    pub const BRAILLE_PATTERN_DOTS_DASH_2368: char = '⢦';
    /// \u{28a7}: '⢧'
    pub const BRAILLE_PATTERN_DOTS_DASH_12368: char = '⢧';
    /// \u{28a8}: '⢨'
    pub const BRAILLE_PATTERN_DOTS_DASH_468: char = '⢨';
    /// \u{28a9}: '⢩'
    pub const BRAILLE_PATTERN_DOTS_DASH_1468: char = '⢩';
    /// \u{28aa}: '⢪'
    pub const BRAILLE_PATTERN_DOTS_DASH_2468: char = '⢪';
    /// \u{28ab}: '⢫'
    pub const BRAILLE_PATTERN_DOTS_DASH_12468: char = '⢫';
    /// \u{28ac}: '⢬'
    pub const BRAILLE_PATTERN_DOTS_DASH_3468: char = '⢬';
    /// \u{28ad}: '⢭'
    pub const BRAILLE_PATTERN_DOTS_DASH_13468: char = '⢭';
    /// \u{28ae}: '⢮'
    pub const BRAILLE_PATTERN_DOTS_DASH_23468: char = '⢮';
    /// \u{28af}: '⢯'
    pub const BRAILLE_PATTERN_DOTS_DASH_123468: char = '⢯';
    /// \u{28b0}: '⢰'
    pub const BRAILLE_PATTERN_DOTS_DASH_568: char = '⢰';
    /// \u{28b1}: '⢱'
    pub const BRAILLE_PATTERN_DOTS_DASH_1568: char = '⢱';
    /// \u{28b2}: '⢲'
    pub const BRAILLE_PATTERN_DOTS_DASH_2568: char = '⢲';
    /// \u{28b3}: '⢳'
    pub const BRAILLE_PATTERN_DOTS_DASH_12568: char = '⢳';
    /// \u{28b4}: '⢴'
    pub const BRAILLE_PATTERN_DOTS_DASH_3568: char = '⢴';
    /// \u{28b5}: '⢵'
    pub const BRAILLE_PATTERN_DOTS_DASH_13568: char = '⢵';
    /// \u{28b6}: '⢶'
    pub const BRAILLE_PATTERN_DOTS_DASH_23568: char = '⢶';
    /// \u{28b7}: '⢷'
    pub const BRAILLE_PATTERN_DOTS_DASH_123568: char = '⢷';
    /// \u{28b8}: '⢸'
    pub const BRAILLE_PATTERN_DOTS_DASH_4568: char = '⢸';
    /// \u{28b9}: '⢹'
    pub const BRAILLE_PATTERN_DOTS_DASH_14568: char = '⢹';
    /// \u{28ba}: '⢺'
    pub const BRAILLE_PATTERN_DOTS_DASH_24568: char = '⢺';
    /// \u{28bb}: '⢻'
    pub const BRAILLE_PATTERN_DOTS_DASH_124568: char = '⢻';
    /// \u{28bc}: '⢼'
    pub const BRAILLE_PATTERN_DOTS_DASH_34568: char = '⢼';
    /// \u{28bd}: '⢽'
    pub const BRAILLE_PATTERN_DOTS_DASH_134568: char = '⢽';
    /// \u{28be}: '⢾'
    pub const BRAILLE_PATTERN_DOTS_DASH_234568: char = '⢾';
    /// \u{28bf}: '⢿'
    pub const BRAILLE_PATTERN_DOTS_DASH_1234568: char = '⢿';
    /// \u{28c0}: '⣀'
    pub const BRAILLE_PATTERN_DOTS_DASH_78: char = '⣀';
    /// \u{28c1}: '⣁'
    pub const BRAILLE_PATTERN_DOTS_DASH_178: char = '⣁';
    /// \u{28c2}: '⣂'
    pub const BRAILLE_PATTERN_DOTS_DASH_278: char = '⣂';
    /// \u{28c3}: '⣃'
    pub const BRAILLE_PATTERN_DOTS_DASH_1278: char = '⣃';
    /// \u{28c4}: '⣄'
    pub const BRAILLE_PATTERN_DOTS_DASH_378: char = '⣄';
    /// \u{28c5}: '⣅'
    pub const BRAILLE_PATTERN_DOTS_DASH_1378: char = '⣅';
    /// \u{28c6}: '⣆'
    pub const BRAILLE_PATTERN_DOTS_DASH_2378: char = '⣆';
    /// \u{28c7}: '⣇'
    pub const BRAILLE_PATTERN_DOTS_DASH_12378: char = '⣇';
    /// \u{28c8}: '⣈'
    pub const BRAILLE_PATTERN_DOTS_DASH_478: char = '⣈';
    /// \u{28c9}: '⣉'
    pub const BRAILLE_PATTERN_DOTS_DASH_1478: char = '⣉';
    /// \u{28ca}: '⣊'
    pub const BRAILLE_PATTERN_DOTS_DASH_2478: char = '⣊';
    /// \u{28cb}: '⣋'
    pub const BRAILLE_PATTERN_DOTS_DASH_12478: char = '⣋';
    /// \u{28cc}: '⣌'
    pub const BRAILLE_PATTERN_DOTS_DASH_3478: char = '⣌';
    /// \u{28cd}: '⣍'
    pub const BRAILLE_PATTERN_DOTS_DASH_13478: char = '⣍';
    /// \u{28ce}: '⣎'
    pub const BRAILLE_PATTERN_DOTS_DASH_23478: char = '⣎';
    /// \u{28cf}: '⣏'
    pub const BRAILLE_PATTERN_DOTS_DASH_123478: char = '⣏';
    /// \u{28d0}: '⣐'
    pub const BRAILLE_PATTERN_DOTS_DASH_578: char = '⣐';
    /// \u{28d1}: '⣑'
    pub const BRAILLE_PATTERN_DOTS_DASH_1578: char = '⣑';
    /// \u{28d2}: '⣒'
    pub const BRAILLE_PATTERN_DOTS_DASH_2578: char = '⣒';
    /// \u{28d3}: '⣓'
    pub const BRAILLE_PATTERN_DOTS_DASH_12578: char = '⣓';
    /// \u{28d4}: '⣔'
    pub const BRAILLE_PATTERN_DOTS_DASH_3578: char = '⣔';
    /// \u{28d5}: '⣕'
    pub const BRAILLE_PATTERN_DOTS_DASH_13578: char = '⣕';
    /// \u{28d6}: '⣖'
    pub const BRAILLE_PATTERN_DOTS_DASH_23578: char = '⣖';
    /// \u{28d7}: '⣗'
    pub const BRAILLE_PATTERN_DOTS_DASH_123578: char = '⣗';
    /// \u{28d8}: '⣘'
    pub const BRAILLE_PATTERN_DOTS_DASH_4578: char = '⣘';
    /// \u{28d9}: '⣙'
    pub const BRAILLE_PATTERN_DOTS_DASH_14578: char = '⣙';
    /// \u{28da}: '⣚'
    pub const BRAILLE_PATTERN_DOTS_DASH_24578: char = '⣚';
    /// \u{28db}: '⣛'
    pub const BRAILLE_PATTERN_DOTS_DASH_124578: char = '⣛';
    /// \u{28dc}: '⣜'
    pub const BRAILLE_PATTERN_DOTS_DASH_34578: char = '⣜';
    /// \u{28dd}: '⣝'
    pub const BRAILLE_PATTERN_DOTS_DASH_134578: char = '⣝';
    /// \u{28de}: '⣞'
    pub const BRAILLE_PATTERN_DOTS_DASH_234578: char = '⣞';
    /// \u{28df}: '⣟'
    pub const BRAILLE_PATTERN_DOTS_DASH_1234578: char = '⣟';
    /// \u{28e0}: '⣠'
    pub const BRAILLE_PATTERN_DOTS_DASH_678: char = '⣠';
    /// \u{28e1}: '⣡'
    pub const BRAILLE_PATTERN_DOTS_DASH_1678: char = '⣡';
    /// \u{28e2}: '⣢'
    pub const BRAILLE_PATTERN_DOTS_DASH_2678: char = '⣢';
    /// \u{28e3}: '⣣'
    pub const BRAILLE_PATTERN_DOTS_DASH_12678: char = '⣣';
    /// \u{28e4}: '⣤'
    pub const BRAILLE_PATTERN_DOTS_DASH_3678: char = '⣤';
    /// \u{28e5}: '⣥'
    pub const BRAILLE_PATTERN_DOTS_DASH_13678: char = '⣥';
    /// \u{28e6}: '⣦'
    pub const BRAILLE_PATTERN_DOTS_DASH_23678: char = '⣦';
    /// \u{28e7}: '⣧'
    pub const BRAILLE_PATTERN_DOTS_DASH_123678: char = '⣧';
    /// \u{28e8}: '⣨'
    pub const BRAILLE_PATTERN_DOTS_DASH_4678: char = '⣨';
    /// \u{28e9}: '⣩'
    pub const BRAILLE_PATTERN_DOTS_DASH_14678: char = '⣩';
    /// \u{28ea}: '⣪'
    pub const BRAILLE_PATTERN_DOTS_DASH_24678: char = '⣪';
    /// \u{28eb}: '⣫'
    pub const BRAILLE_PATTERN_DOTS_DASH_124678: char = '⣫';
    /// \u{28ec}: '⣬'
    pub const BRAILLE_PATTERN_DOTS_DASH_34678: char = '⣬';
    /// \u{28ed}: '⣭'
    pub const BRAILLE_PATTERN_DOTS_DASH_134678: char = '⣭';
    /// \u{28ee}: '⣮'
    pub const BRAILLE_PATTERN_DOTS_DASH_234678: char = '⣮';
    /// \u{28ef}: '⣯'
    pub const BRAILLE_PATTERN_DOTS_DASH_1234678: char = '⣯';
    /// \u{28f0}: '⣰'
    pub const BRAILLE_PATTERN_DOTS_DASH_5678: char = '⣰';
    /// \u{28f1}: '⣱'
    pub const BRAILLE_PATTERN_DOTS_DASH_15678: char = '⣱';
    /// \u{28f2}: '⣲'
    pub const BRAILLE_PATTERN_DOTS_DASH_25678: char = '⣲';
    /// \u{28f3}: '⣳'
    pub const BRAILLE_PATTERN_DOTS_DASH_125678: char = '⣳';
    /// \u{28f4}: '⣴'
    pub const BRAILLE_PATTERN_DOTS_DASH_35678: char = '⣴';
    /// \u{28f5}: '⣵'
    pub const BRAILLE_PATTERN_DOTS_DASH_135678: char = '⣵';
    /// \u{28f6}: '⣶'
    pub const BRAILLE_PATTERN_DOTS_DASH_235678: char = '⣶';
    /// \u{28f7}: '⣷'
    pub const BRAILLE_PATTERN_DOTS_DASH_1235678: char = '⣷';
    /// \u{28f8}: '⣸'
    pub const BRAILLE_PATTERN_DOTS_DASH_45678: char = '⣸';
    /// \u{28f9}: '⣹'
    pub const BRAILLE_PATTERN_DOTS_DASH_145678: char = '⣹';
    /// \u{28fa}: '⣺'
    pub const BRAILLE_PATTERN_DOTS_DASH_245678: char = '⣺';
    /// \u{28fb}: '⣻'
    pub const BRAILLE_PATTERN_DOTS_DASH_1245678: char = '⣻';
    /// \u{28fc}: '⣼'
    pub const BRAILLE_PATTERN_DOTS_DASH_345678: char = '⣼';
    /// \u{28fd}: '⣽'
    pub const BRAILLE_PATTERN_DOTS_DASH_1345678: char = '⣽';
    /// \u{28fe}: '⣾'
    pub const BRAILLE_PATTERN_DOTS_DASH_2345678: char = '⣾';
}

/// An enum to represent all characters in the BraillePatterns block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BraillePatterns {
    /// \u{2800}: '⠀'
    BraillePatternBlank,
    /// \u{2801}: '⠁'
    BraillePatternDotsDash1,
    /// \u{2802}: '⠂'
    BraillePatternDotsDash2,
    /// \u{2803}: '⠃'
    BraillePatternDotsDash12,
    /// \u{2804}: '⠄'
    BraillePatternDotsDash3,
    /// \u{2805}: '⠅'
    BraillePatternDotsDash13,
    /// \u{2806}: '⠆'
    BraillePatternDotsDash23,
    /// \u{2807}: '⠇'
    BraillePatternDotsDash123,
    /// \u{2808}: '⠈'
    BraillePatternDotsDash4,
    /// \u{2809}: '⠉'
    BraillePatternDotsDash14,
    /// \u{280a}: '⠊'
    BraillePatternDotsDash24,
    /// \u{280b}: '⠋'
    BraillePatternDotsDash124,
    /// \u{280c}: '⠌'
    BraillePatternDotsDash34,
    /// \u{280d}: '⠍'
    BraillePatternDotsDash134,
    /// \u{280e}: '⠎'
    BraillePatternDotsDash234,
    /// \u{280f}: '⠏'
    BraillePatternDotsDash1234,
    /// \u{2810}: '⠐'
    BraillePatternDotsDash5,
    /// \u{2811}: '⠑'
    BraillePatternDotsDash15,
    /// \u{2812}: '⠒'
    BraillePatternDotsDash25,
    /// \u{2813}: '⠓'
    BraillePatternDotsDash125,
    /// \u{2814}: '⠔'
    BraillePatternDotsDash35,
    /// \u{2815}: '⠕'
    BraillePatternDotsDash135,
    /// \u{2816}: '⠖'
    BraillePatternDotsDash235,
    /// \u{2817}: '⠗'
    BraillePatternDotsDash1235,
    /// \u{2818}: '⠘'
    BraillePatternDotsDash45,
    /// \u{2819}: '⠙'
    BraillePatternDotsDash145,
    /// \u{281a}: '⠚'
    BraillePatternDotsDash245,
    /// \u{281b}: '⠛'
    BraillePatternDotsDash1245,
    /// \u{281c}: '⠜'
    BraillePatternDotsDash345,
    /// \u{281d}: '⠝'
    BraillePatternDotsDash1345,
    /// \u{281e}: '⠞'
    BraillePatternDotsDash2345,
    /// \u{281f}: '⠟'
    BraillePatternDotsDash12345,
    /// \u{2820}: '⠠'
    BraillePatternDotsDash6,
    /// \u{2821}: '⠡'
    BraillePatternDotsDash16,
    /// \u{2822}: '⠢'
    BraillePatternDotsDash26,
    /// \u{2823}: '⠣'
    BraillePatternDotsDash126,
    /// \u{2824}: '⠤'
    BraillePatternDotsDash36,
    /// \u{2825}: '⠥'
    BraillePatternDotsDash136,
    /// \u{2826}: '⠦'
    BraillePatternDotsDash236,
    /// \u{2827}: '⠧'
    BraillePatternDotsDash1236,
    /// \u{2828}: '⠨'
    BraillePatternDotsDash46,
    /// \u{2829}: '⠩'
    BraillePatternDotsDash146,
    /// \u{282a}: '⠪'
    BraillePatternDotsDash246,
    /// \u{282b}: '⠫'
    BraillePatternDotsDash1246,
    /// \u{282c}: '⠬'
    BraillePatternDotsDash346,
    /// \u{282d}: '⠭'
    BraillePatternDotsDash1346,
    /// \u{282e}: '⠮'
    BraillePatternDotsDash2346,
    /// \u{282f}: '⠯'
    BraillePatternDotsDash12346,
    /// \u{2830}: '⠰'
    BraillePatternDotsDash56,
    /// \u{2831}: '⠱'
    BraillePatternDotsDash156,
    /// \u{2832}: '⠲'
    BraillePatternDotsDash256,
    /// \u{2833}: '⠳'
    BraillePatternDotsDash1256,
    /// \u{2834}: '⠴'
    BraillePatternDotsDash356,
    /// \u{2835}: '⠵'
    BraillePatternDotsDash1356,
    /// \u{2836}: '⠶'
    BraillePatternDotsDash2356,
    /// \u{2837}: '⠷'
    BraillePatternDotsDash12356,
    /// \u{2838}: '⠸'
    BraillePatternDotsDash456,
    /// \u{2839}: '⠹'
    BraillePatternDotsDash1456,
    /// \u{283a}: '⠺'
    BraillePatternDotsDash2456,
    /// \u{283b}: '⠻'
    BraillePatternDotsDash12456,
    /// \u{283c}: '⠼'
    BraillePatternDotsDash3456,
    /// \u{283d}: '⠽'
    BraillePatternDotsDash13456,
    /// \u{283e}: '⠾'
    BraillePatternDotsDash23456,
    /// \u{283f}: '⠿'
    BraillePatternDotsDash123456,
    /// \u{2840}: '⡀'
    BraillePatternDotsDash7,
    /// \u{2841}: '⡁'
    BraillePatternDotsDash17,
    /// \u{2842}: '⡂'
    BraillePatternDotsDash27,
    /// \u{2843}: '⡃'
    BraillePatternDotsDash127,
    /// \u{2844}: '⡄'
    BraillePatternDotsDash37,
    /// \u{2845}: '⡅'
    BraillePatternDotsDash137,
    /// \u{2846}: '⡆'
    BraillePatternDotsDash237,
    /// \u{2847}: '⡇'
    BraillePatternDotsDash1237,
    /// \u{2848}: '⡈'
    BraillePatternDotsDash47,
    /// \u{2849}: '⡉'
    BraillePatternDotsDash147,
    /// \u{284a}: '⡊'
    BraillePatternDotsDash247,
    /// \u{284b}: '⡋'
    BraillePatternDotsDash1247,
    /// \u{284c}: '⡌'
    BraillePatternDotsDash347,
    /// \u{284d}: '⡍'
    BraillePatternDotsDash1347,
    /// \u{284e}: '⡎'
    BraillePatternDotsDash2347,
    /// \u{284f}: '⡏'
    BraillePatternDotsDash12347,
    /// \u{2850}: '⡐'
    BraillePatternDotsDash57,
    /// \u{2851}: '⡑'
    BraillePatternDotsDash157,
    /// \u{2852}: '⡒'
    BraillePatternDotsDash257,
    /// \u{2853}: '⡓'
    BraillePatternDotsDash1257,
    /// \u{2854}: '⡔'
    BraillePatternDotsDash357,
    /// \u{2855}: '⡕'
    BraillePatternDotsDash1357,
    /// \u{2856}: '⡖'
    BraillePatternDotsDash2357,
    /// \u{2857}: '⡗'
    BraillePatternDotsDash12357,
    /// \u{2858}: '⡘'
    BraillePatternDotsDash457,
    /// \u{2859}: '⡙'
    BraillePatternDotsDash1457,
    /// \u{285a}: '⡚'
    BraillePatternDotsDash2457,
    /// \u{285b}: '⡛'
    BraillePatternDotsDash12457,
    /// \u{285c}: '⡜'
    BraillePatternDotsDash3457,
    /// \u{285d}: '⡝'
    BraillePatternDotsDash13457,
    /// \u{285e}: '⡞'
    BraillePatternDotsDash23457,
    /// \u{285f}: '⡟'
    BraillePatternDotsDash123457,
    /// \u{2860}: '⡠'
    BraillePatternDotsDash67,
    /// \u{2861}: '⡡'
    BraillePatternDotsDash167,
    /// \u{2862}: '⡢'
    BraillePatternDotsDash267,
    /// \u{2863}: '⡣'
    BraillePatternDotsDash1267,
    /// \u{2864}: '⡤'
    BraillePatternDotsDash367,
    /// \u{2865}: '⡥'
    BraillePatternDotsDash1367,
    /// \u{2866}: '⡦'
    BraillePatternDotsDash2367,
    /// \u{2867}: '⡧'
    BraillePatternDotsDash12367,
    /// \u{2868}: '⡨'
    BraillePatternDotsDash467,
    /// \u{2869}: '⡩'
    BraillePatternDotsDash1467,
    /// \u{286a}: '⡪'
    BraillePatternDotsDash2467,
    /// \u{286b}: '⡫'
    BraillePatternDotsDash12467,
    /// \u{286c}: '⡬'
    BraillePatternDotsDash3467,
    /// \u{286d}: '⡭'
    BraillePatternDotsDash13467,
    /// \u{286e}: '⡮'
    BraillePatternDotsDash23467,
    /// \u{286f}: '⡯'
    BraillePatternDotsDash123467,
    /// \u{2870}: '⡰'
    BraillePatternDotsDash567,
    /// \u{2871}: '⡱'
    BraillePatternDotsDash1567,
    /// \u{2872}: '⡲'
    BraillePatternDotsDash2567,
    /// \u{2873}: '⡳'
    BraillePatternDotsDash12567,
    /// \u{2874}: '⡴'
    BraillePatternDotsDash3567,
    /// \u{2875}: '⡵'
    BraillePatternDotsDash13567,
    /// \u{2876}: '⡶'
    BraillePatternDotsDash23567,
    /// \u{2877}: '⡷'
    BraillePatternDotsDash123567,
    /// \u{2878}: '⡸'
    BraillePatternDotsDash4567,
    /// \u{2879}: '⡹'
    BraillePatternDotsDash14567,
    /// \u{287a}: '⡺'
    BraillePatternDotsDash24567,
    /// \u{287b}: '⡻'
    BraillePatternDotsDash124567,
    /// \u{287c}: '⡼'
    BraillePatternDotsDash34567,
    /// \u{287d}: '⡽'
    BraillePatternDotsDash134567,
    /// \u{287e}: '⡾'
    BraillePatternDotsDash234567,
    /// \u{287f}: '⡿'
    BraillePatternDotsDash1234567,
    /// \u{2880}: '⢀'
    BraillePatternDotsDash8,
    /// \u{2881}: '⢁'
    BraillePatternDotsDash18,
    /// \u{2882}: '⢂'
    BraillePatternDotsDash28,
    /// \u{2883}: '⢃'
    BraillePatternDotsDash128,
    /// \u{2884}: '⢄'
    BraillePatternDotsDash38,
    /// \u{2885}: '⢅'
    BraillePatternDotsDash138,
    /// \u{2886}: '⢆'
    BraillePatternDotsDash238,
    /// \u{2887}: '⢇'
    BraillePatternDotsDash1238,
    /// \u{2888}: '⢈'
    BraillePatternDotsDash48,
    /// \u{2889}: '⢉'
    BraillePatternDotsDash148,
    /// \u{288a}: '⢊'
    BraillePatternDotsDash248,
    /// \u{288b}: '⢋'
    BraillePatternDotsDash1248,
    /// \u{288c}: '⢌'
    BraillePatternDotsDash348,
    /// \u{288d}: '⢍'
    BraillePatternDotsDash1348,
    /// \u{288e}: '⢎'
    BraillePatternDotsDash2348,
    /// \u{288f}: '⢏'
    BraillePatternDotsDash12348,
    /// \u{2890}: '⢐'
    BraillePatternDotsDash58,
    /// \u{2891}: '⢑'
    BraillePatternDotsDash158,
    /// \u{2892}: '⢒'
    BraillePatternDotsDash258,
    /// \u{2893}: '⢓'
    BraillePatternDotsDash1258,
    /// \u{2894}: '⢔'
    BraillePatternDotsDash358,
    /// \u{2895}: '⢕'
    BraillePatternDotsDash1358,
    /// \u{2896}: '⢖'
    BraillePatternDotsDash2358,
    /// \u{2897}: '⢗'
    BraillePatternDotsDash12358,
    /// \u{2898}: '⢘'
    BraillePatternDotsDash458,
    /// \u{2899}: '⢙'
    BraillePatternDotsDash1458,
    /// \u{289a}: '⢚'
    BraillePatternDotsDash2458,
    /// \u{289b}: '⢛'
    BraillePatternDotsDash12458,
    /// \u{289c}: '⢜'
    BraillePatternDotsDash3458,
    /// \u{289d}: '⢝'
    BraillePatternDotsDash13458,
    /// \u{289e}: '⢞'
    BraillePatternDotsDash23458,
    /// \u{289f}: '⢟'
    BraillePatternDotsDash123458,
    /// \u{28a0}: '⢠'
    BraillePatternDotsDash68,
    /// \u{28a1}: '⢡'
    BraillePatternDotsDash168,
    /// \u{28a2}: '⢢'
    BraillePatternDotsDash268,
    /// \u{28a3}: '⢣'
    BraillePatternDotsDash1268,
    /// \u{28a4}: '⢤'
    BraillePatternDotsDash368,
    /// \u{28a5}: '⢥'
    BraillePatternDotsDash1368,
    /// \u{28a6}: '⢦'
    BraillePatternDotsDash2368,
    /// \u{28a7}: '⢧'
    BraillePatternDotsDash12368,
    /// \u{28a8}: '⢨'
    BraillePatternDotsDash468,
    /// \u{28a9}: '⢩'
    BraillePatternDotsDash1468,
    /// \u{28aa}: '⢪'
    BraillePatternDotsDash2468,
    /// \u{28ab}: '⢫'
    BraillePatternDotsDash12468,
    /// \u{28ac}: '⢬'
    BraillePatternDotsDash3468,
    /// \u{28ad}: '⢭'
    BraillePatternDotsDash13468,
    /// \u{28ae}: '⢮'
    BraillePatternDotsDash23468,
    /// \u{28af}: '⢯'
    BraillePatternDotsDash123468,
    /// \u{28b0}: '⢰'
    BraillePatternDotsDash568,
    /// \u{28b1}: '⢱'
    BraillePatternDotsDash1568,
    /// \u{28b2}: '⢲'
    BraillePatternDotsDash2568,
    /// \u{28b3}: '⢳'
    BraillePatternDotsDash12568,
    /// \u{28b4}: '⢴'
    BraillePatternDotsDash3568,
    /// \u{28b5}: '⢵'
    BraillePatternDotsDash13568,
    /// \u{28b6}: '⢶'
    BraillePatternDotsDash23568,
    /// \u{28b7}: '⢷'
    BraillePatternDotsDash123568,
    /// \u{28b8}: '⢸'
    BraillePatternDotsDash4568,
    /// \u{28b9}: '⢹'
    BraillePatternDotsDash14568,
    /// \u{28ba}: '⢺'
    BraillePatternDotsDash24568,
    /// \u{28bb}: '⢻'
    BraillePatternDotsDash124568,
    /// \u{28bc}: '⢼'
    BraillePatternDotsDash34568,
    /// \u{28bd}: '⢽'
    BraillePatternDotsDash134568,
    /// \u{28be}: '⢾'
    BraillePatternDotsDash234568,
    /// \u{28bf}: '⢿'
    BraillePatternDotsDash1234568,
    /// \u{28c0}: '⣀'
    BraillePatternDotsDash78,
    /// \u{28c1}: '⣁'
    BraillePatternDotsDash178,
    /// \u{28c2}: '⣂'
    BraillePatternDotsDash278,
    /// \u{28c3}: '⣃'
    BraillePatternDotsDash1278,
    /// \u{28c4}: '⣄'
    BraillePatternDotsDash378,
    /// \u{28c5}: '⣅'
    BraillePatternDotsDash1378,
    /// \u{28c6}: '⣆'
    BraillePatternDotsDash2378,
    /// \u{28c7}: '⣇'
    BraillePatternDotsDash12378,
    /// \u{28c8}: '⣈'
    BraillePatternDotsDash478,
    /// \u{28c9}: '⣉'
    BraillePatternDotsDash1478,
    /// \u{28ca}: '⣊'
    BraillePatternDotsDash2478,
    /// \u{28cb}: '⣋'
    BraillePatternDotsDash12478,
    /// \u{28cc}: '⣌'
    BraillePatternDotsDash3478,
    /// \u{28cd}: '⣍'
    BraillePatternDotsDash13478,
    /// \u{28ce}: '⣎'
    BraillePatternDotsDash23478,
    /// \u{28cf}: '⣏'
    BraillePatternDotsDash123478,
    /// \u{28d0}: '⣐'
    BraillePatternDotsDash578,
    /// \u{28d1}: '⣑'
    BraillePatternDotsDash1578,
    /// \u{28d2}: '⣒'
    BraillePatternDotsDash2578,
    /// \u{28d3}: '⣓'
    BraillePatternDotsDash12578,
    /// \u{28d4}: '⣔'
    BraillePatternDotsDash3578,
    /// \u{28d5}: '⣕'
    BraillePatternDotsDash13578,
    /// \u{28d6}: '⣖'
    BraillePatternDotsDash23578,
    /// \u{28d7}: '⣗'
    BraillePatternDotsDash123578,
    /// \u{28d8}: '⣘'
    BraillePatternDotsDash4578,
    /// \u{28d9}: '⣙'
    BraillePatternDotsDash14578,
    /// \u{28da}: '⣚'
    BraillePatternDotsDash24578,
    /// \u{28db}: '⣛'
    BraillePatternDotsDash124578,
    /// \u{28dc}: '⣜'
    BraillePatternDotsDash34578,
    /// \u{28dd}: '⣝'
    BraillePatternDotsDash134578,
    /// \u{28de}: '⣞'
    BraillePatternDotsDash234578,
    /// \u{28df}: '⣟'
    BraillePatternDotsDash1234578,
    /// \u{28e0}: '⣠'
    BraillePatternDotsDash678,
    /// \u{28e1}: '⣡'
    BraillePatternDotsDash1678,
    /// \u{28e2}: '⣢'
    BraillePatternDotsDash2678,
    /// \u{28e3}: '⣣'
    BraillePatternDotsDash12678,
    /// \u{28e4}: '⣤'
    BraillePatternDotsDash3678,
    /// \u{28e5}: '⣥'
    BraillePatternDotsDash13678,
    /// \u{28e6}: '⣦'
    BraillePatternDotsDash23678,
    /// \u{28e7}: '⣧'
    BraillePatternDotsDash123678,
    /// \u{28e8}: '⣨'
    BraillePatternDotsDash4678,
    /// \u{28e9}: '⣩'
    BraillePatternDotsDash14678,
    /// \u{28ea}: '⣪'
    BraillePatternDotsDash24678,
    /// \u{28eb}: '⣫'
    BraillePatternDotsDash124678,
    /// \u{28ec}: '⣬'
    BraillePatternDotsDash34678,
    /// \u{28ed}: '⣭'
    BraillePatternDotsDash134678,
    /// \u{28ee}: '⣮'
    BraillePatternDotsDash234678,
    /// \u{28ef}: '⣯'
    BraillePatternDotsDash1234678,
    /// \u{28f0}: '⣰'
    BraillePatternDotsDash5678,
    /// \u{28f1}: '⣱'
    BraillePatternDotsDash15678,
    /// \u{28f2}: '⣲'
    BraillePatternDotsDash25678,
    /// \u{28f3}: '⣳'
    BraillePatternDotsDash125678,
    /// \u{28f4}: '⣴'
    BraillePatternDotsDash35678,
    /// \u{28f5}: '⣵'
    BraillePatternDotsDash135678,
    /// \u{28f6}: '⣶'
    BraillePatternDotsDash235678,
    /// \u{28f7}: '⣷'
    BraillePatternDotsDash1235678,
    /// \u{28f8}: '⣸'
    BraillePatternDotsDash45678,
    /// \u{28f9}: '⣹'
    BraillePatternDotsDash145678,
    /// \u{28fa}: '⣺'
    BraillePatternDotsDash245678,
    /// \u{28fb}: '⣻'
    BraillePatternDotsDash1245678,
    /// \u{28fc}: '⣼'
    BraillePatternDotsDash345678,
    /// \u{28fd}: '⣽'
    BraillePatternDotsDash1345678,
    /// \u{28fe}: '⣾'
    BraillePatternDotsDash2345678,
}

impl Into<char> for BraillePatterns {
    fn into(self) -> char {
        use constants::*;
        match self {
            BraillePatterns::BraillePatternBlank => BRAILLE_PATTERN_BLANK,
            BraillePatterns::BraillePatternDotsDash1 => BRAILLE_PATTERN_DOTS_DASH_1,
            BraillePatterns::BraillePatternDotsDash2 => BRAILLE_PATTERN_DOTS_DASH_2,
            BraillePatterns::BraillePatternDotsDash12 => BRAILLE_PATTERN_DOTS_DASH_12,
            BraillePatterns::BraillePatternDotsDash3 => BRAILLE_PATTERN_DOTS_DASH_3,
            BraillePatterns::BraillePatternDotsDash13 => BRAILLE_PATTERN_DOTS_DASH_13,
            BraillePatterns::BraillePatternDotsDash23 => BRAILLE_PATTERN_DOTS_DASH_23,
            BraillePatterns::BraillePatternDotsDash123 => BRAILLE_PATTERN_DOTS_DASH_123,
            BraillePatterns::BraillePatternDotsDash4 => BRAILLE_PATTERN_DOTS_DASH_4,
            BraillePatterns::BraillePatternDotsDash14 => BRAILLE_PATTERN_DOTS_DASH_14,
            BraillePatterns::BraillePatternDotsDash24 => BRAILLE_PATTERN_DOTS_DASH_24,
            BraillePatterns::BraillePatternDotsDash124 => BRAILLE_PATTERN_DOTS_DASH_124,
            BraillePatterns::BraillePatternDotsDash34 => BRAILLE_PATTERN_DOTS_DASH_34,
            BraillePatterns::BraillePatternDotsDash134 => BRAILLE_PATTERN_DOTS_DASH_134,
            BraillePatterns::BraillePatternDotsDash234 => BRAILLE_PATTERN_DOTS_DASH_234,
            BraillePatterns::BraillePatternDotsDash1234 => BRAILLE_PATTERN_DOTS_DASH_1234,
            BraillePatterns::BraillePatternDotsDash5 => BRAILLE_PATTERN_DOTS_DASH_5,
            BraillePatterns::BraillePatternDotsDash15 => BRAILLE_PATTERN_DOTS_DASH_15,
            BraillePatterns::BraillePatternDotsDash25 => BRAILLE_PATTERN_DOTS_DASH_25,
            BraillePatterns::BraillePatternDotsDash125 => BRAILLE_PATTERN_DOTS_DASH_125,
            BraillePatterns::BraillePatternDotsDash35 => BRAILLE_PATTERN_DOTS_DASH_35,
            BraillePatterns::BraillePatternDotsDash135 => BRAILLE_PATTERN_DOTS_DASH_135,
            BraillePatterns::BraillePatternDotsDash235 => BRAILLE_PATTERN_DOTS_DASH_235,
            BraillePatterns::BraillePatternDotsDash1235 => BRAILLE_PATTERN_DOTS_DASH_1235,
            BraillePatterns::BraillePatternDotsDash45 => BRAILLE_PATTERN_DOTS_DASH_45,
            BraillePatterns::BraillePatternDotsDash145 => BRAILLE_PATTERN_DOTS_DASH_145,
            BraillePatterns::BraillePatternDotsDash245 => BRAILLE_PATTERN_DOTS_DASH_245,
            BraillePatterns::BraillePatternDotsDash1245 => BRAILLE_PATTERN_DOTS_DASH_1245,
            BraillePatterns::BraillePatternDotsDash345 => BRAILLE_PATTERN_DOTS_DASH_345,
            BraillePatterns::BraillePatternDotsDash1345 => BRAILLE_PATTERN_DOTS_DASH_1345,
            BraillePatterns::BraillePatternDotsDash2345 => BRAILLE_PATTERN_DOTS_DASH_2345,
            BraillePatterns::BraillePatternDotsDash12345 => BRAILLE_PATTERN_DOTS_DASH_12345,
            BraillePatterns::BraillePatternDotsDash6 => BRAILLE_PATTERN_DOTS_DASH_6,
            BraillePatterns::BraillePatternDotsDash16 => BRAILLE_PATTERN_DOTS_DASH_16,
            BraillePatterns::BraillePatternDotsDash26 => BRAILLE_PATTERN_DOTS_DASH_26,
            BraillePatterns::BraillePatternDotsDash126 => BRAILLE_PATTERN_DOTS_DASH_126,
            BraillePatterns::BraillePatternDotsDash36 => BRAILLE_PATTERN_DOTS_DASH_36,
            BraillePatterns::BraillePatternDotsDash136 => BRAILLE_PATTERN_DOTS_DASH_136,
            BraillePatterns::BraillePatternDotsDash236 => BRAILLE_PATTERN_DOTS_DASH_236,
            BraillePatterns::BraillePatternDotsDash1236 => BRAILLE_PATTERN_DOTS_DASH_1236,
            BraillePatterns::BraillePatternDotsDash46 => BRAILLE_PATTERN_DOTS_DASH_46,
            BraillePatterns::BraillePatternDotsDash146 => BRAILLE_PATTERN_DOTS_DASH_146,
            BraillePatterns::BraillePatternDotsDash246 => BRAILLE_PATTERN_DOTS_DASH_246,
            BraillePatterns::BraillePatternDotsDash1246 => BRAILLE_PATTERN_DOTS_DASH_1246,
            BraillePatterns::BraillePatternDotsDash346 => BRAILLE_PATTERN_DOTS_DASH_346,
            BraillePatterns::BraillePatternDotsDash1346 => BRAILLE_PATTERN_DOTS_DASH_1346,
            BraillePatterns::BraillePatternDotsDash2346 => BRAILLE_PATTERN_DOTS_DASH_2346,
            BraillePatterns::BraillePatternDotsDash12346 => BRAILLE_PATTERN_DOTS_DASH_12346,
            BraillePatterns::BraillePatternDotsDash56 => BRAILLE_PATTERN_DOTS_DASH_56,
            BraillePatterns::BraillePatternDotsDash156 => BRAILLE_PATTERN_DOTS_DASH_156,
            BraillePatterns::BraillePatternDotsDash256 => BRAILLE_PATTERN_DOTS_DASH_256,
            BraillePatterns::BraillePatternDotsDash1256 => BRAILLE_PATTERN_DOTS_DASH_1256,
            BraillePatterns::BraillePatternDotsDash356 => BRAILLE_PATTERN_DOTS_DASH_356,
            BraillePatterns::BraillePatternDotsDash1356 => BRAILLE_PATTERN_DOTS_DASH_1356,
            BraillePatterns::BraillePatternDotsDash2356 => BRAILLE_PATTERN_DOTS_DASH_2356,
            BraillePatterns::BraillePatternDotsDash12356 => BRAILLE_PATTERN_DOTS_DASH_12356,
            BraillePatterns::BraillePatternDotsDash456 => BRAILLE_PATTERN_DOTS_DASH_456,
            BraillePatterns::BraillePatternDotsDash1456 => BRAILLE_PATTERN_DOTS_DASH_1456,
            BraillePatterns::BraillePatternDotsDash2456 => BRAILLE_PATTERN_DOTS_DASH_2456,
            BraillePatterns::BraillePatternDotsDash12456 => BRAILLE_PATTERN_DOTS_DASH_12456,
            BraillePatterns::BraillePatternDotsDash3456 => BRAILLE_PATTERN_DOTS_DASH_3456,
            BraillePatterns::BraillePatternDotsDash13456 => BRAILLE_PATTERN_DOTS_DASH_13456,
            BraillePatterns::BraillePatternDotsDash23456 => BRAILLE_PATTERN_DOTS_DASH_23456,
            BraillePatterns::BraillePatternDotsDash123456 => BRAILLE_PATTERN_DOTS_DASH_123456,
            BraillePatterns::BraillePatternDotsDash7 => BRAILLE_PATTERN_DOTS_DASH_7,
            BraillePatterns::BraillePatternDotsDash17 => BRAILLE_PATTERN_DOTS_DASH_17,
            BraillePatterns::BraillePatternDotsDash27 => BRAILLE_PATTERN_DOTS_DASH_27,
            BraillePatterns::BraillePatternDotsDash127 => BRAILLE_PATTERN_DOTS_DASH_127,
            BraillePatterns::BraillePatternDotsDash37 => BRAILLE_PATTERN_DOTS_DASH_37,
            BraillePatterns::BraillePatternDotsDash137 => BRAILLE_PATTERN_DOTS_DASH_137,
            BraillePatterns::BraillePatternDotsDash237 => BRAILLE_PATTERN_DOTS_DASH_237,
            BraillePatterns::BraillePatternDotsDash1237 => BRAILLE_PATTERN_DOTS_DASH_1237,
            BraillePatterns::BraillePatternDotsDash47 => BRAILLE_PATTERN_DOTS_DASH_47,
            BraillePatterns::BraillePatternDotsDash147 => BRAILLE_PATTERN_DOTS_DASH_147,
            BraillePatterns::BraillePatternDotsDash247 => BRAILLE_PATTERN_DOTS_DASH_247,
            BraillePatterns::BraillePatternDotsDash1247 => BRAILLE_PATTERN_DOTS_DASH_1247,
            BraillePatterns::BraillePatternDotsDash347 => BRAILLE_PATTERN_DOTS_DASH_347,
            BraillePatterns::BraillePatternDotsDash1347 => BRAILLE_PATTERN_DOTS_DASH_1347,
            BraillePatterns::BraillePatternDotsDash2347 => BRAILLE_PATTERN_DOTS_DASH_2347,
            BraillePatterns::BraillePatternDotsDash12347 => BRAILLE_PATTERN_DOTS_DASH_12347,
            BraillePatterns::BraillePatternDotsDash57 => BRAILLE_PATTERN_DOTS_DASH_57,
            BraillePatterns::BraillePatternDotsDash157 => BRAILLE_PATTERN_DOTS_DASH_157,
            BraillePatterns::BraillePatternDotsDash257 => BRAILLE_PATTERN_DOTS_DASH_257,
            BraillePatterns::BraillePatternDotsDash1257 => BRAILLE_PATTERN_DOTS_DASH_1257,
            BraillePatterns::BraillePatternDotsDash357 => BRAILLE_PATTERN_DOTS_DASH_357,
            BraillePatterns::BraillePatternDotsDash1357 => BRAILLE_PATTERN_DOTS_DASH_1357,
            BraillePatterns::BraillePatternDotsDash2357 => BRAILLE_PATTERN_DOTS_DASH_2357,
            BraillePatterns::BraillePatternDotsDash12357 => BRAILLE_PATTERN_DOTS_DASH_12357,
            BraillePatterns::BraillePatternDotsDash457 => BRAILLE_PATTERN_DOTS_DASH_457,
            BraillePatterns::BraillePatternDotsDash1457 => BRAILLE_PATTERN_DOTS_DASH_1457,
            BraillePatterns::BraillePatternDotsDash2457 => BRAILLE_PATTERN_DOTS_DASH_2457,
            BraillePatterns::BraillePatternDotsDash12457 => BRAILLE_PATTERN_DOTS_DASH_12457,
            BraillePatterns::BraillePatternDotsDash3457 => BRAILLE_PATTERN_DOTS_DASH_3457,
            BraillePatterns::BraillePatternDotsDash13457 => BRAILLE_PATTERN_DOTS_DASH_13457,
            BraillePatterns::BraillePatternDotsDash23457 => BRAILLE_PATTERN_DOTS_DASH_23457,
            BraillePatterns::BraillePatternDotsDash123457 => BRAILLE_PATTERN_DOTS_DASH_123457,
            BraillePatterns::BraillePatternDotsDash67 => BRAILLE_PATTERN_DOTS_DASH_67,
            BraillePatterns::BraillePatternDotsDash167 => BRAILLE_PATTERN_DOTS_DASH_167,
            BraillePatterns::BraillePatternDotsDash267 => BRAILLE_PATTERN_DOTS_DASH_267,
            BraillePatterns::BraillePatternDotsDash1267 => BRAILLE_PATTERN_DOTS_DASH_1267,
            BraillePatterns::BraillePatternDotsDash367 => BRAILLE_PATTERN_DOTS_DASH_367,
            BraillePatterns::BraillePatternDotsDash1367 => BRAILLE_PATTERN_DOTS_DASH_1367,
            BraillePatterns::BraillePatternDotsDash2367 => BRAILLE_PATTERN_DOTS_DASH_2367,
            BraillePatterns::BraillePatternDotsDash12367 => BRAILLE_PATTERN_DOTS_DASH_12367,
            BraillePatterns::BraillePatternDotsDash467 => BRAILLE_PATTERN_DOTS_DASH_467,
            BraillePatterns::BraillePatternDotsDash1467 => BRAILLE_PATTERN_DOTS_DASH_1467,
            BraillePatterns::BraillePatternDotsDash2467 => BRAILLE_PATTERN_DOTS_DASH_2467,
            BraillePatterns::BraillePatternDotsDash12467 => BRAILLE_PATTERN_DOTS_DASH_12467,
            BraillePatterns::BraillePatternDotsDash3467 => BRAILLE_PATTERN_DOTS_DASH_3467,
            BraillePatterns::BraillePatternDotsDash13467 => BRAILLE_PATTERN_DOTS_DASH_13467,
            BraillePatterns::BraillePatternDotsDash23467 => BRAILLE_PATTERN_DOTS_DASH_23467,
            BraillePatterns::BraillePatternDotsDash123467 => BRAILLE_PATTERN_DOTS_DASH_123467,
            BraillePatterns::BraillePatternDotsDash567 => BRAILLE_PATTERN_DOTS_DASH_567,
            BraillePatterns::BraillePatternDotsDash1567 => BRAILLE_PATTERN_DOTS_DASH_1567,
            BraillePatterns::BraillePatternDotsDash2567 => BRAILLE_PATTERN_DOTS_DASH_2567,
            BraillePatterns::BraillePatternDotsDash12567 => BRAILLE_PATTERN_DOTS_DASH_12567,
            BraillePatterns::BraillePatternDotsDash3567 => BRAILLE_PATTERN_DOTS_DASH_3567,
            BraillePatterns::BraillePatternDotsDash13567 => BRAILLE_PATTERN_DOTS_DASH_13567,
            BraillePatterns::BraillePatternDotsDash23567 => BRAILLE_PATTERN_DOTS_DASH_23567,
            BraillePatterns::BraillePatternDotsDash123567 => BRAILLE_PATTERN_DOTS_DASH_123567,
            BraillePatterns::BraillePatternDotsDash4567 => BRAILLE_PATTERN_DOTS_DASH_4567,
            BraillePatterns::BraillePatternDotsDash14567 => BRAILLE_PATTERN_DOTS_DASH_14567,
            BraillePatterns::BraillePatternDotsDash24567 => BRAILLE_PATTERN_DOTS_DASH_24567,
            BraillePatterns::BraillePatternDotsDash124567 => BRAILLE_PATTERN_DOTS_DASH_124567,
            BraillePatterns::BraillePatternDotsDash34567 => BRAILLE_PATTERN_DOTS_DASH_34567,
            BraillePatterns::BraillePatternDotsDash134567 => BRAILLE_PATTERN_DOTS_DASH_134567,
            BraillePatterns::BraillePatternDotsDash234567 => BRAILLE_PATTERN_DOTS_DASH_234567,
            BraillePatterns::BraillePatternDotsDash1234567 => BRAILLE_PATTERN_DOTS_DASH_1234567,
            BraillePatterns::BraillePatternDotsDash8 => BRAILLE_PATTERN_DOTS_DASH_8,
            BraillePatterns::BraillePatternDotsDash18 => BRAILLE_PATTERN_DOTS_DASH_18,
            BraillePatterns::BraillePatternDotsDash28 => BRAILLE_PATTERN_DOTS_DASH_28,
            BraillePatterns::BraillePatternDotsDash128 => BRAILLE_PATTERN_DOTS_DASH_128,
            BraillePatterns::BraillePatternDotsDash38 => BRAILLE_PATTERN_DOTS_DASH_38,
            BraillePatterns::BraillePatternDotsDash138 => BRAILLE_PATTERN_DOTS_DASH_138,
            BraillePatterns::BraillePatternDotsDash238 => BRAILLE_PATTERN_DOTS_DASH_238,
            BraillePatterns::BraillePatternDotsDash1238 => BRAILLE_PATTERN_DOTS_DASH_1238,
            BraillePatterns::BraillePatternDotsDash48 => BRAILLE_PATTERN_DOTS_DASH_48,
            BraillePatterns::BraillePatternDotsDash148 => BRAILLE_PATTERN_DOTS_DASH_148,
            BraillePatterns::BraillePatternDotsDash248 => BRAILLE_PATTERN_DOTS_DASH_248,
            BraillePatterns::BraillePatternDotsDash1248 => BRAILLE_PATTERN_DOTS_DASH_1248,
            BraillePatterns::BraillePatternDotsDash348 => BRAILLE_PATTERN_DOTS_DASH_348,
            BraillePatterns::BraillePatternDotsDash1348 => BRAILLE_PATTERN_DOTS_DASH_1348,
            BraillePatterns::BraillePatternDotsDash2348 => BRAILLE_PATTERN_DOTS_DASH_2348,
            BraillePatterns::BraillePatternDotsDash12348 => BRAILLE_PATTERN_DOTS_DASH_12348,
            BraillePatterns::BraillePatternDotsDash58 => BRAILLE_PATTERN_DOTS_DASH_58,
            BraillePatterns::BraillePatternDotsDash158 => BRAILLE_PATTERN_DOTS_DASH_158,
            BraillePatterns::BraillePatternDotsDash258 => BRAILLE_PATTERN_DOTS_DASH_258,
            BraillePatterns::BraillePatternDotsDash1258 => BRAILLE_PATTERN_DOTS_DASH_1258,
            BraillePatterns::BraillePatternDotsDash358 => BRAILLE_PATTERN_DOTS_DASH_358,
            BraillePatterns::BraillePatternDotsDash1358 => BRAILLE_PATTERN_DOTS_DASH_1358,
            BraillePatterns::BraillePatternDotsDash2358 => BRAILLE_PATTERN_DOTS_DASH_2358,
            BraillePatterns::BraillePatternDotsDash12358 => BRAILLE_PATTERN_DOTS_DASH_12358,
            BraillePatterns::BraillePatternDotsDash458 => BRAILLE_PATTERN_DOTS_DASH_458,
            BraillePatterns::BraillePatternDotsDash1458 => BRAILLE_PATTERN_DOTS_DASH_1458,
            BraillePatterns::BraillePatternDotsDash2458 => BRAILLE_PATTERN_DOTS_DASH_2458,
            BraillePatterns::BraillePatternDotsDash12458 => BRAILLE_PATTERN_DOTS_DASH_12458,
            BraillePatterns::BraillePatternDotsDash3458 => BRAILLE_PATTERN_DOTS_DASH_3458,
            BraillePatterns::BraillePatternDotsDash13458 => BRAILLE_PATTERN_DOTS_DASH_13458,
            BraillePatterns::BraillePatternDotsDash23458 => BRAILLE_PATTERN_DOTS_DASH_23458,
            BraillePatterns::BraillePatternDotsDash123458 => BRAILLE_PATTERN_DOTS_DASH_123458,
            BraillePatterns::BraillePatternDotsDash68 => BRAILLE_PATTERN_DOTS_DASH_68,
            BraillePatterns::BraillePatternDotsDash168 => BRAILLE_PATTERN_DOTS_DASH_168,
            BraillePatterns::BraillePatternDotsDash268 => BRAILLE_PATTERN_DOTS_DASH_268,
            BraillePatterns::BraillePatternDotsDash1268 => BRAILLE_PATTERN_DOTS_DASH_1268,
            BraillePatterns::BraillePatternDotsDash368 => BRAILLE_PATTERN_DOTS_DASH_368,
            BraillePatterns::BraillePatternDotsDash1368 => BRAILLE_PATTERN_DOTS_DASH_1368,
            BraillePatterns::BraillePatternDotsDash2368 => BRAILLE_PATTERN_DOTS_DASH_2368,
            BraillePatterns::BraillePatternDotsDash12368 => BRAILLE_PATTERN_DOTS_DASH_12368,
            BraillePatterns::BraillePatternDotsDash468 => BRAILLE_PATTERN_DOTS_DASH_468,
            BraillePatterns::BraillePatternDotsDash1468 => BRAILLE_PATTERN_DOTS_DASH_1468,
            BraillePatterns::BraillePatternDotsDash2468 => BRAILLE_PATTERN_DOTS_DASH_2468,
            BraillePatterns::BraillePatternDotsDash12468 => BRAILLE_PATTERN_DOTS_DASH_12468,
            BraillePatterns::BraillePatternDotsDash3468 => BRAILLE_PATTERN_DOTS_DASH_3468,
            BraillePatterns::BraillePatternDotsDash13468 => BRAILLE_PATTERN_DOTS_DASH_13468,
            BraillePatterns::BraillePatternDotsDash23468 => BRAILLE_PATTERN_DOTS_DASH_23468,
            BraillePatterns::BraillePatternDotsDash123468 => BRAILLE_PATTERN_DOTS_DASH_123468,
            BraillePatterns::BraillePatternDotsDash568 => BRAILLE_PATTERN_DOTS_DASH_568,
            BraillePatterns::BraillePatternDotsDash1568 => BRAILLE_PATTERN_DOTS_DASH_1568,
            BraillePatterns::BraillePatternDotsDash2568 => BRAILLE_PATTERN_DOTS_DASH_2568,
            BraillePatterns::BraillePatternDotsDash12568 => BRAILLE_PATTERN_DOTS_DASH_12568,
            BraillePatterns::BraillePatternDotsDash3568 => BRAILLE_PATTERN_DOTS_DASH_3568,
            BraillePatterns::BraillePatternDotsDash13568 => BRAILLE_PATTERN_DOTS_DASH_13568,
            BraillePatterns::BraillePatternDotsDash23568 => BRAILLE_PATTERN_DOTS_DASH_23568,
            BraillePatterns::BraillePatternDotsDash123568 => BRAILLE_PATTERN_DOTS_DASH_123568,
            BraillePatterns::BraillePatternDotsDash4568 => BRAILLE_PATTERN_DOTS_DASH_4568,
            BraillePatterns::BraillePatternDotsDash14568 => BRAILLE_PATTERN_DOTS_DASH_14568,
            BraillePatterns::BraillePatternDotsDash24568 => BRAILLE_PATTERN_DOTS_DASH_24568,
            BraillePatterns::BraillePatternDotsDash124568 => BRAILLE_PATTERN_DOTS_DASH_124568,
            BraillePatterns::BraillePatternDotsDash34568 => BRAILLE_PATTERN_DOTS_DASH_34568,
            BraillePatterns::BraillePatternDotsDash134568 => BRAILLE_PATTERN_DOTS_DASH_134568,
            BraillePatterns::BraillePatternDotsDash234568 => BRAILLE_PATTERN_DOTS_DASH_234568,
            BraillePatterns::BraillePatternDotsDash1234568 => BRAILLE_PATTERN_DOTS_DASH_1234568,
            BraillePatterns::BraillePatternDotsDash78 => BRAILLE_PATTERN_DOTS_DASH_78,
            BraillePatterns::BraillePatternDotsDash178 => BRAILLE_PATTERN_DOTS_DASH_178,
            BraillePatterns::BraillePatternDotsDash278 => BRAILLE_PATTERN_DOTS_DASH_278,
            BraillePatterns::BraillePatternDotsDash1278 => BRAILLE_PATTERN_DOTS_DASH_1278,
            BraillePatterns::BraillePatternDotsDash378 => BRAILLE_PATTERN_DOTS_DASH_378,
            BraillePatterns::BraillePatternDotsDash1378 => BRAILLE_PATTERN_DOTS_DASH_1378,
            BraillePatterns::BraillePatternDotsDash2378 => BRAILLE_PATTERN_DOTS_DASH_2378,
            BraillePatterns::BraillePatternDotsDash12378 => BRAILLE_PATTERN_DOTS_DASH_12378,
            BraillePatterns::BraillePatternDotsDash478 => BRAILLE_PATTERN_DOTS_DASH_478,
            BraillePatterns::BraillePatternDotsDash1478 => BRAILLE_PATTERN_DOTS_DASH_1478,
            BraillePatterns::BraillePatternDotsDash2478 => BRAILLE_PATTERN_DOTS_DASH_2478,
            BraillePatterns::BraillePatternDotsDash12478 => BRAILLE_PATTERN_DOTS_DASH_12478,
            BraillePatterns::BraillePatternDotsDash3478 => BRAILLE_PATTERN_DOTS_DASH_3478,
            BraillePatterns::BraillePatternDotsDash13478 => BRAILLE_PATTERN_DOTS_DASH_13478,
            BraillePatterns::BraillePatternDotsDash23478 => BRAILLE_PATTERN_DOTS_DASH_23478,
            BraillePatterns::BraillePatternDotsDash123478 => BRAILLE_PATTERN_DOTS_DASH_123478,
            BraillePatterns::BraillePatternDotsDash578 => BRAILLE_PATTERN_DOTS_DASH_578,
            BraillePatterns::BraillePatternDotsDash1578 => BRAILLE_PATTERN_DOTS_DASH_1578,
            BraillePatterns::BraillePatternDotsDash2578 => BRAILLE_PATTERN_DOTS_DASH_2578,
            BraillePatterns::BraillePatternDotsDash12578 => BRAILLE_PATTERN_DOTS_DASH_12578,
            BraillePatterns::BraillePatternDotsDash3578 => BRAILLE_PATTERN_DOTS_DASH_3578,
            BraillePatterns::BraillePatternDotsDash13578 => BRAILLE_PATTERN_DOTS_DASH_13578,
            BraillePatterns::BraillePatternDotsDash23578 => BRAILLE_PATTERN_DOTS_DASH_23578,
            BraillePatterns::BraillePatternDotsDash123578 => BRAILLE_PATTERN_DOTS_DASH_123578,
            BraillePatterns::BraillePatternDotsDash4578 => BRAILLE_PATTERN_DOTS_DASH_4578,
            BraillePatterns::BraillePatternDotsDash14578 => BRAILLE_PATTERN_DOTS_DASH_14578,
            BraillePatterns::BraillePatternDotsDash24578 => BRAILLE_PATTERN_DOTS_DASH_24578,
            BraillePatterns::BraillePatternDotsDash124578 => BRAILLE_PATTERN_DOTS_DASH_124578,
            BraillePatterns::BraillePatternDotsDash34578 => BRAILLE_PATTERN_DOTS_DASH_34578,
            BraillePatterns::BraillePatternDotsDash134578 => BRAILLE_PATTERN_DOTS_DASH_134578,
            BraillePatterns::BraillePatternDotsDash234578 => BRAILLE_PATTERN_DOTS_DASH_234578,
            BraillePatterns::BraillePatternDotsDash1234578 => BRAILLE_PATTERN_DOTS_DASH_1234578,
            BraillePatterns::BraillePatternDotsDash678 => BRAILLE_PATTERN_DOTS_DASH_678,
            BraillePatterns::BraillePatternDotsDash1678 => BRAILLE_PATTERN_DOTS_DASH_1678,
            BraillePatterns::BraillePatternDotsDash2678 => BRAILLE_PATTERN_DOTS_DASH_2678,
            BraillePatterns::BraillePatternDotsDash12678 => BRAILLE_PATTERN_DOTS_DASH_12678,
            BraillePatterns::BraillePatternDotsDash3678 => BRAILLE_PATTERN_DOTS_DASH_3678,
            BraillePatterns::BraillePatternDotsDash13678 => BRAILLE_PATTERN_DOTS_DASH_13678,
            BraillePatterns::BraillePatternDotsDash23678 => BRAILLE_PATTERN_DOTS_DASH_23678,
            BraillePatterns::BraillePatternDotsDash123678 => BRAILLE_PATTERN_DOTS_DASH_123678,
            BraillePatterns::BraillePatternDotsDash4678 => BRAILLE_PATTERN_DOTS_DASH_4678,
            BraillePatterns::BraillePatternDotsDash14678 => BRAILLE_PATTERN_DOTS_DASH_14678,
            BraillePatterns::BraillePatternDotsDash24678 => BRAILLE_PATTERN_DOTS_DASH_24678,
            BraillePatterns::BraillePatternDotsDash124678 => BRAILLE_PATTERN_DOTS_DASH_124678,
            BraillePatterns::BraillePatternDotsDash34678 => BRAILLE_PATTERN_DOTS_DASH_34678,
            BraillePatterns::BraillePatternDotsDash134678 => BRAILLE_PATTERN_DOTS_DASH_134678,
            BraillePatterns::BraillePatternDotsDash234678 => BRAILLE_PATTERN_DOTS_DASH_234678,
            BraillePatterns::BraillePatternDotsDash1234678 => BRAILLE_PATTERN_DOTS_DASH_1234678,
            BraillePatterns::BraillePatternDotsDash5678 => BRAILLE_PATTERN_DOTS_DASH_5678,
            BraillePatterns::BraillePatternDotsDash15678 => BRAILLE_PATTERN_DOTS_DASH_15678,
            BraillePatterns::BraillePatternDotsDash25678 => BRAILLE_PATTERN_DOTS_DASH_25678,
            BraillePatterns::BraillePatternDotsDash125678 => BRAILLE_PATTERN_DOTS_DASH_125678,
            BraillePatterns::BraillePatternDotsDash35678 => BRAILLE_PATTERN_DOTS_DASH_35678,
            BraillePatterns::BraillePatternDotsDash135678 => BRAILLE_PATTERN_DOTS_DASH_135678,
            BraillePatterns::BraillePatternDotsDash235678 => BRAILLE_PATTERN_DOTS_DASH_235678,
            BraillePatterns::BraillePatternDotsDash1235678 => BRAILLE_PATTERN_DOTS_DASH_1235678,
            BraillePatterns::BraillePatternDotsDash45678 => BRAILLE_PATTERN_DOTS_DASH_45678,
            BraillePatterns::BraillePatternDotsDash145678 => BRAILLE_PATTERN_DOTS_DASH_145678,
            BraillePatterns::BraillePatternDotsDash245678 => BRAILLE_PATTERN_DOTS_DASH_245678,
            BraillePatterns::BraillePatternDotsDash1245678 => BRAILLE_PATTERN_DOTS_DASH_1245678,
            BraillePatterns::BraillePatternDotsDash345678 => BRAILLE_PATTERN_DOTS_DASH_345678,
            BraillePatterns::BraillePatternDotsDash1345678 => BRAILLE_PATTERN_DOTS_DASH_1345678,
            BraillePatterns::BraillePatternDotsDash2345678 => BRAILLE_PATTERN_DOTS_DASH_2345678,
        }
    }
}

impl std::convert::TryFrom<char> for BraillePatterns {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BRAILLE_PATTERN_BLANK => Ok(BraillePatterns::BraillePatternBlank),
            BRAILLE_PATTERN_DOTS_DASH_1 => Ok(BraillePatterns::BraillePatternDotsDash1),
            BRAILLE_PATTERN_DOTS_DASH_2 => Ok(BraillePatterns::BraillePatternDotsDash2),
            BRAILLE_PATTERN_DOTS_DASH_12 => Ok(BraillePatterns::BraillePatternDotsDash12),
            BRAILLE_PATTERN_DOTS_DASH_3 => Ok(BraillePatterns::BraillePatternDotsDash3),
            BRAILLE_PATTERN_DOTS_DASH_13 => Ok(BraillePatterns::BraillePatternDotsDash13),
            BRAILLE_PATTERN_DOTS_DASH_23 => Ok(BraillePatterns::BraillePatternDotsDash23),
            BRAILLE_PATTERN_DOTS_DASH_123 => Ok(BraillePatterns::BraillePatternDotsDash123),
            BRAILLE_PATTERN_DOTS_DASH_4 => Ok(BraillePatterns::BraillePatternDotsDash4),
            BRAILLE_PATTERN_DOTS_DASH_14 => Ok(BraillePatterns::BraillePatternDotsDash14),
            BRAILLE_PATTERN_DOTS_DASH_24 => Ok(BraillePatterns::BraillePatternDotsDash24),
            BRAILLE_PATTERN_DOTS_DASH_124 => Ok(BraillePatterns::BraillePatternDotsDash124),
            BRAILLE_PATTERN_DOTS_DASH_34 => Ok(BraillePatterns::BraillePatternDotsDash34),
            BRAILLE_PATTERN_DOTS_DASH_134 => Ok(BraillePatterns::BraillePatternDotsDash134),
            BRAILLE_PATTERN_DOTS_DASH_234 => Ok(BraillePatterns::BraillePatternDotsDash234),
            BRAILLE_PATTERN_DOTS_DASH_1234 => Ok(BraillePatterns::BraillePatternDotsDash1234),
            BRAILLE_PATTERN_DOTS_DASH_5 => Ok(BraillePatterns::BraillePatternDotsDash5),
            BRAILLE_PATTERN_DOTS_DASH_15 => Ok(BraillePatterns::BraillePatternDotsDash15),
            BRAILLE_PATTERN_DOTS_DASH_25 => Ok(BraillePatterns::BraillePatternDotsDash25),
            BRAILLE_PATTERN_DOTS_DASH_125 => Ok(BraillePatterns::BraillePatternDotsDash125),
            BRAILLE_PATTERN_DOTS_DASH_35 => Ok(BraillePatterns::BraillePatternDotsDash35),
            BRAILLE_PATTERN_DOTS_DASH_135 => Ok(BraillePatterns::BraillePatternDotsDash135),
            BRAILLE_PATTERN_DOTS_DASH_235 => Ok(BraillePatterns::BraillePatternDotsDash235),
            BRAILLE_PATTERN_DOTS_DASH_1235 => Ok(BraillePatterns::BraillePatternDotsDash1235),
            BRAILLE_PATTERN_DOTS_DASH_45 => Ok(BraillePatterns::BraillePatternDotsDash45),
            BRAILLE_PATTERN_DOTS_DASH_145 => Ok(BraillePatterns::BraillePatternDotsDash145),
            BRAILLE_PATTERN_DOTS_DASH_245 => Ok(BraillePatterns::BraillePatternDotsDash245),
            BRAILLE_PATTERN_DOTS_DASH_1245 => Ok(BraillePatterns::BraillePatternDotsDash1245),
            BRAILLE_PATTERN_DOTS_DASH_345 => Ok(BraillePatterns::BraillePatternDotsDash345),
            BRAILLE_PATTERN_DOTS_DASH_1345 => Ok(BraillePatterns::BraillePatternDotsDash1345),
            BRAILLE_PATTERN_DOTS_DASH_2345 => Ok(BraillePatterns::BraillePatternDotsDash2345),
            BRAILLE_PATTERN_DOTS_DASH_12345 => Ok(BraillePatterns::BraillePatternDotsDash12345),
            BRAILLE_PATTERN_DOTS_DASH_6 => Ok(BraillePatterns::BraillePatternDotsDash6),
            BRAILLE_PATTERN_DOTS_DASH_16 => Ok(BraillePatterns::BraillePatternDotsDash16),
            BRAILLE_PATTERN_DOTS_DASH_26 => Ok(BraillePatterns::BraillePatternDotsDash26),
            BRAILLE_PATTERN_DOTS_DASH_126 => Ok(BraillePatterns::BraillePatternDotsDash126),
            BRAILLE_PATTERN_DOTS_DASH_36 => Ok(BraillePatterns::BraillePatternDotsDash36),
            BRAILLE_PATTERN_DOTS_DASH_136 => Ok(BraillePatterns::BraillePatternDotsDash136),
            BRAILLE_PATTERN_DOTS_DASH_236 => Ok(BraillePatterns::BraillePatternDotsDash236),
            BRAILLE_PATTERN_DOTS_DASH_1236 => Ok(BraillePatterns::BraillePatternDotsDash1236),
            BRAILLE_PATTERN_DOTS_DASH_46 => Ok(BraillePatterns::BraillePatternDotsDash46),
            BRAILLE_PATTERN_DOTS_DASH_146 => Ok(BraillePatterns::BraillePatternDotsDash146),
            BRAILLE_PATTERN_DOTS_DASH_246 => Ok(BraillePatterns::BraillePatternDotsDash246),
            BRAILLE_PATTERN_DOTS_DASH_1246 => Ok(BraillePatterns::BraillePatternDotsDash1246),
            BRAILLE_PATTERN_DOTS_DASH_346 => Ok(BraillePatterns::BraillePatternDotsDash346),
            BRAILLE_PATTERN_DOTS_DASH_1346 => Ok(BraillePatterns::BraillePatternDotsDash1346),
            BRAILLE_PATTERN_DOTS_DASH_2346 => Ok(BraillePatterns::BraillePatternDotsDash2346),
            BRAILLE_PATTERN_DOTS_DASH_12346 => Ok(BraillePatterns::BraillePatternDotsDash12346),
            BRAILLE_PATTERN_DOTS_DASH_56 => Ok(BraillePatterns::BraillePatternDotsDash56),
            BRAILLE_PATTERN_DOTS_DASH_156 => Ok(BraillePatterns::BraillePatternDotsDash156),
            BRAILLE_PATTERN_DOTS_DASH_256 => Ok(BraillePatterns::BraillePatternDotsDash256),
            BRAILLE_PATTERN_DOTS_DASH_1256 => Ok(BraillePatterns::BraillePatternDotsDash1256),
            BRAILLE_PATTERN_DOTS_DASH_356 => Ok(BraillePatterns::BraillePatternDotsDash356),
            BRAILLE_PATTERN_DOTS_DASH_1356 => Ok(BraillePatterns::BraillePatternDotsDash1356),
            BRAILLE_PATTERN_DOTS_DASH_2356 => Ok(BraillePatterns::BraillePatternDotsDash2356),
            BRAILLE_PATTERN_DOTS_DASH_12356 => Ok(BraillePatterns::BraillePatternDotsDash12356),
            BRAILLE_PATTERN_DOTS_DASH_456 => Ok(BraillePatterns::BraillePatternDotsDash456),
            BRAILLE_PATTERN_DOTS_DASH_1456 => Ok(BraillePatterns::BraillePatternDotsDash1456),
            BRAILLE_PATTERN_DOTS_DASH_2456 => Ok(BraillePatterns::BraillePatternDotsDash2456),
            BRAILLE_PATTERN_DOTS_DASH_12456 => Ok(BraillePatterns::BraillePatternDotsDash12456),
            BRAILLE_PATTERN_DOTS_DASH_3456 => Ok(BraillePatterns::BraillePatternDotsDash3456),
            BRAILLE_PATTERN_DOTS_DASH_13456 => Ok(BraillePatterns::BraillePatternDotsDash13456),
            BRAILLE_PATTERN_DOTS_DASH_23456 => Ok(BraillePatterns::BraillePatternDotsDash23456),
            BRAILLE_PATTERN_DOTS_DASH_123456 => Ok(BraillePatterns::BraillePatternDotsDash123456),
            BRAILLE_PATTERN_DOTS_DASH_7 => Ok(BraillePatterns::BraillePatternDotsDash7),
            BRAILLE_PATTERN_DOTS_DASH_17 => Ok(BraillePatterns::BraillePatternDotsDash17),
            BRAILLE_PATTERN_DOTS_DASH_27 => Ok(BraillePatterns::BraillePatternDotsDash27),
            BRAILLE_PATTERN_DOTS_DASH_127 => Ok(BraillePatterns::BraillePatternDotsDash127),
            BRAILLE_PATTERN_DOTS_DASH_37 => Ok(BraillePatterns::BraillePatternDotsDash37),
            BRAILLE_PATTERN_DOTS_DASH_137 => Ok(BraillePatterns::BraillePatternDotsDash137),
            BRAILLE_PATTERN_DOTS_DASH_237 => Ok(BraillePatterns::BraillePatternDotsDash237),
            BRAILLE_PATTERN_DOTS_DASH_1237 => Ok(BraillePatterns::BraillePatternDotsDash1237),
            BRAILLE_PATTERN_DOTS_DASH_47 => Ok(BraillePatterns::BraillePatternDotsDash47),
            BRAILLE_PATTERN_DOTS_DASH_147 => Ok(BraillePatterns::BraillePatternDotsDash147),
            BRAILLE_PATTERN_DOTS_DASH_247 => Ok(BraillePatterns::BraillePatternDotsDash247),
            BRAILLE_PATTERN_DOTS_DASH_1247 => Ok(BraillePatterns::BraillePatternDotsDash1247),
            BRAILLE_PATTERN_DOTS_DASH_347 => Ok(BraillePatterns::BraillePatternDotsDash347),
            BRAILLE_PATTERN_DOTS_DASH_1347 => Ok(BraillePatterns::BraillePatternDotsDash1347),
            BRAILLE_PATTERN_DOTS_DASH_2347 => Ok(BraillePatterns::BraillePatternDotsDash2347),
            BRAILLE_PATTERN_DOTS_DASH_12347 => Ok(BraillePatterns::BraillePatternDotsDash12347),
            BRAILLE_PATTERN_DOTS_DASH_57 => Ok(BraillePatterns::BraillePatternDotsDash57),
            BRAILLE_PATTERN_DOTS_DASH_157 => Ok(BraillePatterns::BraillePatternDotsDash157),
            BRAILLE_PATTERN_DOTS_DASH_257 => Ok(BraillePatterns::BraillePatternDotsDash257),
            BRAILLE_PATTERN_DOTS_DASH_1257 => Ok(BraillePatterns::BraillePatternDotsDash1257),
            BRAILLE_PATTERN_DOTS_DASH_357 => Ok(BraillePatterns::BraillePatternDotsDash357),
            BRAILLE_PATTERN_DOTS_DASH_1357 => Ok(BraillePatterns::BraillePatternDotsDash1357),
            BRAILLE_PATTERN_DOTS_DASH_2357 => Ok(BraillePatterns::BraillePatternDotsDash2357),
            BRAILLE_PATTERN_DOTS_DASH_12357 => Ok(BraillePatterns::BraillePatternDotsDash12357),
            BRAILLE_PATTERN_DOTS_DASH_457 => Ok(BraillePatterns::BraillePatternDotsDash457),
            BRAILLE_PATTERN_DOTS_DASH_1457 => Ok(BraillePatterns::BraillePatternDotsDash1457),
            BRAILLE_PATTERN_DOTS_DASH_2457 => Ok(BraillePatterns::BraillePatternDotsDash2457),
            BRAILLE_PATTERN_DOTS_DASH_12457 => Ok(BraillePatterns::BraillePatternDotsDash12457),
            BRAILLE_PATTERN_DOTS_DASH_3457 => Ok(BraillePatterns::BraillePatternDotsDash3457),
            BRAILLE_PATTERN_DOTS_DASH_13457 => Ok(BraillePatterns::BraillePatternDotsDash13457),
            BRAILLE_PATTERN_DOTS_DASH_23457 => Ok(BraillePatterns::BraillePatternDotsDash23457),
            BRAILLE_PATTERN_DOTS_DASH_123457 => Ok(BraillePatterns::BraillePatternDotsDash123457),
            BRAILLE_PATTERN_DOTS_DASH_67 => Ok(BraillePatterns::BraillePatternDotsDash67),
            BRAILLE_PATTERN_DOTS_DASH_167 => Ok(BraillePatterns::BraillePatternDotsDash167),
            BRAILLE_PATTERN_DOTS_DASH_267 => Ok(BraillePatterns::BraillePatternDotsDash267),
            BRAILLE_PATTERN_DOTS_DASH_1267 => Ok(BraillePatterns::BraillePatternDotsDash1267),
            BRAILLE_PATTERN_DOTS_DASH_367 => Ok(BraillePatterns::BraillePatternDotsDash367),
            BRAILLE_PATTERN_DOTS_DASH_1367 => Ok(BraillePatterns::BraillePatternDotsDash1367),
            BRAILLE_PATTERN_DOTS_DASH_2367 => Ok(BraillePatterns::BraillePatternDotsDash2367),
            BRAILLE_PATTERN_DOTS_DASH_12367 => Ok(BraillePatterns::BraillePatternDotsDash12367),
            BRAILLE_PATTERN_DOTS_DASH_467 => Ok(BraillePatterns::BraillePatternDotsDash467),
            BRAILLE_PATTERN_DOTS_DASH_1467 => Ok(BraillePatterns::BraillePatternDotsDash1467),
            BRAILLE_PATTERN_DOTS_DASH_2467 => Ok(BraillePatterns::BraillePatternDotsDash2467),
            BRAILLE_PATTERN_DOTS_DASH_12467 => Ok(BraillePatterns::BraillePatternDotsDash12467),
            BRAILLE_PATTERN_DOTS_DASH_3467 => Ok(BraillePatterns::BraillePatternDotsDash3467),
            BRAILLE_PATTERN_DOTS_DASH_13467 => Ok(BraillePatterns::BraillePatternDotsDash13467),
            BRAILLE_PATTERN_DOTS_DASH_23467 => Ok(BraillePatterns::BraillePatternDotsDash23467),
            BRAILLE_PATTERN_DOTS_DASH_123467 => Ok(BraillePatterns::BraillePatternDotsDash123467),
            BRAILLE_PATTERN_DOTS_DASH_567 => Ok(BraillePatterns::BraillePatternDotsDash567),
            BRAILLE_PATTERN_DOTS_DASH_1567 => Ok(BraillePatterns::BraillePatternDotsDash1567),
            BRAILLE_PATTERN_DOTS_DASH_2567 => Ok(BraillePatterns::BraillePatternDotsDash2567),
            BRAILLE_PATTERN_DOTS_DASH_12567 => Ok(BraillePatterns::BraillePatternDotsDash12567),
            BRAILLE_PATTERN_DOTS_DASH_3567 => Ok(BraillePatterns::BraillePatternDotsDash3567),
            BRAILLE_PATTERN_DOTS_DASH_13567 => Ok(BraillePatterns::BraillePatternDotsDash13567),
            BRAILLE_PATTERN_DOTS_DASH_23567 => Ok(BraillePatterns::BraillePatternDotsDash23567),
            BRAILLE_PATTERN_DOTS_DASH_123567 => Ok(BraillePatterns::BraillePatternDotsDash123567),
            BRAILLE_PATTERN_DOTS_DASH_4567 => Ok(BraillePatterns::BraillePatternDotsDash4567),
            BRAILLE_PATTERN_DOTS_DASH_14567 => Ok(BraillePatterns::BraillePatternDotsDash14567),
            BRAILLE_PATTERN_DOTS_DASH_24567 => Ok(BraillePatterns::BraillePatternDotsDash24567),
            BRAILLE_PATTERN_DOTS_DASH_124567 => Ok(BraillePatterns::BraillePatternDotsDash124567),
            BRAILLE_PATTERN_DOTS_DASH_34567 => Ok(BraillePatterns::BraillePatternDotsDash34567),
            BRAILLE_PATTERN_DOTS_DASH_134567 => Ok(BraillePatterns::BraillePatternDotsDash134567),
            BRAILLE_PATTERN_DOTS_DASH_234567 => Ok(BraillePatterns::BraillePatternDotsDash234567),
            BRAILLE_PATTERN_DOTS_DASH_1234567 => Ok(BraillePatterns::BraillePatternDotsDash1234567),
            BRAILLE_PATTERN_DOTS_DASH_8 => Ok(BraillePatterns::BraillePatternDotsDash8),
            BRAILLE_PATTERN_DOTS_DASH_18 => Ok(BraillePatterns::BraillePatternDotsDash18),
            BRAILLE_PATTERN_DOTS_DASH_28 => Ok(BraillePatterns::BraillePatternDotsDash28),
            BRAILLE_PATTERN_DOTS_DASH_128 => Ok(BraillePatterns::BraillePatternDotsDash128),
            BRAILLE_PATTERN_DOTS_DASH_38 => Ok(BraillePatterns::BraillePatternDotsDash38),
            BRAILLE_PATTERN_DOTS_DASH_138 => Ok(BraillePatterns::BraillePatternDotsDash138),
            BRAILLE_PATTERN_DOTS_DASH_238 => Ok(BraillePatterns::BraillePatternDotsDash238),
            BRAILLE_PATTERN_DOTS_DASH_1238 => Ok(BraillePatterns::BraillePatternDotsDash1238),
            BRAILLE_PATTERN_DOTS_DASH_48 => Ok(BraillePatterns::BraillePatternDotsDash48),
            BRAILLE_PATTERN_DOTS_DASH_148 => Ok(BraillePatterns::BraillePatternDotsDash148),
            BRAILLE_PATTERN_DOTS_DASH_248 => Ok(BraillePatterns::BraillePatternDotsDash248),
            BRAILLE_PATTERN_DOTS_DASH_1248 => Ok(BraillePatterns::BraillePatternDotsDash1248),
            BRAILLE_PATTERN_DOTS_DASH_348 => Ok(BraillePatterns::BraillePatternDotsDash348),
            BRAILLE_PATTERN_DOTS_DASH_1348 => Ok(BraillePatterns::BraillePatternDotsDash1348),
            BRAILLE_PATTERN_DOTS_DASH_2348 => Ok(BraillePatterns::BraillePatternDotsDash2348),
            BRAILLE_PATTERN_DOTS_DASH_12348 => Ok(BraillePatterns::BraillePatternDotsDash12348),
            BRAILLE_PATTERN_DOTS_DASH_58 => Ok(BraillePatterns::BraillePatternDotsDash58),
            BRAILLE_PATTERN_DOTS_DASH_158 => Ok(BraillePatterns::BraillePatternDotsDash158),
            BRAILLE_PATTERN_DOTS_DASH_258 => Ok(BraillePatterns::BraillePatternDotsDash258),
            BRAILLE_PATTERN_DOTS_DASH_1258 => Ok(BraillePatterns::BraillePatternDotsDash1258),
            BRAILLE_PATTERN_DOTS_DASH_358 => Ok(BraillePatterns::BraillePatternDotsDash358),
            BRAILLE_PATTERN_DOTS_DASH_1358 => Ok(BraillePatterns::BraillePatternDotsDash1358),
            BRAILLE_PATTERN_DOTS_DASH_2358 => Ok(BraillePatterns::BraillePatternDotsDash2358),
            BRAILLE_PATTERN_DOTS_DASH_12358 => Ok(BraillePatterns::BraillePatternDotsDash12358),
            BRAILLE_PATTERN_DOTS_DASH_458 => Ok(BraillePatterns::BraillePatternDotsDash458),
            BRAILLE_PATTERN_DOTS_DASH_1458 => Ok(BraillePatterns::BraillePatternDotsDash1458),
            BRAILLE_PATTERN_DOTS_DASH_2458 => Ok(BraillePatterns::BraillePatternDotsDash2458),
            BRAILLE_PATTERN_DOTS_DASH_12458 => Ok(BraillePatterns::BraillePatternDotsDash12458),
            BRAILLE_PATTERN_DOTS_DASH_3458 => Ok(BraillePatterns::BraillePatternDotsDash3458),
            BRAILLE_PATTERN_DOTS_DASH_13458 => Ok(BraillePatterns::BraillePatternDotsDash13458),
            BRAILLE_PATTERN_DOTS_DASH_23458 => Ok(BraillePatterns::BraillePatternDotsDash23458),
            BRAILLE_PATTERN_DOTS_DASH_123458 => Ok(BraillePatterns::BraillePatternDotsDash123458),
            BRAILLE_PATTERN_DOTS_DASH_68 => Ok(BraillePatterns::BraillePatternDotsDash68),
            BRAILLE_PATTERN_DOTS_DASH_168 => Ok(BraillePatterns::BraillePatternDotsDash168),
            BRAILLE_PATTERN_DOTS_DASH_268 => Ok(BraillePatterns::BraillePatternDotsDash268),
            BRAILLE_PATTERN_DOTS_DASH_1268 => Ok(BraillePatterns::BraillePatternDotsDash1268),
            BRAILLE_PATTERN_DOTS_DASH_368 => Ok(BraillePatterns::BraillePatternDotsDash368),
            BRAILLE_PATTERN_DOTS_DASH_1368 => Ok(BraillePatterns::BraillePatternDotsDash1368),
            BRAILLE_PATTERN_DOTS_DASH_2368 => Ok(BraillePatterns::BraillePatternDotsDash2368),
            BRAILLE_PATTERN_DOTS_DASH_12368 => Ok(BraillePatterns::BraillePatternDotsDash12368),
            BRAILLE_PATTERN_DOTS_DASH_468 => Ok(BraillePatterns::BraillePatternDotsDash468),
            BRAILLE_PATTERN_DOTS_DASH_1468 => Ok(BraillePatterns::BraillePatternDotsDash1468),
            BRAILLE_PATTERN_DOTS_DASH_2468 => Ok(BraillePatterns::BraillePatternDotsDash2468),
            BRAILLE_PATTERN_DOTS_DASH_12468 => Ok(BraillePatterns::BraillePatternDotsDash12468),
            BRAILLE_PATTERN_DOTS_DASH_3468 => Ok(BraillePatterns::BraillePatternDotsDash3468),
            BRAILLE_PATTERN_DOTS_DASH_13468 => Ok(BraillePatterns::BraillePatternDotsDash13468),
            BRAILLE_PATTERN_DOTS_DASH_23468 => Ok(BraillePatterns::BraillePatternDotsDash23468),
            BRAILLE_PATTERN_DOTS_DASH_123468 => Ok(BraillePatterns::BraillePatternDotsDash123468),
            BRAILLE_PATTERN_DOTS_DASH_568 => Ok(BraillePatterns::BraillePatternDotsDash568),
            BRAILLE_PATTERN_DOTS_DASH_1568 => Ok(BraillePatterns::BraillePatternDotsDash1568),
            BRAILLE_PATTERN_DOTS_DASH_2568 => Ok(BraillePatterns::BraillePatternDotsDash2568),
            BRAILLE_PATTERN_DOTS_DASH_12568 => Ok(BraillePatterns::BraillePatternDotsDash12568),
            BRAILLE_PATTERN_DOTS_DASH_3568 => Ok(BraillePatterns::BraillePatternDotsDash3568),
            BRAILLE_PATTERN_DOTS_DASH_13568 => Ok(BraillePatterns::BraillePatternDotsDash13568),
            BRAILLE_PATTERN_DOTS_DASH_23568 => Ok(BraillePatterns::BraillePatternDotsDash23568),
            BRAILLE_PATTERN_DOTS_DASH_123568 => Ok(BraillePatterns::BraillePatternDotsDash123568),
            BRAILLE_PATTERN_DOTS_DASH_4568 => Ok(BraillePatterns::BraillePatternDotsDash4568),
            BRAILLE_PATTERN_DOTS_DASH_14568 => Ok(BraillePatterns::BraillePatternDotsDash14568),
            BRAILLE_PATTERN_DOTS_DASH_24568 => Ok(BraillePatterns::BraillePatternDotsDash24568),
            BRAILLE_PATTERN_DOTS_DASH_124568 => Ok(BraillePatterns::BraillePatternDotsDash124568),
            BRAILLE_PATTERN_DOTS_DASH_34568 => Ok(BraillePatterns::BraillePatternDotsDash34568),
            BRAILLE_PATTERN_DOTS_DASH_134568 => Ok(BraillePatterns::BraillePatternDotsDash134568),
            BRAILLE_PATTERN_DOTS_DASH_234568 => Ok(BraillePatterns::BraillePatternDotsDash234568),
            BRAILLE_PATTERN_DOTS_DASH_1234568 => Ok(BraillePatterns::BraillePatternDotsDash1234568),
            BRAILLE_PATTERN_DOTS_DASH_78 => Ok(BraillePatterns::BraillePatternDotsDash78),
            BRAILLE_PATTERN_DOTS_DASH_178 => Ok(BraillePatterns::BraillePatternDotsDash178),
            BRAILLE_PATTERN_DOTS_DASH_278 => Ok(BraillePatterns::BraillePatternDotsDash278),
            BRAILLE_PATTERN_DOTS_DASH_1278 => Ok(BraillePatterns::BraillePatternDotsDash1278),
            BRAILLE_PATTERN_DOTS_DASH_378 => Ok(BraillePatterns::BraillePatternDotsDash378),
            BRAILLE_PATTERN_DOTS_DASH_1378 => Ok(BraillePatterns::BraillePatternDotsDash1378),
            BRAILLE_PATTERN_DOTS_DASH_2378 => Ok(BraillePatterns::BraillePatternDotsDash2378),
            BRAILLE_PATTERN_DOTS_DASH_12378 => Ok(BraillePatterns::BraillePatternDotsDash12378),
            BRAILLE_PATTERN_DOTS_DASH_478 => Ok(BraillePatterns::BraillePatternDotsDash478),
            BRAILLE_PATTERN_DOTS_DASH_1478 => Ok(BraillePatterns::BraillePatternDotsDash1478),
            BRAILLE_PATTERN_DOTS_DASH_2478 => Ok(BraillePatterns::BraillePatternDotsDash2478),
            BRAILLE_PATTERN_DOTS_DASH_12478 => Ok(BraillePatterns::BraillePatternDotsDash12478),
            BRAILLE_PATTERN_DOTS_DASH_3478 => Ok(BraillePatterns::BraillePatternDotsDash3478),
            BRAILLE_PATTERN_DOTS_DASH_13478 => Ok(BraillePatterns::BraillePatternDotsDash13478),
            BRAILLE_PATTERN_DOTS_DASH_23478 => Ok(BraillePatterns::BraillePatternDotsDash23478),
            BRAILLE_PATTERN_DOTS_DASH_123478 => Ok(BraillePatterns::BraillePatternDotsDash123478),
            BRAILLE_PATTERN_DOTS_DASH_578 => Ok(BraillePatterns::BraillePatternDotsDash578),
            BRAILLE_PATTERN_DOTS_DASH_1578 => Ok(BraillePatterns::BraillePatternDotsDash1578),
            BRAILLE_PATTERN_DOTS_DASH_2578 => Ok(BraillePatterns::BraillePatternDotsDash2578),
            BRAILLE_PATTERN_DOTS_DASH_12578 => Ok(BraillePatterns::BraillePatternDotsDash12578),
            BRAILLE_PATTERN_DOTS_DASH_3578 => Ok(BraillePatterns::BraillePatternDotsDash3578),
            BRAILLE_PATTERN_DOTS_DASH_13578 => Ok(BraillePatterns::BraillePatternDotsDash13578),
            BRAILLE_PATTERN_DOTS_DASH_23578 => Ok(BraillePatterns::BraillePatternDotsDash23578),
            BRAILLE_PATTERN_DOTS_DASH_123578 => Ok(BraillePatterns::BraillePatternDotsDash123578),
            BRAILLE_PATTERN_DOTS_DASH_4578 => Ok(BraillePatterns::BraillePatternDotsDash4578),
            BRAILLE_PATTERN_DOTS_DASH_14578 => Ok(BraillePatterns::BraillePatternDotsDash14578),
            BRAILLE_PATTERN_DOTS_DASH_24578 => Ok(BraillePatterns::BraillePatternDotsDash24578),
            BRAILLE_PATTERN_DOTS_DASH_124578 => Ok(BraillePatterns::BraillePatternDotsDash124578),
            BRAILLE_PATTERN_DOTS_DASH_34578 => Ok(BraillePatterns::BraillePatternDotsDash34578),
            BRAILLE_PATTERN_DOTS_DASH_134578 => Ok(BraillePatterns::BraillePatternDotsDash134578),
            BRAILLE_PATTERN_DOTS_DASH_234578 => Ok(BraillePatterns::BraillePatternDotsDash234578),
            BRAILLE_PATTERN_DOTS_DASH_1234578 => Ok(BraillePatterns::BraillePatternDotsDash1234578),
            BRAILLE_PATTERN_DOTS_DASH_678 => Ok(BraillePatterns::BraillePatternDotsDash678),
            BRAILLE_PATTERN_DOTS_DASH_1678 => Ok(BraillePatterns::BraillePatternDotsDash1678),
            BRAILLE_PATTERN_DOTS_DASH_2678 => Ok(BraillePatterns::BraillePatternDotsDash2678),
            BRAILLE_PATTERN_DOTS_DASH_12678 => Ok(BraillePatterns::BraillePatternDotsDash12678),
            BRAILLE_PATTERN_DOTS_DASH_3678 => Ok(BraillePatterns::BraillePatternDotsDash3678),
            BRAILLE_PATTERN_DOTS_DASH_13678 => Ok(BraillePatterns::BraillePatternDotsDash13678),
            BRAILLE_PATTERN_DOTS_DASH_23678 => Ok(BraillePatterns::BraillePatternDotsDash23678),
            BRAILLE_PATTERN_DOTS_DASH_123678 => Ok(BraillePatterns::BraillePatternDotsDash123678),
            BRAILLE_PATTERN_DOTS_DASH_4678 => Ok(BraillePatterns::BraillePatternDotsDash4678),
            BRAILLE_PATTERN_DOTS_DASH_14678 => Ok(BraillePatterns::BraillePatternDotsDash14678),
            BRAILLE_PATTERN_DOTS_DASH_24678 => Ok(BraillePatterns::BraillePatternDotsDash24678),
            BRAILLE_PATTERN_DOTS_DASH_124678 => Ok(BraillePatterns::BraillePatternDotsDash124678),
            BRAILLE_PATTERN_DOTS_DASH_34678 => Ok(BraillePatterns::BraillePatternDotsDash34678),
            BRAILLE_PATTERN_DOTS_DASH_134678 => Ok(BraillePatterns::BraillePatternDotsDash134678),
            BRAILLE_PATTERN_DOTS_DASH_234678 => Ok(BraillePatterns::BraillePatternDotsDash234678),
            BRAILLE_PATTERN_DOTS_DASH_1234678 => Ok(BraillePatterns::BraillePatternDotsDash1234678),
            BRAILLE_PATTERN_DOTS_DASH_5678 => Ok(BraillePatterns::BraillePatternDotsDash5678),
            BRAILLE_PATTERN_DOTS_DASH_15678 => Ok(BraillePatterns::BraillePatternDotsDash15678),
            BRAILLE_PATTERN_DOTS_DASH_25678 => Ok(BraillePatterns::BraillePatternDotsDash25678),
            BRAILLE_PATTERN_DOTS_DASH_125678 => Ok(BraillePatterns::BraillePatternDotsDash125678),
            BRAILLE_PATTERN_DOTS_DASH_35678 => Ok(BraillePatterns::BraillePatternDotsDash35678),
            BRAILLE_PATTERN_DOTS_DASH_135678 => Ok(BraillePatterns::BraillePatternDotsDash135678),
            BRAILLE_PATTERN_DOTS_DASH_235678 => Ok(BraillePatterns::BraillePatternDotsDash235678),
            BRAILLE_PATTERN_DOTS_DASH_1235678 => Ok(BraillePatterns::BraillePatternDotsDash1235678),
            BRAILLE_PATTERN_DOTS_DASH_45678 => Ok(BraillePatterns::BraillePatternDotsDash45678),
            BRAILLE_PATTERN_DOTS_DASH_145678 => Ok(BraillePatterns::BraillePatternDotsDash145678),
            BRAILLE_PATTERN_DOTS_DASH_245678 => Ok(BraillePatterns::BraillePatternDotsDash245678),
            BRAILLE_PATTERN_DOTS_DASH_1245678 => Ok(BraillePatterns::BraillePatternDotsDash1245678),
            BRAILLE_PATTERN_DOTS_DASH_345678 => Ok(BraillePatterns::BraillePatternDotsDash345678),
            BRAILLE_PATTERN_DOTS_DASH_1345678 => Ok(BraillePatterns::BraillePatternDotsDash1345678),
            BRAILLE_PATTERN_DOTS_DASH_2345678 => Ok(BraillePatterns::BraillePatternDotsDash2345678),
            _ => Err(()),
        }
    }
}

impl Into<u32> for BraillePatterns {
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

impl std::convert::TryFrom<u32> for BraillePatterns {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for BraillePatterns {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl BraillePatterns {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        BraillePatterns::BraillePatternBlank
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BraillePatterns::BraillePatternBlank => "braille pattern blank",
            BraillePatterns::BraillePatternDotsDash1 => "braille pattern dots-1",
            BraillePatterns::BraillePatternDotsDash2 => "braille pattern dots-2",
            BraillePatterns::BraillePatternDotsDash12 => "braille pattern dots-12",
            BraillePatterns::BraillePatternDotsDash3 => "braille pattern dots-3",
            BraillePatterns::BraillePatternDotsDash13 => "braille pattern dots-13",
            BraillePatterns::BraillePatternDotsDash23 => "braille pattern dots-23",
            BraillePatterns::BraillePatternDotsDash123 => "braille pattern dots-123",
            BraillePatterns::BraillePatternDotsDash4 => "braille pattern dots-4",
            BraillePatterns::BraillePatternDotsDash14 => "braille pattern dots-14",
            BraillePatterns::BraillePatternDotsDash24 => "braille pattern dots-24",
            BraillePatterns::BraillePatternDotsDash124 => "braille pattern dots-124",
            BraillePatterns::BraillePatternDotsDash34 => "braille pattern dots-34",
            BraillePatterns::BraillePatternDotsDash134 => "braille pattern dots-134",
            BraillePatterns::BraillePatternDotsDash234 => "braille pattern dots-234",
            BraillePatterns::BraillePatternDotsDash1234 => "braille pattern dots-1234",
            BraillePatterns::BraillePatternDotsDash5 => "braille pattern dots-5",
            BraillePatterns::BraillePatternDotsDash15 => "braille pattern dots-15",
            BraillePatterns::BraillePatternDotsDash25 => "braille pattern dots-25",
            BraillePatterns::BraillePatternDotsDash125 => "braille pattern dots-125",
            BraillePatterns::BraillePatternDotsDash35 => "braille pattern dots-35",
            BraillePatterns::BraillePatternDotsDash135 => "braille pattern dots-135",
            BraillePatterns::BraillePatternDotsDash235 => "braille pattern dots-235",
            BraillePatterns::BraillePatternDotsDash1235 => "braille pattern dots-1235",
            BraillePatterns::BraillePatternDotsDash45 => "braille pattern dots-45",
            BraillePatterns::BraillePatternDotsDash145 => "braille pattern dots-145",
            BraillePatterns::BraillePatternDotsDash245 => "braille pattern dots-245",
            BraillePatterns::BraillePatternDotsDash1245 => "braille pattern dots-1245",
            BraillePatterns::BraillePatternDotsDash345 => "braille pattern dots-345",
            BraillePatterns::BraillePatternDotsDash1345 => "braille pattern dots-1345",
            BraillePatterns::BraillePatternDotsDash2345 => "braille pattern dots-2345",
            BraillePatterns::BraillePatternDotsDash12345 => "braille pattern dots-12345",
            BraillePatterns::BraillePatternDotsDash6 => "braille pattern dots-6",
            BraillePatterns::BraillePatternDotsDash16 => "braille pattern dots-16",
            BraillePatterns::BraillePatternDotsDash26 => "braille pattern dots-26",
            BraillePatterns::BraillePatternDotsDash126 => "braille pattern dots-126",
            BraillePatterns::BraillePatternDotsDash36 => "braille pattern dots-36",
            BraillePatterns::BraillePatternDotsDash136 => "braille pattern dots-136",
            BraillePatterns::BraillePatternDotsDash236 => "braille pattern dots-236",
            BraillePatterns::BraillePatternDotsDash1236 => "braille pattern dots-1236",
            BraillePatterns::BraillePatternDotsDash46 => "braille pattern dots-46",
            BraillePatterns::BraillePatternDotsDash146 => "braille pattern dots-146",
            BraillePatterns::BraillePatternDotsDash246 => "braille pattern dots-246",
            BraillePatterns::BraillePatternDotsDash1246 => "braille pattern dots-1246",
            BraillePatterns::BraillePatternDotsDash346 => "braille pattern dots-346",
            BraillePatterns::BraillePatternDotsDash1346 => "braille pattern dots-1346",
            BraillePatterns::BraillePatternDotsDash2346 => "braille pattern dots-2346",
            BraillePatterns::BraillePatternDotsDash12346 => "braille pattern dots-12346",
            BraillePatterns::BraillePatternDotsDash56 => "braille pattern dots-56",
            BraillePatterns::BraillePatternDotsDash156 => "braille pattern dots-156",
            BraillePatterns::BraillePatternDotsDash256 => "braille pattern dots-256",
            BraillePatterns::BraillePatternDotsDash1256 => "braille pattern dots-1256",
            BraillePatterns::BraillePatternDotsDash356 => "braille pattern dots-356",
            BraillePatterns::BraillePatternDotsDash1356 => "braille pattern dots-1356",
            BraillePatterns::BraillePatternDotsDash2356 => "braille pattern dots-2356",
            BraillePatterns::BraillePatternDotsDash12356 => "braille pattern dots-12356",
            BraillePatterns::BraillePatternDotsDash456 => "braille pattern dots-456",
            BraillePatterns::BraillePatternDotsDash1456 => "braille pattern dots-1456",
            BraillePatterns::BraillePatternDotsDash2456 => "braille pattern dots-2456",
            BraillePatterns::BraillePatternDotsDash12456 => "braille pattern dots-12456",
            BraillePatterns::BraillePatternDotsDash3456 => "braille pattern dots-3456",
            BraillePatterns::BraillePatternDotsDash13456 => "braille pattern dots-13456",
            BraillePatterns::BraillePatternDotsDash23456 => "braille pattern dots-23456",
            BraillePatterns::BraillePatternDotsDash123456 => "braille pattern dots-123456",
            BraillePatterns::BraillePatternDotsDash7 => "braille pattern dots-7",
            BraillePatterns::BraillePatternDotsDash17 => "braille pattern dots-17",
            BraillePatterns::BraillePatternDotsDash27 => "braille pattern dots-27",
            BraillePatterns::BraillePatternDotsDash127 => "braille pattern dots-127",
            BraillePatterns::BraillePatternDotsDash37 => "braille pattern dots-37",
            BraillePatterns::BraillePatternDotsDash137 => "braille pattern dots-137",
            BraillePatterns::BraillePatternDotsDash237 => "braille pattern dots-237",
            BraillePatterns::BraillePatternDotsDash1237 => "braille pattern dots-1237",
            BraillePatterns::BraillePatternDotsDash47 => "braille pattern dots-47",
            BraillePatterns::BraillePatternDotsDash147 => "braille pattern dots-147",
            BraillePatterns::BraillePatternDotsDash247 => "braille pattern dots-247",
            BraillePatterns::BraillePatternDotsDash1247 => "braille pattern dots-1247",
            BraillePatterns::BraillePatternDotsDash347 => "braille pattern dots-347",
            BraillePatterns::BraillePatternDotsDash1347 => "braille pattern dots-1347",
            BraillePatterns::BraillePatternDotsDash2347 => "braille pattern dots-2347",
            BraillePatterns::BraillePatternDotsDash12347 => "braille pattern dots-12347",
            BraillePatterns::BraillePatternDotsDash57 => "braille pattern dots-57",
            BraillePatterns::BraillePatternDotsDash157 => "braille pattern dots-157",
            BraillePatterns::BraillePatternDotsDash257 => "braille pattern dots-257",
            BraillePatterns::BraillePatternDotsDash1257 => "braille pattern dots-1257",
            BraillePatterns::BraillePatternDotsDash357 => "braille pattern dots-357",
            BraillePatterns::BraillePatternDotsDash1357 => "braille pattern dots-1357",
            BraillePatterns::BraillePatternDotsDash2357 => "braille pattern dots-2357",
            BraillePatterns::BraillePatternDotsDash12357 => "braille pattern dots-12357",
            BraillePatterns::BraillePatternDotsDash457 => "braille pattern dots-457",
            BraillePatterns::BraillePatternDotsDash1457 => "braille pattern dots-1457",
            BraillePatterns::BraillePatternDotsDash2457 => "braille pattern dots-2457",
            BraillePatterns::BraillePatternDotsDash12457 => "braille pattern dots-12457",
            BraillePatterns::BraillePatternDotsDash3457 => "braille pattern dots-3457",
            BraillePatterns::BraillePatternDotsDash13457 => "braille pattern dots-13457",
            BraillePatterns::BraillePatternDotsDash23457 => "braille pattern dots-23457",
            BraillePatterns::BraillePatternDotsDash123457 => "braille pattern dots-123457",
            BraillePatterns::BraillePatternDotsDash67 => "braille pattern dots-67",
            BraillePatterns::BraillePatternDotsDash167 => "braille pattern dots-167",
            BraillePatterns::BraillePatternDotsDash267 => "braille pattern dots-267",
            BraillePatterns::BraillePatternDotsDash1267 => "braille pattern dots-1267",
            BraillePatterns::BraillePatternDotsDash367 => "braille pattern dots-367",
            BraillePatterns::BraillePatternDotsDash1367 => "braille pattern dots-1367",
            BraillePatterns::BraillePatternDotsDash2367 => "braille pattern dots-2367",
            BraillePatterns::BraillePatternDotsDash12367 => "braille pattern dots-12367",
            BraillePatterns::BraillePatternDotsDash467 => "braille pattern dots-467",
            BraillePatterns::BraillePatternDotsDash1467 => "braille pattern dots-1467",
            BraillePatterns::BraillePatternDotsDash2467 => "braille pattern dots-2467",
            BraillePatterns::BraillePatternDotsDash12467 => "braille pattern dots-12467",
            BraillePatterns::BraillePatternDotsDash3467 => "braille pattern dots-3467",
            BraillePatterns::BraillePatternDotsDash13467 => "braille pattern dots-13467",
            BraillePatterns::BraillePatternDotsDash23467 => "braille pattern dots-23467",
            BraillePatterns::BraillePatternDotsDash123467 => "braille pattern dots-123467",
            BraillePatterns::BraillePatternDotsDash567 => "braille pattern dots-567",
            BraillePatterns::BraillePatternDotsDash1567 => "braille pattern dots-1567",
            BraillePatterns::BraillePatternDotsDash2567 => "braille pattern dots-2567",
            BraillePatterns::BraillePatternDotsDash12567 => "braille pattern dots-12567",
            BraillePatterns::BraillePatternDotsDash3567 => "braille pattern dots-3567",
            BraillePatterns::BraillePatternDotsDash13567 => "braille pattern dots-13567",
            BraillePatterns::BraillePatternDotsDash23567 => "braille pattern dots-23567",
            BraillePatterns::BraillePatternDotsDash123567 => "braille pattern dots-123567",
            BraillePatterns::BraillePatternDotsDash4567 => "braille pattern dots-4567",
            BraillePatterns::BraillePatternDotsDash14567 => "braille pattern dots-14567",
            BraillePatterns::BraillePatternDotsDash24567 => "braille pattern dots-24567",
            BraillePatterns::BraillePatternDotsDash124567 => "braille pattern dots-124567",
            BraillePatterns::BraillePatternDotsDash34567 => "braille pattern dots-34567",
            BraillePatterns::BraillePatternDotsDash134567 => "braille pattern dots-134567",
            BraillePatterns::BraillePatternDotsDash234567 => "braille pattern dots-234567",
            BraillePatterns::BraillePatternDotsDash1234567 => "braille pattern dots-1234567",
            BraillePatterns::BraillePatternDotsDash8 => "braille pattern dots-8",
            BraillePatterns::BraillePatternDotsDash18 => "braille pattern dots-18",
            BraillePatterns::BraillePatternDotsDash28 => "braille pattern dots-28",
            BraillePatterns::BraillePatternDotsDash128 => "braille pattern dots-128",
            BraillePatterns::BraillePatternDotsDash38 => "braille pattern dots-38",
            BraillePatterns::BraillePatternDotsDash138 => "braille pattern dots-138",
            BraillePatterns::BraillePatternDotsDash238 => "braille pattern dots-238",
            BraillePatterns::BraillePatternDotsDash1238 => "braille pattern dots-1238",
            BraillePatterns::BraillePatternDotsDash48 => "braille pattern dots-48",
            BraillePatterns::BraillePatternDotsDash148 => "braille pattern dots-148",
            BraillePatterns::BraillePatternDotsDash248 => "braille pattern dots-248",
            BraillePatterns::BraillePatternDotsDash1248 => "braille pattern dots-1248",
            BraillePatterns::BraillePatternDotsDash348 => "braille pattern dots-348",
            BraillePatterns::BraillePatternDotsDash1348 => "braille pattern dots-1348",
            BraillePatterns::BraillePatternDotsDash2348 => "braille pattern dots-2348",
            BraillePatterns::BraillePatternDotsDash12348 => "braille pattern dots-12348",
            BraillePatterns::BraillePatternDotsDash58 => "braille pattern dots-58",
            BraillePatterns::BraillePatternDotsDash158 => "braille pattern dots-158",
            BraillePatterns::BraillePatternDotsDash258 => "braille pattern dots-258",
            BraillePatterns::BraillePatternDotsDash1258 => "braille pattern dots-1258",
            BraillePatterns::BraillePatternDotsDash358 => "braille pattern dots-358",
            BraillePatterns::BraillePatternDotsDash1358 => "braille pattern dots-1358",
            BraillePatterns::BraillePatternDotsDash2358 => "braille pattern dots-2358",
            BraillePatterns::BraillePatternDotsDash12358 => "braille pattern dots-12358",
            BraillePatterns::BraillePatternDotsDash458 => "braille pattern dots-458",
            BraillePatterns::BraillePatternDotsDash1458 => "braille pattern dots-1458",
            BraillePatterns::BraillePatternDotsDash2458 => "braille pattern dots-2458",
            BraillePatterns::BraillePatternDotsDash12458 => "braille pattern dots-12458",
            BraillePatterns::BraillePatternDotsDash3458 => "braille pattern dots-3458",
            BraillePatterns::BraillePatternDotsDash13458 => "braille pattern dots-13458",
            BraillePatterns::BraillePatternDotsDash23458 => "braille pattern dots-23458",
            BraillePatterns::BraillePatternDotsDash123458 => "braille pattern dots-123458",
            BraillePatterns::BraillePatternDotsDash68 => "braille pattern dots-68",
            BraillePatterns::BraillePatternDotsDash168 => "braille pattern dots-168",
            BraillePatterns::BraillePatternDotsDash268 => "braille pattern dots-268",
            BraillePatterns::BraillePatternDotsDash1268 => "braille pattern dots-1268",
            BraillePatterns::BraillePatternDotsDash368 => "braille pattern dots-368",
            BraillePatterns::BraillePatternDotsDash1368 => "braille pattern dots-1368",
            BraillePatterns::BraillePatternDotsDash2368 => "braille pattern dots-2368",
            BraillePatterns::BraillePatternDotsDash12368 => "braille pattern dots-12368",
            BraillePatterns::BraillePatternDotsDash468 => "braille pattern dots-468",
            BraillePatterns::BraillePatternDotsDash1468 => "braille pattern dots-1468",
            BraillePatterns::BraillePatternDotsDash2468 => "braille pattern dots-2468",
            BraillePatterns::BraillePatternDotsDash12468 => "braille pattern dots-12468",
            BraillePatterns::BraillePatternDotsDash3468 => "braille pattern dots-3468",
            BraillePatterns::BraillePatternDotsDash13468 => "braille pattern dots-13468",
            BraillePatterns::BraillePatternDotsDash23468 => "braille pattern dots-23468",
            BraillePatterns::BraillePatternDotsDash123468 => "braille pattern dots-123468",
            BraillePatterns::BraillePatternDotsDash568 => "braille pattern dots-568",
            BraillePatterns::BraillePatternDotsDash1568 => "braille pattern dots-1568",
            BraillePatterns::BraillePatternDotsDash2568 => "braille pattern dots-2568",
            BraillePatterns::BraillePatternDotsDash12568 => "braille pattern dots-12568",
            BraillePatterns::BraillePatternDotsDash3568 => "braille pattern dots-3568",
            BraillePatterns::BraillePatternDotsDash13568 => "braille pattern dots-13568",
            BraillePatterns::BraillePatternDotsDash23568 => "braille pattern dots-23568",
            BraillePatterns::BraillePatternDotsDash123568 => "braille pattern dots-123568",
            BraillePatterns::BraillePatternDotsDash4568 => "braille pattern dots-4568",
            BraillePatterns::BraillePatternDotsDash14568 => "braille pattern dots-14568",
            BraillePatterns::BraillePatternDotsDash24568 => "braille pattern dots-24568",
            BraillePatterns::BraillePatternDotsDash124568 => "braille pattern dots-124568",
            BraillePatterns::BraillePatternDotsDash34568 => "braille pattern dots-34568",
            BraillePatterns::BraillePatternDotsDash134568 => "braille pattern dots-134568",
            BraillePatterns::BraillePatternDotsDash234568 => "braille pattern dots-234568",
            BraillePatterns::BraillePatternDotsDash1234568 => "braille pattern dots-1234568",
            BraillePatterns::BraillePatternDotsDash78 => "braille pattern dots-78",
            BraillePatterns::BraillePatternDotsDash178 => "braille pattern dots-178",
            BraillePatterns::BraillePatternDotsDash278 => "braille pattern dots-278",
            BraillePatterns::BraillePatternDotsDash1278 => "braille pattern dots-1278",
            BraillePatterns::BraillePatternDotsDash378 => "braille pattern dots-378",
            BraillePatterns::BraillePatternDotsDash1378 => "braille pattern dots-1378",
            BraillePatterns::BraillePatternDotsDash2378 => "braille pattern dots-2378",
            BraillePatterns::BraillePatternDotsDash12378 => "braille pattern dots-12378",
            BraillePatterns::BraillePatternDotsDash478 => "braille pattern dots-478",
            BraillePatterns::BraillePatternDotsDash1478 => "braille pattern dots-1478",
            BraillePatterns::BraillePatternDotsDash2478 => "braille pattern dots-2478",
            BraillePatterns::BraillePatternDotsDash12478 => "braille pattern dots-12478",
            BraillePatterns::BraillePatternDotsDash3478 => "braille pattern dots-3478",
            BraillePatterns::BraillePatternDotsDash13478 => "braille pattern dots-13478",
            BraillePatterns::BraillePatternDotsDash23478 => "braille pattern dots-23478",
            BraillePatterns::BraillePatternDotsDash123478 => "braille pattern dots-123478",
            BraillePatterns::BraillePatternDotsDash578 => "braille pattern dots-578",
            BraillePatterns::BraillePatternDotsDash1578 => "braille pattern dots-1578",
            BraillePatterns::BraillePatternDotsDash2578 => "braille pattern dots-2578",
            BraillePatterns::BraillePatternDotsDash12578 => "braille pattern dots-12578",
            BraillePatterns::BraillePatternDotsDash3578 => "braille pattern dots-3578",
            BraillePatterns::BraillePatternDotsDash13578 => "braille pattern dots-13578",
            BraillePatterns::BraillePatternDotsDash23578 => "braille pattern dots-23578",
            BraillePatterns::BraillePatternDotsDash123578 => "braille pattern dots-123578",
            BraillePatterns::BraillePatternDotsDash4578 => "braille pattern dots-4578",
            BraillePatterns::BraillePatternDotsDash14578 => "braille pattern dots-14578",
            BraillePatterns::BraillePatternDotsDash24578 => "braille pattern dots-24578",
            BraillePatterns::BraillePatternDotsDash124578 => "braille pattern dots-124578",
            BraillePatterns::BraillePatternDotsDash34578 => "braille pattern dots-34578",
            BraillePatterns::BraillePatternDotsDash134578 => "braille pattern dots-134578",
            BraillePatterns::BraillePatternDotsDash234578 => "braille pattern dots-234578",
            BraillePatterns::BraillePatternDotsDash1234578 => "braille pattern dots-1234578",
            BraillePatterns::BraillePatternDotsDash678 => "braille pattern dots-678",
            BraillePatterns::BraillePatternDotsDash1678 => "braille pattern dots-1678",
            BraillePatterns::BraillePatternDotsDash2678 => "braille pattern dots-2678",
            BraillePatterns::BraillePatternDotsDash12678 => "braille pattern dots-12678",
            BraillePatterns::BraillePatternDotsDash3678 => "braille pattern dots-3678",
            BraillePatterns::BraillePatternDotsDash13678 => "braille pattern dots-13678",
            BraillePatterns::BraillePatternDotsDash23678 => "braille pattern dots-23678",
            BraillePatterns::BraillePatternDotsDash123678 => "braille pattern dots-123678",
            BraillePatterns::BraillePatternDotsDash4678 => "braille pattern dots-4678",
            BraillePatterns::BraillePatternDotsDash14678 => "braille pattern dots-14678",
            BraillePatterns::BraillePatternDotsDash24678 => "braille pattern dots-24678",
            BraillePatterns::BraillePatternDotsDash124678 => "braille pattern dots-124678",
            BraillePatterns::BraillePatternDotsDash34678 => "braille pattern dots-34678",
            BraillePatterns::BraillePatternDotsDash134678 => "braille pattern dots-134678",
            BraillePatterns::BraillePatternDotsDash234678 => "braille pattern dots-234678",
            BraillePatterns::BraillePatternDotsDash1234678 => "braille pattern dots-1234678",
            BraillePatterns::BraillePatternDotsDash5678 => "braille pattern dots-5678",
            BraillePatterns::BraillePatternDotsDash15678 => "braille pattern dots-15678",
            BraillePatterns::BraillePatternDotsDash25678 => "braille pattern dots-25678",
            BraillePatterns::BraillePatternDotsDash125678 => "braille pattern dots-125678",
            BraillePatterns::BraillePatternDotsDash35678 => "braille pattern dots-35678",
            BraillePatterns::BraillePatternDotsDash135678 => "braille pattern dots-135678",
            BraillePatterns::BraillePatternDotsDash235678 => "braille pattern dots-235678",
            BraillePatterns::BraillePatternDotsDash1235678 => "braille pattern dots-1235678",
            BraillePatterns::BraillePatternDotsDash45678 => "braille pattern dots-45678",
            BraillePatterns::BraillePatternDotsDash145678 => "braille pattern dots-145678",
            BraillePatterns::BraillePatternDotsDash245678 => "braille pattern dots-245678",
            BraillePatterns::BraillePatternDotsDash1245678 => "braille pattern dots-1245678",
            BraillePatterns::BraillePatternDotsDash345678 => "braille pattern dots-345678",
            BraillePatterns::BraillePatternDotsDash1345678 => "braille pattern dots-1345678",
            BraillePatterns::BraillePatternDotsDash2345678 => "braille pattern dots-2345678",
        }
    }
}
