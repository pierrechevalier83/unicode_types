
/// An enum to represent all characters in the HangulJamo block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HangulJamo {
    /// \u{1100}: 'ᄀ'
    HangulChoseongKiyeok,
    /// \u{1101}: 'ᄁ'
    HangulChoseongSsangkiyeok,
    /// \u{1102}: 'ᄂ'
    HangulChoseongNieun,
    /// \u{1103}: 'ᄃ'
    HangulChoseongTikeut,
    /// \u{1104}: 'ᄄ'
    HangulChoseongSsangtikeut,
    /// \u{1105}: 'ᄅ'
    HangulChoseongRieul,
    /// \u{1106}: 'ᄆ'
    HangulChoseongMieum,
    /// \u{1107}: 'ᄇ'
    HangulChoseongPieup,
    /// \u{1108}: 'ᄈ'
    HangulChoseongSsangpieup,
    /// \u{1109}: 'ᄉ'
    HangulChoseongSios,
    /// \u{110a}: 'ᄊ'
    HangulChoseongSsangsios,
    /// \u{110b}: 'ᄋ'
    HangulChoseongIeung,
    /// \u{110c}: 'ᄌ'
    HangulChoseongCieuc,
    /// \u{110d}: 'ᄍ'
    HangulChoseongSsangcieuc,
    /// \u{110e}: 'ᄎ'
    HangulChoseongChieuch,
    /// \u{110f}: 'ᄏ'
    HangulChoseongKhieukh,
    /// \u{1110}: 'ᄐ'
    HangulChoseongThieuth,
    /// \u{1111}: 'ᄑ'
    HangulChoseongPhieuph,
    /// \u{1112}: 'ᄒ'
    HangulChoseongHieuh,
    /// \u{1113}: 'ᄓ'
    HangulChoseongNieunDashKiyeok,
    /// \u{1114}: 'ᄔ'
    HangulChoseongSsangnieun,
    /// \u{1115}: 'ᄕ'
    HangulChoseongNieunDashTikeut,
    /// \u{1116}: 'ᄖ'
    HangulChoseongNieunDashPieup,
    /// \u{1117}: 'ᄗ'
    HangulChoseongTikeutDashKiyeok,
    /// \u{1118}: 'ᄘ'
    HangulChoseongRieulDashNieun,
    /// \u{1119}: 'ᄙ'
    HangulChoseongSsangrieul,
    /// \u{111a}: 'ᄚ'
    HangulChoseongRieulDashHieuh,
    /// \u{111b}: 'ᄛ'
    HangulChoseongKapyeounrieul,
    /// \u{111c}: 'ᄜ'
    HangulChoseongMieumDashPieup,
    /// \u{111d}: 'ᄝ'
    HangulChoseongKapyeounmieum,
    /// \u{111e}: 'ᄞ'
    HangulChoseongPieupDashKiyeok,
    /// \u{111f}: 'ᄟ'
    HangulChoseongPieupDashNieun,
    /// \u{1120}: 'ᄠ'
    HangulChoseongPieupDashTikeut,
    /// \u{1121}: 'ᄡ'
    HangulChoseongPieupDashSios,
    /// \u{1122}: 'ᄢ'
    HangulChoseongPieupDashSiosDashKiyeok,
    /// \u{1123}: 'ᄣ'
    HangulChoseongPieupDashSiosDashTikeut,
    /// \u{1124}: 'ᄤ'
    HangulChoseongPieupDashSiosDashPieup,
    /// \u{1125}: 'ᄥ'
    HangulChoseongPieupDashSsangsios,
    /// \u{1126}: 'ᄦ'
    HangulChoseongPieupDashSiosDashCieuc,
    /// \u{1127}: 'ᄧ'
    HangulChoseongPieupDashCieuc,
    /// \u{1128}: 'ᄨ'
    HangulChoseongPieupDashChieuch,
    /// \u{1129}: 'ᄩ'
    HangulChoseongPieupDashThieuth,
    /// \u{112a}: 'ᄪ'
    HangulChoseongPieupDashPhieuph,
    /// \u{112b}: 'ᄫ'
    HangulChoseongKapyeounpieup,
    /// \u{112c}: 'ᄬ'
    HangulChoseongKapyeounssangpieup,
    /// \u{112d}: 'ᄭ'
    HangulChoseongSiosDashKiyeok,
    /// \u{112e}: 'ᄮ'
    HangulChoseongSiosDashNieun,
    /// \u{112f}: 'ᄯ'
    HangulChoseongSiosDashTikeut,
    /// \u{1130}: 'ᄰ'
    HangulChoseongSiosDashRieul,
    /// \u{1131}: 'ᄱ'
    HangulChoseongSiosDashMieum,
    /// \u{1132}: 'ᄲ'
    HangulChoseongSiosDashPieup,
    /// \u{1133}: 'ᄳ'
    HangulChoseongSiosDashPieupDashKiyeok,
    /// \u{1134}: 'ᄴ'
    HangulChoseongSiosDashSsangsios,
    /// \u{1135}: 'ᄵ'
    HangulChoseongSiosDashIeung,
    /// \u{1136}: 'ᄶ'
    HangulChoseongSiosDashCieuc,
    /// \u{1137}: 'ᄷ'
    HangulChoseongSiosDashChieuch,
    /// \u{1138}: 'ᄸ'
    HangulChoseongSiosDashKhieukh,
    /// \u{1139}: 'ᄹ'
    HangulChoseongSiosDashThieuth,
    /// \u{113a}: 'ᄺ'
    HangulChoseongSiosDashPhieuph,
    /// \u{113b}: 'ᄻ'
    HangulChoseongSiosDashHieuh,
    /// \u{113c}: 'ᄼ'
    HangulChoseongChitueumsios,
    /// \u{113d}: 'ᄽ'
    HangulChoseongChitueumssangsios,
    /// \u{113e}: 'ᄾ'
    HangulChoseongCeongchieumsios,
    /// \u{113f}: 'ᄿ'
    HangulChoseongCeongchieumssangsios,
    /// \u{1140}: 'ᅀ'
    HangulChoseongPansios,
    /// \u{1141}: 'ᅁ'
    HangulChoseongIeungDashKiyeok,
    /// \u{1142}: 'ᅂ'
    HangulChoseongIeungDashTikeut,
    /// \u{1143}: 'ᅃ'
    HangulChoseongIeungDashMieum,
    /// \u{1144}: 'ᅄ'
    HangulChoseongIeungDashPieup,
    /// \u{1145}: 'ᅅ'
    HangulChoseongIeungDashSios,
    /// \u{1146}: 'ᅆ'
    HangulChoseongIeungDashPansios,
    /// \u{1147}: 'ᅇ'
    HangulChoseongSsangieung,
    /// \u{1148}: 'ᅈ'
    HangulChoseongIeungDashCieuc,
    /// \u{1149}: 'ᅉ'
    HangulChoseongIeungDashChieuch,
    /// \u{114a}: 'ᅊ'
    HangulChoseongIeungDashThieuth,
    /// \u{114b}: 'ᅋ'
    HangulChoseongIeungDashPhieuph,
    /// \u{114c}: 'ᅌ'
    HangulChoseongYesieung,
    /// \u{114d}: 'ᅍ'
    HangulChoseongCieucDashIeung,
    /// \u{114e}: 'ᅎ'
    HangulChoseongChitueumcieuc,
    /// \u{114f}: 'ᅏ'
    HangulChoseongChitueumssangcieuc,
    /// \u{1150}: 'ᅐ'
    HangulChoseongCeongchieumcieuc,
    /// \u{1151}: 'ᅑ'
    HangulChoseongCeongchieumssangcieuc,
    /// \u{1152}: 'ᅒ'
    HangulChoseongChieuchDashKhieukh,
    /// \u{1153}: 'ᅓ'
    HangulChoseongChieuchDashHieuh,
    /// \u{1154}: 'ᅔ'
    HangulChoseongChitueumchieuch,
    /// \u{1155}: 'ᅕ'
    HangulChoseongCeongchieumchieuch,
    /// \u{1156}: 'ᅖ'
    HangulChoseongPhieuphDashPieup,
    /// \u{1157}: 'ᅗ'
    HangulChoseongKapyeounphieuph,
    /// \u{1158}: 'ᅘ'
    HangulChoseongSsanghieuh,
    /// \u{1159}: 'ᅙ'
    HangulChoseongYeorinhieuh,
    /// \u{115a}: 'ᅚ'
    HangulChoseongKiyeokDashTikeut,
    /// \u{115b}: 'ᅛ'
    HangulChoseongNieunDashSios,
    /// \u{115c}: 'ᅜ'
    HangulChoseongNieunDashCieuc,
    /// \u{115d}: 'ᅝ'
    HangulChoseongNieunDashHieuh,
    /// \u{115e}: 'ᅞ'
    HangulChoseongTikeutDashRieul,
    /// \u{115f}: 'ᅟ'
    HangulChoseongFiller,
    /// \u{1160}: 'ᅠ'
    HangulJungseongFiller,
    /// \u{1161}: 'ᅡ'
    HangulJungseongA,
    /// \u{1162}: 'ᅢ'
    HangulJungseongAe,
    /// \u{1163}: 'ᅣ'
    HangulJungseongYa,
    /// \u{1164}: 'ᅤ'
    HangulJungseongYae,
    /// \u{1165}: 'ᅥ'
    HangulJungseongEo,
    /// \u{1166}: 'ᅦ'
    HangulJungseongE,
    /// \u{1167}: 'ᅧ'
    HangulJungseongYeo,
    /// \u{1168}: 'ᅨ'
    HangulJungseongYe,
    /// \u{1169}: 'ᅩ'
    HangulJungseongO,
    /// \u{116a}: 'ᅪ'
    HangulJungseongWa,
    /// \u{116b}: 'ᅫ'
    HangulJungseongWae,
    /// \u{116c}: 'ᅬ'
    HangulJungseongOe,
    /// \u{116d}: 'ᅭ'
    HangulJungseongYo,
    /// \u{116e}: 'ᅮ'
    HangulJungseongU,
    /// \u{116f}: 'ᅯ'
    HangulJungseongWeo,
    /// \u{1170}: 'ᅰ'
    HangulJungseongWe,
    /// \u{1171}: 'ᅱ'
    HangulJungseongWi,
    /// \u{1172}: 'ᅲ'
    HangulJungseongYu,
    /// \u{1173}: 'ᅳ'
    HangulJungseongEu,
    /// \u{1174}: 'ᅴ'
    HangulJungseongYi,
    /// \u{1175}: 'ᅵ'
    HangulJungseongI,
    /// \u{1176}: 'ᅶ'
    HangulJungseongADashO,
    /// \u{1177}: 'ᅷ'
    HangulJungseongADashU,
    /// \u{1178}: 'ᅸ'
    HangulJungseongYaDashO,
    /// \u{1179}: 'ᅹ'
    HangulJungseongYaDashYo,
    /// \u{117a}: 'ᅺ'
    HangulJungseongEoDashO,
    /// \u{117b}: 'ᅻ'
    HangulJungseongEoDashU,
    /// \u{117c}: 'ᅼ'
    HangulJungseongEoDashEu,
    /// \u{117d}: 'ᅽ'
    HangulJungseongYeoDashO,
    /// \u{117e}: 'ᅾ'
    HangulJungseongYeoDashU,
    /// \u{117f}: 'ᅿ'
    HangulJungseongODashEo,
    /// \u{1180}: 'ᆀ'
    HangulJungseongODashE,
    /// \u{1181}: 'ᆁ'
    HangulJungseongODashYe,
    /// \u{1182}: 'ᆂ'
    HangulJungseongODashO,
    /// \u{1183}: 'ᆃ'
    HangulJungseongODashU,
    /// \u{1184}: 'ᆄ'
    HangulJungseongYoDashYa,
    /// \u{1185}: 'ᆅ'
    HangulJungseongYoDashYae,
    /// \u{1186}: 'ᆆ'
    HangulJungseongYoDashYeo,
    /// \u{1187}: 'ᆇ'
    HangulJungseongYoDashO,
    /// \u{1188}: 'ᆈ'
    HangulJungseongYoDashI,
    /// \u{1189}: 'ᆉ'
    HangulJungseongUDashA,
    /// \u{118a}: 'ᆊ'
    HangulJungseongUDashAe,
    /// \u{118b}: 'ᆋ'
    HangulJungseongUDashEoDashEu,
    /// \u{118c}: 'ᆌ'
    HangulJungseongUDashYe,
    /// \u{118d}: 'ᆍ'
    HangulJungseongUDashU,
    /// \u{118e}: 'ᆎ'
    HangulJungseongYuDashA,
    /// \u{118f}: 'ᆏ'
    HangulJungseongYuDashEo,
    /// \u{1190}: 'ᆐ'
    HangulJungseongYuDashE,
    /// \u{1191}: 'ᆑ'
    HangulJungseongYuDashYeo,
    /// \u{1192}: 'ᆒ'
    HangulJungseongYuDashYe,
    /// \u{1193}: 'ᆓ'
    HangulJungseongYuDashU,
    /// \u{1194}: 'ᆔ'
    HangulJungseongYuDashI,
    /// \u{1195}: 'ᆕ'
    HangulJungseongEuDashU,
    /// \u{1196}: 'ᆖ'
    HangulJungseongEuDashEu,
    /// \u{1197}: 'ᆗ'
    HangulJungseongYiDashU,
    /// \u{1198}: 'ᆘ'
    HangulJungseongIDashA,
    /// \u{1199}: 'ᆙ'
    HangulJungseongIDashYa,
    /// \u{119a}: 'ᆚ'
    HangulJungseongIDashO,
    /// \u{119b}: 'ᆛ'
    HangulJungseongIDashU,
    /// \u{119c}: 'ᆜ'
    HangulJungseongIDashEu,
    /// \u{119d}: 'ᆝ'
    HangulJungseongIDashAraea,
    /// \u{119e}: 'ᆞ'
    HangulJungseongAraea,
    /// \u{119f}: 'ᆟ'
    HangulJungseongAraeaDashEo,
    /// \u{11a0}: 'ᆠ'
    HangulJungseongAraeaDashU,
    /// \u{11a1}: 'ᆡ'
    HangulJungseongAraeaDashI,
    /// \u{11a2}: 'ᆢ'
    HangulJungseongSsangaraea,
    /// \u{11a3}: 'ᆣ'
    HangulJungseongADashEu,
    /// \u{11a4}: 'ᆤ'
    HangulJungseongYaDashU,
    /// \u{11a5}: 'ᆥ'
    HangulJungseongYeoDashYa,
    /// \u{11a6}: 'ᆦ'
    HangulJungseongODashYa,
    /// \u{11a7}: 'ᆧ'
    HangulJungseongODashYae,
    /// \u{11a8}: 'ᆨ'
    HangulJongseongKiyeok,
    /// \u{11a9}: 'ᆩ'
    HangulJongseongSsangkiyeok,
    /// \u{11aa}: 'ᆪ'
    HangulJongseongKiyeokDashSios,
    /// \u{11ab}: 'ᆫ'
    HangulJongseongNieun,
    /// \u{11ac}: 'ᆬ'
    HangulJongseongNieunDashCieuc,
    /// \u{11ad}: 'ᆭ'
    HangulJongseongNieunDashHieuh,
    /// \u{11ae}: 'ᆮ'
    HangulJongseongTikeut,
    /// \u{11af}: 'ᆯ'
    HangulJongseongRieul,
    /// \u{11b0}: 'ᆰ'
    HangulJongseongRieulDashKiyeok,
    /// \u{11b1}: 'ᆱ'
    HangulJongseongRieulDashMieum,
    /// \u{11b2}: 'ᆲ'
    HangulJongseongRieulDashPieup,
    /// \u{11b3}: 'ᆳ'
    HangulJongseongRieulDashSios,
    /// \u{11b4}: 'ᆴ'
    HangulJongseongRieulDashThieuth,
    /// \u{11b5}: 'ᆵ'
    HangulJongseongRieulDashPhieuph,
    /// \u{11b6}: 'ᆶ'
    HangulJongseongRieulDashHieuh,
    /// \u{11b7}: 'ᆷ'
    HangulJongseongMieum,
    /// \u{11b8}: 'ᆸ'
    HangulJongseongPieup,
    /// \u{11b9}: 'ᆹ'
    HangulJongseongPieupDashSios,
    /// \u{11ba}: 'ᆺ'
    HangulJongseongSios,
    /// \u{11bb}: 'ᆻ'
    HangulJongseongSsangsios,
    /// \u{11bc}: 'ᆼ'
    HangulJongseongIeung,
    /// \u{11bd}: 'ᆽ'
    HangulJongseongCieuc,
    /// \u{11be}: 'ᆾ'
    HangulJongseongChieuch,
    /// \u{11bf}: 'ᆿ'
    HangulJongseongKhieukh,
    /// \u{11c0}: 'ᇀ'
    HangulJongseongThieuth,
    /// \u{11c1}: 'ᇁ'
    HangulJongseongPhieuph,
    /// \u{11c2}: 'ᇂ'
    HangulJongseongHieuh,
    /// \u{11c3}: 'ᇃ'
    HangulJongseongKiyeokDashRieul,
    /// \u{11c4}: 'ᇄ'
    HangulJongseongKiyeokDashSiosDashKiyeok,
    /// \u{11c5}: 'ᇅ'
    HangulJongseongNieunDashKiyeok,
    /// \u{11c6}: 'ᇆ'
    HangulJongseongNieunDashTikeut,
    /// \u{11c7}: 'ᇇ'
    HangulJongseongNieunDashSios,
    /// \u{11c8}: 'ᇈ'
    HangulJongseongNieunDashPansios,
    /// \u{11c9}: 'ᇉ'
    HangulJongseongNieunDashThieuth,
    /// \u{11ca}: 'ᇊ'
    HangulJongseongTikeutDashKiyeok,
    /// \u{11cb}: 'ᇋ'
    HangulJongseongTikeutDashRieul,
    /// \u{11cc}: 'ᇌ'
    HangulJongseongRieulDashKiyeokDashSios,
    /// \u{11cd}: 'ᇍ'
    HangulJongseongRieulDashNieun,
    /// \u{11ce}: 'ᇎ'
    HangulJongseongRieulDashTikeut,
    /// \u{11cf}: 'ᇏ'
    HangulJongseongRieulDashTikeutDashHieuh,
    /// \u{11d0}: 'ᇐ'
    HangulJongseongSsangrieul,
    /// \u{11d1}: 'ᇑ'
    HangulJongseongRieulDashMieumDashKiyeok,
    /// \u{11d2}: 'ᇒ'
    HangulJongseongRieulDashMieumDashSios,
    /// \u{11d3}: 'ᇓ'
    HangulJongseongRieulDashPieupDashSios,
    /// \u{11d4}: 'ᇔ'
    HangulJongseongRieulDashPieupDashHieuh,
    /// \u{11d5}: 'ᇕ'
    HangulJongseongRieulDashKapyeounpieup,
    /// \u{11d6}: 'ᇖ'
    HangulJongseongRieulDashSsangsios,
    /// \u{11d7}: 'ᇗ'
    HangulJongseongRieulDashPansios,
    /// \u{11d8}: 'ᇘ'
    HangulJongseongRieulDashKhieukh,
    /// \u{11d9}: 'ᇙ'
    HangulJongseongRieulDashYeorinhieuh,
    /// \u{11da}: 'ᇚ'
    HangulJongseongMieumDashKiyeok,
    /// \u{11db}: 'ᇛ'
    HangulJongseongMieumDashRieul,
    /// \u{11dc}: 'ᇜ'
    HangulJongseongMieumDashPieup,
    /// \u{11dd}: 'ᇝ'
    HangulJongseongMieumDashSios,
    /// \u{11de}: 'ᇞ'
    HangulJongseongMieumDashSsangsios,
    /// \u{11df}: 'ᇟ'
    HangulJongseongMieumDashPansios,
    /// \u{11e0}: 'ᇠ'
    HangulJongseongMieumDashChieuch,
    /// \u{11e1}: 'ᇡ'
    HangulJongseongMieumDashHieuh,
    /// \u{11e2}: 'ᇢ'
    HangulJongseongKapyeounmieum,
    /// \u{11e3}: 'ᇣ'
    HangulJongseongPieupDashRieul,
    /// \u{11e4}: 'ᇤ'
    HangulJongseongPieupDashPhieuph,
    /// \u{11e5}: 'ᇥ'
    HangulJongseongPieupDashHieuh,
    /// \u{11e6}: 'ᇦ'
    HangulJongseongKapyeounpieup,
    /// \u{11e7}: 'ᇧ'
    HangulJongseongSiosDashKiyeok,
    /// \u{11e8}: 'ᇨ'
    HangulJongseongSiosDashTikeut,
    /// \u{11e9}: 'ᇩ'
    HangulJongseongSiosDashRieul,
    /// \u{11ea}: 'ᇪ'
    HangulJongseongSiosDashPieup,
    /// \u{11eb}: 'ᇫ'
    HangulJongseongPansios,
    /// \u{11ec}: 'ᇬ'
    HangulJongseongIeungDashKiyeok,
    /// \u{11ed}: 'ᇭ'
    HangulJongseongIeungDashSsangkiyeok,
    /// \u{11ee}: 'ᇮ'
    HangulJongseongSsangieung,
    /// \u{11ef}: 'ᇯ'
    HangulJongseongIeungDashKhieukh,
    /// \u{11f0}: 'ᇰ'
    HangulJongseongYesieung,
    /// \u{11f1}: 'ᇱ'
    HangulJongseongYesieungDashSios,
    /// \u{11f2}: 'ᇲ'
    HangulJongseongYesieungDashPansios,
    /// \u{11f3}: 'ᇳ'
    HangulJongseongPhieuphDashPieup,
    /// \u{11f4}: 'ᇴ'
    HangulJongseongKapyeounphieuph,
    /// \u{11f5}: 'ᇵ'
    HangulJongseongHieuhDashNieun,
    /// \u{11f6}: 'ᇶ'
    HangulJongseongHieuhDashRieul,
    /// \u{11f7}: 'ᇷ'
    HangulJongseongHieuhDashMieum,
    /// \u{11f8}: 'ᇸ'
    HangulJongseongHieuhDashPieup,
    /// \u{11f9}: 'ᇹ'
    HangulJongseongYeorinhieuh,
    /// \u{11fa}: 'ᇺ'
    HangulJongseongKiyeokDashNieun,
    /// \u{11fb}: 'ᇻ'
    HangulJongseongKiyeokDashPieup,
    /// \u{11fc}: 'ᇼ'
    HangulJongseongKiyeokDashChieuch,
    /// \u{11fd}: 'ᇽ'
    HangulJongseongKiyeokDashKhieukh,
    /// \u{11fe}: 'ᇾ'
    HangulJongseongKiyeokDashHieuh,
}

impl Into<char> for HangulJamo {
    fn into(self) -> char {
        match self {
            HangulJamo::HangulChoseongKiyeok => 'ᄀ',
            HangulJamo::HangulChoseongSsangkiyeok => 'ᄁ',
            HangulJamo::HangulChoseongNieun => 'ᄂ',
            HangulJamo::HangulChoseongTikeut => 'ᄃ',
            HangulJamo::HangulChoseongSsangtikeut => 'ᄄ',
            HangulJamo::HangulChoseongRieul => 'ᄅ',
            HangulJamo::HangulChoseongMieum => 'ᄆ',
            HangulJamo::HangulChoseongPieup => 'ᄇ',
            HangulJamo::HangulChoseongSsangpieup => 'ᄈ',
            HangulJamo::HangulChoseongSios => 'ᄉ',
            HangulJamo::HangulChoseongSsangsios => 'ᄊ',
            HangulJamo::HangulChoseongIeung => 'ᄋ',
            HangulJamo::HangulChoseongCieuc => 'ᄌ',
            HangulJamo::HangulChoseongSsangcieuc => 'ᄍ',
            HangulJamo::HangulChoseongChieuch => 'ᄎ',
            HangulJamo::HangulChoseongKhieukh => 'ᄏ',
            HangulJamo::HangulChoseongThieuth => 'ᄐ',
            HangulJamo::HangulChoseongPhieuph => 'ᄑ',
            HangulJamo::HangulChoseongHieuh => 'ᄒ',
            HangulJamo::HangulChoseongNieunDashKiyeok => 'ᄓ',
            HangulJamo::HangulChoseongSsangnieun => 'ᄔ',
            HangulJamo::HangulChoseongNieunDashTikeut => 'ᄕ',
            HangulJamo::HangulChoseongNieunDashPieup => 'ᄖ',
            HangulJamo::HangulChoseongTikeutDashKiyeok => 'ᄗ',
            HangulJamo::HangulChoseongRieulDashNieun => 'ᄘ',
            HangulJamo::HangulChoseongSsangrieul => 'ᄙ',
            HangulJamo::HangulChoseongRieulDashHieuh => 'ᄚ',
            HangulJamo::HangulChoseongKapyeounrieul => 'ᄛ',
            HangulJamo::HangulChoseongMieumDashPieup => 'ᄜ',
            HangulJamo::HangulChoseongKapyeounmieum => 'ᄝ',
            HangulJamo::HangulChoseongPieupDashKiyeok => 'ᄞ',
            HangulJamo::HangulChoseongPieupDashNieun => 'ᄟ',
            HangulJamo::HangulChoseongPieupDashTikeut => 'ᄠ',
            HangulJamo::HangulChoseongPieupDashSios => 'ᄡ',
            HangulJamo::HangulChoseongPieupDashSiosDashKiyeok => 'ᄢ',
            HangulJamo::HangulChoseongPieupDashSiosDashTikeut => 'ᄣ',
            HangulJamo::HangulChoseongPieupDashSiosDashPieup => 'ᄤ',
            HangulJamo::HangulChoseongPieupDashSsangsios => 'ᄥ',
            HangulJamo::HangulChoseongPieupDashSiosDashCieuc => 'ᄦ',
            HangulJamo::HangulChoseongPieupDashCieuc => 'ᄧ',
            HangulJamo::HangulChoseongPieupDashChieuch => 'ᄨ',
            HangulJamo::HangulChoseongPieupDashThieuth => 'ᄩ',
            HangulJamo::HangulChoseongPieupDashPhieuph => 'ᄪ',
            HangulJamo::HangulChoseongKapyeounpieup => 'ᄫ',
            HangulJamo::HangulChoseongKapyeounssangpieup => 'ᄬ',
            HangulJamo::HangulChoseongSiosDashKiyeok => 'ᄭ',
            HangulJamo::HangulChoseongSiosDashNieun => 'ᄮ',
            HangulJamo::HangulChoseongSiosDashTikeut => 'ᄯ',
            HangulJamo::HangulChoseongSiosDashRieul => 'ᄰ',
            HangulJamo::HangulChoseongSiosDashMieum => 'ᄱ',
            HangulJamo::HangulChoseongSiosDashPieup => 'ᄲ',
            HangulJamo::HangulChoseongSiosDashPieupDashKiyeok => 'ᄳ',
            HangulJamo::HangulChoseongSiosDashSsangsios => 'ᄴ',
            HangulJamo::HangulChoseongSiosDashIeung => 'ᄵ',
            HangulJamo::HangulChoseongSiosDashCieuc => 'ᄶ',
            HangulJamo::HangulChoseongSiosDashChieuch => 'ᄷ',
            HangulJamo::HangulChoseongSiosDashKhieukh => 'ᄸ',
            HangulJamo::HangulChoseongSiosDashThieuth => 'ᄹ',
            HangulJamo::HangulChoseongSiosDashPhieuph => 'ᄺ',
            HangulJamo::HangulChoseongSiosDashHieuh => 'ᄻ',
            HangulJamo::HangulChoseongChitueumsios => 'ᄼ',
            HangulJamo::HangulChoseongChitueumssangsios => 'ᄽ',
            HangulJamo::HangulChoseongCeongchieumsios => 'ᄾ',
            HangulJamo::HangulChoseongCeongchieumssangsios => 'ᄿ',
            HangulJamo::HangulChoseongPansios => 'ᅀ',
            HangulJamo::HangulChoseongIeungDashKiyeok => 'ᅁ',
            HangulJamo::HangulChoseongIeungDashTikeut => 'ᅂ',
            HangulJamo::HangulChoseongIeungDashMieum => 'ᅃ',
            HangulJamo::HangulChoseongIeungDashPieup => 'ᅄ',
            HangulJamo::HangulChoseongIeungDashSios => 'ᅅ',
            HangulJamo::HangulChoseongIeungDashPansios => 'ᅆ',
            HangulJamo::HangulChoseongSsangieung => 'ᅇ',
            HangulJamo::HangulChoseongIeungDashCieuc => 'ᅈ',
            HangulJamo::HangulChoseongIeungDashChieuch => 'ᅉ',
            HangulJamo::HangulChoseongIeungDashThieuth => 'ᅊ',
            HangulJamo::HangulChoseongIeungDashPhieuph => 'ᅋ',
            HangulJamo::HangulChoseongYesieung => 'ᅌ',
            HangulJamo::HangulChoseongCieucDashIeung => 'ᅍ',
            HangulJamo::HangulChoseongChitueumcieuc => 'ᅎ',
            HangulJamo::HangulChoseongChitueumssangcieuc => 'ᅏ',
            HangulJamo::HangulChoseongCeongchieumcieuc => 'ᅐ',
            HangulJamo::HangulChoseongCeongchieumssangcieuc => 'ᅑ',
            HangulJamo::HangulChoseongChieuchDashKhieukh => 'ᅒ',
            HangulJamo::HangulChoseongChieuchDashHieuh => 'ᅓ',
            HangulJamo::HangulChoseongChitueumchieuch => 'ᅔ',
            HangulJamo::HangulChoseongCeongchieumchieuch => 'ᅕ',
            HangulJamo::HangulChoseongPhieuphDashPieup => 'ᅖ',
            HangulJamo::HangulChoseongKapyeounphieuph => 'ᅗ',
            HangulJamo::HangulChoseongSsanghieuh => 'ᅘ',
            HangulJamo::HangulChoseongYeorinhieuh => 'ᅙ',
            HangulJamo::HangulChoseongKiyeokDashTikeut => 'ᅚ',
            HangulJamo::HangulChoseongNieunDashSios => 'ᅛ',
            HangulJamo::HangulChoseongNieunDashCieuc => 'ᅜ',
            HangulJamo::HangulChoseongNieunDashHieuh => 'ᅝ',
            HangulJamo::HangulChoseongTikeutDashRieul => 'ᅞ',
            HangulJamo::HangulChoseongFiller => 'ᅟ',
            HangulJamo::HangulJungseongFiller => 'ᅠ',
            HangulJamo::HangulJungseongA => 'ᅡ',
            HangulJamo::HangulJungseongAe => 'ᅢ',
            HangulJamo::HangulJungseongYa => 'ᅣ',
            HangulJamo::HangulJungseongYae => 'ᅤ',
            HangulJamo::HangulJungseongEo => 'ᅥ',
            HangulJamo::HangulJungseongE => 'ᅦ',
            HangulJamo::HangulJungseongYeo => 'ᅧ',
            HangulJamo::HangulJungseongYe => 'ᅨ',
            HangulJamo::HangulJungseongO => 'ᅩ',
            HangulJamo::HangulJungseongWa => 'ᅪ',
            HangulJamo::HangulJungseongWae => 'ᅫ',
            HangulJamo::HangulJungseongOe => 'ᅬ',
            HangulJamo::HangulJungseongYo => 'ᅭ',
            HangulJamo::HangulJungseongU => 'ᅮ',
            HangulJamo::HangulJungseongWeo => 'ᅯ',
            HangulJamo::HangulJungseongWe => 'ᅰ',
            HangulJamo::HangulJungseongWi => 'ᅱ',
            HangulJamo::HangulJungseongYu => 'ᅲ',
            HangulJamo::HangulJungseongEu => 'ᅳ',
            HangulJamo::HangulJungseongYi => 'ᅴ',
            HangulJamo::HangulJungseongI => 'ᅵ',
            HangulJamo::HangulJungseongADashO => 'ᅶ',
            HangulJamo::HangulJungseongADashU => 'ᅷ',
            HangulJamo::HangulJungseongYaDashO => 'ᅸ',
            HangulJamo::HangulJungseongYaDashYo => 'ᅹ',
            HangulJamo::HangulJungseongEoDashO => 'ᅺ',
            HangulJamo::HangulJungseongEoDashU => 'ᅻ',
            HangulJamo::HangulJungseongEoDashEu => 'ᅼ',
            HangulJamo::HangulJungseongYeoDashO => 'ᅽ',
            HangulJamo::HangulJungseongYeoDashU => 'ᅾ',
            HangulJamo::HangulJungseongODashEo => 'ᅿ',
            HangulJamo::HangulJungseongODashE => 'ᆀ',
            HangulJamo::HangulJungseongODashYe => 'ᆁ',
            HangulJamo::HangulJungseongODashO => 'ᆂ',
            HangulJamo::HangulJungseongODashU => 'ᆃ',
            HangulJamo::HangulJungseongYoDashYa => 'ᆄ',
            HangulJamo::HangulJungseongYoDashYae => 'ᆅ',
            HangulJamo::HangulJungseongYoDashYeo => 'ᆆ',
            HangulJamo::HangulJungseongYoDashO => 'ᆇ',
            HangulJamo::HangulJungseongYoDashI => 'ᆈ',
            HangulJamo::HangulJungseongUDashA => 'ᆉ',
            HangulJamo::HangulJungseongUDashAe => 'ᆊ',
            HangulJamo::HangulJungseongUDashEoDashEu => 'ᆋ',
            HangulJamo::HangulJungseongUDashYe => 'ᆌ',
            HangulJamo::HangulJungseongUDashU => 'ᆍ',
            HangulJamo::HangulJungseongYuDashA => 'ᆎ',
            HangulJamo::HangulJungseongYuDashEo => 'ᆏ',
            HangulJamo::HangulJungseongYuDashE => 'ᆐ',
            HangulJamo::HangulJungseongYuDashYeo => 'ᆑ',
            HangulJamo::HangulJungseongYuDashYe => 'ᆒ',
            HangulJamo::HangulJungseongYuDashU => 'ᆓ',
            HangulJamo::HangulJungseongYuDashI => 'ᆔ',
            HangulJamo::HangulJungseongEuDashU => 'ᆕ',
            HangulJamo::HangulJungseongEuDashEu => 'ᆖ',
            HangulJamo::HangulJungseongYiDashU => 'ᆗ',
            HangulJamo::HangulJungseongIDashA => 'ᆘ',
            HangulJamo::HangulJungseongIDashYa => 'ᆙ',
            HangulJamo::HangulJungseongIDashO => 'ᆚ',
            HangulJamo::HangulJungseongIDashU => 'ᆛ',
            HangulJamo::HangulJungseongIDashEu => 'ᆜ',
            HangulJamo::HangulJungseongIDashAraea => 'ᆝ',
            HangulJamo::HangulJungseongAraea => 'ᆞ',
            HangulJamo::HangulJungseongAraeaDashEo => 'ᆟ',
            HangulJamo::HangulJungseongAraeaDashU => 'ᆠ',
            HangulJamo::HangulJungseongAraeaDashI => 'ᆡ',
            HangulJamo::HangulJungseongSsangaraea => 'ᆢ',
            HangulJamo::HangulJungseongADashEu => 'ᆣ',
            HangulJamo::HangulJungseongYaDashU => 'ᆤ',
            HangulJamo::HangulJungseongYeoDashYa => 'ᆥ',
            HangulJamo::HangulJungseongODashYa => 'ᆦ',
            HangulJamo::HangulJungseongODashYae => 'ᆧ',
            HangulJamo::HangulJongseongKiyeok => 'ᆨ',
            HangulJamo::HangulJongseongSsangkiyeok => 'ᆩ',
            HangulJamo::HangulJongseongKiyeokDashSios => 'ᆪ',
            HangulJamo::HangulJongseongNieun => 'ᆫ',
            HangulJamo::HangulJongseongNieunDashCieuc => 'ᆬ',
            HangulJamo::HangulJongseongNieunDashHieuh => 'ᆭ',
            HangulJamo::HangulJongseongTikeut => 'ᆮ',
            HangulJamo::HangulJongseongRieul => 'ᆯ',
            HangulJamo::HangulJongseongRieulDashKiyeok => 'ᆰ',
            HangulJamo::HangulJongseongRieulDashMieum => 'ᆱ',
            HangulJamo::HangulJongseongRieulDashPieup => 'ᆲ',
            HangulJamo::HangulJongseongRieulDashSios => 'ᆳ',
            HangulJamo::HangulJongseongRieulDashThieuth => 'ᆴ',
            HangulJamo::HangulJongseongRieulDashPhieuph => 'ᆵ',
            HangulJamo::HangulJongseongRieulDashHieuh => 'ᆶ',
            HangulJamo::HangulJongseongMieum => 'ᆷ',
            HangulJamo::HangulJongseongPieup => 'ᆸ',
            HangulJamo::HangulJongseongPieupDashSios => 'ᆹ',
            HangulJamo::HangulJongseongSios => 'ᆺ',
            HangulJamo::HangulJongseongSsangsios => 'ᆻ',
            HangulJamo::HangulJongseongIeung => 'ᆼ',
            HangulJamo::HangulJongseongCieuc => 'ᆽ',
            HangulJamo::HangulJongseongChieuch => 'ᆾ',
            HangulJamo::HangulJongseongKhieukh => 'ᆿ',
            HangulJamo::HangulJongseongThieuth => 'ᇀ',
            HangulJamo::HangulJongseongPhieuph => 'ᇁ',
            HangulJamo::HangulJongseongHieuh => 'ᇂ',
            HangulJamo::HangulJongseongKiyeokDashRieul => 'ᇃ',
            HangulJamo::HangulJongseongKiyeokDashSiosDashKiyeok => 'ᇄ',
            HangulJamo::HangulJongseongNieunDashKiyeok => 'ᇅ',
            HangulJamo::HangulJongseongNieunDashTikeut => 'ᇆ',
            HangulJamo::HangulJongseongNieunDashSios => 'ᇇ',
            HangulJamo::HangulJongseongNieunDashPansios => 'ᇈ',
            HangulJamo::HangulJongseongNieunDashThieuth => 'ᇉ',
            HangulJamo::HangulJongseongTikeutDashKiyeok => 'ᇊ',
            HangulJamo::HangulJongseongTikeutDashRieul => 'ᇋ',
            HangulJamo::HangulJongseongRieulDashKiyeokDashSios => 'ᇌ',
            HangulJamo::HangulJongseongRieulDashNieun => 'ᇍ',
            HangulJamo::HangulJongseongRieulDashTikeut => 'ᇎ',
            HangulJamo::HangulJongseongRieulDashTikeutDashHieuh => 'ᇏ',
            HangulJamo::HangulJongseongSsangrieul => 'ᇐ',
            HangulJamo::HangulJongseongRieulDashMieumDashKiyeok => 'ᇑ',
            HangulJamo::HangulJongseongRieulDashMieumDashSios => 'ᇒ',
            HangulJamo::HangulJongseongRieulDashPieupDashSios => 'ᇓ',
            HangulJamo::HangulJongseongRieulDashPieupDashHieuh => 'ᇔ',
            HangulJamo::HangulJongseongRieulDashKapyeounpieup => 'ᇕ',
            HangulJamo::HangulJongseongRieulDashSsangsios => 'ᇖ',
            HangulJamo::HangulJongseongRieulDashPansios => 'ᇗ',
            HangulJamo::HangulJongseongRieulDashKhieukh => 'ᇘ',
            HangulJamo::HangulJongseongRieulDashYeorinhieuh => 'ᇙ',
            HangulJamo::HangulJongseongMieumDashKiyeok => 'ᇚ',
            HangulJamo::HangulJongseongMieumDashRieul => 'ᇛ',
            HangulJamo::HangulJongseongMieumDashPieup => 'ᇜ',
            HangulJamo::HangulJongseongMieumDashSios => 'ᇝ',
            HangulJamo::HangulJongseongMieumDashSsangsios => 'ᇞ',
            HangulJamo::HangulJongseongMieumDashPansios => 'ᇟ',
            HangulJamo::HangulJongseongMieumDashChieuch => 'ᇠ',
            HangulJamo::HangulJongseongMieumDashHieuh => 'ᇡ',
            HangulJamo::HangulJongseongKapyeounmieum => 'ᇢ',
            HangulJamo::HangulJongseongPieupDashRieul => 'ᇣ',
            HangulJamo::HangulJongseongPieupDashPhieuph => 'ᇤ',
            HangulJamo::HangulJongseongPieupDashHieuh => 'ᇥ',
            HangulJamo::HangulJongseongKapyeounpieup => 'ᇦ',
            HangulJamo::HangulJongseongSiosDashKiyeok => 'ᇧ',
            HangulJamo::HangulJongseongSiosDashTikeut => 'ᇨ',
            HangulJamo::HangulJongseongSiosDashRieul => 'ᇩ',
            HangulJamo::HangulJongseongSiosDashPieup => 'ᇪ',
            HangulJamo::HangulJongseongPansios => 'ᇫ',
            HangulJamo::HangulJongseongIeungDashKiyeok => 'ᇬ',
            HangulJamo::HangulJongseongIeungDashSsangkiyeok => 'ᇭ',
            HangulJamo::HangulJongseongSsangieung => 'ᇮ',
            HangulJamo::HangulJongseongIeungDashKhieukh => 'ᇯ',
            HangulJamo::HangulJongseongYesieung => 'ᇰ',
            HangulJamo::HangulJongseongYesieungDashSios => 'ᇱ',
            HangulJamo::HangulJongseongYesieungDashPansios => 'ᇲ',
            HangulJamo::HangulJongseongPhieuphDashPieup => 'ᇳ',
            HangulJamo::HangulJongseongKapyeounphieuph => 'ᇴ',
            HangulJamo::HangulJongseongHieuhDashNieun => 'ᇵ',
            HangulJamo::HangulJongseongHieuhDashRieul => 'ᇶ',
            HangulJamo::HangulJongseongHieuhDashMieum => 'ᇷ',
            HangulJamo::HangulJongseongHieuhDashPieup => 'ᇸ',
            HangulJamo::HangulJongseongYeorinhieuh => 'ᇹ',
            HangulJamo::HangulJongseongKiyeokDashNieun => 'ᇺ',
            HangulJamo::HangulJongseongKiyeokDashPieup => 'ᇻ',
            HangulJamo::HangulJongseongKiyeokDashChieuch => 'ᇼ',
            HangulJamo::HangulJongseongKiyeokDashKhieukh => 'ᇽ',
            HangulJamo::HangulJongseongKiyeokDashHieuh => 'ᇾ',
        }
    }
}

impl std::convert::TryFrom<char> for HangulJamo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᄀ' => Ok(HangulJamo::HangulChoseongKiyeok),
            'ᄁ' => Ok(HangulJamo::HangulChoseongSsangkiyeok),
            'ᄂ' => Ok(HangulJamo::HangulChoseongNieun),
            'ᄃ' => Ok(HangulJamo::HangulChoseongTikeut),
            'ᄄ' => Ok(HangulJamo::HangulChoseongSsangtikeut),
            'ᄅ' => Ok(HangulJamo::HangulChoseongRieul),
            'ᄆ' => Ok(HangulJamo::HangulChoseongMieum),
            'ᄇ' => Ok(HangulJamo::HangulChoseongPieup),
            'ᄈ' => Ok(HangulJamo::HangulChoseongSsangpieup),
            'ᄉ' => Ok(HangulJamo::HangulChoseongSios),
            'ᄊ' => Ok(HangulJamo::HangulChoseongSsangsios),
            'ᄋ' => Ok(HangulJamo::HangulChoseongIeung),
            'ᄌ' => Ok(HangulJamo::HangulChoseongCieuc),
            'ᄍ' => Ok(HangulJamo::HangulChoseongSsangcieuc),
            'ᄎ' => Ok(HangulJamo::HangulChoseongChieuch),
            'ᄏ' => Ok(HangulJamo::HangulChoseongKhieukh),
            'ᄐ' => Ok(HangulJamo::HangulChoseongThieuth),
            'ᄑ' => Ok(HangulJamo::HangulChoseongPhieuph),
            'ᄒ' => Ok(HangulJamo::HangulChoseongHieuh),
            'ᄓ' => Ok(HangulJamo::HangulChoseongNieunDashKiyeok),
            'ᄔ' => Ok(HangulJamo::HangulChoseongSsangnieun),
            'ᄕ' => Ok(HangulJamo::HangulChoseongNieunDashTikeut),
            'ᄖ' => Ok(HangulJamo::HangulChoseongNieunDashPieup),
            'ᄗ' => Ok(HangulJamo::HangulChoseongTikeutDashKiyeok),
            'ᄘ' => Ok(HangulJamo::HangulChoseongRieulDashNieun),
            'ᄙ' => Ok(HangulJamo::HangulChoseongSsangrieul),
            'ᄚ' => Ok(HangulJamo::HangulChoseongRieulDashHieuh),
            'ᄛ' => Ok(HangulJamo::HangulChoseongKapyeounrieul),
            'ᄜ' => Ok(HangulJamo::HangulChoseongMieumDashPieup),
            'ᄝ' => Ok(HangulJamo::HangulChoseongKapyeounmieum),
            'ᄞ' => Ok(HangulJamo::HangulChoseongPieupDashKiyeok),
            'ᄟ' => Ok(HangulJamo::HangulChoseongPieupDashNieun),
            'ᄠ' => Ok(HangulJamo::HangulChoseongPieupDashTikeut),
            'ᄡ' => Ok(HangulJamo::HangulChoseongPieupDashSios),
            'ᄢ' => Ok(HangulJamo::HangulChoseongPieupDashSiosDashKiyeok),
            'ᄣ' => Ok(HangulJamo::HangulChoseongPieupDashSiosDashTikeut),
            'ᄤ' => Ok(HangulJamo::HangulChoseongPieupDashSiosDashPieup),
            'ᄥ' => Ok(HangulJamo::HangulChoseongPieupDashSsangsios),
            'ᄦ' => Ok(HangulJamo::HangulChoseongPieupDashSiosDashCieuc),
            'ᄧ' => Ok(HangulJamo::HangulChoseongPieupDashCieuc),
            'ᄨ' => Ok(HangulJamo::HangulChoseongPieupDashChieuch),
            'ᄩ' => Ok(HangulJamo::HangulChoseongPieupDashThieuth),
            'ᄪ' => Ok(HangulJamo::HangulChoseongPieupDashPhieuph),
            'ᄫ' => Ok(HangulJamo::HangulChoseongKapyeounpieup),
            'ᄬ' => Ok(HangulJamo::HangulChoseongKapyeounssangpieup),
            'ᄭ' => Ok(HangulJamo::HangulChoseongSiosDashKiyeok),
            'ᄮ' => Ok(HangulJamo::HangulChoseongSiosDashNieun),
            'ᄯ' => Ok(HangulJamo::HangulChoseongSiosDashTikeut),
            'ᄰ' => Ok(HangulJamo::HangulChoseongSiosDashRieul),
            'ᄱ' => Ok(HangulJamo::HangulChoseongSiosDashMieum),
            'ᄲ' => Ok(HangulJamo::HangulChoseongSiosDashPieup),
            'ᄳ' => Ok(HangulJamo::HangulChoseongSiosDashPieupDashKiyeok),
            'ᄴ' => Ok(HangulJamo::HangulChoseongSiosDashSsangsios),
            'ᄵ' => Ok(HangulJamo::HangulChoseongSiosDashIeung),
            'ᄶ' => Ok(HangulJamo::HangulChoseongSiosDashCieuc),
            'ᄷ' => Ok(HangulJamo::HangulChoseongSiosDashChieuch),
            'ᄸ' => Ok(HangulJamo::HangulChoseongSiosDashKhieukh),
            'ᄹ' => Ok(HangulJamo::HangulChoseongSiosDashThieuth),
            'ᄺ' => Ok(HangulJamo::HangulChoseongSiosDashPhieuph),
            'ᄻ' => Ok(HangulJamo::HangulChoseongSiosDashHieuh),
            'ᄼ' => Ok(HangulJamo::HangulChoseongChitueumsios),
            'ᄽ' => Ok(HangulJamo::HangulChoseongChitueumssangsios),
            'ᄾ' => Ok(HangulJamo::HangulChoseongCeongchieumsios),
            'ᄿ' => Ok(HangulJamo::HangulChoseongCeongchieumssangsios),
            'ᅀ' => Ok(HangulJamo::HangulChoseongPansios),
            'ᅁ' => Ok(HangulJamo::HangulChoseongIeungDashKiyeok),
            'ᅂ' => Ok(HangulJamo::HangulChoseongIeungDashTikeut),
            'ᅃ' => Ok(HangulJamo::HangulChoseongIeungDashMieum),
            'ᅄ' => Ok(HangulJamo::HangulChoseongIeungDashPieup),
            'ᅅ' => Ok(HangulJamo::HangulChoseongIeungDashSios),
            'ᅆ' => Ok(HangulJamo::HangulChoseongIeungDashPansios),
            'ᅇ' => Ok(HangulJamo::HangulChoseongSsangieung),
            'ᅈ' => Ok(HangulJamo::HangulChoseongIeungDashCieuc),
            'ᅉ' => Ok(HangulJamo::HangulChoseongIeungDashChieuch),
            'ᅊ' => Ok(HangulJamo::HangulChoseongIeungDashThieuth),
            'ᅋ' => Ok(HangulJamo::HangulChoseongIeungDashPhieuph),
            'ᅌ' => Ok(HangulJamo::HangulChoseongYesieung),
            'ᅍ' => Ok(HangulJamo::HangulChoseongCieucDashIeung),
            'ᅎ' => Ok(HangulJamo::HangulChoseongChitueumcieuc),
            'ᅏ' => Ok(HangulJamo::HangulChoseongChitueumssangcieuc),
            'ᅐ' => Ok(HangulJamo::HangulChoseongCeongchieumcieuc),
            'ᅑ' => Ok(HangulJamo::HangulChoseongCeongchieumssangcieuc),
            'ᅒ' => Ok(HangulJamo::HangulChoseongChieuchDashKhieukh),
            'ᅓ' => Ok(HangulJamo::HangulChoseongChieuchDashHieuh),
            'ᅔ' => Ok(HangulJamo::HangulChoseongChitueumchieuch),
            'ᅕ' => Ok(HangulJamo::HangulChoseongCeongchieumchieuch),
            'ᅖ' => Ok(HangulJamo::HangulChoseongPhieuphDashPieup),
            'ᅗ' => Ok(HangulJamo::HangulChoseongKapyeounphieuph),
            'ᅘ' => Ok(HangulJamo::HangulChoseongSsanghieuh),
            'ᅙ' => Ok(HangulJamo::HangulChoseongYeorinhieuh),
            'ᅚ' => Ok(HangulJamo::HangulChoseongKiyeokDashTikeut),
            'ᅛ' => Ok(HangulJamo::HangulChoseongNieunDashSios),
            'ᅜ' => Ok(HangulJamo::HangulChoseongNieunDashCieuc),
            'ᅝ' => Ok(HangulJamo::HangulChoseongNieunDashHieuh),
            'ᅞ' => Ok(HangulJamo::HangulChoseongTikeutDashRieul),
            'ᅟ' => Ok(HangulJamo::HangulChoseongFiller),
            'ᅠ' => Ok(HangulJamo::HangulJungseongFiller),
            'ᅡ' => Ok(HangulJamo::HangulJungseongA),
            'ᅢ' => Ok(HangulJamo::HangulJungseongAe),
            'ᅣ' => Ok(HangulJamo::HangulJungseongYa),
            'ᅤ' => Ok(HangulJamo::HangulJungseongYae),
            'ᅥ' => Ok(HangulJamo::HangulJungseongEo),
            'ᅦ' => Ok(HangulJamo::HangulJungseongE),
            'ᅧ' => Ok(HangulJamo::HangulJungseongYeo),
            'ᅨ' => Ok(HangulJamo::HangulJungseongYe),
            'ᅩ' => Ok(HangulJamo::HangulJungseongO),
            'ᅪ' => Ok(HangulJamo::HangulJungseongWa),
            'ᅫ' => Ok(HangulJamo::HangulJungseongWae),
            'ᅬ' => Ok(HangulJamo::HangulJungseongOe),
            'ᅭ' => Ok(HangulJamo::HangulJungseongYo),
            'ᅮ' => Ok(HangulJamo::HangulJungseongU),
            'ᅯ' => Ok(HangulJamo::HangulJungseongWeo),
            'ᅰ' => Ok(HangulJamo::HangulJungseongWe),
            'ᅱ' => Ok(HangulJamo::HangulJungseongWi),
            'ᅲ' => Ok(HangulJamo::HangulJungseongYu),
            'ᅳ' => Ok(HangulJamo::HangulJungseongEu),
            'ᅴ' => Ok(HangulJamo::HangulJungseongYi),
            'ᅵ' => Ok(HangulJamo::HangulJungseongI),
            'ᅶ' => Ok(HangulJamo::HangulJungseongADashO),
            'ᅷ' => Ok(HangulJamo::HangulJungseongADashU),
            'ᅸ' => Ok(HangulJamo::HangulJungseongYaDashO),
            'ᅹ' => Ok(HangulJamo::HangulJungseongYaDashYo),
            'ᅺ' => Ok(HangulJamo::HangulJungseongEoDashO),
            'ᅻ' => Ok(HangulJamo::HangulJungseongEoDashU),
            'ᅼ' => Ok(HangulJamo::HangulJungseongEoDashEu),
            'ᅽ' => Ok(HangulJamo::HangulJungseongYeoDashO),
            'ᅾ' => Ok(HangulJamo::HangulJungseongYeoDashU),
            'ᅿ' => Ok(HangulJamo::HangulJungseongODashEo),
            'ᆀ' => Ok(HangulJamo::HangulJungseongODashE),
            'ᆁ' => Ok(HangulJamo::HangulJungseongODashYe),
            'ᆂ' => Ok(HangulJamo::HangulJungseongODashO),
            'ᆃ' => Ok(HangulJamo::HangulJungseongODashU),
            'ᆄ' => Ok(HangulJamo::HangulJungseongYoDashYa),
            'ᆅ' => Ok(HangulJamo::HangulJungseongYoDashYae),
            'ᆆ' => Ok(HangulJamo::HangulJungseongYoDashYeo),
            'ᆇ' => Ok(HangulJamo::HangulJungseongYoDashO),
            'ᆈ' => Ok(HangulJamo::HangulJungseongYoDashI),
            'ᆉ' => Ok(HangulJamo::HangulJungseongUDashA),
            'ᆊ' => Ok(HangulJamo::HangulJungseongUDashAe),
            'ᆋ' => Ok(HangulJamo::HangulJungseongUDashEoDashEu),
            'ᆌ' => Ok(HangulJamo::HangulJungseongUDashYe),
            'ᆍ' => Ok(HangulJamo::HangulJungseongUDashU),
            'ᆎ' => Ok(HangulJamo::HangulJungseongYuDashA),
            'ᆏ' => Ok(HangulJamo::HangulJungseongYuDashEo),
            'ᆐ' => Ok(HangulJamo::HangulJungseongYuDashE),
            'ᆑ' => Ok(HangulJamo::HangulJungseongYuDashYeo),
            'ᆒ' => Ok(HangulJamo::HangulJungseongYuDashYe),
            'ᆓ' => Ok(HangulJamo::HangulJungseongYuDashU),
            'ᆔ' => Ok(HangulJamo::HangulJungseongYuDashI),
            'ᆕ' => Ok(HangulJamo::HangulJungseongEuDashU),
            'ᆖ' => Ok(HangulJamo::HangulJungseongEuDashEu),
            'ᆗ' => Ok(HangulJamo::HangulJungseongYiDashU),
            'ᆘ' => Ok(HangulJamo::HangulJungseongIDashA),
            'ᆙ' => Ok(HangulJamo::HangulJungseongIDashYa),
            'ᆚ' => Ok(HangulJamo::HangulJungseongIDashO),
            'ᆛ' => Ok(HangulJamo::HangulJungseongIDashU),
            'ᆜ' => Ok(HangulJamo::HangulJungseongIDashEu),
            'ᆝ' => Ok(HangulJamo::HangulJungseongIDashAraea),
            'ᆞ' => Ok(HangulJamo::HangulJungseongAraea),
            'ᆟ' => Ok(HangulJamo::HangulJungseongAraeaDashEo),
            'ᆠ' => Ok(HangulJamo::HangulJungseongAraeaDashU),
            'ᆡ' => Ok(HangulJamo::HangulJungseongAraeaDashI),
            'ᆢ' => Ok(HangulJamo::HangulJungseongSsangaraea),
            'ᆣ' => Ok(HangulJamo::HangulJungseongADashEu),
            'ᆤ' => Ok(HangulJamo::HangulJungseongYaDashU),
            'ᆥ' => Ok(HangulJamo::HangulJungseongYeoDashYa),
            'ᆦ' => Ok(HangulJamo::HangulJungseongODashYa),
            'ᆧ' => Ok(HangulJamo::HangulJungseongODashYae),
            'ᆨ' => Ok(HangulJamo::HangulJongseongKiyeok),
            'ᆩ' => Ok(HangulJamo::HangulJongseongSsangkiyeok),
            'ᆪ' => Ok(HangulJamo::HangulJongseongKiyeokDashSios),
            'ᆫ' => Ok(HangulJamo::HangulJongseongNieun),
            'ᆬ' => Ok(HangulJamo::HangulJongseongNieunDashCieuc),
            'ᆭ' => Ok(HangulJamo::HangulJongseongNieunDashHieuh),
            'ᆮ' => Ok(HangulJamo::HangulJongseongTikeut),
            'ᆯ' => Ok(HangulJamo::HangulJongseongRieul),
            'ᆰ' => Ok(HangulJamo::HangulJongseongRieulDashKiyeok),
            'ᆱ' => Ok(HangulJamo::HangulJongseongRieulDashMieum),
            'ᆲ' => Ok(HangulJamo::HangulJongseongRieulDashPieup),
            'ᆳ' => Ok(HangulJamo::HangulJongseongRieulDashSios),
            'ᆴ' => Ok(HangulJamo::HangulJongseongRieulDashThieuth),
            'ᆵ' => Ok(HangulJamo::HangulJongseongRieulDashPhieuph),
            'ᆶ' => Ok(HangulJamo::HangulJongseongRieulDashHieuh),
            'ᆷ' => Ok(HangulJamo::HangulJongseongMieum),
            'ᆸ' => Ok(HangulJamo::HangulJongseongPieup),
            'ᆹ' => Ok(HangulJamo::HangulJongseongPieupDashSios),
            'ᆺ' => Ok(HangulJamo::HangulJongseongSios),
            'ᆻ' => Ok(HangulJamo::HangulJongseongSsangsios),
            'ᆼ' => Ok(HangulJamo::HangulJongseongIeung),
            'ᆽ' => Ok(HangulJamo::HangulJongseongCieuc),
            'ᆾ' => Ok(HangulJamo::HangulJongseongChieuch),
            'ᆿ' => Ok(HangulJamo::HangulJongseongKhieukh),
            'ᇀ' => Ok(HangulJamo::HangulJongseongThieuth),
            'ᇁ' => Ok(HangulJamo::HangulJongseongPhieuph),
            'ᇂ' => Ok(HangulJamo::HangulJongseongHieuh),
            'ᇃ' => Ok(HangulJamo::HangulJongseongKiyeokDashRieul),
            'ᇄ' => Ok(HangulJamo::HangulJongseongKiyeokDashSiosDashKiyeok),
            'ᇅ' => Ok(HangulJamo::HangulJongseongNieunDashKiyeok),
            'ᇆ' => Ok(HangulJamo::HangulJongseongNieunDashTikeut),
            'ᇇ' => Ok(HangulJamo::HangulJongseongNieunDashSios),
            'ᇈ' => Ok(HangulJamo::HangulJongseongNieunDashPansios),
            'ᇉ' => Ok(HangulJamo::HangulJongseongNieunDashThieuth),
            'ᇊ' => Ok(HangulJamo::HangulJongseongTikeutDashKiyeok),
            'ᇋ' => Ok(HangulJamo::HangulJongseongTikeutDashRieul),
            'ᇌ' => Ok(HangulJamo::HangulJongseongRieulDashKiyeokDashSios),
            'ᇍ' => Ok(HangulJamo::HangulJongseongRieulDashNieun),
            'ᇎ' => Ok(HangulJamo::HangulJongseongRieulDashTikeut),
            'ᇏ' => Ok(HangulJamo::HangulJongseongRieulDashTikeutDashHieuh),
            'ᇐ' => Ok(HangulJamo::HangulJongseongSsangrieul),
            'ᇑ' => Ok(HangulJamo::HangulJongseongRieulDashMieumDashKiyeok),
            'ᇒ' => Ok(HangulJamo::HangulJongseongRieulDashMieumDashSios),
            'ᇓ' => Ok(HangulJamo::HangulJongseongRieulDashPieupDashSios),
            'ᇔ' => Ok(HangulJamo::HangulJongseongRieulDashPieupDashHieuh),
            'ᇕ' => Ok(HangulJamo::HangulJongseongRieulDashKapyeounpieup),
            'ᇖ' => Ok(HangulJamo::HangulJongseongRieulDashSsangsios),
            'ᇗ' => Ok(HangulJamo::HangulJongseongRieulDashPansios),
            'ᇘ' => Ok(HangulJamo::HangulJongseongRieulDashKhieukh),
            'ᇙ' => Ok(HangulJamo::HangulJongseongRieulDashYeorinhieuh),
            'ᇚ' => Ok(HangulJamo::HangulJongseongMieumDashKiyeok),
            'ᇛ' => Ok(HangulJamo::HangulJongseongMieumDashRieul),
            'ᇜ' => Ok(HangulJamo::HangulJongseongMieumDashPieup),
            'ᇝ' => Ok(HangulJamo::HangulJongseongMieumDashSios),
            'ᇞ' => Ok(HangulJamo::HangulJongseongMieumDashSsangsios),
            'ᇟ' => Ok(HangulJamo::HangulJongseongMieumDashPansios),
            'ᇠ' => Ok(HangulJamo::HangulJongseongMieumDashChieuch),
            'ᇡ' => Ok(HangulJamo::HangulJongseongMieumDashHieuh),
            'ᇢ' => Ok(HangulJamo::HangulJongseongKapyeounmieum),
            'ᇣ' => Ok(HangulJamo::HangulJongseongPieupDashRieul),
            'ᇤ' => Ok(HangulJamo::HangulJongseongPieupDashPhieuph),
            'ᇥ' => Ok(HangulJamo::HangulJongseongPieupDashHieuh),
            'ᇦ' => Ok(HangulJamo::HangulJongseongKapyeounpieup),
            'ᇧ' => Ok(HangulJamo::HangulJongseongSiosDashKiyeok),
            'ᇨ' => Ok(HangulJamo::HangulJongseongSiosDashTikeut),
            'ᇩ' => Ok(HangulJamo::HangulJongseongSiosDashRieul),
            'ᇪ' => Ok(HangulJamo::HangulJongseongSiosDashPieup),
            'ᇫ' => Ok(HangulJamo::HangulJongseongPansios),
            'ᇬ' => Ok(HangulJamo::HangulJongseongIeungDashKiyeok),
            'ᇭ' => Ok(HangulJamo::HangulJongseongIeungDashSsangkiyeok),
            'ᇮ' => Ok(HangulJamo::HangulJongseongSsangieung),
            'ᇯ' => Ok(HangulJamo::HangulJongseongIeungDashKhieukh),
            'ᇰ' => Ok(HangulJamo::HangulJongseongYesieung),
            'ᇱ' => Ok(HangulJamo::HangulJongseongYesieungDashSios),
            'ᇲ' => Ok(HangulJamo::HangulJongseongYesieungDashPansios),
            'ᇳ' => Ok(HangulJamo::HangulJongseongPhieuphDashPieup),
            'ᇴ' => Ok(HangulJamo::HangulJongseongKapyeounphieuph),
            'ᇵ' => Ok(HangulJamo::HangulJongseongHieuhDashNieun),
            'ᇶ' => Ok(HangulJamo::HangulJongseongHieuhDashRieul),
            'ᇷ' => Ok(HangulJamo::HangulJongseongHieuhDashMieum),
            'ᇸ' => Ok(HangulJamo::HangulJongseongHieuhDashPieup),
            'ᇹ' => Ok(HangulJamo::HangulJongseongYeorinhieuh),
            'ᇺ' => Ok(HangulJamo::HangulJongseongKiyeokDashNieun),
            'ᇻ' => Ok(HangulJamo::HangulJongseongKiyeokDashPieup),
            'ᇼ' => Ok(HangulJamo::HangulJongseongKiyeokDashChieuch),
            'ᇽ' => Ok(HangulJamo::HangulJongseongKiyeokDashKhieukh),
            'ᇾ' => Ok(HangulJamo::HangulJongseongKiyeokDashHieuh),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HangulJamo {
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

impl std::convert::TryFrom<u32> for HangulJamo {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HangulJamo {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HangulJamo {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HangulJamo::HangulChoseongKiyeok
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("HangulJamo{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
