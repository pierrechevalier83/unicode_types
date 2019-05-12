
/// An enum to represent all characters in the LinearA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LinearA {
    /// \u{10600}: '𐘀'
    SignAb001,
    /// \u{10601}: '𐘁'
    SignAb002,
    /// \u{10602}: '𐘂'
    SignAb003,
    /// \u{10603}: '𐘃'
    SignAb004,
    /// \u{10604}: '𐘄'
    SignAb005,
    /// \u{10605}: '𐘅'
    SignAb006,
    /// \u{10606}: '𐘆'
    SignAb007,
    /// \u{10607}: '𐘇'
    SignAb008,
    /// \u{10608}: '𐘈'
    SignAb009,
    /// \u{10609}: '𐘉'
    SignAb010,
    /// \u{1060a}: '𐘊'
    SignAb011,
    /// \u{1060b}: '𐘋'
    SignAb013,
    /// \u{1060c}: '𐘌'
    SignAb016,
    /// \u{1060d}: '𐘍'
    SignAb017,
    /// \u{1060e}: '𐘎'
    SignAb020,
    /// \u{1060f}: '𐘏'
    SignAb021,
    /// \u{10610}: '𐘐'
    SignAb021f,
    /// \u{10611}: '𐘑'
    SignAb021m,
    /// \u{10612}: '𐘒'
    SignAb022,
    /// \u{10613}: '𐘓'
    SignAb022f,
    /// \u{10614}: '𐘔'
    SignAb022m,
    /// \u{10615}: '𐘕'
    SignAb023,
    /// \u{10616}: '𐘖'
    SignAb023m,
    /// \u{10617}: '𐘗'
    SignAb024,
    /// \u{10618}: '𐘘'
    SignAb026,
    /// \u{10619}: '𐘙'
    SignAb027,
    /// \u{1061a}: '𐘚'
    SignAb028,
    /// \u{1061b}: '𐘛'
    SignA028b,
    /// \u{1061c}: '𐘜'
    SignAb029,
    /// \u{1061d}: '𐘝'
    SignAb030,
    /// \u{1061e}: '𐘞'
    SignAb031,
    /// \u{1061f}: '𐘟'
    SignAb034,
    /// \u{10620}: '𐘠'
    SignAb037,
    /// \u{10621}: '𐘡'
    SignAb038,
    /// \u{10622}: '𐘢'
    SignAb039,
    /// \u{10623}: '𐘣'
    SignAb040,
    /// \u{10624}: '𐘤'
    SignAb041,
    /// \u{10625}: '𐘥'
    SignAb044,
    /// \u{10626}: '𐘦'
    SignAb045,
    /// \u{10627}: '𐘧'
    SignAb046,
    /// \u{10628}: '𐘨'
    SignAb047,
    /// \u{10629}: '𐘩'
    SignAb048,
    /// \u{1062a}: '𐘪'
    SignAb049,
    /// \u{1062b}: '𐘫'
    SignAb050,
    /// \u{1062c}: '𐘬'
    SignAb051,
    /// \u{1062d}: '𐘭'
    SignAb053,
    /// \u{1062e}: '𐘮'
    SignAb054,
    /// \u{1062f}: '𐘯'
    SignAb055,
    /// \u{10630}: '𐘰'
    SignAb056,
    /// \u{10631}: '𐘱'
    SignAb057,
    /// \u{10632}: '𐘲'
    SignAb058,
    /// \u{10633}: '𐘳'
    SignAb059,
    /// \u{10634}: '𐘴'
    SignAb060,
    /// \u{10635}: '𐘵'
    SignAb061,
    /// \u{10636}: '𐘶'
    SignAb065,
    /// \u{10637}: '𐘷'
    SignAb066,
    /// \u{10638}: '𐘸'
    SignAb067,
    /// \u{10639}: '𐘹'
    SignAb069,
    /// \u{1063a}: '𐘺'
    SignAb070,
    /// \u{1063b}: '𐘻'
    SignAb073,
    /// \u{1063c}: '𐘼'
    SignAb074,
    /// \u{1063d}: '𐘽'
    SignAb076,
    /// \u{1063e}: '𐘾'
    SignAb077,
    /// \u{1063f}: '𐘿'
    SignAb078,
    /// \u{10640}: '𐙀'
    SignAb079,
    /// \u{10641}: '𐙁'
    SignAb080,
    /// \u{10642}: '𐙂'
    SignAb081,
    /// \u{10643}: '𐙃'
    SignAb082,
    /// \u{10644}: '𐙄'
    SignAb085,
    /// \u{10645}: '𐙅'
    SignAb086,
    /// \u{10646}: '𐙆'
    SignAb087,
    /// \u{10647}: '𐙇'
    SignA100Dash102,
    /// \u{10648}: '𐙈'
    SignAb118,
    /// \u{10649}: '𐙉'
    SignAb120,
    /// \u{1064a}: '𐙊'
    SignA120b,
    /// \u{1064b}: '𐙋'
    SignAb122,
    /// \u{1064c}: '𐙌'
    SignAb123,
    /// \u{1064d}: '𐙍'
    SignAb131a,
    /// \u{1064e}: '𐙎'
    SignAb131b,
    /// \u{1064f}: '𐙏'
    SignA131c,
    /// \u{10650}: '𐙐'
    SignAb164,
    /// \u{10651}: '𐙑'
    SignAb171,
    /// \u{10652}: '𐙒'
    SignAb180,
    /// \u{10653}: '𐙓'
    SignAb188,
    /// \u{10654}: '𐙔'
    SignAb191,
    /// \u{10655}: '𐙕'
    SignA301,
    /// \u{10656}: '𐙖'
    SignA302,
    /// \u{10657}: '𐙗'
    SignA303,
    /// \u{10658}: '𐙘'
    SignA304,
    /// \u{10659}: '𐙙'
    SignA305,
    /// \u{1065a}: '𐙚'
    SignA306,
    /// \u{1065b}: '𐙛'
    SignA307,
    /// \u{1065c}: '𐙜'
    SignA308,
    /// \u{1065d}: '𐙝'
    SignA309a,
    /// \u{1065e}: '𐙞'
    SignA309b,
    /// \u{1065f}: '𐙟'
    SignA309c,
    /// \u{10660}: '𐙠'
    SignA310,
    /// \u{10661}: '𐙡'
    SignA311,
    /// \u{10662}: '𐙢'
    SignA312,
    /// \u{10663}: '𐙣'
    SignA313a,
    /// \u{10664}: '𐙤'
    SignA313b,
    /// \u{10665}: '𐙥'
    SignA313c,
    /// \u{10666}: '𐙦'
    SignA314,
    /// \u{10667}: '𐙧'
    SignA315,
    /// \u{10668}: '𐙨'
    SignA316,
    /// \u{10669}: '𐙩'
    SignA317,
    /// \u{1066a}: '𐙪'
    SignA318,
    /// \u{1066b}: '𐙫'
    SignA319,
    /// \u{1066c}: '𐙬'
    SignA320,
    /// \u{1066d}: '𐙭'
    SignA321,
    /// \u{1066e}: '𐙮'
    SignA322,
    /// \u{1066f}: '𐙯'
    SignA323,
    /// \u{10670}: '𐙰'
    SignA324,
    /// \u{10671}: '𐙱'
    SignA325,
    /// \u{10672}: '𐙲'
    SignA326,
    /// \u{10673}: '𐙳'
    SignA327,
    /// \u{10674}: '𐙴'
    SignA328,
    /// \u{10675}: '𐙵'
    SignA329,
    /// \u{10676}: '𐙶'
    SignA330,
    /// \u{10677}: '𐙷'
    SignA331,
    /// \u{10678}: '𐙸'
    SignA332,
    /// \u{10679}: '𐙹'
    SignA333,
    /// \u{1067a}: '𐙺'
    SignA334,
    /// \u{1067b}: '𐙻'
    SignA335,
    /// \u{1067c}: '𐙼'
    SignA336,
    /// \u{1067d}: '𐙽'
    SignA337,
    /// \u{1067e}: '𐙾'
    SignA338,
    /// \u{1067f}: '𐙿'
    SignA339,
    /// \u{10680}: '𐚀'
    SignA340,
    /// \u{10681}: '𐚁'
    SignA341,
    /// \u{10682}: '𐚂'
    SignA342,
    /// \u{10683}: '𐚃'
    SignA343,
    /// \u{10684}: '𐚄'
    SignA344,
    /// \u{10685}: '𐚅'
    SignA345,
    /// \u{10686}: '𐚆'
    SignA346,
    /// \u{10687}: '𐚇'
    SignA347,
    /// \u{10688}: '𐚈'
    SignA348,
    /// \u{10689}: '𐚉'
    SignA349,
    /// \u{1068a}: '𐚊'
    SignA350,
    /// \u{1068b}: '𐚋'
    SignA351,
    /// \u{1068c}: '𐚌'
    SignA352,
    /// \u{1068d}: '𐚍'
    SignA353,
    /// \u{1068e}: '𐚎'
    SignA354,
    /// \u{1068f}: '𐚏'
    SignA355,
    /// \u{10690}: '𐚐'
    SignA356,
    /// \u{10691}: '𐚑'
    SignA357,
    /// \u{10692}: '𐚒'
    SignA358,
    /// \u{10693}: '𐚓'
    SignA359,
    /// \u{10694}: '𐚔'
    SignA360,
    /// \u{10695}: '𐚕'
    SignA361,
    /// \u{10696}: '𐚖'
    SignA362,
    /// \u{10697}: '𐚗'
    SignA363,
    /// \u{10698}: '𐚘'
    SignA364,
    /// \u{10699}: '𐚙'
    SignA365,
    /// \u{1069a}: '𐚚'
    SignA366,
    /// \u{1069b}: '𐚛'
    SignA367,
    /// \u{1069c}: '𐚜'
    SignA368,
    /// \u{1069d}: '𐚝'
    SignA369,
    /// \u{1069e}: '𐚞'
    SignA370,
    /// \u{1069f}: '𐚟'
    SignA371,
    /// \u{106a0}: '𐚠'
    SignA400DashVas,
    /// \u{106a1}: '𐚡'
    SignA401DashVas,
    /// \u{106a2}: '𐚢'
    SignA402DashVas,
    /// \u{106a3}: '𐚣'
    SignA403DashVas,
    /// \u{106a4}: '𐚤'
    SignA404DashVas,
    /// \u{106a5}: '𐚥'
    SignA405DashVas,
    /// \u{106a6}: '𐚦'
    SignA406DashVas,
    /// \u{106a7}: '𐚧'
    SignA407DashVas,
    /// \u{106a8}: '𐚨'
    SignA408DashVas,
    /// \u{106a9}: '𐚩'
    SignA409DashVas,
    /// \u{106aa}: '𐚪'
    SignA410DashVas,
    /// \u{106ab}: '𐚫'
    SignA411DashVas,
    /// \u{106ac}: '𐚬'
    SignA412DashVas,
    /// \u{106ad}: '𐚭'
    SignA413DashVas,
    /// \u{106ae}: '𐚮'
    SignA414DashVas,
    /// \u{106af}: '𐚯'
    SignA415DashVas,
    /// \u{106b0}: '𐚰'
    SignA416DashVas,
    /// \u{106b1}: '𐚱'
    SignA417DashVas,
    /// \u{106b2}: '𐚲'
    SignA418DashVas,
    /// \u{106b3}: '𐚳'
    SignA501,
    /// \u{106b4}: '𐚴'
    SignA502,
    /// \u{106b5}: '𐚵'
    SignA503,
    /// \u{106b6}: '𐚶'
    SignA504,
    /// \u{106b7}: '𐚷'
    SignA505,
    /// \u{106b8}: '𐚸'
    SignA506,
    /// \u{106b9}: '𐚹'
    SignA508,
    /// \u{106ba}: '𐚺'
    SignA509,
    /// \u{106bb}: '𐚻'
    SignA510,
    /// \u{106bc}: '𐚼'
    SignA511,
    /// \u{106bd}: '𐚽'
    SignA512,
    /// \u{106be}: '𐚾'
    SignA513,
    /// \u{106bf}: '𐚿'
    SignA515,
    /// \u{106c0}: '𐛀'
    SignA516,
    /// \u{106c1}: '𐛁'
    SignA520,
    /// \u{106c2}: '𐛂'
    SignA521,
    /// \u{106c3}: '𐛃'
    SignA523,
    /// \u{106c4}: '𐛄'
    SignA524,
    /// \u{106c5}: '𐛅'
    SignA525,
    /// \u{106c6}: '𐛆'
    SignA526,
    /// \u{106c7}: '𐛇'
    SignA527,
    /// \u{106c8}: '𐛈'
    SignA528,
    /// \u{106c9}: '𐛉'
    SignA529,
    /// \u{106ca}: '𐛊'
    SignA530,
    /// \u{106cb}: '𐛋'
    SignA531,
    /// \u{106cc}: '𐛌'
    SignA532,
    /// \u{106cd}: '𐛍'
    SignA534,
    /// \u{106ce}: '𐛎'
    SignA535,
    /// \u{106cf}: '𐛏'
    SignA536,
    /// \u{106d0}: '𐛐'
    SignA537,
    /// \u{106d1}: '𐛑'
    SignA538,
    /// \u{106d2}: '𐛒'
    SignA539,
    /// \u{106d3}: '𐛓'
    SignA540,
    /// \u{106d4}: '𐛔'
    SignA541,
    /// \u{106d5}: '𐛕'
    SignA542,
    /// \u{106d6}: '𐛖'
    SignA545,
    /// \u{106d7}: '𐛗'
    SignA547,
    /// \u{106d8}: '𐛘'
    SignA548,
    /// \u{106d9}: '𐛙'
    SignA549,
    /// \u{106da}: '𐛚'
    SignA550,
    /// \u{106db}: '𐛛'
    SignA551,
    /// \u{106dc}: '𐛜'
    SignA552,
    /// \u{106dd}: '𐛝'
    SignA553,
    /// \u{106de}: '𐛞'
    SignA554,
    /// \u{106df}: '𐛟'
    SignA555,
    /// \u{106e0}: '𐛠'
    SignA556,
    /// \u{106e1}: '𐛡'
    SignA557,
    /// \u{106e2}: '𐛢'
    SignA559,
    /// \u{106e3}: '𐛣'
    SignA563,
    /// \u{106e4}: '𐛤'
    SignA564,
    /// \u{106e5}: '𐛥'
    SignA565,
    /// \u{106e6}: '𐛦'
    SignA566,
    /// \u{106e7}: '𐛧'
    SignA568,
    /// \u{106e8}: '𐛨'
    SignA569,
    /// \u{106e9}: '𐛩'
    SignA570,
    /// \u{106ea}: '𐛪'
    SignA571,
    /// \u{106eb}: '𐛫'
    SignA572,
    /// \u{106ec}: '𐛬'
    SignA573,
    /// \u{106ed}: '𐛭'
    SignA574,
    /// \u{106ee}: '𐛮'
    SignA575,
    /// \u{106ef}: '𐛯'
    SignA576,
    /// \u{106f0}: '𐛰'
    SignA577,
    /// \u{106f1}: '𐛱'
    SignA578,
    /// \u{106f2}: '𐛲'
    SignA579,
    /// \u{106f3}: '𐛳'
    SignA580,
    /// \u{106f4}: '𐛴'
    SignA581,
    /// \u{106f5}: '𐛵'
    SignA582,
    /// \u{106f6}: '𐛶'
    SignA583,
    /// \u{106f7}: '𐛷'
    SignA584,
    /// \u{106f8}: '𐛸'
    SignA585,
    /// \u{106f9}: '𐛹'
    SignA586,
    /// \u{106fa}: '𐛺'
    SignA587,
    /// \u{106fb}: '𐛻'
    SignA588,
    /// \u{106fc}: '𐛼'
    SignA589,
    /// \u{106fd}: '𐛽'
    SignA591,
    /// \u{106fe}: '𐛾'
    SignA592,
    /// \u{106ff}: '𐛿'
    SignA594,
    /// \u{10700}: '𐜀'
    SignA595,
    /// \u{10701}: '𐜁'
    SignA596,
    /// \u{10702}: '𐜂'
    SignA598,
    /// \u{10703}: '𐜃'
    SignA600,
    /// \u{10704}: '𐜄'
    SignA601,
    /// \u{10705}: '𐜅'
    SignA602,
    /// \u{10706}: '𐜆'
    SignA603,
    /// \u{10707}: '𐜇'
    SignA604,
    /// \u{10708}: '𐜈'
    SignA606,
    /// \u{10709}: '𐜉'
    SignA608,
    /// \u{1070a}: '𐜊'
    SignA609,
    /// \u{1070b}: '𐜋'
    SignA610,
    /// \u{1070c}: '𐜌'
    SignA611,
    /// \u{1070d}: '𐜍'
    SignA612,
    /// \u{1070e}: '𐜎'
    SignA613,
    /// \u{1070f}: '𐜏'
    SignA614,
    /// \u{10710}: '𐜐'
    SignA615,
    /// \u{10711}: '𐜑'
    SignA616,
    /// \u{10712}: '𐜒'
    SignA617,
    /// \u{10713}: '𐜓'
    SignA618,
    /// \u{10714}: '𐜔'
    SignA619,
    /// \u{10715}: '𐜕'
    SignA620,
    /// \u{10716}: '𐜖'
    SignA621,
    /// \u{10717}: '𐜗'
    SignA622,
    /// \u{10718}: '𐜘'
    SignA623,
    /// \u{10719}: '𐜙'
    SignA624,
    /// \u{1071a}: '𐜚'
    SignA626,
    /// \u{1071b}: '𐜛'
    SignA627,
    /// \u{1071c}: '𐜜'
    SignA628,
    /// \u{1071d}: '𐜝'
    SignA629,
    /// \u{1071e}: '𐜞'
    SignA634,
    /// \u{1071f}: '𐜟'
    SignA637,
    /// \u{10720}: '𐜠'
    SignA638,
    /// \u{10721}: '𐜡'
    SignA640,
    /// \u{10722}: '𐜢'
    SignA642,
    /// \u{10723}: '𐜣'
    SignA643,
    /// \u{10724}: '𐜤'
    SignA644,
    /// \u{10725}: '𐜥'
    SignA645,
    /// \u{10726}: '𐜦'
    SignA646,
    /// \u{10727}: '𐜧'
    SignA648,
    /// \u{10728}: '𐜨'
    SignA649,
    /// \u{10729}: '𐜩'
    SignA651,
    /// \u{1072a}: '𐜪'
    SignA652,
    /// \u{1072b}: '𐜫'
    SignA653,
    /// \u{1072c}: '𐜬'
    SignA654,
    /// \u{1072d}: '𐜭'
    SignA655,
    /// \u{1072e}: '𐜮'
    SignA656,
    /// \u{1072f}: '𐜯'
    SignA657,
    /// \u{10730}: '𐜰'
    SignA658,
    /// \u{10731}: '𐜱'
    SignA659,
    /// \u{10732}: '𐜲'
    SignA660,
    /// \u{10733}: '𐜳'
    SignA661,
    /// \u{10734}: '𐜴'
    SignA662,
    /// \u{10735}: '𐜵'
    SignA663,
    /// \u{10736}: '𐜶'
    SignA664,
    /// \u{10740}: '𐝀'
    SignA701A,
    /// \u{10741}: '𐝁'
    SignA702B,
    /// \u{10742}: '𐝂'
    SignA703D,
    /// \u{10743}: '𐝃'
    SignA704E,
    /// \u{10744}: '𐝄'
    SignA705F,
    /// \u{10745}: '𐝅'
    SignA706H,
    /// \u{10746}: '𐝆'
    SignA707J,
    /// \u{10747}: '𐝇'
    SignA708K,
    /// \u{10748}: '𐝈'
    SignA709L,
    /// \u{10749}: '𐝉'
    SignA709Dash2L2,
    /// \u{1074a}: '𐝊'
    SignA709Dash3L3,
    /// \u{1074b}: '𐝋'
    SignA709Dash4L4,
    /// \u{1074c}: '𐝌'
    SignA709Dash6L6,
    /// \u{1074d}: '𐝍'
    SignA710W,
    /// \u{1074e}: '𐝎'
    SignA711X,
    /// \u{1074f}: '𐝏'
    SignA712Y,
    /// \u{10750}: '𐝐'
    SignA713Omega,
    /// \u{10751}: '𐝑'
    SignA714Abb,
    /// \u{10752}: '𐝒'
    SignA715Bb,
    /// \u{10753}: '𐝓'
    SignA717Dd,
    /// \u{10754}: '𐝔'
    SignA726Eyyy,
    /// \u{10755}: '𐝕'
    SignA732Je,
    /// \u{10760}: '𐝠'
    SignA800,
    /// \u{10761}: '𐝡'
    SignA801,
    /// \u{10762}: '𐝢'
    SignA802,
    /// \u{10763}: '𐝣'
    SignA803,
    /// \u{10764}: '𐝤'
    SignA804,
    /// \u{10765}: '𐝥'
    SignA805,
    /// \u{10766}: '𐝦'
    SignA806,
    /// \u{10767}: '𐝧'
    SignA807,
}

impl Into<char> for LinearA {
    fn into(self) -> char {
        match self {
            LinearA::SignAb001 => '𐘀',
            LinearA::SignAb002 => '𐘁',
            LinearA::SignAb003 => '𐘂',
            LinearA::SignAb004 => '𐘃',
            LinearA::SignAb005 => '𐘄',
            LinearA::SignAb006 => '𐘅',
            LinearA::SignAb007 => '𐘆',
            LinearA::SignAb008 => '𐘇',
            LinearA::SignAb009 => '𐘈',
            LinearA::SignAb010 => '𐘉',
            LinearA::SignAb011 => '𐘊',
            LinearA::SignAb013 => '𐘋',
            LinearA::SignAb016 => '𐘌',
            LinearA::SignAb017 => '𐘍',
            LinearA::SignAb020 => '𐘎',
            LinearA::SignAb021 => '𐘏',
            LinearA::SignAb021f => '𐘐',
            LinearA::SignAb021m => '𐘑',
            LinearA::SignAb022 => '𐘒',
            LinearA::SignAb022f => '𐘓',
            LinearA::SignAb022m => '𐘔',
            LinearA::SignAb023 => '𐘕',
            LinearA::SignAb023m => '𐘖',
            LinearA::SignAb024 => '𐘗',
            LinearA::SignAb026 => '𐘘',
            LinearA::SignAb027 => '𐘙',
            LinearA::SignAb028 => '𐘚',
            LinearA::SignA028b => '𐘛',
            LinearA::SignAb029 => '𐘜',
            LinearA::SignAb030 => '𐘝',
            LinearA::SignAb031 => '𐘞',
            LinearA::SignAb034 => '𐘟',
            LinearA::SignAb037 => '𐘠',
            LinearA::SignAb038 => '𐘡',
            LinearA::SignAb039 => '𐘢',
            LinearA::SignAb040 => '𐘣',
            LinearA::SignAb041 => '𐘤',
            LinearA::SignAb044 => '𐘥',
            LinearA::SignAb045 => '𐘦',
            LinearA::SignAb046 => '𐘧',
            LinearA::SignAb047 => '𐘨',
            LinearA::SignAb048 => '𐘩',
            LinearA::SignAb049 => '𐘪',
            LinearA::SignAb050 => '𐘫',
            LinearA::SignAb051 => '𐘬',
            LinearA::SignAb053 => '𐘭',
            LinearA::SignAb054 => '𐘮',
            LinearA::SignAb055 => '𐘯',
            LinearA::SignAb056 => '𐘰',
            LinearA::SignAb057 => '𐘱',
            LinearA::SignAb058 => '𐘲',
            LinearA::SignAb059 => '𐘳',
            LinearA::SignAb060 => '𐘴',
            LinearA::SignAb061 => '𐘵',
            LinearA::SignAb065 => '𐘶',
            LinearA::SignAb066 => '𐘷',
            LinearA::SignAb067 => '𐘸',
            LinearA::SignAb069 => '𐘹',
            LinearA::SignAb070 => '𐘺',
            LinearA::SignAb073 => '𐘻',
            LinearA::SignAb074 => '𐘼',
            LinearA::SignAb076 => '𐘽',
            LinearA::SignAb077 => '𐘾',
            LinearA::SignAb078 => '𐘿',
            LinearA::SignAb079 => '𐙀',
            LinearA::SignAb080 => '𐙁',
            LinearA::SignAb081 => '𐙂',
            LinearA::SignAb082 => '𐙃',
            LinearA::SignAb085 => '𐙄',
            LinearA::SignAb086 => '𐙅',
            LinearA::SignAb087 => '𐙆',
            LinearA::SignA100Dash102 => '𐙇',
            LinearA::SignAb118 => '𐙈',
            LinearA::SignAb120 => '𐙉',
            LinearA::SignA120b => '𐙊',
            LinearA::SignAb122 => '𐙋',
            LinearA::SignAb123 => '𐙌',
            LinearA::SignAb131a => '𐙍',
            LinearA::SignAb131b => '𐙎',
            LinearA::SignA131c => '𐙏',
            LinearA::SignAb164 => '𐙐',
            LinearA::SignAb171 => '𐙑',
            LinearA::SignAb180 => '𐙒',
            LinearA::SignAb188 => '𐙓',
            LinearA::SignAb191 => '𐙔',
            LinearA::SignA301 => '𐙕',
            LinearA::SignA302 => '𐙖',
            LinearA::SignA303 => '𐙗',
            LinearA::SignA304 => '𐙘',
            LinearA::SignA305 => '𐙙',
            LinearA::SignA306 => '𐙚',
            LinearA::SignA307 => '𐙛',
            LinearA::SignA308 => '𐙜',
            LinearA::SignA309a => '𐙝',
            LinearA::SignA309b => '𐙞',
            LinearA::SignA309c => '𐙟',
            LinearA::SignA310 => '𐙠',
            LinearA::SignA311 => '𐙡',
            LinearA::SignA312 => '𐙢',
            LinearA::SignA313a => '𐙣',
            LinearA::SignA313b => '𐙤',
            LinearA::SignA313c => '𐙥',
            LinearA::SignA314 => '𐙦',
            LinearA::SignA315 => '𐙧',
            LinearA::SignA316 => '𐙨',
            LinearA::SignA317 => '𐙩',
            LinearA::SignA318 => '𐙪',
            LinearA::SignA319 => '𐙫',
            LinearA::SignA320 => '𐙬',
            LinearA::SignA321 => '𐙭',
            LinearA::SignA322 => '𐙮',
            LinearA::SignA323 => '𐙯',
            LinearA::SignA324 => '𐙰',
            LinearA::SignA325 => '𐙱',
            LinearA::SignA326 => '𐙲',
            LinearA::SignA327 => '𐙳',
            LinearA::SignA328 => '𐙴',
            LinearA::SignA329 => '𐙵',
            LinearA::SignA330 => '𐙶',
            LinearA::SignA331 => '𐙷',
            LinearA::SignA332 => '𐙸',
            LinearA::SignA333 => '𐙹',
            LinearA::SignA334 => '𐙺',
            LinearA::SignA335 => '𐙻',
            LinearA::SignA336 => '𐙼',
            LinearA::SignA337 => '𐙽',
            LinearA::SignA338 => '𐙾',
            LinearA::SignA339 => '𐙿',
            LinearA::SignA340 => '𐚀',
            LinearA::SignA341 => '𐚁',
            LinearA::SignA342 => '𐚂',
            LinearA::SignA343 => '𐚃',
            LinearA::SignA344 => '𐚄',
            LinearA::SignA345 => '𐚅',
            LinearA::SignA346 => '𐚆',
            LinearA::SignA347 => '𐚇',
            LinearA::SignA348 => '𐚈',
            LinearA::SignA349 => '𐚉',
            LinearA::SignA350 => '𐚊',
            LinearA::SignA351 => '𐚋',
            LinearA::SignA352 => '𐚌',
            LinearA::SignA353 => '𐚍',
            LinearA::SignA354 => '𐚎',
            LinearA::SignA355 => '𐚏',
            LinearA::SignA356 => '𐚐',
            LinearA::SignA357 => '𐚑',
            LinearA::SignA358 => '𐚒',
            LinearA::SignA359 => '𐚓',
            LinearA::SignA360 => '𐚔',
            LinearA::SignA361 => '𐚕',
            LinearA::SignA362 => '𐚖',
            LinearA::SignA363 => '𐚗',
            LinearA::SignA364 => '𐚘',
            LinearA::SignA365 => '𐚙',
            LinearA::SignA366 => '𐚚',
            LinearA::SignA367 => '𐚛',
            LinearA::SignA368 => '𐚜',
            LinearA::SignA369 => '𐚝',
            LinearA::SignA370 => '𐚞',
            LinearA::SignA371 => '𐚟',
            LinearA::SignA400DashVas => '𐚠',
            LinearA::SignA401DashVas => '𐚡',
            LinearA::SignA402DashVas => '𐚢',
            LinearA::SignA403DashVas => '𐚣',
            LinearA::SignA404DashVas => '𐚤',
            LinearA::SignA405DashVas => '𐚥',
            LinearA::SignA406DashVas => '𐚦',
            LinearA::SignA407DashVas => '𐚧',
            LinearA::SignA408DashVas => '𐚨',
            LinearA::SignA409DashVas => '𐚩',
            LinearA::SignA410DashVas => '𐚪',
            LinearA::SignA411DashVas => '𐚫',
            LinearA::SignA412DashVas => '𐚬',
            LinearA::SignA413DashVas => '𐚭',
            LinearA::SignA414DashVas => '𐚮',
            LinearA::SignA415DashVas => '𐚯',
            LinearA::SignA416DashVas => '𐚰',
            LinearA::SignA417DashVas => '𐚱',
            LinearA::SignA418DashVas => '𐚲',
            LinearA::SignA501 => '𐚳',
            LinearA::SignA502 => '𐚴',
            LinearA::SignA503 => '𐚵',
            LinearA::SignA504 => '𐚶',
            LinearA::SignA505 => '𐚷',
            LinearA::SignA506 => '𐚸',
            LinearA::SignA508 => '𐚹',
            LinearA::SignA509 => '𐚺',
            LinearA::SignA510 => '𐚻',
            LinearA::SignA511 => '𐚼',
            LinearA::SignA512 => '𐚽',
            LinearA::SignA513 => '𐚾',
            LinearA::SignA515 => '𐚿',
            LinearA::SignA516 => '𐛀',
            LinearA::SignA520 => '𐛁',
            LinearA::SignA521 => '𐛂',
            LinearA::SignA523 => '𐛃',
            LinearA::SignA524 => '𐛄',
            LinearA::SignA525 => '𐛅',
            LinearA::SignA526 => '𐛆',
            LinearA::SignA527 => '𐛇',
            LinearA::SignA528 => '𐛈',
            LinearA::SignA529 => '𐛉',
            LinearA::SignA530 => '𐛊',
            LinearA::SignA531 => '𐛋',
            LinearA::SignA532 => '𐛌',
            LinearA::SignA534 => '𐛍',
            LinearA::SignA535 => '𐛎',
            LinearA::SignA536 => '𐛏',
            LinearA::SignA537 => '𐛐',
            LinearA::SignA538 => '𐛑',
            LinearA::SignA539 => '𐛒',
            LinearA::SignA540 => '𐛓',
            LinearA::SignA541 => '𐛔',
            LinearA::SignA542 => '𐛕',
            LinearA::SignA545 => '𐛖',
            LinearA::SignA547 => '𐛗',
            LinearA::SignA548 => '𐛘',
            LinearA::SignA549 => '𐛙',
            LinearA::SignA550 => '𐛚',
            LinearA::SignA551 => '𐛛',
            LinearA::SignA552 => '𐛜',
            LinearA::SignA553 => '𐛝',
            LinearA::SignA554 => '𐛞',
            LinearA::SignA555 => '𐛟',
            LinearA::SignA556 => '𐛠',
            LinearA::SignA557 => '𐛡',
            LinearA::SignA559 => '𐛢',
            LinearA::SignA563 => '𐛣',
            LinearA::SignA564 => '𐛤',
            LinearA::SignA565 => '𐛥',
            LinearA::SignA566 => '𐛦',
            LinearA::SignA568 => '𐛧',
            LinearA::SignA569 => '𐛨',
            LinearA::SignA570 => '𐛩',
            LinearA::SignA571 => '𐛪',
            LinearA::SignA572 => '𐛫',
            LinearA::SignA573 => '𐛬',
            LinearA::SignA574 => '𐛭',
            LinearA::SignA575 => '𐛮',
            LinearA::SignA576 => '𐛯',
            LinearA::SignA577 => '𐛰',
            LinearA::SignA578 => '𐛱',
            LinearA::SignA579 => '𐛲',
            LinearA::SignA580 => '𐛳',
            LinearA::SignA581 => '𐛴',
            LinearA::SignA582 => '𐛵',
            LinearA::SignA583 => '𐛶',
            LinearA::SignA584 => '𐛷',
            LinearA::SignA585 => '𐛸',
            LinearA::SignA586 => '𐛹',
            LinearA::SignA587 => '𐛺',
            LinearA::SignA588 => '𐛻',
            LinearA::SignA589 => '𐛼',
            LinearA::SignA591 => '𐛽',
            LinearA::SignA592 => '𐛾',
            LinearA::SignA594 => '𐛿',
            LinearA::SignA595 => '𐜀',
            LinearA::SignA596 => '𐜁',
            LinearA::SignA598 => '𐜂',
            LinearA::SignA600 => '𐜃',
            LinearA::SignA601 => '𐜄',
            LinearA::SignA602 => '𐜅',
            LinearA::SignA603 => '𐜆',
            LinearA::SignA604 => '𐜇',
            LinearA::SignA606 => '𐜈',
            LinearA::SignA608 => '𐜉',
            LinearA::SignA609 => '𐜊',
            LinearA::SignA610 => '𐜋',
            LinearA::SignA611 => '𐜌',
            LinearA::SignA612 => '𐜍',
            LinearA::SignA613 => '𐜎',
            LinearA::SignA614 => '𐜏',
            LinearA::SignA615 => '𐜐',
            LinearA::SignA616 => '𐜑',
            LinearA::SignA617 => '𐜒',
            LinearA::SignA618 => '𐜓',
            LinearA::SignA619 => '𐜔',
            LinearA::SignA620 => '𐜕',
            LinearA::SignA621 => '𐜖',
            LinearA::SignA622 => '𐜗',
            LinearA::SignA623 => '𐜘',
            LinearA::SignA624 => '𐜙',
            LinearA::SignA626 => '𐜚',
            LinearA::SignA627 => '𐜛',
            LinearA::SignA628 => '𐜜',
            LinearA::SignA629 => '𐜝',
            LinearA::SignA634 => '𐜞',
            LinearA::SignA637 => '𐜟',
            LinearA::SignA638 => '𐜠',
            LinearA::SignA640 => '𐜡',
            LinearA::SignA642 => '𐜢',
            LinearA::SignA643 => '𐜣',
            LinearA::SignA644 => '𐜤',
            LinearA::SignA645 => '𐜥',
            LinearA::SignA646 => '𐜦',
            LinearA::SignA648 => '𐜧',
            LinearA::SignA649 => '𐜨',
            LinearA::SignA651 => '𐜩',
            LinearA::SignA652 => '𐜪',
            LinearA::SignA653 => '𐜫',
            LinearA::SignA654 => '𐜬',
            LinearA::SignA655 => '𐜭',
            LinearA::SignA656 => '𐜮',
            LinearA::SignA657 => '𐜯',
            LinearA::SignA658 => '𐜰',
            LinearA::SignA659 => '𐜱',
            LinearA::SignA660 => '𐜲',
            LinearA::SignA661 => '𐜳',
            LinearA::SignA662 => '𐜴',
            LinearA::SignA663 => '𐜵',
            LinearA::SignA664 => '𐜶',
            LinearA::SignA701A => '𐝀',
            LinearA::SignA702B => '𐝁',
            LinearA::SignA703D => '𐝂',
            LinearA::SignA704E => '𐝃',
            LinearA::SignA705F => '𐝄',
            LinearA::SignA706H => '𐝅',
            LinearA::SignA707J => '𐝆',
            LinearA::SignA708K => '𐝇',
            LinearA::SignA709L => '𐝈',
            LinearA::SignA709Dash2L2 => '𐝉',
            LinearA::SignA709Dash3L3 => '𐝊',
            LinearA::SignA709Dash4L4 => '𐝋',
            LinearA::SignA709Dash6L6 => '𐝌',
            LinearA::SignA710W => '𐝍',
            LinearA::SignA711X => '𐝎',
            LinearA::SignA712Y => '𐝏',
            LinearA::SignA713Omega => '𐝐',
            LinearA::SignA714Abb => '𐝑',
            LinearA::SignA715Bb => '𐝒',
            LinearA::SignA717Dd => '𐝓',
            LinearA::SignA726Eyyy => '𐝔',
            LinearA::SignA732Je => '𐝕',
            LinearA::SignA800 => '𐝠',
            LinearA::SignA801 => '𐝡',
            LinearA::SignA802 => '𐝢',
            LinearA::SignA803 => '𐝣',
            LinearA::SignA804 => '𐝤',
            LinearA::SignA805 => '𐝥',
            LinearA::SignA806 => '𐝦',
            LinearA::SignA807 => '𐝧',
        }
    }
}

impl std::convert::TryFrom<char> for LinearA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐘀' => Ok(LinearA::SignAb001),
            '𐘁' => Ok(LinearA::SignAb002),
            '𐘂' => Ok(LinearA::SignAb003),
            '𐘃' => Ok(LinearA::SignAb004),
            '𐘄' => Ok(LinearA::SignAb005),
            '𐘅' => Ok(LinearA::SignAb006),
            '𐘆' => Ok(LinearA::SignAb007),
            '𐘇' => Ok(LinearA::SignAb008),
            '𐘈' => Ok(LinearA::SignAb009),
            '𐘉' => Ok(LinearA::SignAb010),
            '𐘊' => Ok(LinearA::SignAb011),
            '𐘋' => Ok(LinearA::SignAb013),
            '𐘌' => Ok(LinearA::SignAb016),
            '𐘍' => Ok(LinearA::SignAb017),
            '𐘎' => Ok(LinearA::SignAb020),
            '𐘏' => Ok(LinearA::SignAb021),
            '𐘐' => Ok(LinearA::SignAb021f),
            '𐘑' => Ok(LinearA::SignAb021m),
            '𐘒' => Ok(LinearA::SignAb022),
            '𐘓' => Ok(LinearA::SignAb022f),
            '𐘔' => Ok(LinearA::SignAb022m),
            '𐘕' => Ok(LinearA::SignAb023),
            '𐘖' => Ok(LinearA::SignAb023m),
            '𐘗' => Ok(LinearA::SignAb024),
            '𐘘' => Ok(LinearA::SignAb026),
            '𐘙' => Ok(LinearA::SignAb027),
            '𐘚' => Ok(LinearA::SignAb028),
            '𐘛' => Ok(LinearA::SignA028b),
            '𐘜' => Ok(LinearA::SignAb029),
            '𐘝' => Ok(LinearA::SignAb030),
            '𐘞' => Ok(LinearA::SignAb031),
            '𐘟' => Ok(LinearA::SignAb034),
            '𐘠' => Ok(LinearA::SignAb037),
            '𐘡' => Ok(LinearA::SignAb038),
            '𐘢' => Ok(LinearA::SignAb039),
            '𐘣' => Ok(LinearA::SignAb040),
            '𐘤' => Ok(LinearA::SignAb041),
            '𐘥' => Ok(LinearA::SignAb044),
            '𐘦' => Ok(LinearA::SignAb045),
            '𐘧' => Ok(LinearA::SignAb046),
            '𐘨' => Ok(LinearA::SignAb047),
            '𐘩' => Ok(LinearA::SignAb048),
            '𐘪' => Ok(LinearA::SignAb049),
            '𐘫' => Ok(LinearA::SignAb050),
            '𐘬' => Ok(LinearA::SignAb051),
            '𐘭' => Ok(LinearA::SignAb053),
            '𐘮' => Ok(LinearA::SignAb054),
            '𐘯' => Ok(LinearA::SignAb055),
            '𐘰' => Ok(LinearA::SignAb056),
            '𐘱' => Ok(LinearA::SignAb057),
            '𐘲' => Ok(LinearA::SignAb058),
            '𐘳' => Ok(LinearA::SignAb059),
            '𐘴' => Ok(LinearA::SignAb060),
            '𐘵' => Ok(LinearA::SignAb061),
            '𐘶' => Ok(LinearA::SignAb065),
            '𐘷' => Ok(LinearA::SignAb066),
            '𐘸' => Ok(LinearA::SignAb067),
            '𐘹' => Ok(LinearA::SignAb069),
            '𐘺' => Ok(LinearA::SignAb070),
            '𐘻' => Ok(LinearA::SignAb073),
            '𐘼' => Ok(LinearA::SignAb074),
            '𐘽' => Ok(LinearA::SignAb076),
            '𐘾' => Ok(LinearA::SignAb077),
            '𐘿' => Ok(LinearA::SignAb078),
            '𐙀' => Ok(LinearA::SignAb079),
            '𐙁' => Ok(LinearA::SignAb080),
            '𐙂' => Ok(LinearA::SignAb081),
            '𐙃' => Ok(LinearA::SignAb082),
            '𐙄' => Ok(LinearA::SignAb085),
            '𐙅' => Ok(LinearA::SignAb086),
            '𐙆' => Ok(LinearA::SignAb087),
            '𐙇' => Ok(LinearA::SignA100Dash102),
            '𐙈' => Ok(LinearA::SignAb118),
            '𐙉' => Ok(LinearA::SignAb120),
            '𐙊' => Ok(LinearA::SignA120b),
            '𐙋' => Ok(LinearA::SignAb122),
            '𐙌' => Ok(LinearA::SignAb123),
            '𐙍' => Ok(LinearA::SignAb131a),
            '𐙎' => Ok(LinearA::SignAb131b),
            '𐙏' => Ok(LinearA::SignA131c),
            '𐙐' => Ok(LinearA::SignAb164),
            '𐙑' => Ok(LinearA::SignAb171),
            '𐙒' => Ok(LinearA::SignAb180),
            '𐙓' => Ok(LinearA::SignAb188),
            '𐙔' => Ok(LinearA::SignAb191),
            '𐙕' => Ok(LinearA::SignA301),
            '𐙖' => Ok(LinearA::SignA302),
            '𐙗' => Ok(LinearA::SignA303),
            '𐙘' => Ok(LinearA::SignA304),
            '𐙙' => Ok(LinearA::SignA305),
            '𐙚' => Ok(LinearA::SignA306),
            '𐙛' => Ok(LinearA::SignA307),
            '𐙜' => Ok(LinearA::SignA308),
            '𐙝' => Ok(LinearA::SignA309a),
            '𐙞' => Ok(LinearA::SignA309b),
            '𐙟' => Ok(LinearA::SignA309c),
            '𐙠' => Ok(LinearA::SignA310),
            '𐙡' => Ok(LinearA::SignA311),
            '𐙢' => Ok(LinearA::SignA312),
            '𐙣' => Ok(LinearA::SignA313a),
            '𐙤' => Ok(LinearA::SignA313b),
            '𐙥' => Ok(LinearA::SignA313c),
            '𐙦' => Ok(LinearA::SignA314),
            '𐙧' => Ok(LinearA::SignA315),
            '𐙨' => Ok(LinearA::SignA316),
            '𐙩' => Ok(LinearA::SignA317),
            '𐙪' => Ok(LinearA::SignA318),
            '𐙫' => Ok(LinearA::SignA319),
            '𐙬' => Ok(LinearA::SignA320),
            '𐙭' => Ok(LinearA::SignA321),
            '𐙮' => Ok(LinearA::SignA322),
            '𐙯' => Ok(LinearA::SignA323),
            '𐙰' => Ok(LinearA::SignA324),
            '𐙱' => Ok(LinearA::SignA325),
            '𐙲' => Ok(LinearA::SignA326),
            '𐙳' => Ok(LinearA::SignA327),
            '𐙴' => Ok(LinearA::SignA328),
            '𐙵' => Ok(LinearA::SignA329),
            '𐙶' => Ok(LinearA::SignA330),
            '𐙷' => Ok(LinearA::SignA331),
            '𐙸' => Ok(LinearA::SignA332),
            '𐙹' => Ok(LinearA::SignA333),
            '𐙺' => Ok(LinearA::SignA334),
            '𐙻' => Ok(LinearA::SignA335),
            '𐙼' => Ok(LinearA::SignA336),
            '𐙽' => Ok(LinearA::SignA337),
            '𐙾' => Ok(LinearA::SignA338),
            '𐙿' => Ok(LinearA::SignA339),
            '𐚀' => Ok(LinearA::SignA340),
            '𐚁' => Ok(LinearA::SignA341),
            '𐚂' => Ok(LinearA::SignA342),
            '𐚃' => Ok(LinearA::SignA343),
            '𐚄' => Ok(LinearA::SignA344),
            '𐚅' => Ok(LinearA::SignA345),
            '𐚆' => Ok(LinearA::SignA346),
            '𐚇' => Ok(LinearA::SignA347),
            '𐚈' => Ok(LinearA::SignA348),
            '𐚉' => Ok(LinearA::SignA349),
            '𐚊' => Ok(LinearA::SignA350),
            '𐚋' => Ok(LinearA::SignA351),
            '𐚌' => Ok(LinearA::SignA352),
            '𐚍' => Ok(LinearA::SignA353),
            '𐚎' => Ok(LinearA::SignA354),
            '𐚏' => Ok(LinearA::SignA355),
            '𐚐' => Ok(LinearA::SignA356),
            '𐚑' => Ok(LinearA::SignA357),
            '𐚒' => Ok(LinearA::SignA358),
            '𐚓' => Ok(LinearA::SignA359),
            '𐚔' => Ok(LinearA::SignA360),
            '𐚕' => Ok(LinearA::SignA361),
            '𐚖' => Ok(LinearA::SignA362),
            '𐚗' => Ok(LinearA::SignA363),
            '𐚘' => Ok(LinearA::SignA364),
            '𐚙' => Ok(LinearA::SignA365),
            '𐚚' => Ok(LinearA::SignA366),
            '𐚛' => Ok(LinearA::SignA367),
            '𐚜' => Ok(LinearA::SignA368),
            '𐚝' => Ok(LinearA::SignA369),
            '𐚞' => Ok(LinearA::SignA370),
            '𐚟' => Ok(LinearA::SignA371),
            '𐚠' => Ok(LinearA::SignA400DashVas),
            '𐚡' => Ok(LinearA::SignA401DashVas),
            '𐚢' => Ok(LinearA::SignA402DashVas),
            '𐚣' => Ok(LinearA::SignA403DashVas),
            '𐚤' => Ok(LinearA::SignA404DashVas),
            '𐚥' => Ok(LinearA::SignA405DashVas),
            '𐚦' => Ok(LinearA::SignA406DashVas),
            '𐚧' => Ok(LinearA::SignA407DashVas),
            '𐚨' => Ok(LinearA::SignA408DashVas),
            '𐚩' => Ok(LinearA::SignA409DashVas),
            '𐚪' => Ok(LinearA::SignA410DashVas),
            '𐚫' => Ok(LinearA::SignA411DashVas),
            '𐚬' => Ok(LinearA::SignA412DashVas),
            '𐚭' => Ok(LinearA::SignA413DashVas),
            '𐚮' => Ok(LinearA::SignA414DashVas),
            '𐚯' => Ok(LinearA::SignA415DashVas),
            '𐚰' => Ok(LinearA::SignA416DashVas),
            '𐚱' => Ok(LinearA::SignA417DashVas),
            '𐚲' => Ok(LinearA::SignA418DashVas),
            '𐚳' => Ok(LinearA::SignA501),
            '𐚴' => Ok(LinearA::SignA502),
            '𐚵' => Ok(LinearA::SignA503),
            '𐚶' => Ok(LinearA::SignA504),
            '𐚷' => Ok(LinearA::SignA505),
            '𐚸' => Ok(LinearA::SignA506),
            '𐚹' => Ok(LinearA::SignA508),
            '𐚺' => Ok(LinearA::SignA509),
            '𐚻' => Ok(LinearA::SignA510),
            '𐚼' => Ok(LinearA::SignA511),
            '𐚽' => Ok(LinearA::SignA512),
            '𐚾' => Ok(LinearA::SignA513),
            '𐚿' => Ok(LinearA::SignA515),
            '𐛀' => Ok(LinearA::SignA516),
            '𐛁' => Ok(LinearA::SignA520),
            '𐛂' => Ok(LinearA::SignA521),
            '𐛃' => Ok(LinearA::SignA523),
            '𐛄' => Ok(LinearA::SignA524),
            '𐛅' => Ok(LinearA::SignA525),
            '𐛆' => Ok(LinearA::SignA526),
            '𐛇' => Ok(LinearA::SignA527),
            '𐛈' => Ok(LinearA::SignA528),
            '𐛉' => Ok(LinearA::SignA529),
            '𐛊' => Ok(LinearA::SignA530),
            '𐛋' => Ok(LinearA::SignA531),
            '𐛌' => Ok(LinearA::SignA532),
            '𐛍' => Ok(LinearA::SignA534),
            '𐛎' => Ok(LinearA::SignA535),
            '𐛏' => Ok(LinearA::SignA536),
            '𐛐' => Ok(LinearA::SignA537),
            '𐛑' => Ok(LinearA::SignA538),
            '𐛒' => Ok(LinearA::SignA539),
            '𐛓' => Ok(LinearA::SignA540),
            '𐛔' => Ok(LinearA::SignA541),
            '𐛕' => Ok(LinearA::SignA542),
            '𐛖' => Ok(LinearA::SignA545),
            '𐛗' => Ok(LinearA::SignA547),
            '𐛘' => Ok(LinearA::SignA548),
            '𐛙' => Ok(LinearA::SignA549),
            '𐛚' => Ok(LinearA::SignA550),
            '𐛛' => Ok(LinearA::SignA551),
            '𐛜' => Ok(LinearA::SignA552),
            '𐛝' => Ok(LinearA::SignA553),
            '𐛞' => Ok(LinearA::SignA554),
            '𐛟' => Ok(LinearA::SignA555),
            '𐛠' => Ok(LinearA::SignA556),
            '𐛡' => Ok(LinearA::SignA557),
            '𐛢' => Ok(LinearA::SignA559),
            '𐛣' => Ok(LinearA::SignA563),
            '𐛤' => Ok(LinearA::SignA564),
            '𐛥' => Ok(LinearA::SignA565),
            '𐛦' => Ok(LinearA::SignA566),
            '𐛧' => Ok(LinearA::SignA568),
            '𐛨' => Ok(LinearA::SignA569),
            '𐛩' => Ok(LinearA::SignA570),
            '𐛪' => Ok(LinearA::SignA571),
            '𐛫' => Ok(LinearA::SignA572),
            '𐛬' => Ok(LinearA::SignA573),
            '𐛭' => Ok(LinearA::SignA574),
            '𐛮' => Ok(LinearA::SignA575),
            '𐛯' => Ok(LinearA::SignA576),
            '𐛰' => Ok(LinearA::SignA577),
            '𐛱' => Ok(LinearA::SignA578),
            '𐛲' => Ok(LinearA::SignA579),
            '𐛳' => Ok(LinearA::SignA580),
            '𐛴' => Ok(LinearA::SignA581),
            '𐛵' => Ok(LinearA::SignA582),
            '𐛶' => Ok(LinearA::SignA583),
            '𐛷' => Ok(LinearA::SignA584),
            '𐛸' => Ok(LinearA::SignA585),
            '𐛹' => Ok(LinearA::SignA586),
            '𐛺' => Ok(LinearA::SignA587),
            '𐛻' => Ok(LinearA::SignA588),
            '𐛼' => Ok(LinearA::SignA589),
            '𐛽' => Ok(LinearA::SignA591),
            '𐛾' => Ok(LinearA::SignA592),
            '𐛿' => Ok(LinearA::SignA594),
            '𐜀' => Ok(LinearA::SignA595),
            '𐜁' => Ok(LinearA::SignA596),
            '𐜂' => Ok(LinearA::SignA598),
            '𐜃' => Ok(LinearA::SignA600),
            '𐜄' => Ok(LinearA::SignA601),
            '𐜅' => Ok(LinearA::SignA602),
            '𐜆' => Ok(LinearA::SignA603),
            '𐜇' => Ok(LinearA::SignA604),
            '𐜈' => Ok(LinearA::SignA606),
            '𐜉' => Ok(LinearA::SignA608),
            '𐜊' => Ok(LinearA::SignA609),
            '𐜋' => Ok(LinearA::SignA610),
            '𐜌' => Ok(LinearA::SignA611),
            '𐜍' => Ok(LinearA::SignA612),
            '𐜎' => Ok(LinearA::SignA613),
            '𐜏' => Ok(LinearA::SignA614),
            '𐜐' => Ok(LinearA::SignA615),
            '𐜑' => Ok(LinearA::SignA616),
            '𐜒' => Ok(LinearA::SignA617),
            '𐜓' => Ok(LinearA::SignA618),
            '𐜔' => Ok(LinearA::SignA619),
            '𐜕' => Ok(LinearA::SignA620),
            '𐜖' => Ok(LinearA::SignA621),
            '𐜗' => Ok(LinearA::SignA622),
            '𐜘' => Ok(LinearA::SignA623),
            '𐜙' => Ok(LinearA::SignA624),
            '𐜚' => Ok(LinearA::SignA626),
            '𐜛' => Ok(LinearA::SignA627),
            '𐜜' => Ok(LinearA::SignA628),
            '𐜝' => Ok(LinearA::SignA629),
            '𐜞' => Ok(LinearA::SignA634),
            '𐜟' => Ok(LinearA::SignA637),
            '𐜠' => Ok(LinearA::SignA638),
            '𐜡' => Ok(LinearA::SignA640),
            '𐜢' => Ok(LinearA::SignA642),
            '𐜣' => Ok(LinearA::SignA643),
            '𐜤' => Ok(LinearA::SignA644),
            '𐜥' => Ok(LinearA::SignA645),
            '𐜦' => Ok(LinearA::SignA646),
            '𐜧' => Ok(LinearA::SignA648),
            '𐜨' => Ok(LinearA::SignA649),
            '𐜩' => Ok(LinearA::SignA651),
            '𐜪' => Ok(LinearA::SignA652),
            '𐜫' => Ok(LinearA::SignA653),
            '𐜬' => Ok(LinearA::SignA654),
            '𐜭' => Ok(LinearA::SignA655),
            '𐜮' => Ok(LinearA::SignA656),
            '𐜯' => Ok(LinearA::SignA657),
            '𐜰' => Ok(LinearA::SignA658),
            '𐜱' => Ok(LinearA::SignA659),
            '𐜲' => Ok(LinearA::SignA660),
            '𐜳' => Ok(LinearA::SignA661),
            '𐜴' => Ok(LinearA::SignA662),
            '𐜵' => Ok(LinearA::SignA663),
            '𐜶' => Ok(LinearA::SignA664),
            '𐝀' => Ok(LinearA::SignA701A),
            '𐝁' => Ok(LinearA::SignA702B),
            '𐝂' => Ok(LinearA::SignA703D),
            '𐝃' => Ok(LinearA::SignA704E),
            '𐝄' => Ok(LinearA::SignA705F),
            '𐝅' => Ok(LinearA::SignA706H),
            '𐝆' => Ok(LinearA::SignA707J),
            '𐝇' => Ok(LinearA::SignA708K),
            '𐝈' => Ok(LinearA::SignA709L),
            '𐝉' => Ok(LinearA::SignA709Dash2L2),
            '𐝊' => Ok(LinearA::SignA709Dash3L3),
            '𐝋' => Ok(LinearA::SignA709Dash4L4),
            '𐝌' => Ok(LinearA::SignA709Dash6L6),
            '𐝍' => Ok(LinearA::SignA710W),
            '𐝎' => Ok(LinearA::SignA711X),
            '𐝏' => Ok(LinearA::SignA712Y),
            '𐝐' => Ok(LinearA::SignA713Omega),
            '𐝑' => Ok(LinearA::SignA714Abb),
            '𐝒' => Ok(LinearA::SignA715Bb),
            '𐝓' => Ok(LinearA::SignA717Dd),
            '𐝔' => Ok(LinearA::SignA726Eyyy),
            '𐝕' => Ok(LinearA::SignA732Je),
            '𐝠' => Ok(LinearA::SignA800),
            '𐝡' => Ok(LinearA::SignA801),
            '𐝢' => Ok(LinearA::SignA802),
            '𐝣' => Ok(LinearA::SignA803),
            '𐝤' => Ok(LinearA::SignA804),
            '𐝥' => Ok(LinearA::SignA805),
            '𐝦' => Ok(LinearA::SignA806),
            '𐝧' => Ok(LinearA::SignA807),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LinearA {
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

impl std::convert::TryFrom<u32> for LinearA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LinearA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LinearA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LinearA::SignAb001
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("LinearA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
