
/// An enum to represent all characters in the MendeKikakui block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MendeKikakui {
    /// \u{1e800}: '𞠀'
    SyllableM001Ki,
    /// \u{1e801}: '𞠁'
    SyllableM002Ka,
    /// \u{1e802}: '𞠂'
    SyllableM003Ku,
    /// \u{1e803}: '𞠃'
    SyllableM065Kee,
    /// \u{1e804}: '𞠄'
    SyllableM095Ke,
    /// \u{1e805}: '𞠅'
    SyllableM076Koo,
    /// \u{1e806}: '𞠆'
    SyllableM048Ko,
    /// \u{1e807}: '𞠇'
    SyllableM179Kua,
    /// \u{1e808}: '𞠈'
    SyllableM004Wi,
    /// \u{1e809}: '𞠉'
    SyllableM005Wa,
    /// \u{1e80a}: '𞠊'
    SyllableM006Wu,
    /// \u{1e80b}: '𞠋'
    SyllableM126Wee,
    /// \u{1e80c}: '𞠌'
    SyllableM118We,
    /// \u{1e80d}: '𞠍'
    SyllableM114Woo,
    /// \u{1e80e}: '𞠎'
    SyllableM045Wo,
    /// \u{1e80f}: '𞠏'
    SyllableM194Wui,
    /// \u{1e810}: '𞠐'
    SyllableM143Wei,
    /// \u{1e811}: '𞠑'
    SyllableM061Wvi,
    /// \u{1e812}: '𞠒'
    SyllableM049Wva,
    /// \u{1e813}: '𞠓'
    SyllableM139Wve,
    /// \u{1e814}: '𞠔'
    SyllableM007Min,
    /// \u{1e815}: '𞠕'
    SyllableM008Man,
    /// \u{1e816}: '𞠖'
    SyllableM009Mun,
    /// \u{1e817}: '𞠗'
    SyllableM059Men,
    /// \u{1e818}: '𞠘'
    SyllableM094Mon,
    /// \u{1e819}: '𞠙'
    SyllableM154Muan,
    /// \u{1e81a}: '𞠚'
    SyllableM189Muen,
    /// \u{1e81b}: '𞠛'
    SyllableM010Bi,
    /// \u{1e81c}: '𞠜'
    SyllableM011Ba,
    /// \u{1e81d}: '𞠝'
    SyllableM012Bu,
    /// \u{1e81e}: '𞠞'
    SyllableM150Bee,
    /// \u{1e81f}: '𞠟'
    SyllableM097Be,
    /// \u{1e820}: '𞠠'
    SyllableM103Boo,
    /// \u{1e821}: '𞠡'
    SyllableM138Bo,
    /// \u{1e822}: '𞠢'
    SyllableM013I,
    /// \u{1e823}: '𞠣'
    SyllableM014A,
    /// \u{1e824}: '𞠤'
    SyllableM015U,
    /// \u{1e825}: '𞠥'
    SyllableM163Ee,
    /// \u{1e826}: '𞠦'
    SyllableM100E,
    /// \u{1e827}: '𞠧'
    SyllableM165Oo,
    /// \u{1e828}: '𞠨'
    SyllableM147O,
    /// \u{1e829}: '𞠩'
    SyllableM137Ei,
    /// \u{1e82a}: '𞠪'
    SyllableM131In,
    /// \u{1e82b}: '𞠫'
    SyllableM135In,
    /// \u{1e82c}: '𞠬'
    SyllableM195An,
    /// \u{1e82d}: '𞠭'
    SyllableM178En,
    /// \u{1e82e}: '𞠮'
    SyllableM019Si,
    /// \u{1e82f}: '𞠯'
    SyllableM020Sa,
    /// \u{1e830}: '𞠰'
    SyllableM021Su,
    /// \u{1e831}: '𞠱'
    SyllableM162See,
    /// \u{1e832}: '𞠲'
    SyllableM116Se,
    /// \u{1e833}: '𞠳'
    SyllableM136Soo,
    /// \u{1e834}: '𞠴'
    SyllableM079So,
    /// \u{1e835}: '𞠵'
    SyllableM196Sia,
    /// \u{1e836}: '𞠶'
    SyllableM025Li,
    /// \u{1e837}: '𞠷'
    SyllableM026La,
    /// \u{1e838}: '𞠸'
    SyllableM027Lu,
    /// \u{1e839}: '𞠹'
    SyllableM084Lee,
    /// \u{1e83a}: '𞠺'
    SyllableM073Le,
    /// \u{1e83b}: '𞠻'
    SyllableM054Loo,
    /// \u{1e83c}: '𞠼'
    SyllableM153Lo,
    /// \u{1e83d}: '𞠽'
    SyllableM110LongLe,
    /// \u{1e83e}: '𞠾'
    SyllableM016Di,
    /// \u{1e83f}: '𞠿'
    SyllableM017Da,
    /// \u{1e840}: '𞡀'
    SyllableM018Du,
    /// \u{1e841}: '𞡁'
    SyllableM089Dee,
    /// \u{1e842}: '𞡂'
    SyllableM180Doo,
    /// \u{1e843}: '𞡃'
    SyllableM181Do,
    /// \u{1e844}: '𞡄'
    SyllableM022Ti,
    /// \u{1e845}: '𞡅'
    SyllableM023Ta,
    /// \u{1e846}: '𞡆'
    SyllableM024Tu,
    /// \u{1e847}: '𞡇'
    SyllableM091Tee,
    /// \u{1e848}: '𞡈'
    SyllableM055Te,
    /// \u{1e849}: '𞡉'
    SyllableM104Too,
    /// \u{1e84a}: '𞡊'
    SyllableM069To,
    /// \u{1e84b}: '𞡋'
    SyllableM028Ji,
    /// \u{1e84c}: '𞡌'
    SyllableM029Ja,
    /// \u{1e84d}: '𞡍'
    SyllableM030Ju,
    /// \u{1e84e}: '𞡎'
    SyllableM157Jee,
    /// \u{1e84f}: '𞡏'
    SyllableM113Je,
    /// \u{1e850}: '𞡐'
    SyllableM160Joo,
    /// \u{1e851}: '𞡑'
    SyllableM063Jo,
    /// \u{1e852}: '𞡒'
    SyllableM175LongJo,
    /// \u{1e853}: '𞡓'
    SyllableM031Yi,
    /// \u{1e854}: '𞡔'
    SyllableM032Ya,
    /// \u{1e855}: '𞡕'
    SyllableM033Yu,
    /// \u{1e856}: '𞡖'
    SyllableM109Yee,
    /// \u{1e857}: '𞡗'
    SyllableM080Ye,
    /// \u{1e858}: '𞡘'
    SyllableM141Yoo,
    /// \u{1e859}: '𞡙'
    SyllableM121Yo,
    /// \u{1e85a}: '𞡚'
    SyllableM034Fi,
    /// \u{1e85b}: '𞡛'
    SyllableM035Fa,
    /// \u{1e85c}: '𞡜'
    SyllableM036Fu,
    /// \u{1e85d}: '𞡝'
    SyllableM078Fee,
    /// \u{1e85e}: '𞡞'
    SyllableM075Fe,
    /// \u{1e85f}: '𞡟'
    SyllableM133Foo,
    /// \u{1e860}: '𞡠'
    SyllableM088Fo,
    /// \u{1e861}: '𞡡'
    SyllableM197Fua,
    /// \u{1e862}: '𞡢'
    SyllableM101Fan,
    /// \u{1e863}: '𞡣'
    SyllableM037Nin,
    /// \u{1e864}: '𞡤'
    SyllableM038Nan,
    /// \u{1e865}: '𞡥'
    SyllableM039Nun,
    /// \u{1e866}: '𞡦'
    SyllableM117Nen,
    /// \u{1e867}: '𞡧'
    SyllableM169Non,
    /// \u{1e868}: '𞡨'
    SyllableM176Hi,
    /// \u{1e869}: '𞡩'
    SyllableM041Ha,
    /// \u{1e86a}: '𞡪'
    SyllableM186Hu,
    /// \u{1e86b}: '𞡫'
    SyllableM040Hee,
    /// \u{1e86c}: '𞡬'
    SyllableM096He,
    /// \u{1e86d}: '𞡭'
    SyllableM042Hoo,
    /// \u{1e86e}: '𞡮'
    SyllableM140Ho,
    /// \u{1e86f}: '𞡯'
    SyllableM083Heei,
    /// \u{1e870}: '𞡰'
    SyllableM128Hoou,
    /// \u{1e871}: '𞡱'
    SyllableM053Hin,
    /// \u{1e872}: '𞡲'
    SyllableM130Han,
    /// \u{1e873}: '𞡳'
    SyllableM087Hun,
    /// \u{1e874}: '𞡴'
    SyllableM052Hen,
    /// \u{1e875}: '𞡵'
    SyllableM193Hon,
    /// \u{1e876}: '𞡶'
    SyllableM046Huan,
    /// \u{1e877}: '𞡷'
    SyllableM090Nggi,
    /// \u{1e878}: '𞡸'
    SyllableM043Ngga,
    /// \u{1e879}: '𞡹'
    SyllableM082Nggu,
    /// \u{1e87a}: '𞡺'
    SyllableM115Nggee,
    /// \u{1e87b}: '𞡻'
    SyllableM146Ngge,
    /// \u{1e87c}: '𞡼'
    SyllableM156Nggoo,
    /// \u{1e87d}: '𞡽'
    SyllableM120Nggo,
    /// \u{1e87e}: '𞡾'
    SyllableM159Nggaa,
    /// \u{1e87f}: '𞡿'
    SyllableM127Nggua,
    /// \u{1e880}: '𞢀'
    SyllableM086LongNgge,
    /// \u{1e881}: '𞢁'
    SyllableM106LongNggoo,
    /// \u{1e882}: '𞢂'
    SyllableM183LongNggo,
    /// \u{1e883}: '𞢃'
    SyllableM155Gi,
    /// \u{1e884}: '𞢄'
    SyllableM111Ga,
    /// \u{1e885}: '𞢅'
    SyllableM168Gu,
    /// \u{1e886}: '𞢆'
    SyllableM190Gee,
    /// \u{1e887}: '𞢇'
    SyllableM166Guei,
    /// \u{1e888}: '𞢈'
    SyllableM167Guan,
    /// \u{1e889}: '𞢉'
    SyllableM184Ngen,
    /// \u{1e88a}: '𞢊'
    SyllableM057Ngon,
    /// \u{1e88b}: '𞢋'
    SyllableM177Nguan,
    /// \u{1e88c}: '𞢌'
    SyllableM068Pi,
    /// \u{1e88d}: '𞢍'
    SyllableM099Pa,
    /// \u{1e88e}: '𞢎'
    SyllableM050Pu,
    /// \u{1e88f}: '𞢏'
    SyllableM081Pee,
    /// \u{1e890}: '𞢐'
    SyllableM051Pe,
    /// \u{1e891}: '𞢑'
    SyllableM102Poo,
    /// \u{1e892}: '𞢒'
    SyllableM066Po,
    /// \u{1e893}: '𞢓'
    SyllableM145Mbi,
    /// \u{1e894}: '𞢔'
    SyllableM062Mba,
    /// \u{1e895}: '𞢕'
    SyllableM122Mbu,
    /// \u{1e896}: '𞢖'
    SyllableM047Mbee,
    /// \u{1e897}: '𞢗'
    SyllableM188Mbee,
    /// \u{1e898}: '𞢘'
    SyllableM072Mbe,
    /// \u{1e899}: '𞢙'
    SyllableM172Mboo,
    /// \u{1e89a}: '𞢚'
    SyllableM174Mbo,
    /// \u{1e89b}: '𞢛'
    SyllableM187Mbuu,
    /// \u{1e89c}: '𞢜'
    SyllableM161LongMbe,
    /// \u{1e89d}: '𞢝'
    SyllableM105LongMboo,
    /// \u{1e89e}: '𞢞'
    SyllableM142LongMbo,
    /// \u{1e89f}: '𞢟'
    SyllableM132Kpi,
    /// \u{1e8a0}: '𞢠'
    SyllableM092Kpa,
    /// \u{1e8a1}: '𞢡'
    SyllableM074Kpu,
    /// \u{1e8a2}: '𞢢'
    SyllableM044Kpee,
    /// \u{1e8a3}: '𞢣'
    SyllableM108Kpe,
    /// \u{1e8a4}: '𞢤'
    SyllableM112Kpoo,
    /// \u{1e8a5}: '𞢥'
    SyllableM158Kpo,
    /// \u{1e8a6}: '𞢦'
    SyllableM124Gbi,
    /// \u{1e8a7}: '𞢧'
    SyllableM056Gba,
    /// \u{1e8a8}: '𞢨'
    SyllableM148Gbu,
    /// \u{1e8a9}: '𞢩'
    SyllableM093Gbee,
    /// \u{1e8aa}: '𞢪'
    SyllableM107Gbe,
    /// \u{1e8ab}: '𞢫'
    SyllableM071Gboo,
    /// \u{1e8ac}: '𞢬'
    SyllableM070Gbo,
    /// \u{1e8ad}: '𞢭'
    SyllableM171Ra,
    /// \u{1e8ae}: '𞢮'
    SyllableM123Ndi,
    /// \u{1e8af}: '𞢯'
    SyllableM129Nda,
    /// \u{1e8b0}: '𞢰'
    SyllableM125Ndu,
    /// \u{1e8b1}: '𞢱'
    SyllableM191Ndee,
    /// \u{1e8b2}: '𞢲'
    SyllableM119Nde,
    /// \u{1e8b3}: '𞢳'
    SyllableM067Ndoo,
    /// \u{1e8b4}: '𞢴'
    SyllableM064Ndo,
    /// \u{1e8b5}: '𞢵'
    SyllableM152Nja,
    /// \u{1e8b6}: '𞢶'
    SyllableM192Nju,
    /// \u{1e8b7}: '𞢷'
    SyllableM149Njee,
    /// \u{1e8b8}: '𞢸'
    SyllableM134Njoo,
    /// \u{1e8b9}: '𞢹'
    SyllableM182Vi,
    /// \u{1e8ba}: '𞢺'
    SyllableM185Va,
    /// \u{1e8bb}: '𞢻'
    SyllableM151Vu,
    /// \u{1e8bc}: '𞢼'
    SyllableM173Vee,
    /// \u{1e8bd}: '𞢽'
    SyllableM085Ve,
    /// \u{1e8be}: '𞢾'
    SyllableM144Voo,
    /// \u{1e8bf}: '𞢿'
    SyllableM077Vo,
    /// \u{1e8c0}: '𞣀'
    SyllableM164Nyin,
    /// \u{1e8c1}: '𞣁'
    SyllableM058Nyan,
    /// \u{1e8c2}: '𞣂'
    SyllableM170Nyun,
    /// \u{1e8c3}: '𞣃'
    SyllableM098Nyen,
    /// \u{1e8c4}: '𞣄'
    SyllableM060Nyon,
    /// \u{1e8c7}: '𞣇'
    DigitOne,
    /// \u{1e8c8}: '𞣈'
    DigitTwo,
    /// \u{1e8c9}: '𞣉'
    DigitThree,
    /// \u{1e8ca}: '𞣊'
    DigitFour,
    /// \u{1e8cb}: '𞣋'
    DigitFive,
    /// \u{1e8cc}: '𞣌'
    DigitSix,
    /// \u{1e8cd}: '𞣍'
    DigitSeven,
    /// \u{1e8ce}: '𞣎'
    DigitEight,
    /// \u{1e8cf}: '𞣏'
    DigitNine,
    /// \u{1e8d0}: '𞣐'
    CombiningNumberTeens,
    /// \u{1e8d1}: '𞣑'
    CombiningNumberTens,
    /// \u{1e8d2}: '𞣒'
    CombiningNumberHundreds,
    /// \u{1e8d3}: '𞣓'
    CombiningNumberThousands,
    /// \u{1e8d4}: '𞣔'
    CombiningNumberTenThousands,
    /// \u{1e8d5}: '𞣕'
    CombiningNumberHundredThousands,
    /// \u{1e8d6}: '𞣖'
    CombiningNumberMillions,
}

impl Into<char> for MendeKikakui {
    fn into(self) -> char {
        match self {
            MendeKikakui::SyllableM001Ki => '𞠀',
            MendeKikakui::SyllableM002Ka => '𞠁',
            MendeKikakui::SyllableM003Ku => '𞠂',
            MendeKikakui::SyllableM065Kee => '𞠃',
            MendeKikakui::SyllableM095Ke => '𞠄',
            MendeKikakui::SyllableM076Koo => '𞠅',
            MendeKikakui::SyllableM048Ko => '𞠆',
            MendeKikakui::SyllableM179Kua => '𞠇',
            MendeKikakui::SyllableM004Wi => '𞠈',
            MendeKikakui::SyllableM005Wa => '𞠉',
            MendeKikakui::SyllableM006Wu => '𞠊',
            MendeKikakui::SyllableM126Wee => '𞠋',
            MendeKikakui::SyllableM118We => '𞠌',
            MendeKikakui::SyllableM114Woo => '𞠍',
            MendeKikakui::SyllableM045Wo => '𞠎',
            MendeKikakui::SyllableM194Wui => '𞠏',
            MendeKikakui::SyllableM143Wei => '𞠐',
            MendeKikakui::SyllableM061Wvi => '𞠑',
            MendeKikakui::SyllableM049Wva => '𞠒',
            MendeKikakui::SyllableM139Wve => '𞠓',
            MendeKikakui::SyllableM007Min => '𞠔',
            MendeKikakui::SyllableM008Man => '𞠕',
            MendeKikakui::SyllableM009Mun => '𞠖',
            MendeKikakui::SyllableM059Men => '𞠗',
            MendeKikakui::SyllableM094Mon => '𞠘',
            MendeKikakui::SyllableM154Muan => '𞠙',
            MendeKikakui::SyllableM189Muen => '𞠚',
            MendeKikakui::SyllableM010Bi => '𞠛',
            MendeKikakui::SyllableM011Ba => '𞠜',
            MendeKikakui::SyllableM012Bu => '𞠝',
            MendeKikakui::SyllableM150Bee => '𞠞',
            MendeKikakui::SyllableM097Be => '𞠟',
            MendeKikakui::SyllableM103Boo => '𞠠',
            MendeKikakui::SyllableM138Bo => '𞠡',
            MendeKikakui::SyllableM013I => '𞠢',
            MendeKikakui::SyllableM014A => '𞠣',
            MendeKikakui::SyllableM015U => '𞠤',
            MendeKikakui::SyllableM163Ee => '𞠥',
            MendeKikakui::SyllableM100E => '𞠦',
            MendeKikakui::SyllableM165Oo => '𞠧',
            MendeKikakui::SyllableM147O => '𞠨',
            MendeKikakui::SyllableM137Ei => '𞠩',
            MendeKikakui::SyllableM131In => '𞠪',
            MendeKikakui::SyllableM135In => '𞠫',
            MendeKikakui::SyllableM195An => '𞠬',
            MendeKikakui::SyllableM178En => '𞠭',
            MendeKikakui::SyllableM019Si => '𞠮',
            MendeKikakui::SyllableM020Sa => '𞠯',
            MendeKikakui::SyllableM021Su => '𞠰',
            MendeKikakui::SyllableM162See => '𞠱',
            MendeKikakui::SyllableM116Se => '𞠲',
            MendeKikakui::SyllableM136Soo => '𞠳',
            MendeKikakui::SyllableM079So => '𞠴',
            MendeKikakui::SyllableM196Sia => '𞠵',
            MendeKikakui::SyllableM025Li => '𞠶',
            MendeKikakui::SyllableM026La => '𞠷',
            MendeKikakui::SyllableM027Lu => '𞠸',
            MendeKikakui::SyllableM084Lee => '𞠹',
            MendeKikakui::SyllableM073Le => '𞠺',
            MendeKikakui::SyllableM054Loo => '𞠻',
            MendeKikakui::SyllableM153Lo => '𞠼',
            MendeKikakui::SyllableM110LongLe => '𞠽',
            MendeKikakui::SyllableM016Di => '𞠾',
            MendeKikakui::SyllableM017Da => '𞠿',
            MendeKikakui::SyllableM018Du => '𞡀',
            MendeKikakui::SyllableM089Dee => '𞡁',
            MendeKikakui::SyllableM180Doo => '𞡂',
            MendeKikakui::SyllableM181Do => '𞡃',
            MendeKikakui::SyllableM022Ti => '𞡄',
            MendeKikakui::SyllableM023Ta => '𞡅',
            MendeKikakui::SyllableM024Tu => '𞡆',
            MendeKikakui::SyllableM091Tee => '𞡇',
            MendeKikakui::SyllableM055Te => '𞡈',
            MendeKikakui::SyllableM104Too => '𞡉',
            MendeKikakui::SyllableM069To => '𞡊',
            MendeKikakui::SyllableM028Ji => '𞡋',
            MendeKikakui::SyllableM029Ja => '𞡌',
            MendeKikakui::SyllableM030Ju => '𞡍',
            MendeKikakui::SyllableM157Jee => '𞡎',
            MendeKikakui::SyllableM113Je => '𞡏',
            MendeKikakui::SyllableM160Joo => '𞡐',
            MendeKikakui::SyllableM063Jo => '𞡑',
            MendeKikakui::SyllableM175LongJo => '𞡒',
            MendeKikakui::SyllableM031Yi => '𞡓',
            MendeKikakui::SyllableM032Ya => '𞡔',
            MendeKikakui::SyllableM033Yu => '𞡕',
            MendeKikakui::SyllableM109Yee => '𞡖',
            MendeKikakui::SyllableM080Ye => '𞡗',
            MendeKikakui::SyllableM141Yoo => '𞡘',
            MendeKikakui::SyllableM121Yo => '𞡙',
            MendeKikakui::SyllableM034Fi => '𞡚',
            MendeKikakui::SyllableM035Fa => '𞡛',
            MendeKikakui::SyllableM036Fu => '𞡜',
            MendeKikakui::SyllableM078Fee => '𞡝',
            MendeKikakui::SyllableM075Fe => '𞡞',
            MendeKikakui::SyllableM133Foo => '𞡟',
            MendeKikakui::SyllableM088Fo => '𞡠',
            MendeKikakui::SyllableM197Fua => '𞡡',
            MendeKikakui::SyllableM101Fan => '𞡢',
            MendeKikakui::SyllableM037Nin => '𞡣',
            MendeKikakui::SyllableM038Nan => '𞡤',
            MendeKikakui::SyllableM039Nun => '𞡥',
            MendeKikakui::SyllableM117Nen => '𞡦',
            MendeKikakui::SyllableM169Non => '𞡧',
            MendeKikakui::SyllableM176Hi => '𞡨',
            MendeKikakui::SyllableM041Ha => '𞡩',
            MendeKikakui::SyllableM186Hu => '𞡪',
            MendeKikakui::SyllableM040Hee => '𞡫',
            MendeKikakui::SyllableM096He => '𞡬',
            MendeKikakui::SyllableM042Hoo => '𞡭',
            MendeKikakui::SyllableM140Ho => '𞡮',
            MendeKikakui::SyllableM083Heei => '𞡯',
            MendeKikakui::SyllableM128Hoou => '𞡰',
            MendeKikakui::SyllableM053Hin => '𞡱',
            MendeKikakui::SyllableM130Han => '𞡲',
            MendeKikakui::SyllableM087Hun => '𞡳',
            MendeKikakui::SyllableM052Hen => '𞡴',
            MendeKikakui::SyllableM193Hon => '𞡵',
            MendeKikakui::SyllableM046Huan => '𞡶',
            MendeKikakui::SyllableM090Nggi => '𞡷',
            MendeKikakui::SyllableM043Ngga => '𞡸',
            MendeKikakui::SyllableM082Nggu => '𞡹',
            MendeKikakui::SyllableM115Nggee => '𞡺',
            MendeKikakui::SyllableM146Ngge => '𞡻',
            MendeKikakui::SyllableM156Nggoo => '𞡼',
            MendeKikakui::SyllableM120Nggo => '𞡽',
            MendeKikakui::SyllableM159Nggaa => '𞡾',
            MendeKikakui::SyllableM127Nggua => '𞡿',
            MendeKikakui::SyllableM086LongNgge => '𞢀',
            MendeKikakui::SyllableM106LongNggoo => '𞢁',
            MendeKikakui::SyllableM183LongNggo => '𞢂',
            MendeKikakui::SyllableM155Gi => '𞢃',
            MendeKikakui::SyllableM111Ga => '𞢄',
            MendeKikakui::SyllableM168Gu => '𞢅',
            MendeKikakui::SyllableM190Gee => '𞢆',
            MendeKikakui::SyllableM166Guei => '𞢇',
            MendeKikakui::SyllableM167Guan => '𞢈',
            MendeKikakui::SyllableM184Ngen => '𞢉',
            MendeKikakui::SyllableM057Ngon => '𞢊',
            MendeKikakui::SyllableM177Nguan => '𞢋',
            MendeKikakui::SyllableM068Pi => '𞢌',
            MendeKikakui::SyllableM099Pa => '𞢍',
            MendeKikakui::SyllableM050Pu => '𞢎',
            MendeKikakui::SyllableM081Pee => '𞢏',
            MendeKikakui::SyllableM051Pe => '𞢐',
            MendeKikakui::SyllableM102Poo => '𞢑',
            MendeKikakui::SyllableM066Po => '𞢒',
            MendeKikakui::SyllableM145Mbi => '𞢓',
            MendeKikakui::SyllableM062Mba => '𞢔',
            MendeKikakui::SyllableM122Mbu => '𞢕',
            MendeKikakui::SyllableM047Mbee => '𞢖',
            MendeKikakui::SyllableM188Mbee => '𞢗',
            MendeKikakui::SyllableM072Mbe => '𞢘',
            MendeKikakui::SyllableM172Mboo => '𞢙',
            MendeKikakui::SyllableM174Mbo => '𞢚',
            MendeKikakui::SyllableM187Mbuu => '𞢛',
            MendeKikakui::SyllableM161LongMbe => '𞢜',
            MendeKikakui::SyllableM105LongMboo => '𞢝',
            MendeKikakui::SyllableM142LongMbo => '𞢞',
            MendeKikakui::SyllableM132Kpi => '𞢟',
            MendeKikakui::SyllableM092Kpa => '𞢠',
            MendeKikakui::SyllableM074Kpu => '𞢡',
            MendeKikakui::SyllableM044Kpee => '𞢢',
            MendeKikakui::SyllableM108Kpe => '𞢣',
            MendeKikakui::SyllableM112Kpoo => '𞢤',
            MendeKikakui::SyllableM158Kpo => '𞢥',
            MendeKikakui::SyllableM124Gbi => '𞢦',
            MendeKikakui::SyllableM056Gba => '𞢧',
            MendeKikakui::SyllableM148Gbu => '𞢨',
            MendeKikakui::SyllableM093Gbee => '𞢩',
            MendeKikakui::SyllableM107Gbe => '𞢪',
            MendeKikakui::SyllableM071Gboo => '𞢫',
            MendeKikakui::SyllableM070Gbo => '𞢬',
            MendeKikakui::SyllableM171Ra => '𞢭',
            MendeKikakui::SyllableM123Ndi => '𞢮',
            MendeKikakui::SyllableM129Nda => '𞢯',
            MendeKikakui::SyllableM125Ndu => '𞢰',
            MendeKikakui::SyllableM191Ndee => '𞢱',
            MendeKikakui::SyllableM119Nde => '𞢲',
            MendeKikakui::SyllableM067Ndoo => '𞢳',
            MendeKikakui::SyllableM064Ndo => '𞢴',
            MendeKikakui::SyllableM152Nja => '𞢵',
            MendeKikakui::SyllableM192Nju => '𞢶',
            MendeKikakui::SyllableM149Njee => '𞢷',
            MendeKikakui::SyllableM134Njoo => '𞢸',
            MendeKikakui::SyllableM182Vi => '𞢹',
            MendeKikakui::SyllableM185Va => '𞢺',
            MendeKikakui::SyllableM151Vu => '𞢻',
            MendeKikakui::SyllableM173Vee => '𞢼',
            MendeKikakui::SyllableM085Ve => '𞢽',
            MendeKikakui::SyllableM144Voo => '𞢾',
            MendeKikakui::SyllableM077Vo => '𞢿',
            MendeKikakui::SyllableM164Nyin => '𞣀',
            MendeKikakui::SyllableM058Nyan => '𞣁',
            MendeKikakui::SyllableM170Nyun => '𞣂',
            MendeKikakui::SyllableM098Nyen => '𞣃',
            MendeKikakui::SyllableM060Nyon => '𞣄',
            MendeKikakui::DigitOne => '𞣇',
            MendeKikakui::DigitTwo => '𞣈',
            MendeKikakui::DigitThree => '𞣉',
            MendeKikakui::DigitFour => '𞣊',
            MendeKikakui::DigitFive => '𞣋',
            MendeKikakui::DigitSix => '𞣌',
            MendeKikakui::DigitSeven => '𞣍',
            MendeKikakui::DigitEight => '𞣎',
            MendeKikakui::DigitNine => '𞣏',
            MendeKikakui::CombiningNumberTeens => '𞣐',
            MendeKikakui::CombiningNumberTens => '𞣑',
            MendeKikakui::CombiningNumberHundreds => '𞣒',
            MendeKikakui::CombiningNumberThousands => '𞣓',
            MendeKikakui::CombiningNumberTenThousands => '𞣔',
            MendeKikakui::CombiningNumberHundredThousands => '𞣕',
            MendeKikakui::CombiningNumberMillions => '𞣖',
        }
    }
}

impl std::convert::TryFrom<char> for MendeKikakui {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𞠀' => Ok(MendeKikakui::SyllableM001Ki),
            '𞠁' => Ok(MendeKikakui::SyllableM002Ka),
            '𞠂' => Ok(MendeKikakui::SyllableM003Ku),
            '𞠃' => Ok(MendeKikakui::SyllableM065Kee),
            '𞠄' => Ok(MendeKikakui::SyllableM095Ke),
            '𞠅' => Ok(MendeKikakui::SyllableM076Koo),
            '𞠆' => Ok(MendeKikakui::SyllableM048Ko),
            '𞠇' => Ok(MendeKikakui::SyllableM179Kua),
            '𞠈' => Ok(MendeKikakui::SyllableM004Wi),
            '𞠉' => Ok(MendeKikakui::SyllableM005Wa),
            '𞠊' => Ok(MendeKikakui::SyllableM006Wu),
            '𞠋' => Ok(MendeKikakui::SyllableM126Wee),
            '𞠌' => Ok(MendeKikakui::SyllableM118We),
            '𞠍' => Ok(MendeKikakui::SyllableM114Woo),
            '𞠎' => Ok(MendeKikakui::SyllableM045Wo),
            '𞠏' => Ok(MendeKikakui::SyllableM194Wui),
            '𞠐' => Ok(MendeKikakui::SyllableM143Wei),
            '𞠑' => Ok(MendeKikakui::SyllableM061Wvi),
            '𞠒' => Ok(MendeKikakui::SyllableM049Wva),
            '𞠓' => Ok(MendeKikakui::SyllableM139Wve),
            '𞠔' => Ok(MendeKikakui::SyllableM007Min),
            '𞠕' => Ok(MendeKikakui::SyllableM008Man),
            '𞠖' => Ok(MendeKikakui::SyllableM009Mun),
            '𞠗' => Ok(MendeKikakui::SyllableM059Men),
            '𞠘' => Ok(MendeKikakui::SyllableM094Mon),
            '𞠙' => Ok(MendeKikakui::SyllableM154Muan),
            '𞠚' => Ok(MendeKikakui::SyllableM189Muen),
            '𞠛' => Ok(MendeKikakui::SyllableM010Bi),
            '𞠜' => Ok(MendeKikakui::SyllableM011Ba),
            '𞠝' => Ok(MendeKikakui::SyllableM012Bu),
            '𞠞' => Ok(MendeKikakui::SyllableM150Bee),
            '𞠟' => Ok(MendeKikakui::SyllableM097Be),
            '𞠠' => Ok(MendeKikakui::SyllableM103Boo),
            '𞠡' => Ok(MendeKikakui::SyllableM138Bo),
            '𞠢' => Ok(MendeKikakui::SyllableM013I),
            '𞠣' => Ok(MendeKikakui::SyllableM014A),
            '𞠤' => Ok(MendeKikakui::SyllableM015U),
            '𞠥' => Ok(MendeKikakui::SyllableM163Ee),
            '𞠦' => Ok(MendeKikakui::SyllableM100E),
            '𞠧' => Ok(MendeKikakui::SyllableM165Oo),
            '𞠨' => Ok(MendeKikakui::SyllableM147O),
            '𞠩' => Ok(MendeKikakui::SyllableM137Ei),
            '𞠪' => Ok(MendeKikakui::SyllableM131In),
            '𞠫' => Ok(MendeKikakui::SyllableM135In),
            '𞠬' => Ok(MendeKikakui::SyllableM195An),
            '𞠭' => Ok(MendeKikakui::SyllableM178En),
            '𞠮' => Ok(MendeKikakui::SyllableM019Si),
            '𞠯' => Ok(MendeKikakui::SyllableM020Sa),
            '𞠰' => Ok(MendeKikakui::SyllableM021Su),
            '𞠱' => Ok(MendeKikakui::SyllableM162See),
            '𞠲' => Ok(MendeKikakui::SyllableM116Se),
            '𞠳' => Ok(MendeKikakui::SyllableM136Soo),
            '𞠴' => Ok(MendeKikakui::SyllableM079So),
            '𞠵' => Ok(MendeKikakui::SyllableM196Sia),
            '𞠶' => Ok(MendeKikakui::SyllableM025Li),
            '𞠷' => Ok(MendeKikakui::SyllableM026La),
            '𞠸' => Ok(MendeKikakui::SyllableM027Lu),
            '𞠹' => Ok(MendeKikakui::SyllableM084Lee),
            '𞠺' => Ok(MendeKikakui::SyllableM073Le),
            '𞠻' => Ok(MendeKikakui::SyllableM054Loo),
            '𞠼' => Ok(MendeKikakui::SyllableM153Lo),
            '𞠽' => Ok(MendeKikakui::SyllableM110LongLe),
            '𞠾' => Ok(MendeKikakui::SyllableM016Di),
            '𞠿' => Ok(MendeKikakui::SyllableM017Da),
            '𞡀' => Ok(MendeKikakui::SyllableM018Du),
            '𞡁' => Ok(MendeKikakui::SyllableM089Dee),
            '𞡂' => Ok(MendeKikakui::SyllableM180Doo),
            '𞡃' => Ok(MendeKikakui::SyllableM181Do),
            '𞡄' => Ok(MendeKikakui::SyllableM022Ti),
            '𞡅' => Ok(MendeKikakui::SyllableM023Ta),
            '𞡆' => Ok(MendeKikakui::SyllableM024Tu),
            '𞡇' => Ok(MendeKikakui::SyllableM091Tee),
            '𞡈' => Ok(MendeKikakui::SyllableM055Te),
            '𞡉' => Ok(MendeKikakui::SyllableM104Too),
            '𞡊' => Ok(MendeKikakui::SyllableM069To),
            '𞡋' => Ok(MendeKikakui::SyllableM028Ji),
            '𞡌' => Ok(MendeKikakui::SyllableM029Ja),
            '𞡍' => Ok(MendeKikakui::SyllableM030Ju),
            '𞡎' => Ok(MendeKikakui::SyllableM157Jee),
            '𞡏' => Ok(MendeKikakui::SyllableM113Je),
            '𞡐' => Ok(MendeKikakui::SyllableM160Joo),
            '𞡑' => Ok(MendeKikakui::SyllableM063Jo),
            '𞡒' => Ok(MendeKikakui::SyllableM175LongJo),
            '𞡓' => Ok(MendeKikakui::SyllableM031Yi),
            '𞡔' => Ok(MendeKikakui::SyllableM032Ya),
            '𞡕' => Ok(MendeKikakui::SyllableM033Yu),
            '𞡖' => Ok(MendeKikakui::SyllableM109Yee),
            '𞡗' => Ok(MendeKikakui::SyllableM080Ye),
            '𞡘' => Ok(MendeKikakui::SyllableM141Yoo),
            '𞡙' => Ok(MendeKikakui::SyllableM121Yo),
            '𞡚' => Ok(MendeKikakui::SyllableM034Fi),
            '𞡛' => Ok(MendeKikakui::SyllableM035Fa),
            '𞡜' => Ok(MendeKikakui::SyllableM036Fu),
            '𞡝' => Ok(MendeKikakui::SyllableM078Fee),
            '𞡞' => Ok(MendeKikakui::SyllableM075Fe),
            '𞡟' => Ok(MendeKikakui::SyllableM133Foo),
            '𞡠' => Ok(MendeKikakui::SyllableM088Fo),
            '𞡡' => Ok(MendeKikakui::SyllableM197Fua),
            '𞡢' => Ok(MendeKikakui::SyllableM101Fan),
            '𞡣' => Ok(MendeKikakui::SyllableM037Nin),
            '𞡤' => Ok(MendeKikakui::SyllableM038Nan),
            '𞡥' => Ok(MendeKikakui::SyllableM039Nun),
            '𞡦' => Ok(MendeKikakui::SyllableM117Nen),
            '𞡧' => Ok(MendeKikakui::SyllableM169Non),
            '𞡨' => Ok(MendeKikakui::SyllableM176Hi),
            '𞡩' => Ok(MendeKikakui::SyllableM041Ha),
            '𞡪' => Ok(MendeKikakui::SyllableM186Hu),
            '𞡫' => Ok(MendeKikakui::SyllableM040Hee),
            '𞡬' => Ok(MendeKikakui::SyllableM096He),
            '𞡭' => Ok(MendeKikakui::SyllableM042Hoo),
            '𞡮' => Ok(MendeKikakui::SyllableM140Ho),
            '𞡯' => Ok(MendeKikakui::SyllableM083Heei),
            '𞡰' => Ok(MendeKikakui::SyllableM128Hoou),
            '𞡱' => Ok(MendeKikakui::SyllableM053Hin),
            '𞡲' => Ok(MendeKikakui::SyllableM130Han),
            '𞡳' => Ok(MendeKikakui::SyllableM087Hun),
            '𞡴' => Ok(MendeKikakui::SyllableM052Hen),
            '𞡵' => Ok(MendeKikakui::SyllableM193Hon),
            '𞡶' => Ok(MendeKikakui::SyllableM046Huan),
            '𞡷' => Ok(MendeKikakui::SyllableM090Nggi),
            '𞡸' => Ok(MendeKikakui::SyllableM043Ngga),
            '𞡹' => Ok(MendeKikakui::SyllableM082Nggu),
            '𞡺' => Ok(MendeKikakui::SyllableM115Nggee),
            '𞡻' => Ok(MendeKikakui::SyllableM146Ngge),
            '𞡼' => Ok(MendeKikakui::SyllableM156Nggoo),
            '𞡽' => Ok(MendeKikakui::SyllableM120Nggo),
            '𞡾' => Ok(MendeKikakui::SyllableM159Nggaa),
            '𞡿' => Ok(MendeKikakui::SyllableM127Nggua),
            '𞢀' => Ok(MendeKikakui::SyllableM086LongNgge),
            '𞢁' => Ok(MendeKikakui::SyllableM106LongNggoo),
            '𞢂' => Ok(MendeKikakui::SyllableM183LongNggo),
            '𞢃' => Ok(MendeKikakui::SyllableM155Gi),
            '𞢄' => Ok(MendeKikakui::SyllableM111Ga),
            '𞢅' => Ok(MendeKikakui::SyllableM168Gu),
            '𞢆' => Ok(MendeKikakui::SyllableM190Gee),
            '𞢇' => Ok(MendeKikakui::SyllableM166Guei),
            '𞢈' => Ok(MendeKikakui::SyllableM167Guan),
            '𞢉' => Ok(MendeKikakui::SyllableM184Ngen),
            '𞢊' => Ok(MendeKikakui::SyllableM057Ngon),
            '𞢋' => Ok(MendeKikakui::SyllableM177Nguan),
            '𞢌' => Ok(MendeKikakui::SyllableM068Pi),
            '𞢍' => Ok(MendeKikakui::SyllableM099Pa),
            '𞢎' => Ok(MendeKikakui::SyllableM050Pu),
            '𞢏' => Ok(MendeKikakui::SyllableM081Pee),
            '𞢐' => Ok(MendeKikakui::SyllableM051Pe),
            '𞢑' => Ok(MendeKikakui::SyllableM102Poo),
            '𞢒' => Ok(MendeKikakui::SyllableM066Po),
            '𞢓' => Ok(MendeKikakui::SyllableM145Mbi),
            '𞢔' => Ok(MendeKikakui::SyllableM062Mba),
            '𞢕' => Ok(MendeKikakui::SyllableM122Mbu),
            '𞢖' => Ok(MendeKikakui::SyllableM047Mbee),
            '𞢗' => Ok(MendeKikakui::SyllableM188Mbee),
            '𞢘' => Ok(MendeKikakui::SyllableM072Mbe),
            '𞢙' => Ok(MendeKikakui::SyllableM172Mboo),
            '𞢚' => Ok(MendeKikakui::SyllableM174Mbo),
            '𞢛' => Ok(MendeKikakui::SyllableM187Mbuu),
            '𞢜' => Ok(MendeKikakui::SyllableM161LongMbe),
            '𞢝' => Ok(MendeKikakui::SyllableM105LongMboo),
            '𞢞' => Ok(MendeKikakui::SyllableM142LongMbo),
            '𞢟' => Ok(MendeKikakui::SyllableM132Kpi),
            '𞢠' => Ok(MendeKikakui::SyllableM092Kpa),
            '𞢡' => Ok(MendeKikakui::SyllableM074Kpu),
            '𞢢' => Ok(MendeKikakui::SyllableM044Kpee),
            '𞢣' => Ok(MendeKikakui::SyllableM108Kpe),
            '𞢤' => Ok(MendeKikakui::SyllableM112Kpoo),
            '𞢥' => Ok(MendeKikakui::SyllableM158Kpo),
            '𞢦' => Ok(MendeKikakui::SyllableM124Gbi),
            '𞢧' => Ok(MendeKikakui::SyllableM056Gba),
            '𞢨' => Ok(MendeKikakui::SyllableM148Gbu),
            '𞢩' => Ok(MendeKikakui::SyllableM093Gbee),
            '𞢪' => Ok(MendeKikakui::SyllableM107Gbe),
            '𞢫' => Ok(MendeKikakui::SyllableM071Gboo),
            '𞢬' => Ok(MendeKikakui::SyllableM070Gbo),
            '𞢭' => Ok(MendeKikakui::SyllableM171Ra),
            '𞢮' => Ok(MendeKikakui::SyllableM123Ndi),
            '𞢯' => Ok(MendeKikakui::SyllableM129Nda),
            '𞢰' => Ok(MendeKikakui::SyllableM125Ndu),
            '𞢱' => Ok(MendeKikakui::SyllableM191Ndee),
            '𞢲' => Ok(MendeKikakui::SyllableM119Nde),
            '𞢳' => Ok(MendeKikakui::SyllableM067Ndoo),
            '𞢴' => Ok(MendeKikakui::SyllableM064Ndo),
            '𞢵' => Ok(MendeKikakui::SyllableM152Nja),
            '𞢶' => Ok(MendeKikakui::SyllableM192Nju),
            '𞢷' => Ok(MendeKikakui::SyllableM149Njee),
            '𞢸' => Ok(MendeKikakui::SyllableM134Njoo),
            '𞢹' => Ok(MendeKikakui::SyllableM182Vi),
            '𞢺' => Ok(MendeKikakui::SyllableM185Va),
            '𞢻' => Ok(MendeKikakui::SyllableM151Vu),
            '𞢼' => Ok(MendeKikakui::SyllableM173Vee),
            '𞢽' => Ok(MendeKikakui::SyllableM085Ve),
            '𞢾' => Ok(MendeKikakui::SyllableM144Voo),
            '𞢿' => Ok(MendeKikakui::SyllableM077Vo),
            '𞣀' => Ok(MendeKikakui::SyllableM164Nyin),
            '𞣁' => Ok(MendeKikakui::SyllableM058Nyan),
            '𞣂' => Ok(MendeKikakui::SyllableM170Nyun),
            '𞣃' => Ok(MendeKikakui::SyllableM098Nyen),
            '𞣄' => Ok(MendeKikakui::SyllableM060Nyon),
            '𞣇' => Ok(MendeKikakui::DigitOne),
            '𞣈' => Ok(MendeKikakui::DigitTwo),
            '𞣉' => Ok(MendeKikakui::DigitThree),
            '𞣊' => Ok(MendeKikakui::DigitFour),
            '𞣋' => Ok(MendeKikakui::DigitFive),
            '𞣌' => Ok(MendeKikakui::DigitSix),
            '𞣍' => Ok(MendeKikakui::DigitSeven),
            '𞣎' => Ok(MendeKikakui::DigitEight),
            '𞣏' => Ok(MendeKikakui::DigitNine),
            '𞣐' => Ok(MendeKikakui::CombiningNumberTeens),
            '𞣑' => Ok(MendeKikakui::CombiningNumberTens),
            '𞣒' => Ok(MendeKikakui::CombiningNumberHundreds),
            '𞣓' => Ok(MendeKikakui::CombiningNumberThousands),
            '𞣔' => Ok(MendeKikakui::CombiningNumberTenThousands),
            '𞣕' => Ok(MendeKikakui::CombiningNumberHundredThousands),
            '𞣖' => Ok(MendeKikakui::CombiningNumberMillions),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MendeKikakui {
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

impl std::convert::TryFrom<u32> for MendeKikakui {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MendeKikakui {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MendeKikakui {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MendeKikakui::SyllableM001Ki
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MendeKikakui{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
