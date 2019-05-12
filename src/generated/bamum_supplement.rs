
/// An enum to represent all characters in the BamumSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BamumSupplement {
    /// \u{16800}: '𖠀'
    BamumLetterPhaseDashANgkueMfon,
    /// \u{16801}: '𖠁'
    BamumLetterPhaseDashAGbieeFon,
    /// \u{16802}: '𖠂'
    BamumLetterPhaseDashAPonMfonPipaemgbiee,
    /// \u{16803}: '𖠃'
    BamumLetterPhaseDashAPonMfonPipaemba,
    /// \u{16804}: '𖠄'
    BamumLetterPhaseDashANaaMfon,
    /// \u{16805}: '𖠅'
    BamumLetterPhaseDashAShuenshuet,
    /// \u{16806}: '𖠆'
    BamumLetterPhaseDashATitaMfon,
    /// \u{16807}: '𖠇'
    BamumLetterPhaseDashANzaMfon,
    /// \u{16808}: '𖠈'
    BamumLetterPhaseDashAShindaPaNji,
    /// \u{16809}: '𖠉'
    BamumLetterPhaseDashAPonPaNjiPipaemgbiee,
    /// \u{1680a}: '𖠊'
    BamumLetterPhaseDashAPonPaNjiPipaemba,
    /// \u{1680b}: '𖠋'
    BamumLetterPhaseDashAMaembgbiee,
    /// \u{1680c}: '𖠌'
    BamumLetterPhaseDashATuMaemba,
    /// \u{1680d}: '𖠍'
    BamumLetterPhaseDashANgangu,
    /// \u{1680e}: '𖠎'
    BamumLetterPhaseDashAMaemveux,
    /// \u{1680f}: '𖠏'
    BamumLetterPhaseDashAMansuae,
    /// \u{16810}: '𖠐'
    BamumLetterPhaseDashAMveuaengam,
    /// \u{16811}: '𖠑'
    BamumLetterPhaseDashASeunyam,
    /// \u{16812}: '𖠒'
    BamumLetterPhaseDashANtoqpen,
    /// \u{16813}: '𖠓'
    BamumLetterPhaseDashAKeukeutnda,
    /// \u{16814}: '𖠔'
    BamumLetterPhaseDashANkindi,
    /// \u{16815}: '𖠕'
    BamumLetterPhaseDashASuu,
    /// \u{16816}: '𖠖'
    BamumLetterPhaseDashANgkuenzeum,
    /// \u{16817}: '𖠗'
    BamumLetterPhaseDashALapaq,
    /// \u{16818}: '𖠘'
    BamumLetterPhaseDashALetKut,
    /// \u{16819}: '𖠙'
    BamumLetterPhaseDashANtapMfaa,
    /// \u{1681a}: '𖠚'
    BamumLetterPhaseDashAMaekeup,
    /// \u{1681b}: '𖠛'
    BamumLetterPhaseDashAPashae,
    /// \u{1681c}: '𖠜'
    BamumLetterPhaseDashAGheuaerae,
    /// \u{1681d}: '𖠝'
    BamumLetterPhaseDashAPamshae,
    /// \u{1681e}: '𖠞'
    BamumLetterPhaseDashAMonNggeuaet,
    /// \u{1681f}: '𖠟'
    BamumLetterPhaseDashANzunMeut,
    /// \u{16820}: '𖠠'
    BamumLetterPhaseDashAUYuqNae,
    /// \u{16821}: '𖠡'
    BamumLetterPhaseDashAGheuaegheuae,
    /// \u{16822}: '𖠢'
    BamumLetterPhaseDashANtapNtaa,
    /// \u{16823}: '𖠣'
    BamumLetterPhaseDashASisa,
    /// \u{16824}: '𖠤'
    BamumLetterPhaseDashAMgbasa,
    /// \u{16825}: '𖠥'
    BamumLetterPhaseDashAMeunjomndeuq,
    /// \u{16826}: '𖠦'
    BamumLetterPhaseDashAMoompuq,
    /// \u{16827}: '𖠧'
    BamumLetterPhaseDashAKafa,
    /// \u{16828}: '𖠨'
    BamumLetterPhaseDashAPaLeeraewa,
    /// \u{16829}: '𖠩'
    BamumLetterPhaseDashANdaLeeraewa,
    /// \u{1682a}: '𖠪'
    BamumLetterPhaseDashAPet,
    /// \u{1682b}: '𖠫'
    BamumLetterPhaseDashAMaemkpen,
    /// \u{1682c}: '𖠬'
    BamumLetterPhaseDashANika,
    /// \u{1682d}: '𖠭'
    BamumLetterPhaseDashAPup,
    /// \u{1682e}: '𖠮'
    BamumLetterPhaseDashATuaep,
    /// \u{1682f}: '𖠯'
    BamumLetterPhaseDashALuaep,
    /// \u{16830}: '𖠰'
    BamumLetterPhaseDashASonjam,
    /// \u{16831}: '𖠱'
    BamumLetterPhaseDashATeuteuwen,
    /// \u{16832}: '𖠲'
    BamumLetterPhaseDashAMaenyi,
    /// \u{16833}: '𖠳'
    BamumLetterPhaseDashAKet,
    /// \u{16834}: '𖠴'
    BamumLetterPhaseDashANdaanggeuaet,
    /// \u{16835}: '𖠵'
    BamumLetterPhaseDashAKuoq,
    /// \u{16836}: '𖠶'
    BamumLetterPhaseDashAMoomeut,
    /// \u{16837}: '𖠷'
    BamumLetterPhaseDashAShum,
    /// \u{16838}: '𖠸'
    BamumLetterPhaseDashALommae,
    /// \u{16839}: '𖠹'
    BamumLetterPhaseDashAFiri,
    /// \u{1683a}: '𖠺'
    BamumLetterPhaseDashARom,
    /// \u{1683b}: '𖠻'
    BamumLetterPhaseDashAKpoq,
    /// \u{1683c}: '𖠼'
    BamumLetterPhaseDashASoq,
    /// \u{1683d}: '𖠽'
    BamumLetterPhaseDashAMapPieet,
    /// \u{1683e}: '𖠾'
    BamumLetterPhaseDashAShirae,
    /// \u{1683f}: '𖠿'
    BamumLetterPhaseDashANtap,
    /// \u{16840}: '𖡀'
    BamumLetterPhaseDashAShoqNshutYum,
    /// \u{16841}: '𖡁'
    BamumLetterPhaseDashANyitMongkeuaeq,
    /// \u{16842}: '𖡂'
    BamumLetterPhaseDashAPaarae,
    /// \u{16843}: '𖡃'
    BamumLetterPhaseDashANkaarae,
    /// \u{16844}: '𖡄'
    BamumLetterPhaseDashAUnknown,
    /// \u{16845}: '𖡅'
    BamumLetterPhaseDashANggen,
    /// \u{16846}: '𖡆'
    BamumLetterPhaseDashAMaesi,
    /// \u{16847}: '𖡇'
    BamumLetterPhaseDashANjam,
    /// \u{16848}: '𖡈'
    BamumLetterPhaseDashAMbanyi,
    /// \u{16849}: '𖡉'
    BamumLetterPhaseDashANyet,
    /// \u{1684a}: '𖡊'
    BamumLetterPhaseDashATeuaen,
    /// \u{1684b}: '𖡋'
    BamumLetterPhaseDashASot,
    /// \u{1684c}: '𖡌'
    BamumLetterPhaseDashAPaam,
    /// \u{1684d}: '𖡍'
    BamumLetterPhaseDashANshiee,
    /// \u{1684e}: '𖡎'
    BamumLetterPhaseDashAMaem,
    /// \u{1684f}: '𖡏'
    BamumLetterPhaseDashANyi,
    /// \u{16850}: '𖡐'
    BamumLetterPhaseDashAKaq,
    /// \u{16851}: '𖡑'
    BamumLetterPhaseDashANsha,
    /// \u{16852}: '𖡒'
    BamumLetterPhaseDashAVee,
    /// \u{16853}: '𖡓'
    BamumLetterPhaseDashALu,
    /// \u{16854}: '𖡔'
    BamumLetterPhaseDashANen,
    /// \u{16855}: '𖡕'
    BamumLetterPhaseDashANaq,
    /// \u{16856}: '𖡖'
    BamumLetterPhaseDashAMbaq,
    /// \u{16857}: '𖡗'
    BamumLetterPhaseDashBNshuet,
    /// \u{16858}: '𖡘'
    BamumLetterPhaseDashBTuMaemgbiee,
    /// \u{16859}: '𖡙'
    BamumLetterPhaseDashBSiee,
    /// \u{1685a}: '𖡚'
    BamumLetterPhaseDashBSetTu,
    /// \u{1685b}: '𖡛'
    BamumLetterPhaseDashBLomNteum,
    /// \u{1685c}: '𖡜'
    BamumLetterPhaseDashBMbaMaelee,
    /// \u{1685d}: '𖡝'
    BamumLetterPhaseDashBKieem,
    /// \u{1685e}: '𖡞'
    BamumLetterPhaseDashBYeurae,
    /// \u{1685f}: '𖡟'
    BamumLetterPhaseDashBMbaarae,
    /// \u{16860}: '𖡠'
    BamumLetterPhaseDashBKam,
    /// \u{16861}: '𖡡'
    BamumLetterPhaseDashBPeeshi,
    /// \u{16862}: '𖡢'
    BamumLetterPhaseDashBYafuLeeraewa,
    /// \u{16863}: '𖡣'
    BamumLetterPhaseDashBLamNshutNyam,
    /// \u{16864}: '𖡤'
    BamumLetterPhaseDashBNtieeSheuoq,
    /// \u{16865}: '𖡥'
    BamumLetterPhaseDashBNduNjaa,
    /// \u{16866}: '𖡦'
    BamumLetterPhaseDashBGheugheuaem,
    /// \u{16867}: '𖡧'
    BamumLetterPhaseDashBPit,
    /// \u{16868}: '𖡨'
    BamumLetterPhaseDashBTuNsiee,
    /// \u{16869}: '𖡩'
    BamumLetterPhaseDashBShetNjaq,
    /// \u{1686a}: '𖡪'
    BamumLetterPhaseDashBSheuaeqtu,
    /// \u{1686b}: '𖡫'
    BamumLetterPhaseDashBMfonTeuaeq,
    /// \u{1686c}: '𖡬'
    BamumLetterPhaseDashBMbitMbaaket,
    /// \u{1686d}: '𖡭'
    BamumLetterPhaseDashBNyiNteum,
    /// \u{1686e}: '𖡮'
    BamumLetterPhaseDashBKeupuq,
    /// \u{1686f}: '𖡯'
    BamumLetterPhaseDashBGheughen,
    /// \u{16870}: '𖡰'
    BamumLetterPhaseDashBKeuyeux,
    /// \u{16871}: '𖡱'
    BamumLetterPhaseDashBLaanae,
    /// \u{16872}: '𖡲'
    BamumLetterPhaseDashBParum,
    /// \u{16873}: '𖡳'
    BamumLetterPhaseDashBVeum,
    /// \u{16874}: '𖡴'
    BamumLetterPhaseDashBNgkindiMvop,
    /// \u{16875}: '𖡵'
    BamumLetterPhaseDashBNggeuMbu,
    /// \u{16876}: '𖡶'
    BamumLetterPhaseDashBWuaet,
    /// \u{16877}: '𖡷'
    BamumLetterPhaseDashBSakeuae,
    /// \u{16878}: '𖡸'
    BamumLetterPhaseDashBTaam,
    /// \u{16879}: '𖡹'
    BamumLetterPhaseDashBMeuq,
    /// \u{1687a}: '𖡺'
    BamumLetterPhaseDashBNgguoq,
    /// \u{1687b}: '𖡻'
    BamumLetterPhaseDashBNgguoqLarge,
    /// \u{1687c}: '𖡼'
    BamumLetterPhaseDashBMfiyaq,
    /// \u{1687d}: '𖡽'
    BamumLetterPhaseDashBSue,
    /// \u{1687e}: '𖡾'
    BamumLetterPhaseDashBMbeuri,
    /// \u{1687f}: '𖡿'
    BamumLetterPhaseDashBMontieen,
    /// \u{16880}: '𖢀'
    BamumLetterPhaseDashBNyaemae,
    /// \u{16881}: '𖢁'
    BamumLetterPhaseDashBPungaam,
    /// \u{16882}: '𖢂'
    BamumLetterPhaseDashBMeutNggeet,
    /// \u{16883}: '𖢃'
    BamumLetterPhaseDashBFeux,
    /// \u{16884}: '𖢄'
    BamumLetterPhaseDashBMbuoq,
    /// \u{16885}: '𖢅'
    BamumLetterPhaseDashBFee,
    /// \u{16886}: '𖢆'
    BamumLetterPhaseDashBKeuaem,
    /// \u{16887}: '𖢇'
    BamumLetterPhaseDashBMaNjeuaena,
    /// \u{16888}: '𖢈'
    BamumLetterPhaseDashBMaNjuqa,
    /// \u{16889}: '𖢉'
    BamumLetterPhaseDashBLet,
    /// \u{1688a}: '𖢊'
    BamumLetterPhaseDashBNggaam,
    /// \u{1688b}: '𖢋'
    BamumLetterPhaseDashBNsen,
    /// \u{1688c}: '𖢌'
    BamumLetterPhaseDashBMa,
    /// \u{1688d}: '𖢍'
    BamumLetterPhaseDashBKiq,
    /// \u{1688e}: '𖢎'
    BamumLetterPhaseDashBNgom,
    /// \u{1688f}: '𖢏'
    BamumLetterPhaseDashCNgkueMaemba,
    /// \u{16890}: '𖢐'
    BamumLetterPhaseDashCNza,
    /// \u{16891}: '𖢑'
    BamumLetterPhaseDashCYum,
    /// \u{16892}: '𖢒'
    BamumLetterPhaseDashCWangkuoq,
    /// \u{16893}: '𖢓'
    BamumLetterPhaseDashCNggen,
    /// \u{16894}: '𖢔'
    BamumLetterPhaseDashCNdeuaeree,
    /// \u{16895}: '𖢕'
    BamumLetterPhaseDashCNgkaq,
    /// \u{16896}: '𖢖'
    BamumLetterPhaseDashCGharae,
    /// \u{16897}: '𖢗'
    BamumLetterPhaseDashCMbeekeet,
    /// \u{16898}: '𖢘'
    BamumLetterPhaseDashCGbayi,
    /// \u{16899}: '𖢙'
    BamumLetterPhaseDashCNyirMkparaqMeun,
    /// \u{1689a}: '𖢚'
    BamumLetterPhaseDashCNtuMbit,
    /// \u{1689b}: '𖢛'
    BamumLetterPhaseDashCMbeum,
    /// \u{1689c}: '𖢜'
    BamumLetterPhaseDashCPirieen,
    /// \u{1689d}: '𖢝'
    BamumLetterPhaseDashCNdombu,
    /// \u{1689e}: '𖢞'
    BamumLetterPhaseDashCMbaaCabbageDashTree,
    /// \u{1689f}: '𖢟'
    BamumLetterPhaseDashCKeusheuaep,
    /// \u{168a0}: '𖢠'
    BamumLetterPhaseDashCGhap,
    /// \u{168a1}: '𖢡'
    BamumLetterPhaseDashCKeukaq,
    /// \u{168a2}: '𖢢'
    BamumLetterPhaseDashCYuMuomae,
    /// \u{168a3}: '𖢣'
    BamumLetterPhaseDashCNzeum,
    /// \u{168a4}: '𖢤'
    BamumLetterPhaseDashCMbue,
    /// \u{168a5}: '𖢥'
    BamumLetterPhaseDashCNseuaen,
    /// \u{168a6}: '𖢦'
    BamumLetterPhaseDashCMbit,
    /// \u{168a7}: '𖢧'
    BamumLetterPhaseDashCYeuq,
    /// \u{168a8}: '𖢨'
    BamumLetterPhaseDashCKparaq,
    /// \u{168a9}: '𖢩'
    BamumLetterPhaseDashCKaa,
    /// \u{168aa}: '𖢪'
    BamumLetterPhaseDashCSeux,
    /// \u{168ab}: '𖢫'
    BamumLetterPhaseDashCNdida,
    /// \u{168ac}: '𖢬'
    BamumLetterPhaseDashCTaashae,
    /// \u{168ad}: '𖢭'
    BamumLetterPhaseDashCNjueq,
    /// \u{168ae}: '𖢮'
    BamumLetterPhaseDashCTitaYue,
    /// \u{168af}: '𖢯'
    BamumLetterPhaseDashCSuaet,
    /// \u{168b0}: '𖢰'
    BamumLetterPhaseDashCNgguaenNyam,
    /// \u{168b1}: '𖢱'
    BamumLetterPhaseDashCVeux,
    /// \u{168b2}: '𖢲'
    BamumLetterPhaseDashCNansanaq,
    /// \u{168b3}: '𖢳'
    BamumLetterPhaseDashCMaKeuaeri,
    /// \u{168b4}: '𖢴'
    BamumLetterPhaseDashCNtaa,
    /// \u{168b5}: '𖢵'
    BamumLetterPhaseDashCNgguon,
    /// \u{168b6}: '𖢶'
    BamumLetterPhaseDashCLap,
    /// \u{168b7}: '𖢷'
    BamumLetterPhaseDashCMbirieen,
    /// \u{168b8}: '𖢸'
    BamumLetterPhaseDashCMgbasaq,
    /// \u{168b9}: '𖢹'
    BamumLetterPhaseDashCNteungba,
    /// \u{168ba}: '𖢺'
    BamumLetterPhaseDashCTeuteux,
    /// \u{168bb}: '𖢻'
    BamumLetterPhaseDashCNggum,
    /// \u{168bc}: '𖢼'
    BamumLetterPhaseDashCFue,
    /// \u{168bd}: '𖢽'
    BamumLetterPhaseDashCNdeut,
    /// \u{168be}: '𖢾'
    BamumLetterPhaseDashCNsa,
    /// \u{168bf}: '𖢿'
    BamumLetterPhaseDashCNshaq,
    /// \u{168c0}: '𖣀'
    BamumLetterPhaseDashCBung,
    /// \u{168c1}: '𖣁'
    BamumLetterPhaseDashCVeuaepen,
    /// \u{168c2}: '𖣂'
    BamumLetterPhaseDashCMberae,
    /// \u{168c3}: '𖣃'
    BamumLetterPhaseDashCRu,
    /// \u{168c4}: '𖣄'
    BamumLetterPhaseDashCNjaem,
    /// \u{168c5}: '𖣅'
    BamumLetterPhaseDashCLam,
    /// \u{168c6}: '𖣆'
    BamumLetterPhaseDashCTituaep,
    /// \u{168c7}: '𖣇'
    BamumLetterPhaseDashCNsuotNgom,
    /// \u{168c8}: '𖣈'
    BamumLetterPhaseDashCNjeeee,
    /// \u{168c9}: '𖣉'
    BamumLetterPhaseDashCKet,
    /// \u{168ca}: '𖣊'
    BamumLetterPhaseDashCNggu,
    /// \u{168cb}: '𖣋'
    BamumLetterPhaseDashCMaesi,
    /// \u{168cc}: '𖣌'
    BamumLetterPhaseDashCMbuaem,
    /// \u{168cd}: '𖣍'
    BamumLetterPhaseDashCLu,
    /// \u{168ce}: '𖣎'
    BamumLetterPhaseDashCKut,
    /// \u{168cf}: '𖣏'
    BamumLetterPhaseDashCNjam,
    /// \u{168d0}: '𖣐'
    BamumLetterPhaseDashCNgom,
    /// \u{168d1}: '𖣑'
    BamumLetterPhaseDashCWup,
    /// \u{168d2}: '𖣒'
    BamumLetterPhaseDashCNggueet,
    /// \u{168d3}: '𖣓'
    BamumLetterPhaseDashCNsom,
    /// \u{168d4}: '𖣔'
    BamumLetterPhaseDashCNten,
    /// \u{168d5}: '𖣕'
    BamumLetterPhaseDashCKuopNkaarae,
    /// \u{168d6}: '𖣖'
    BamumLetterPhaseDashCNsun,
    /// \u{168d7}: '𖣗'
    BamumLetterPhaseDashCNdam,
    /// \u{168d8}: '𖣘'
    BamumLetterPhaseDashCMaNsiee,
    /// \u{168d9}: '𖣙'
    BamumLetterPhaseDashCYaa,
    /// \u{168da}: '𖣚'
    BamumLetterPhaseDashCNdap,
    /// \u{168db}: '𖣛'
    BamumLetterPhaseDashCShueq,
    /// \u{168dc}: '𖣜'
    BamumLetterPhaseDashCSetfon,
    /// \u{168dd}: '𖣝'
    BamumLetterPhaseDashCMbi,
    /// \u{168de}: '𖣞'
    BamumLetterPhaseDashCMaemba,
    /// \u{168df}: '𖣟'
    BamumLetterPhaseDashCMbanyi,
    /// \u{168e0}: '𖣠'
    BamumLetterPhaseDashCKeuseux,
    /// \u{168e1}: '𖣡'
    BamumLetterPhaseDashCMbeux,
    /// \u{168e2}: '𖣢'
    BamumLetterPhaseDashCKeum,
    /// \u{168e3}: '𖣣'
    BamumLetterPhaseDashCMbaaPicket,
    /// \u{168e4}: '𖣤'
    BamumLetterPhaseDashCYuwoq,
    /// \u{168e5}: '𖣥'
    BamumLetterPhaseDashCNjeux,
    /// \u{168e6}: '𖣦'
    BamumLetterPhaseDashCMiee,
    /// \u{168e7}: '𖣧'
    BamumLetterPhaseDashCMuae,
    /// \u{168e8}: '𖣨'
    BamumLetterPhaseDashCShiq,
    /// \u{168e9}: '𖣩'
    BamumLetterPhaseDashCKenLaw,
    /// \u{168ea}: '𖣪'
    BamumLetterPhaseDashCKenFatigue,
    /// \u{168eb}: '𖣫'
    BamumLetterPhaseDashCNgaq,
    /// \u{168ec}: '𖣬'
    BamumLetterPhaseDashCNaq,
    /// \u{168ed}: '𖣭'
    BamumLetterPhaseDashCLiq,
    /// \u{168ee}: '𖣮'
    BamumLetterPhaseDashCPin,
    /// \u{168ef}: '𖣯'
    BamumLetterPhaseDashCPen,
    /// \u{168f0}: '𖣰'
    BamumLetterPhaseDashCTet,
    /// \u{168f1}: '𖣱'
    BamumLetterPhaseDashDMbuo,
    /// \u{168f2}: '𖣲'
    BamumLetterPhaseDashDWap,
    /// \u{168f3}: '𖣳'
    BamumLetterPhaseDashDNji,
    /// \u{168f4}: '𖣴'
    BamumLetterPhaseDashDMfon,
    /// \u{168f5}: '𖣵'
    BamumLetterPhaseDashDNjiee,
    /// \u{168f6}: '𖣶'
    BamumLetterPhaseDashDLiee,
    /// \u{168f7}: '𖣷'
    BamumLetterPhaseDashDNjeut,
    /// \u{168f8}: '𖣸'
    BamumLetterPhaseDashDNshee,
    /// \u{168f9}: '𖣹'
    BamumLetterPhaseDashDNggaamae,
    /// \u{168fa}: '𖣺'
    BamumLetterPhaseDashDNyam,
    /// \u{168fb}: '𖣻'
    BamumLetterPhaseDashDWuaen,
    /// \u{168fc}: '𖣼'
    BamumLetterPhaseDashDNgkun,
    /// \u{168fd}: '𖣽'
    BamumLetterPhaseDashDShee,
    /// \u{168fe}: '𖣾'
    BamumLetterPhaseDashDNgkap,
    /// \u{168ff}: '𖣿'
    BamumLetterPhaseDashDKeuaetmeun,
    /// \u{16900}: '𖤀'
    BamumLetterPhaseDashDTeut,
    /// \u{16901}: '𖤁'
    BamumLetterPhaseDashDSheuae,
    /// \u{16902}: '𖤂'
    BamumLetterPhaseDashDNjap,
    /// \u{16903}: '𖤃'
    BamumLetterPhaseDashDSue,
    /// \u{16904}: '𖤄'
    BamumLetterPhaseDashDKet,
    /// \u{16905}: '𖤅'
    BamumLetterPhaseDashDYaemmae,
    /// \u{16906}: '𖤆'
    BamumLetterPhaseDashDKuom,
    /// \u{16907}: '𖤇'
    BamumLetterPhaseDashDSap,
    /// \u{16908}: '𖤈'
    BamumLetterPhaseDashDMfeut,
    /// \u{16909}: '𖤉'
    BamumLetterPhaseDashDNdeux,
    /// \u{1690a}: '𖤊'
    BamumLetterPhaseDashDMaleeri,
    /// \u{1690b}: '𖤋'
    BamumLetterPhaseDashDMeut,
    /// \u{1690c}: '𖤌'
    BamumLetterPhaseDashDSeuaeq,
    /// \u{1690d}: '𖤍'
    BamumLetterPhaseDashDYen,
    /// \u{1690e}: '𖤎'
    BamumLetterPhaseDashDNjeuaem,
    /// \u{1690f}: '𖤏'
    BamumLetterPhaseDashDKeuotMbuae,
    /// \u{16910}: '𖤐'
    BamumLetterPhaseDashDNgkeuri,
    /// \u{16911}: '𖤑'
    BamumLetterPhaseDashDTu,
    /// \u{16912}: '𖤒'
    BamumLetterPhaseDashDGhaa,
    /// \u{16913}: '𖤓'
    BamumLetterPhaseDashDNgkyee,
    /// \u{16914}: '𖤔'
    BamumLetterPhaseDashDFeufeuaet,
    /// \u{16915}: '𖤕'
    BamumLetterPhaseDashDNdee,
    /// \u{16916}: '𖤖'
    BamumLetterPhaseDashDMgbofum,
    /// \u{16917}: '𖤗'
    BamumLetterPhaseDashDLeuaep,
    /// \u{16918}: '𖤘'
    BamumLetterPhaseDashDNdon,
    /// \u{16919}: '𖤙'
    BamumLetterPhaseDashDMoni,
    /// \u{1691a}: '𖤚'
    BamumLetterPhaseDashDMgbeun,
    /// \u{1691b}: '𖤛'
    BamumLetterPhaseDashDPuut,
    /// \u{1691c}: '𖤜'
    BamumLetterPhaseDashDMgbiee,
    /// \u{1691d}: '𖤝'
    BamumLetterPhaseDashDMfo,
    /// \u{1691e}: '𖤞'
    BamumLetterPhaseDashDLum,
    /// \u{1691f}: '𖤟'
    BamumLetterPhaseDashDNsieep,
    /// \u{16920}: '𖤠'
    BamumLetterPhaseDashDMbaa,
    /// \u{16921}: '𖤡'
    BamumLetterPhaseDashDKwaet,
    /// \u{16922}: '𖤢'
    BamumLetterPhaseDashDNyet,
    /// \u{16923}: '𖤣'
    BamumLetterPhaseDashDTeuaen,
    /// \u{16924}: '𖤤'
    BamumLetterPhaseDashDSot,
    /// \u{16925}: '𖤥'
    BamumLetterPhaseDashDYuwoq,
    /// \u{16926}: '𖤦'
    BamumLetterPhaseDashDKeum,
    /// \u{16927}: '𖤧'
    BamumLetterPhaseDashDRaem,
    /// \u{16928}: '𖤨'
    BamumLetterPhaseDashDTeeee,
    /// \u{16929}: '𖤩'
    BamumLetterPhaseDashDNgkeuaeq,
    /// \u{1692a}: '𖤪'
    BamumLetterPhaseDashDMfeuae,
    /// \u{1692b}: '𖤫'
    BamumLetterPhaseDashDNsieet,
    /// \u{1692c}: '𖤬'
    BamumLetterPhaseDashDKeup,
    /// \u{1692d}: '𖤭'
    BamumLetterPhaseDashDPip,
    /// \u{1692e}: '𖤮'
    BamumLetterPhaseDashDPeutae,
    /// \u{1692f}: '𖤯'
    BamumLetterPhaseDashDNyue,
    /// \u{16930}: '𖤰'
    BamumLetterPhaseDashDLet,
    /// \u{16931}: '𖤱'
    BamumLetterPhaseDashDNggaam,
    /// \u{16932}: '𖤲'
    BamumLetterPhaseDashDMfiee,
    /// \u{16933}: '𖤳'
    BamumLetterPhaseDashDNggwaen,
    /// \u{16934}: '𖤴'
    BamumLetterPhaseDashDYuom,
    /// \u{16935}: '𖤵'
    BamumLetterPhaseDashDPap,
    /// \u{16936}: '𖤶'
    BamumLetterPhaseDashDYuop,
    /// \u{16937}: '𖤷'
    BamumLetterPhaseDashDNdam,
    /// \u{16938}: '𖤸'
    BamumLetterPhaseDashDNteum,
    /// \u{16939}: '𖤹'
    BamumLetterPhaseDashDSuae,
    /// \u{1693a}: '𖤺'
    BamumLetterPhaseDashDKun,
    /// \u{1693b}: '𖤻'
    BamumLetterPhaseDashDNggeux,
    /// \u{1693c}: '𖤼'
    BamumLetterPhaseDashDNgkiee,
    /// \u{1693d}: '𖤽'
    BamumLetterPhaseDashDTuot,
    /// \u{1693e}: '𖤾'
    BamumLetterPhaseDashDMeun,
    /// \u{1693f}: '𖤿'
    BamumLetterPhaseDashDKuq,
    /// \u{16940}: '𖥀'
    BamumLetterPhaseDashDNsum,
    /// \u{16941}: '𖥁'
    BamumLetterPhaseDashDTeun,
    /// \u{16942}: '𖥂'
    BamumLetterPhaseDashDMaenjet,
    /// \u{16943}: '𖥃'
    BamumLetterPhaseDashDNggap,
    /// \u{16944}: '𖥄'
    BamumLetterPhaseDashDLeum,
    /// \u{16945}: '𖥅'
    BamumLetterPhaseDashDNgguom,
    /// \u{16946}: '𖥆'
    BamumLetterPhaseDashDNshut,
    /// \u{16947}: '𖥇'
    BamumLetterPhaseDashDNjueq,
    /// \u{16948}: '𖥈'
    BamumLetterPhaseDashDGheuae,
    /// \u{16949}: '𖥉'
    BamumLetterPhaseDashDKu,
    /// \u{1694a}: '𖥊'
    BamumLetterPhaseDashDRenOld,
    /// \u{1694b}: '𖥋'
    BamumLetterPhaseDashDTae,
    /// \u{1694c}: '𖥌'
    BamumLetterPhaseDashDToq,
    /// \u{1694d}: '𖥍'
    BamumLetterPhaseDashDNyi,
    /// \u{1694e}: '𖥎'
    BamumLetterPhaseDashDRii,
    /// \u{1694f}: '𖥏'
    BamumLetterPhaseDashDLeeee,
    /// \u{16950}: '𖥐'
    BamumLetterPhaseDashDMeeee,
    /// \u{16951}: '𖥑'
    BamumLetterPhaseDashDM,
    /// \u{16952}: '𖥒'
    BamumLetterPhaseDashDSuu,
    /// \u{16953}: '𖥓'
    BamumLetterPhaseDashDMu,
    /// \u{16954}: '𖥔'
    BamumLetterPhaseDashDShii,
    /// \u{16955}: '𖥕'
    BamumLetterPhaseDashDSheux,
    /// \u{16956}: '𖥖'
    BamumLetterPhaseDashDKyee,
    /// \u{16957}: '𖥗'
    BamumLetterPhaseDashDNu,
    /// \u{16958}: '𖥘'
    BamumLetterPhaseDashDShu,
    /// \u{16959}: '𖥙'
    BamumLetterPhaseDashDNtee,
    /// \u{1695a}: '𖥚'
    BamumLetterPhaseDashDPee,
    /// \u{1695b}: '𖥛'
    BamumLetterPhaseDashDNi,
    /// \u{1695c}: '𖥜'
    BamumLetterPhaseDashDShoq,
    /// \u{1695d}: '𖥝'
    BamumLetterPhaseDashDPuq,
    /// \u{1695e}: '𖥞'
    BamumLetterPhaseDashDMvop,
    /// \u{1695f}: '𖥟'
    BamumLetterPhaseDashDLoq,
    /// \u{16960}: '𖥠'
    BamumLetterPhaseDashDRenMuch,
    /// \u{16961}: '𖥡'
    BamumLetterPhaseDashDTi,
    /// \u{16962}: '𖥢'
    BamumLetterPhaseDashDNtuu,
    /// \u{16963}: '𖥣'
    BamumLetterPhaseDashDMbaaSeven,
    /// \u{16964}: '𖥤'
    BamumLetterPhaseDashDSaq,
    /// \u{16965}: '𖥥'
    BamumLetterPhaseDashDFaa,
    /// \u{16966}: '𖥦'
    BamumLetterPhaseDashENdap,
    /// \u{16967}: '𖥧'
    BamumLetterPhaseDashEToon,
    /// \u{16968}: '𖥨'
    BamumLetterPhaseDashEMbeum,
    /// \u{16969}: '𖥩'
    BamumLetterPhaseDashELap,
    /// \u{1696a}: '𖥪'
    BamumLetterPhaseDashEVom,
    /// \u{1696b}: '𖥫'
    BamumLetterPhaseDashELoon,
    /// \u{1696c}: '𖥬'
    BamumLetterPhaseDashEPaa,
    /// \u{1696d}: '𖥭'
    BamumLetterPhaseDashESom,
    /// \u{1696e}: '𖥮'
    BamumLetterPhaseDashERaq,
    /// \u{1696f}: '𖥯'
    BamumLetterPhaseDashENshuop,
    /// \u{16970}: '𖥰'
    BamumLetterPhaseDashENdun,
    /// \u{16971}: '𖥱'
    BamumLetterPhaseDashEPuae,
    /// \u{16972}: '𖥲'
    BamumLetterPhaseDashETam,
    /// \u{16973}: '𖥳'
    BamumLetterPhaseDashENgka,
    /// \u{16974}: '𖥴'
    BamumLetterPhaseDashEKpeux,
    /// \u{16975}: '𖥵'
    BamumLetterPhaseDashEWuo,
    /// \u{16976}: '𖥶'
    BamumLetterPhaseDashESee,
    /// \u{16977}: '𖥷'
    BamumLetterPhaseDashENggeuaet,
    /// \u{16978}: '𖥸'
    BamumLetterPhaseDashEPaam,
    /// \u{16979}: '𖥹'
    BamumLetterPhaseDashEToo,
    /// \u{1697a}: '𖥺'
    BamumLetterPhaseDashEKuop,
    /// \u{1697b}: '𖥻'
    BamumLetterPhaseDashELom,
    /// \u{1697c}: '𖥼'
    BamumLetterPhaseDashENshiee,
    /// \u{1697d}: '𖥽'
    BamumLetterPhaseDashENgop,
    /// \u{1697e}: '𖥾'
    BamumLetterPhaseDashEMaem,
    /// \u{1697f}: '𖥿'
    BamumLetterPhaseDashENgkeux,
    /// \u{16980}: '𖦀'
    BamumLetterPhaseDashENgoq,
    /// \u{16981}: '𖦁'
    BamumLetterPhaseDashENshue,
    /// \u{16982}: '𖦂'
    BamumLetterPhaseDashERimgba,
    /// \u{16983}: '𖦃'
    BamumLetterPhaseDashENjeux,
    /// \u{16984}: '𖦄'
    BamumLetterPhaseDashEPeem,
    /// \u{16985}: '𖦅'
    BamumLetterPhaseDashESaa,
    /// \u{16986}: '𖦆'
    BamumLetterPhaseDashENggurae,
    /// \u{16987}: '𖦇'
    BamumLetterPhaseDashEMgba,
    /// \u{16988}: '𖦈'
    BamumLetterPhaseDashEGheux,
    /// \u{16989}: '𖦉'
    BamumLetterPhaseDashENgkeuaem,
    /// \u{1698a}: '𖦊'
    BamumLetterPhaseDashENjaemli,
    /// \u{1698b}: '𖦋'
    BamumLetterPhaseDashEMap,
    /// \u{1698c}: '𖦌'
    BamumLetterPhaseDashELoot,
    /// \u{1698d}: '𖦍'
    BamumLetterPhaseDashENggeeee,
    /// \u{1698e}: '𖦎'
    BamumLetterPhaseDashENdiq,
    /// \u{1698f}: '𖦏'
    BamumLetterPhaseDashETaenNteum,
    /// \u{16990}: '𖦐'
    BamumLetterPhaseDashESet,
    /// \u{16991}: '𖦑'
    BamumLetterPhaseDashEPum,
    /// \u{16992}: '𖦒'
    BamumLetterPhaseDashENdaaSoftness,
    /// \u{16993}: '𖦓'
    BamumLetterPhaseDashENgguaeshaeNyam,
    /// \u{16994}: '𖦔'
    BamumLetterPhaseDashEYiee,
    /// \u{16995}: '𖦕'
    BamumLetterPhaseDashEGheun,
    /// \u{16996}: '𖦖'
    BamumLetterPhaseDashETuae,
    /// \u{16997}: '𖦗'
    BamumLetterPhaseDashEYeuae,
    /// \u{16998}: '𖦘'
    BamumLetterPhaseDashEPo,
    /// \u{16999}: '𖦙'
    BamumLetterPhaseDashETumae,
    /// \u{1699a}: '𖦚'
    BamumLetterPhaseDashEKeuae,
    /// \u{1699b}: '𖦛'
    BamumLetterPhaseDashESuaen,
    /// \u{1699c}: '𖦜'
    BamumLetterPhaseDashETeuaeq,
    /// \u{1699d}: '𖦝'
    BamumLetterPhaseDashEVeuae,
    /// \u{1699e}: '𖦞'
    BamumLetterPhaseDashEWeux,
    /// \u{1699f}: '𖦟'
    BamumLetterPhaseDashELaam,
    /// \u{169a0}: '𖦠'
    BamumLetterPhaseDashEPu,
    /// \u{169a1}: '𖦡'
    BamumLetterPhaseDashETaaq,
    /// \u{169a2}: '𖦢'
    BamumLetterPhaseDashEGhaamae,
    /// \u{169a3}: '𖦣'
    BamumLetterPhaseDashENgeureut,
    /// \u{169a4}: '𖦤'
    BamumLetterPhaseDashESheuaeq,
    /// \u{169a5}: '𖦥'
    BamumLetterPhaseDashEMgben,
    /// \u{169a6}: '𖦦'
    BamumLetterPhaseDashEMbee,
    /// \u{169a7}: '𖦧'
    BamumLetterPhaseDashENzaq,
    /// \u{169a8}: '𖦨'
    BamumLetterPhaseDashENkom,
    /// \u{169a9}: '𖦩'
    BamumLetterPhaseDashEGbet,
    /// \u{169aa}: '𖦪'
    BamumLetterPhaseDashETum,
    /// \u{169ab}: '𖦫'
    BamumLetterPhaseDashEKuet,
    /// \u{169ac}: '𖦬'
    BamumLetterPhaseDashEYap,
    /// \u{169ad}: '𖦭'
    BamumLetterPhaseDashENyiCleaver,
    /// \u{169ae}: '𖦮'
    BamumLetterPhaseDashEYit,
    /// \u{169af}: '𖦯'
    BamumLetterPhaseDashEMfeuq,
    /// \u{169b0}: '𖦰'
    BamumLetterPhaseDashENdiaq,
    /// \u{169b1}: '𖦱'
    BamumLetterPhaseDashEPieeq,
    /// \u{169b2}: '𖦲'
    BamumLetterPhaseDashEYueq,
    /// \u{169b3}: '𖦳'
    BamumLetterPhaseDashELeuaem,
    /// \u{169b4}: '𖦴'
    BamumLetterPhaseDashEFue,
    /// \u{169b5}: '𖦵'
    BamumLetterPhaseDashEGbeux,
    /// \u{169b6}: '𖦶'
    BamumLetterPhaseDashENgkup,
    /// \u{169b7}: '𖦷'
    BamumLetterPhaseDashEKet,
    /// \u{169b8}: '𖦸'
    BamumLetterPhaseDashEMae,
    /// \u{169b9}: '𖦹'
    BamumLetterPhaseDashENgkaami,
    /// \u{169ba}: '𖦺'
    BamumLetterPhaseDashEGhet,
    /// \u{169bb}: '𖦻'
    BamumLetterPhaseDashEFa,
    /// \u{169bc}: '𖦼'
    BamumLetterPhaseDashENtum,
    /// \u{169bd}: '𖦽'
    BamumLetterPhaseDashEPeut,
    /// \u{169be}: '𖦾'
    BamumLetterPhaseDashEYeum,
    /// \u{169bf}: '𖦿'
    BamumLetterPhaseDashENggeuae,
    /// \u{169c0}: '𖧀'
    BamumLetterPhaseDashENyiBetween,
    /// \u{169c1}: '𖧁'
    BamumLetterPhaseDashENzuq,
    /// \u{169c2}: '𖧂'
    BamumLetterPhaseDashEPoon,
    /// \u{169c3}: '𖧃'
    BamumLetterPhaseDashEMiee,
    /// \u{169c4}: '𖧄'
    BamumLetterPhaseDashEFuet,
    /// \u{169c5}: '𖧅'
    BamumLetterPhaseDashENae,
    /// \u{169c6}: '𖧆'
    BamumLetterPhaseDashEMuae,
    /// \u{169c7}: '𖧇'
    BamumLetterPhaseDashEGheuae,
    /// \u{169c8}: '𖧈'
    BamumLetterPhaseDashEFuI,
    /// \u{169c9}: '𖧉'
    BamumLetterPhaseDashEMvi,
    /// \u{169ca}: '𖧊'
    BamumLetterPhaseDashEPuaq,
    /// \u{169cb}: '𖧋'
    BamumLetterPhaseDashENgkum,
    /// \u{169cc}: '𖧌'
    BamumLetterPhaseDashEKut,
    /// \u{169cd}: '𖧍'
    BamumLetterPhaseDashEPiet,
    /// \u{169ce}: '𖧎'
    BamumLetterPhaseDashENtap,
    /// \u{169cf}: '𖧏'
    BamumLetterPhaseDashEYeuaet,
    /// \u{169d0}: '𖧐'
    BamumLetterPhaseDashENggup,
    /// \u{169d1}: '𖧑'
    BamumLetterPhaseDashEPaPeople,
    /// \u{169d2}: '𖧒'
    BamumLetterPhaseDashEFuCall,
    /// \u{169d3}: '𖧓'
    BamumLetterPhaseDashEFom,
    /// \u{169d4}: '𖧔'
    BamumLetterPhaseDashENjee,
    /// \u{169d5}: '𖧕'
    BamumLetterPhaseDashEA,
    /// \u{169d6}: '𖧖'
    BamumLetterPhaseDashEToq,
    /// \u{169d7}: '𖧗'
    BamumLetterPhaseDashEO,
    /// \u{169d8}: '𖧘'
    BamumLetterPhaseDashEI,
    /// \u{169d9}: '𖧙'
    BamumLetterPhaseDashELaq,
    /// \u{169da}: '𖧚'
    BamumLetterPhaseDashEPaPlural,
    /// \u{169db}: '𖧛'
    BamumLetterPhaseDashETaa,
    /// \u{169dc}: '𖧜'
    BamumLetterPhaseDashETaq,
    /// \u{169dd}: '𖧝'
    BamumLetterPhaseDashENdaaMyHouse,
    /// \u{169de}: '𖧞'
    BamumLetterPhaseDashEShiq,
    /// \u{169df}: '𖧟'
    BamumLetterPhaseDashEYeux,
    /// \u{169e0}: '𖧠'
    BamumLetterPhaseDashENguae,
    /// \u{169e1}: '𖧡'
    BamumLetterPhaseDashEYuaen,
    /// \u{169e2}: '𖧢'
    BamumLetterPhaseDashEYoqSwimming,
    /// \u{169e3}: '𖧣'
    BamumLetterPhaseDashEYoqCover,
    /// \u{169e4}: '𖧤'
    BamumLetterPhaseDashEYuq,
    /// \u{169e5}: '𖧥'
    BamumLetterPhaseDashEYun,
    /// \u{169e6}: '𖧦'
    BamumLetterPhaseDashEKeux,
    /// \u{169e7}: '𖧧'
    BamumLetterPhaseDashEPeux,
    /// \u{169e8}: '𖧨'
    BamumLetterPhaseDashENjeeEpoch,
    /// \u{169e9}: '𖧩'
    BamumLetterPhaseDashEPue,
    /// \u{169ea}: '𖧪'
    BamumLetterPhaseDashEWue,
    /// \u{169eb}: '𖧫'
    BamumLetterPhaseDashEFee,
    /// \u{169ec}: '𖧬'
    BamumLetterPhaseDashEVee,
    /// \u{169ed}: '𖧭'
    BamumLetterPhaseDashELu,
    /// \u{169ee}: '𖧮'
    BamumLetterPhaseDashEMi,
    /// \u{169ef}: '𖧯'
    BamumLetterPhaseDashEReux,
    /// \u{169f0}: '𖧰'
    BamumLetterPhaseDashERae,
    /// \u{169f1}: '𖧱'
    BamumLetterPhaseDashENguaet,
    /// \u{169f2}: '𖧲'
    BamumLetterPhaseDashENga,
    /// \u{169f3}: '𖧳'
    BamumLetterPhaseDashESho,
    /// \u{169f4}: '𖧴'
    BamumLetterPhaseDashEShoq,
    /// \u{169f5}: '𖧵'
    BamumLetterPhaseDashEFuRemedy,
    /// \u{169f6}: '𖧶'
    BamumLetterPhaseDashENa,
    /// \u{169f7}: '𖧷'
    BamumLetterPhaseDashEPi,
    /// \u{169f8}: '𖧸'
    BamumLetterPhaseDashELoq,
    /// \u{169f9}: '𖧹'
    BamumLetterPhaseDashEKo,
    /// \u{169fa}: '𖧺'
    BamumLetterPhaseDashEMen,
    /// \u{169fb}: '𖧻'
    BamumLetterPhaseDashEMa,
    /// \u{169fc}: '𖧼'
    BamumLetterPhaseDashEMaq,
    /// \u{169fd}: '𖧽'
    BamumLetterPhaseDashETeu,
    /// \u{169fe}: '𖧾'
    BamumLetterPhaseDashEKi,
    /// \u{169ff}: '𖧿'
    BamumLetterPhaseDashEMon,
    /// \u{16a00}: '𖨀'
    BamumLetterPhaseDashETen,
    /// \u{16a01}: '𖨁'
    BamumLetterPhaseDashEFaq,
    /// \u{16a02}: '𖨂'
    BamumLetterPhaseDashEGhom,
    /// \u{16a03}: '𖨃'
    BamumLetterPhaseDashFKa,
    /// \u{16a04}: '𖨄'
    BamumLetterPhaseDashFU,
    /// \u{16a05}: '𖨅'
    BamumLetterPhaseDashFKu,
    /// \u{16a06}: '𖨆'
    BamumLetterPhaseDashFEe,
    /// \u{16a07}: '𖨇'
    BamumLetterPhaseDashFRee,
    /// \u{16a08}: '𖨈'
    BamumLetterPhaseDashFTae,
    /// \u{16a09}: '𖨉'
    BamumLetterPhaseDashFNyi,
    /// \u{16a0a}: '𖨊'
    BamumLetterPhaseDashFLa,
    /// \u{16a0b}: '𖨋'
    BamumLetterPhaseDashFRii,
    /// \u{16a0c}: '𖨌'
    BamumLetterPhaseDashFRiee,
    /// \u{16a0d}: '𖨍'
    BamumLetterPhaseDashFMeeee,
    /// \u{16a0e}: '𖨎'
    BamumLetterPhaseDashFTaa,
    /// \u{16a0f}: '𖨏'
    BamumLetterPhaseDashFNdaa,
    /// \u{16a10}: '𖨐'
    BamumLetterPhaseDashFNjaem,
    /// \u{16a11}: '𖨑'
    BamumLetterPhaseDashFM,
    /// \u{16a12}: '𖨒'
    BamumLetterPhaseDashFSuu,
    /// \u{16a13}: '𖨓'
    BamumLetterPhaseDashFShii,
    /// \u{16a14}: '𖨔'
    BamumLetterPhaseDashFSi,
    /// \u{16a15}: '𖨕'
    BamumLetterPhaseDashFSeux,
    /// \u{16a16}: '𖨖'
    BamumLetterPhaseDashFKyee,
    /// \u{16a17}: '𖨗'
    BamumLetterPhaseDashFKet,
    /// \u{16a18}: '𖨘'
    BamumLetterPhaseDashFNuae,
    /// \u{16a19}: '𖨙'
    BamumLetterPhaseDashFNu,
    /// \u{16a1a}: '𖨚'
    BamumLetterPhaseDashFNjuae,
    /// \u{16a1b}: '𖨛'
    BamumLetterPhaseDashFYoq,
    /// \u{16a1c}: '𖨜'
    BamumLetterPhaseDashFShu,
    /// \u{16a1d}: '𖨝'
    BamumLetterPhaseDashFYa,
    /// \u{16a1e}: '𖨞'
    BamumLetterPhaseDashFNsha,
    /// \u{16a1f}: '𖨟'
    BamumLetterPhaseDashFPeux,
    /// \u{16a20}: '𖨠'
    BamumLetterPhaseDashFNtee,
    /// \u{16a21}: '𖨡'
    BamumLetterPhaseDashFWue,
    /// \u{16a22}: '𖨢'
    BamumLetterPhaseDashFPee,
    /// \u{16a23}: '𖨣'
    BamumLetterPhaseDashFRu,
    /// \u{16a24}: '𖨤'
    BamumLetterPhaseDashFNi,
    /// \u{16a25}: '𖨥'
    BamumLetterPhaseDashFReux,
    /// \u{16a26}: '𖨦'
    BamumLetterPhaseDashFKen,
    /// \u{16a27}: '𖨧'
    BamumLetterPhaseDashFNgkwaen,
    /// \u{16a28}: '𖨨'
    BamumLetterPhaseDashFNgga,
    /// \u{16a29}: '𖨩'
    BamumLetterPhaseDashFSho,
    /// \u{16a2a}: '𖨪'
    BamumLetterPhaseDashFPuae,
    /// \u{16a2b}: '𖨫'
    BamumLetterPhaseDashFFom,
    /// \u{16a2c}: '𖨬'
    BamumLetterPhaseDashFWa,
    /// \u{16a2d}: '𖨭'
    BamumLetterPhaseDashFLi,
    /// \u{16a2e}: '𖨮'
    BamumLetterPhaseDashFLoq,
    /// \u{16a2f}: '𖨯'
    BamumLetterPhaseDashFKo,
    /// \u{16a30}: '𖨰'
    BamumLetterPhaseDashFMben,
    /// \u{16a31}: '𖨱'
    BamumLetterPhaseDashFRen,
    /// \u{16a32}: '𖨲'
    BamumLetterPhaseDashFMa,
    /// \u{16a33}: '𖨳'
    BamumLetterPhaseDashFMo,
    /// \u{16a34}: '𖨴'
    BamumLetterPhaseDashFMbaa,
    /// \u{16a35}: '𖨵'
    BamumLetterPhaseDashFTet,
    /// \u{16a36}: '𖨶'
    BamumLetterPhaseDashFKpa,
    /// \u{16a37}: '𖨷'
    BamumLetterPhaseDashFSamba,
    /// \u{16a38}: '𖨸'
    BamumLetterPhaseDashFVueq,
}

impl Into<char> for BamumSupplement {
    fn into(self) -> char {
        match self {
            BamumSupplement::BamumLetterPhaseDashANgkueMfon => '𖠀',
            BamumSupplement::BamumLetterPhaseDashAGbieeFon => '𖠁',
            BamumSupplement::BamumLetterPhaseDashAPonMfonPipaemgbiee => '𖠂',
            BamumSupplement::BamumLetterPhaseDashAPonMfonPipaemba => '𖠃',
            BamumSupplement::BamumLetterPhaseDashANaaMfon => '𖠄',
            BamumSupplement::BamumLetterPhaseDashAShuenshuet => '𖠅',
            BamumSupplement::BamumLetterPhaseDashATitaMfon => '𖠆',
            BamumSupplement::BamumLetterPhaseDashANzaMfon => '𖠇',
            BamumSupplement::BamumLetterPhaseDashAShindaPaNji => '𖠈',
            BamumSupplement::BamumLetterPhaseDashAPonPaNjiPipaemgbiee => '𖠉',
            BamumSupplement::BamumLetterPhaseDashAPonPaNjiPipaemba => '𖠊',
            BamumSupplement::BamumLetterPhaseDashAMaembgbiee => '𖠋',
            BamumSupplement::BamumLetterPhaseDashATuMaemba => '𖠌',
            BamumSupplement::BamumLetterPhaseDashANgangu => '𖠍',
            BamumSupplement::BamumLetterPhaseDashAMaemveux => '𖠎',
            BamumSupplement::BamumLetterPhaseDashAMansuae => '𖠏',
            BamumSupplement::BamumLetterPhaseDashAMveuaengam => '𖠐',
            BamumSupplement::BamumLetterPhaseDashASeunyam => '𖠑',
            BamumSupplement::BamumLetterPhaseDashANtoqpen => '𖠒',
            BamumSupplement::BamumLetterPhaseDashAKeukeutnda => '𖠓',
            BamumSupplement::BamumLetterPhaseDashANkindi => '𖠔',
            BamumSupplement::BamumLetterPhaseDashASuu => '𖠕',
            BamumSupplement::BamumLetterPhaseDashANgkuenzeum => '𖠖',
            BamumSupplement::BamumLetterPhaseDashALapaq => '𖠗',
            BamumSupplement::BamumLetterPhaseDashALetKut => '𖠘',
            BamumSupplement::BamumLetterPhaseDashANtapMfaa => '𖠙',
            BamumSupplement::BamumLetterPhaseDashAMaekeup => '𖠚',
            BamumSupplement::BamumLetterPhaseDashAPashae => '𖠛',
            BamumSupplement::BamumLetterPhaseDashAGheuaerae => '𖠜',
            BamumSupplement::BamumLetterPhaseDashAPamshae => '𖠝',
            BamumSupplement::BamumLetterPhaseDashAMonNggeuaet => '𖠞',
            BamumSupplement::BamumLetterPhaseDashANzunMeut => '𖠟',
            BamumSupplement::BamumLetterPhaseDashAUYuqNae => '𖠠',
            BamumSupplement::BamumLetterPhaseDashAGheuaegheuae => '𖠡',
            BamumSupplement::BamumLetterPhaseDashANtapNtaa => '𖠢',
            BamumSupplement::BamumLetterPhaseDashASisa => '𖠣',
            BamumSupplement::BamumLetterPhaseDashAMgbasa => '𖠤',
            BamumSupplement::BamumLetterPhaseDashAMeunjomndeuq => '𖠥',
            BamumSupplement::BamumLetterPhaseDashAMoompuq => '𖠦',
            BamumSupplement::BamumLetterPhaseDashAKafa => '𖠧',
            BamumSupplement::BamumLetterPhaseDashAPaLeeraewa => '𖠨',
            BamumSupplement::BamumLetterPhaseDashANdaLeeraewa => '𖠩',
            BamumSupplement::BamumLetterPhaseDashAPet => '𖠪',
            BamumSupplement::BamumLetterPhaseDashAMaemkpen => '𖠫',
            BamumSupplement::BamumLetterPhaseDashANika => '𖠬',
            BamumSupplement::BamumLetterPhaseDashAPup => '𖠭',
            BamumSupplement::BamumLetterPhaseDashATuaep => '𖠮',
            BamumSupplement::BamumLetterPhaseDashALuaep => '𖠯',
            BamumSupplement::BamumLetterPhaseDashASonjam => '𖠰',
            BamumSupplement::BamumLetterPhaseDashATeuteuwen => '𖠱',
            BamumSupplement::BamumLetterPhaseDashAMaenyi => '𖠲',
            BamumSupplement::BamumLetterPhaseDashAKet => '𖠳',
            BamumSupplement::BamumLetterPhaseDashANdaanggeuaet => '𖠴',
            BamumSupplement::BamumLetterPhaseDashAKuoq => '𖠵',
            BamumSupplement::BamumLetterPhaseDashAMoomeut => '𖠶',
            BamumSupplement::BamumLetterPhaseDashAShum => '𖠷',
            BamumSupplement::BamumLetterPhaseDashALommae => '𖠸',
            BamumSupplement::BamumLetterPhaseDashAFiri => '𖠹',
            BamumSupplement::BamumLetterPhaseDashARom => '𖠺',
            BamumSupplement::BamumLetterPhaseDashAKpoq => '𖠻',
            BamumSupplement::BamumLetterPhaseDashASoq => '𖠼',
            BamumSupplement::BamumLetterPhaseDashAMapPieet => '𖠽',
            BamumSupplement::BamumLetterPhaseDashAShirae => '𖠾',
            BamumSupplement::BamumLetterPhaseDashANtap => '𖠿',
            BamumSupplement::BamumLetterPhaseDashAShoqNshutYum => '𖡀',
            BamumSupplement::BamumLetterPhaseDashANyitMongkeuaeq => '𖡁',
            BamumSupplement::BamumLetterPhaseDashAPaarae => '𖡂',
            BamumSupplement::BamumLetterPhaseDashANkaarae => '𖡃',
            BamumSupplement::BamumLetterPhaseDashAUnknown => '𖡄',
            BamumSupplement::BamumLetterPhaseDashANggen => '𖡅',
            BamumSupplement::BamumLetterPhaseDashAMaesi => '𖡆',
            BamumSupplement::BamumLetterPhaseDashANjam => '𖡇',
            BamumSupplement::BamumLetterPhaseDashAMbanyi => '𖡈',
            BamumSupplement::BamumLetterPhaseDashANyet => '𖡉',
            BamumSupplement::BamumLetterPhaseDashATeuaen => '𖡊',
            BamumSupplement::BamumLetterPhaseDashASot => '𖡋',
            BamumSupplement::BamumLetterPhaseDashAPaam => '𖡌',
            BamumSupplement::BamumLetterPhaseDashANshiee => '𖡍',
            BamumSupplement::BamumLetterPhaseDashAMaem => '𖡎',
            BamumSupplement::BamumLetterPhaseDashANyi => '𖡏',
            BamumSupplement::BamumLetterPhaseDashAKaq => '𖡐',
            BamumSupplement::BamumLetterPhaseDashANsha => '𖡑',
            BamumSupplement::BamumLetterPhaseDashAVee => '𖡒',
            BamumSupplement::BamumLetterPhaseDashALu => '𖡓',
            BamumSupplement::BamumLetterPhaseDashANen => '𖡔',
            BamumSupplement::BamumLetterPhaseDashANaq => '𖡕',
            BamumSupplement::BamumLetterPhaseDashAMbaq => '𖡖',
            BamumSupplement::BamumLetterPhaseDashBNshuet => '𖡗',
            BamumSupplement::BamumLetterPhaseDashBTuMaemgbiee => '𖡘',
            BamumSupplement::BamumLetterPhaseDashBSiee => '𖡙',
            BamumSupplement::BamumLetterPhaseDashBSetTu => '𖡚',
            BamumSupplement::BamumLetterPhaseDashBLomNteum => '𖡛',
            BamumSupplement::BamumLetterPhaseDashBMbaMaelee => '𖡜',
            BamumSupplement::BamumLetterPhaseDashBKieem => '𖡝',
            BamumSupplement::BamumLetterPhaseDashBYeurae => '𖡞',
            BamumSupplement::BamumLetterPhaseDashBMbaarae => '𖡟',
            BamumSupplement::BamumLetterPhaseDashBKam => '𖡠',
            BamumSupplement::BamumLetterPhaseDashBPeeshi => '𖡡',
            BamumSupplement::BamumLetterPhaseDashBYafuLeeraewa => '𖡢',
            BamumSupplement::BamumLetterPhaseDashBLamNshutNyam => '𖡣',
            BamumSupplement::BamumLetterPhaseDashBNtieeSheuoq => '𖡤',
            BamumSupplement::BamumLetterPhaseDashBNduNjaa => '𖡥',
            BamumSupplement::BamumLetterPhaseDashBGheugheuaem => '𖡦',
            BamumSupplement::BamumLetterPhaseDashBPit => '𖡧',
            BamumSupplement::BamumLetterPhaseDashBTuNsiee => '𖡨',
            BamumSupplement::BamumLetterPhaseDashBShetNjaq => '𖡩',
            BamumSupplement::BamumLetterPhaseDashBSheuaeqtu => '𖡪',
            BamumSupplement::BamumLetterPhaseDashBMfonTeuaeq => '𖡫',
            BamumSupplement::BamumLetterPhaseDashBMbitMbaaket => '𖡬',
            BamumSupplement::BamumLetterPhaseDashBNyiNteum => '𖡭',
            BamumSupplement::BamumLetterPhaseDashBKeupuq => '𖡮',
            BamumSupplement::BamumLetterPhaseDashBGheughen => '𖡯',
            BamumSupplement::BamumLetterPhaseDashBKeuyeux => '𖡰',
            BamumSupplement::BamumLetterPhaseDashBLaanae => '𖡱',
            BamumSupplement::BamumLetterPhaseDashBParum => '𖡲',
            BamumSupplement::BamumLetterPhaseDashBVeum => '𖡳',
            BamumSupplement::BamumLetterPhaseDashBNgkindiMvop => '𖡴',
            BamumSupplement::BamumLetterPhaseDashBNggeuMbu => '𖡵',
            BamumSupplement::BamumLetterPhaseDashBWuaet => '𖡶',
            BamumSupplement::BamumLetterPhaseDashBSakeuae => '𖡷',
            BamumSupplement::BamumLetterPhaseDashBTaam => '𖡸',
            BamumSupplement::BamumLetterPhaseDashBMeuq => '𖡹',
            BamumSupplement::BamumLetterPhaseDashBNgguoq => '𖡺',
            BamumSupplement::BamumLetterPhaseDashBNgguoqLarge => '𖡻',
            BamumSupplement::BamumLetterPhaseDashBMfiyaq => '𖡼',
            BamumSupplement::BamumLetterPhaseDashBSue => '𖡽',
            BamumSupplement::BamumLetterPhaseDashBMbeuri => '𖡾',
            BamumSupplement::BamumLetterPhaseDashBMontieen => '𖡿',
            BamumSupplement::BamumLetterPhaseDashBNyaemae => '𖢀',
            BamumSupplement::BamumLetterPhaseDashBPungaam => '𖢁',
            BamumSupplement::BamumLetterPhaseDashBMeutNggeet => '𖢂',
            BamumSupplement::BamumLetterPhaseDashBFeux => '𖢃',
            BamumSupplement::BamumLetterPhaseDashBMbuoq => '𖢄',
            BamumSupplement::BamumLetterPhaseDashBFee => '𖢅',
            BamumSupplement::BamumLetterPhaseDashBKeuaem => '𖢆',
            BamumSupplement::BamumLetterPhaseDashBMaNjeuaena => '𖢇',
            BamumSupplement::BamumLetterPhaseDashBMaNjuqa => '𖢈',
            BamumSupplement::BamumLetterPhaseDashBLet => '𖢉',
            BamumSupplement::BamumLetterPhaseDashBNggaam => '𖢊',
            BamumSupplement::BamumLetterPhaseDashBNsen => '𖢋',
            BamumSupplement::BamumLetterPhaseDashBMa => '𖢌',
            BamumSupplement::BamumLetterPhaseDashBKiq => '𖢍',
            BamumSupplement::BamumLetterPhaseDashBNgom => '𖢎',
            BamumSupplement::BamumLetterPhaseDashCNgkueMaemba => '𖢏',
            BamumSupplement::BamumLetterPhaseDashCNza => '𖢐',
            BamumSupplement::BamumLetterPhaseDashCYum => '𖢑',
            BamumSupplement::BamumLetterPhaseDashCWangkuoq => '𖢒',
            BamumSupplement::BamumLetterPhaseDashCNggen => '𖢓',
            BamumSupplement::BamumLetterPhaseDashCNdeuaeree => '𖢔',
            BamumSupplement::BamumLetterPhaseDashCNgkaq => '𖢕',
            BamumSupplement::BamumLetterPhaseDashCGharae => '𖢖',
            BamumSupplement::BamumLetterPhaseDashCMbeekeet => '𖢗',
            BamumSupplement::BamumLetterPhaseDashCGbayi => '𖢘',
            BamumSupplement::BamumLetterPhaseDashCNyirMkparaqMeun => '𖢙',
            BamumSupplement::BamumLetterPhaseDashCNtuMbit => '𖢚',
            BamumSupplement::BamumLetterPhaseDashCMbeum => '𖢛',
            BamumSupplement::BamumLetterPhaseDashCPirieen => '𖢜',
            BamumSupplement::BamumLetterPhaseDashCNdombu => '𖢝',
            BamumSupplement::BamumLetterPhaseDashCMbaaCabbageDashTree => '𖢞',
            BamumSupplement::BamumLetterPhaseDashCKeusheuaep => '𖢟',
            BamumSupplement::BamumLetterPhaseDashCGhap => '𖢠',
            BamumSupplement::BamumLetterPhaseDashCKeukaq => '𖢡',
            BamumSupplement::BamumLetterPhaseDashCYuMuomae => '𖢢',
            BamumSupplement::BamumLetterPhaseDashCNzeum => '𖢣',
            BamumSupplement::BamumLetterPhaseDashCMbue => '𖢤',
            BamumSupplement::BamumLetterPhaseDashCNseuaen => '𖢥',
            BamumSupplement::BamumLetterPhaseDashCMbit => '𖢦',
            BamumSupplement::BamumLetterPhaseDashCYeuq => '𖢧',
            BamumSupplement::BamumLetterPhaseDashCKparaq => '𖢨',
            BamumSupplement::BamumLetterPhaseDashCKaa => '𖢩',
            BamumSupplement::BamumLetterPhaseDashCSeux => '𖢪',
            BamumSupplement::BamumLetterPhaseDashCNdida => '𖢫',
            BamumSupplement::BamumLetterPhaseDashCTaashae => '𖢬',
            BamumSupplement::BamumLetterPhaseDashCNjueq => '𖢭',
            BamumSupplement::BamumLetterPhaseDashCTitaYue => '𖢮',
            BamumSupplement::BamumLetterPhaseDashCSuaet => '𖢯',
            BamumSupplement::BamumLetterPhaseDashCNgguaenNyam => '𖢰',
            BamumSupplement::BamumLetterPhaseDashCVeux => '𖢱',
            BamumSupplement::BamumLetterPhaseDashCNansanaq => '𖢲',
            BamumSupplement::BamumLetterPhaseDashCMaKeuaeri => '𖢳',
            BamumSupplement::BamumLetterPhaseDashCNtaa => '𖢴',
            BamumSupplement::BamumLetterPhaseDashCNgguon => '𖢵',
            BamumSupplement::BamumLetterPhaseDashCLap => '𖢶',
            BamumSupplement::BamumLetterPhaseDashCMbirieen => '𖢷',
            BamumSupplement::BamumLetterPhaseDashCMgbasaq => '𖢸',
            BamumSupplement::BamumLetterPhaseDashCNteungba => '𖢹',
            BamumSupplement::BamumLetterPhaseDashCTeuteux => '𖢺',
            BamumSupplement::BamumLetterPhaseDashCNggum => '𖢻',
            BamumSupplement::BamumLetterPhaseDashCFue => '𖢼',
            BamumSupplement::BamumLetterPhaseDashCNdeut => '𖢽',
            BamumSupplement::BamumLetterPhaseDashCNsa => '𖢾',
            BamumSupplement::BamumLetterPhaseDashCNshaq => '𖢿',
            BamumSupplement::BamumLetterPhaseDashCBung => '𖣀',
            BamumSupplement::BamumLetterPhaseDashCVeuaepen => '𖣁',
            BamumSupplement::BamumLetterPhaseDashCMberae => '𖣂',
            BamumSupplement::BamumLetterPhaseDashCRu => '𖣃',
            BamumSupplement::BamumLetterPhaseDashCNjaem => '𖣄',
            BamumSupplement::BamumLetterPhaseDashCLam => '𖣅',
            BamumSupplement::BamumLetterPhaseDashCTituaep => '𖣆',
            BamumSupplement::BamumLetterPhaseDashCNsuotNgom => '𖣇',
            BamumSupplement::BamumLetterPhaseDashCNjeeee => '𖣈',
            BamumSupplement::BamumLetterPhaseDashCKet => '𖣉',
            BamumSupplement::BamumLetterPhaseDashCNggu => '𖣊',
            BamumSupplement::BamumLetterPhaseDashCMaesi => '𖣋',
            BamumSupplement::BamumLetterPhaseDashCMbuaem => '𖣌',
            BamumSupplement::BamumLetterPhaseDashCLu => '𖣍',
            BamumSupplement::BamumLetterPhaseDashCKut => '𖣎',
            BamumSupplement::BamumLetterPhaseDashCNjam => '𖣏',
            BamumSupplement::BamumLetterPhaseDashCNgom => '𖣐',
            BamumSupplement::BamumLetterPhaseDashCWup => '𖣑',
            BamumSupplement::BamumLetterPhaseDashCNggueet => '𖣒',
            BamumSupplement::BamumLetterPhaseDashCNsom => '𖣓',
            BamumSupplement::BamumLetterPhaseDashCNten => '𖣔',
            BamumSupplement::BamumLetterPhaseDashCKuopNkaarae => '𖣕',
            BamumSupplement::BamumLetterPhaseDashCNsun => '𖣖',
            BamumSupplement::BamumLetterPhaseDashCNdam => '𖣗',
            BamumSupplement::BamumLetterPhaseDashCMaNsiee => '𖣘',
            BamumSupplement::BamumLetterPhaseDashCYaa => '𖣙',
            BamumSupplement::BamumLetterPhaseDashCNdap => '𖣚',
            BamumSupplement::BamumLetterPhaseDashCShueq => '𖣛',
            BamumSupplement::BamumLetterPhaseDashCSetfon => '𖣜',
            BamumSupplement::BamumLetterPhaseDashCMbi => '𖣝',
            BamumSupplement::BamumLetterPhaseDashCMaemba => '𖣞',
            BamumSupplement::BamumLetterPhaseDashCMbanyi => '𖣟',
            BamumSupplement::BamumLetterPhaseDashCKeuseux => '𖣠',
            BamumSupplement::BamumLetterPhaseDashCMbeux => '𖣡',
            BamumSupplement::BamumLetterPhaseDashCKeum => '𖣢',
            BamumSupplement::BamumLetterPhaseDashCMbaaPicket => '𖣣',
            BamumSupplement::BamumLetterPhaseDashCYuwoq => '𖣤',
            BamumSupplement::BamumLetterPhaseDashCNjeux => '𖣥',
            BamumSupplement::BamumLetterPhaseDashCMiee => '𖣦',
            BamumSupplement::BamumLetterPhaseDashCMuae => '𖣧',
            BamumSupplement::BamumLetterPhaseDashCShiq => '𖣨',
            BamumSupplement::BamumLetterPhaseDashCKenLaw => '𖣩',
            BamumSupplement::BamumLetterPhaseDashCKenFatigue => '𖣪',
            BamumSupplement::BamumLetterPhaseDashCNgaq => '𖣫',
            BamumSupplement::BamumLetterPhaseDashCNaq => '𖣬',
            BamumSupplement::BamumLetterPhaseDashCLiq => '𖣭',
            BamumSupplement::BamumLetterPhaseDashCPin => '𖣮',
            BamumSupplement::BamumLetterPhaseDashCPen => '𖣯',
            BamumSupplement::BamumLetterPhaseDashCTet => '𖣰',
            BamumSupplement::BamumLetterPhaseDashDMbuo => '𖣱',
            BamumSupplement::BamumLetterPhaseDashDWap => '𖣲',
            BamumSupplement::BamumLetterPhaseDashDNji => '𖣳',
            BamumSupplement::BamumLetterPhaseDashDMfon => '𖣴',
            BamumSupplement::BamumLetterPhaseDashDNjiee => '𖣵',
            BamumSupplement::BamumLetterPhaseDashDLiee => '𖣶',
            BamumSupplement::BamumLetterPhaseDashDNjeut => '𖣷',
            BamumSupplement::BamumLetterPhaseDashDNshee => '𖣸',
            BamumSupplement::BamumLetterPhaseDashDNggaamae => '𖣹',
            BamumSupplement::BamumLetterPhaseDashDNyam => '𖣺',
            BamumSupplement::BamumLetterPhaseDashDWuaen => '𖣻',
            BamumSupplement::BamumLetterPhaseDashDNgkun => '𖣼',
            BamumSupplement::BamumLetterPhaseDashDShee => '𖣽',
            BamumSupplement::BamumLetterPhaseDashDNgkap => '𖣾',
            BamumSupplement::BamumLetterPhaseDashDKeuaetmeun => '𖣿',
            BamumSupplement::BamumLetterPhaseDashDTeut => '𖤀',
            BamumSupplement::BamumLetterPhaseDashDSheuae => '𖤁',
            BamumSupplement::BamumLetterPhaseDashDNjap => '𖤂',
            BamumSupplement::BamumLetterPhaseDashDSue => '𖤃',
            BamumSupplement::BamumLetterPhaseDashDKet => '𖤄',
            BamumSupplement::BamumLetterPhaseDashDYaemmae => '𖤅',
            BamumSupplement::BamumLetterPhaseDashDKuom => '𖤆',
            BamumSupplement::BamumLetterPhaseDashDSap => '𖤇',
            BamumSupplement::BamumLetterPhaseDashDMfeut => '𖤈',
            BamumSupplement::BamumLetterPhaseDashDNdeux => '𖤉',
            BamumSupplement::BamumLetterPhaseDashDMaleeri => '𖤊',
            BamumSupplement::BamumLetterPhaseDashDMeut => '𖤋',
            BamumSupplement::BamumLetterPhaseDashDSeuaeq => '𖤌',
            BamumSupplement::BamumLetterPhaseDashDYen => '𖤍',
            BamumSupplement::BamumLetterPhaseDashDNjeuaem => '𖤎',
            BamumSupplement::BamumLetterPhaseDashDKeuotMbuae => '𖤏',
            BamumSupplement::BamumLetterPhaseDashDNgkeuri => '𖤐',
            BamumSupplement::BamumLetterPhaseDashDTu => '𖤑',
            BamumSupplement::BamumLetterPhaseDashDGhaa => '𖤒',
            BamumSupplement::BamumLetterPhaseDashDNgkyee => '𖤓',
            BamumSupplement::BamumLetterPhaseDashDFeufeuaet => '𖤔',
            BamumSupplement::BamumLetterPhaseDashDNdee => '𖤕',
            BamumSupplement::BamumLetterPhaseDashDMgbofum => '𖤖',
            BamumSupplement::BamumLetterPhaseDashDLeuaep => '𖤗',
            BamumSupplement::BamumLetterPhaseDashDNdon => '𖤘',
            BamumSupplement::BamumLetterPhaseDashDMoni => '𖤙',
            BamumSupplement::BamumLetterPhaseDashDMgbeun => '𖤚',
            BamumSupplement::BamumLetterPhaseDashDPuut => '𖤛',
            BamumSupplement::BamumLetterPhaseDashDMgbiee => '𖤜',
            BamumSupplement::BamumLetterPhaseDashDMfo => '𖤝',
            BamumSupplement::BamumLetterPhaseDashDLum => '𖤞',
            BamumSupplement::BamumLetterPhaseDashDNsieep => '𖤟',
            BamumSupplement::BamumLetterPhaseDashDMbaa => '𖤠',
            BamumSupplement::BamumLetterPhaseDashDKwaet => '𖤡',
            BamumSupplement::BamumLetterPhaseDashDNyet => '𖤢',
            BamumSupplement::BamumLetterPhaseDashDTeuaen => '𖤣',
            BamumSupplement::BamumLetterPhaseDashDSot => '𖤤',
            BamumSupplement::BamumLetterPhaseDashDYuwoq => '𖤥',
            BamumSupplement::BamumLetterPhaseDashDKeum => '𖤦',
            BamumSupplement::BamumLetterPhaseDashDRaem => '𖤧',
            BamumSupplement::BamumLetterPhaseDashDTeeee => '𖤨',
            BamumSupplement::BamumLetterPhaseDashDNgkeuaeq => '𖤩',
            BamumSupplement::BamumLetterPhaseDashDMfeuae => '𖤪',
            BamumSupplement::BamumLetterPhaseDashDNsieet => '𖤫',
            BamumSupplement::BamumLetterPhaseDashDKeup => '𖤬',
            BamumSupplement::BamumLetterPhaseDashDPip => '𖤭',
            BamumSupplement::BamumLetterPhaseDashDPeutae => '𖤮',
            BamumSupplement::BamumLetterPhaseDashDNyue => '𖤯',
            BamumSupplement::BamumLetterPhaseDashDLet => '𖤰',
            BamumSupplement::BamumLetterPhaseDashDNggaam => '𖤱',
            BamumSupplement::BamumLetterPhaseDashDMfiee => '𖤲',
            BamumSupplement::BamumLetterPhaseDashDNggwaen => '𖤳',
            BamumSupplement::BamumLetterPhaseDashDYuom => '𖤴',
            BamumSupplement::BamumLetterPhaseDashDPap => '𖤵',
            BamumSupplement::BamumLetterPhaseDashDYuop => '𖤶',
            BamumSupplement::BamumLetterPhaseDashDNdam => '𖤷',
            BamumSupplement::BamumLetterPhaseDashDNteum => '𖤸',
            BamumSupplement::BamumLetterPhaseDashDSuae => '𖤹',
            BamumSupplement::BamumLetterPhaseDashDKun => '𖤺',
            BamumSupplement::BamumLetterPhaseDashDNggeux => '𖤻',
            BamumSupplement::BamumLetterPhaseDashDNgkiee => '𖤼',
            BamumSupplement::BamumLetterPhaseDashDTuot => '𖤽',
            BamumSupplement::BamumLetterPhaseDashDMeun => '𖤾',
            BamumSupplement::BamumLetterPhaseDashDKuq => '𖤿',
            BamumSupplement::BamumLetterPhaseDashDNsum => '𖥀',
            BamumSupplement::BamumLetterPhaseDashDTeun => '𖥁',
            BamumSupplement::BamumLetterPhaseDashDMaenjet => '𖥂',
            BamumSupplement::BamumLetterPhaseDashDNggap => '𖥃',
            BamumSupplement::BamumLetterPhaseDashDLeum => '𖥄',
            BamumSupplement::BamumLetterPhaseDashDNgguom => '𖥅',
            BamumSupplement::BamumLetterPhaseDashDNshut => '𖥆',
            BamumSupplement::BamumLetterPhaseDashDNjueq => '𖥇',
            BamumSupplement::BamumLetterPhaseDashDGheuae => '𖥈',
            BamumSupplement::BamumLetterPhaseDashDKu => '𖥉',
            BamumSupplement::BamumLetterPhaseDashDRenOld => '𖥊',
            BamumSupplement::BamumLetterPhaseDashDTae => '𖥋',
            BamumSupplement::BamumLetterPhaseDashDToq => '𖥌',
            BamumSupplement::BamumLetterPhaseDashDNyi => '𖥍',
            BamumSupplement::BamumLetterPhaseDashDRii => '𖥎',
            BamumSupplement::BamumLetterPhaseDashDLeeee => '𖥏',
            BamumSupplement::BamumLetterPhaseDashDMeeee => '𖥐',
            BamumSupplement::BamumLetterPhaseDashDM => '𖥑',
            BamumSupplement::BamumLetterPhaseDashDSuu => '𖥒',
            BamumSupplement::BamumLetterPhaseDashDMu => '𖥓',
            BamumSupplement::BamumLetterPhaseDashDShii => '𖥔',
            BamumSupplement::BamumLetterPhaseDashDSheux => '𖥕',
            BamumSupplement::BamumLetterPhaseDashDKyee => '𖥖',
            BamumSupplement::BamumLetterPhaseDashDNu => '𖥗',
            BamumSupplement::BamumLetterPhaseDashDShu => '𖥘',
            BamumSupplement::BamumLetterPhaseDashDNtee => '𖥙',
            BamumSupplement::BamumLetterPhaseDashDPee => '𖥚',
            BamumSupplement::BamumLetterPhaseDashDNi => '𖥛',
            BamumSupplement::BamumLetterPhaseDashDShoq => '𖥜',
            BamumSupplement::BamumLetterPhaseDashDPuq => '𖥝',
            BamumSupplement::BamumLetterPhaseDashDMvop => '𖥞',
            BamumSupplement::BamumLetterPhaseDashDLoq => '𖥟',
            BamumSupplement::BamumLetterPhaseDashDRenMuch => '𖥠',
            BamumSupplement::BamumLetterPhaseDashDTi => '𖥡',
            BamumSupplement::BamumLetterPhaseDashDNtuu => '𖥢',
            BamumSupplement::BamumLetterPhaseDashDMbaaSeven => '𖥣',
            BamumSupplement::BamumLetterPhaseDashDSaq => '𖥤',
            BamumSupplement::BamumLetterPhaseDashDFaa => '𖥥',
            BamumSupplement::BamumLetterPhaseDashENdap => '𖥦',
            BamumSupplement::BamumLetterPhaseDashEToon => '𖥧',
            BamumSupplement::BamumLetterPhaseDashEMbeum => '𖥨',
            BamumSupplement::BamumLetterPhaseDashELap => '𖥩',
            BamumSupplement::BamumLetterPhaseDashEVom => '𖥪',
            BamumSupplement::BamumLetterPhaseDashELoon => '𖥫',
            BamumSupplement::BamumLetterPhaseDashEPaa => '𖥬',
            BamumSupplement::BamumLetterPhaseDashESom => '𖥭',
            BamumSupplement::BamumLetterPhaseDashERaq => '𖥮',
            BamumSupplement::BamumLetterPhaseDashENshuop => '𖥯',
            BamumSupplement::BamumLetterPhaseDashENdun => '𖥰',
            BamumSupplement::BamumLetterPhaseDashEPuae => '𖥱',
            BamumSupplement::BamumLetterPhaseDashETam => '𖥲',
            BamumSupplement::BamumLetterPhaseDashENgka => '𖥳',
            BamumSupplement::BamumLetterPhaseDashEKpeux => '𖥴',
            BamumSupplement::BamumLetterPhaseDashEWuo => '𖥵',
            BamumSupplement::BamumLetterPhaseDashESee => '𖥶',
            BamumSupplement::BamumLetterPhaseDashENggeuaet => '𖥷',
            BamumSupplement::BamumLetterPhaseDashEPaam => '𖥸',
            BamumSupplement::BamumLetterPhaseDashEToo => '𖥹',
            BamumSupplement::BamumLetterPhaseDashEKuop => '𖥺',
            BamumSupplement::BamumLetterPhaseDashELom => '𖥻',
            BamumSupplement::BamumLetterPhaseDashENshiee => '𖥼',
            BamumSupplement::BamumLetterPhaseDashENgop => '𖥽',
            BamumSupplement::BamumLetterPhaseDashEMaem => '𖥾',
            BamumSupplement::BamumLetterPhaseDashENgkeux => '𖥿',
            BamumSupplement::BamumLetterPhaseDashENgoq => '𖦀',
            BamumSupplement::BamumLetterPhaseDashENshue => '𖦁',
            BamumSupplement::BamumLetterPhaseDashERimgba => '𖦂',
            BamumSupplement::BamumLetterPhaseDashENjeux => '𖦃',
            BamumSupplement::BamumLetterPhaseDashEPeem => '𖦄',
            BamumSupplement::BamumLetterPhaseDashESaa => '𖦅',
            BamumSupplement::BamumLetterPhaseDashENggurae => '𖦆',
            BamumSupplement::BamumLetterPhaseDashEMgba => '𖦇',
            BamumSupplement::BamumLetterPhaseDashEGheux => '𖦈',
            BamumSupplement::BamumLetterPhaseDashENgkeuaem => '𖦉',
            BamumSupplement::BamumLetterPhaseDashENjaemli => '𖦊',
            BamumSupplement::BamumLetterPhaseDashEMap => '𖦋',
            BamumSupplement::BamumLetterPhaseDashELoot => '𖦌',
            BamumSupplement::BamumLetterPhaseDashENggeeee => '𖦍',
            BamumSupplement::BamumLetterPhaseDashENdiq => '𖦎',
            BamumSupplement::BamumLetterPhaseDashETaenNteum => '𖦏',
            BamumSupplement::BamumLetterPhaseDashESet => '𖦐',
            BamumSupplement::BamumLetterPhaseDashEPum => '𖦑',
            BamumSupplement::BamumLetterPhaseDashENdaaSoftness => '𖦒',
            BamumSupplement::BamumLetterPhaseDashENgguaeshaeNyam => '𖦓',
            BamumSupplement::BamumLetterPhaseDashEYiee => '𖦔',
            BamumSupplement::BamumLetterPhaseDashEGheun => '𖦕',
            BamumSupplement::BamumLetterPhaseDashETuae => '𖦖',
            BamumSupplement::BamumLetterPhaseDashEYeuae => '𖦗',
            BamumSupplement::BamumLetterPhaseDashEPo => '𖦘',
            BamumSupplement::BamumLetterPhaseDashETumae => '𖦙',
            BamumSupplement::BamumLetterPhaseDashEKeuae => '𖦚',
            BamumSupplement::BamumLetterPhaseDashESuaen => '𖦛',
            BamumSupplement::BamumLetterPhaseDashETeuaeq => '𖦜',
            BamumSupplement::BamumLetterPhaseDashEVeuae => '𖦝',
            BamumSupplement::BamumLetterPhaseDashEWeux => '𖦞',
            BamumSupplement::BamumLetterPhaseDashELaam => '𖦟',
            BamumSupplement::BamumLetterPhaseDashEPu => '𖦠',
            BamumSupplement::BamumLetterPhaseDashETaaq => '𖦡',
            BamumSupplement::BamumLetterPhaseDashEGhaamae => '𖦢',
            BamumSupplement::BamumLetterPhaseDashENgeureut => '𖦣',
            BamumSupplement::BamumLetterPhaseDashESheuaeq => '𖦤',
            BamumSupplement::BamumLetterPhaseDashEMgben => '𖦥',
            BamumSupplement::BamumLetterPhaseDashEMbee => '𖦦',
            BamumSupplement::BamumLetterPhaseDashENzaq => '𖦧',
            BamumSupplement::BamumLetterPhaseDashENkom => '𖦨',
            BamumSupplement::BamumLetterPhaseDashEGbet => '𖦩',
            BamumSupplement::BamumLetterPhaseDashETum => '𖦪',
            BamumSupplement::BamumLetterPhaseDashEKuet => '𖦫',
            BamumSupplement::BamumLetterPhaseDashEYap => '𖦬',
            BamumSupplement::BamumLetterPhaseDashENyiCleaver => '𖦭',
            BamumSupplement::BamumLetterPhaseDashEYit => '𖦮',
            BamumSupplement::BamumLetterPhaseDashEMfeuq => '𖦯',
            BamumSupplement::BamumLetterPhaseDashENdiaq => '𖦰',
            BamumSupplement::BamumLetterPhaseDashEPieeq => '𖦱',
            BamumSupplement::BamumLetterPhaseDashEYueq => '𖦲',
            BamumSupplement::BamumLetterPhaseDashELeuaem => '𖦳',
            BamumSupplement::BamumLetterPhaseDashEFue => '𖦴',
            BamumSupplement::BamumLetterPhaseDashEGbeux => '𖦵',
            BamumSupplement::BamumLetterPhaseDashENgkup => '𖦶',
            BamumSupplement::BamumLetterPhaseDashEKet => '𖦷',
            BamumSupplement::BamumLetterPhaseDashEMae => '𖦸',
            BamumSupplement::BamumLetterPhaseDashENgkaami => '𖦹',
            BamumSupplement::BamumLetterPhaseDashEGhet => '𖦺',
            BamumSupplement::BamumLetterPhaseDashEFa => '𖦻',
            BamumSupplement::BamumLetterPhaseDashENtum => '𖦼',
            BamumSupplement::BamumLetterPhaseDashEPeut => '𖦽',
            BamumSupplement::BamumLetterPhaseDashEYeum => '𖦾',
            BamumSupplement::BamumLetterPhaseDashENggeuae => '𖦿',
            BamumSupplement::BamumLetterPhaseDashENyiBetween => '𖧀',
            BamumSupplement::BamumLetterPhaseDashENzuq => '𖧁',
            BamumSupplement::BamumLetterPhaseDashEPoon => '𖧂',
            BamumSupplement::BamumLetterPhaseDashEMiee => '𖧃',
            BamumSupplement::BamumLetterPhaseDashEFuet => '𖧄',
            BamumSupplement::BamumLetterPhaseDashENae => '𖧅',
            BamumSupplement::BamumLetterPhaseDashEMuae => '𖧆',
            BamumSupplement::BamumLetterPhaseDashEGheuae => '𖧇',
            BamumSupplement::BamumLetterPhaseDashEFuI => '𖧈',
            BamumSupplement::BamumLetterPhaseDashEMvi => '𖧉',
            BamumSupplement::BamumLetterPhaseDashEPuaq => '𖧊',
            BamumSupplement::BamumLetterPhaseDashENgkum => '𖧋',
            BamumSupplement::BamumLetterPhaseDashEKut => '𖧌',
            BamumSupplement::BamumLetterPhaseDashEPiet => '𖧍',
            BamumSupplement::BamumLetterPhaseDashENtap => '𖧎',
            BamumSupplement::BamumLetterPhaseDashEYeuaet => '𖧏',
            BamumSupplement::BamumLetterPhaseDashENggup => '𖧐',
            BamumSupplement::BamumLetterPhaseDashEPaPeople => '𖧑',
            BamumSupplement::BamumLetterPhaseDashEFuCall => '𖧒',
            BamumSupplement::BamumLetterPhaseDashEFom => '𖧓',
            BamumSupplement::BamumLetterPhaseDashENjee => '𖧔',
            BamumSupplement::BamumLetterPhaseDashEA => '𖧕',
            BamumSupplement::BamumLetterPhaseDashEToq => '𖧖',
            BamumSupplement::BamumLetterPhaseDashEO => '𖧗',
            BamumSupplement::BamumLetterPhaseDashEI => '𖧘',
            BamumSupplement::BamumLetterPhaseDashELaq => '𖧙',
            BamumSupplement::BamumLetterPhaseDashEPaPlural => '𖧚',
            BamumSupplement::BamumLetterPhaseDashETaa => '𖧛',
            BamumSupplement::BamumLetterPhaseDashETaq => '𖧜',
            BamumSupplement::BamumLetterPhaseDashENdaaMyHouse => '𖧝',
            BamumSupplement::BamumLetterPhaseDashEShiq => '𖧞',
            BamumSupplement::BamumLetterPhaseDashEYeux => '𖧟',
            BamumSupplement::BamumLetterPhaseDashENguae => '𖧠',
            BamumSupplement::BamumLetterPhaseDashEYuaen => '𖧡',
            BamumSupplement::BamumLetterPhaseDashEYoqSwimming => '𖧢',
            BamumSupplement::BamumLetterPhaseDashEYoqCover => '𖧣',
            BamumSupplement::BamumLetterPhaseDashEYuq => '𖧤',
            BamumSupplement::BamumLetterPhaseDashEYun => '𖧥',
            BamumSupplement::BamumLetterPhaseDashEKeux => '𖧦',
            BamumSupplement::BamumLetterPhaseDashEPeux => '𖧧',
            BamumSupplement::BamumLetterPhaseDashENjeeEpoch => '𖧨',
            BamumSupplement::BamumLetterPhaseDashEPue => '𖧩',
            BamumSupplement::BamumLetterPhaseDashEWue => '𖧪',
            BamumSupplement::BamumLetterPhaseDashEFee => '𖧫',
            BamumSupplement::BamumLetterPhaseDashEVee => '𖧬',
            BamumSupplement::BamumLetterPhaseDashELu => '𖧭',
            BamumSupplement::BamumLetterPhaseDashEMi => '𖧮',
            BamumSupplement::BamumLetterPhaseDashEReux => '𖧯',
            BamumSupplement::BamumLetterPhaseDashERae => '𖧰',
            BamumSupplement::BamumLetterPhaseDashENguaet => '𖧱',
            BamumSupplement::BamumLetterPhaseDashENga => '𖧲',
            BamumSupplement::BamumLetterPhaseDashESho => '𖧳',
            BamumSupplement::BamumLetterPhaseDashEShoq => '𖧴',
            BamumSupplement::BamumLetterPhaseDashEFuRemedy => '𖧵',
            BamumSupplement::BamumLetterPhaseDashENa => '𖧶',
            BamumSupplement::BamumLetterPhaseDashEPi => '𖧷',
            BamumSupplement::BamumLetterPhaseDashELoq => '𖧸',
            BamumSupplement::BamumLetterPhaseDashEKo => '𖧹',
            BamumSupplement::BamumLetterPhaseDashEMen => '𖧺',
            BamumSupplement::BamumLetterPhaseDashEMa => '𖧻',
            BamumSupplement::BamumLetterPhaseDashEMaq => '𖧼',
            BamumSupplement::BamumLetterPhaseDashETeu => '𖧽',
            BamumSupplement::BamumLetterPhaseDashEKi => '𖧾',
            BamumSupplement::BamumLetterPhaseDashEMon => '𖧿',
            BamumSupplement::BamumLetterPhaseDashETen => '𖨀',
            BamumSupplement::BamumLetterPhaseDashEFaq => '𖨁',
            BamumSupplement::BamumLetterPhaseDashEGhom => '𖨂',
            BamumSupplement::BamumLetterPhaseDashFKa => '𖨃',
            BamumSupplement::BamumLetterPhaseDashFU => '𖨄',
            BamumSupplement::BamumLetterPhaseDashFKu => '𖨅',
            BamumSupplement::BamumLetterPhaseDashFEe => '𖨆',
            BamumSupplement::BamumLetterPhaseDashFRee => '𖨇',
            BamumSupplement::BamumLetterPhaseDashFTae => '𖨈',
            BamumSupplement::BamumLetterPhaseDashFNyi => '𖨉',
            BamumSupplement::BamumLetterPhaseDashFLa => '𖨊',
            BamumSupplement::BamumLetterPhaseDashFRii => '𖨋',
            BamumSupplement::BamumLetterPhaseDashFRiee => '𖨌',
            BamumSupplement::BamumLetterPhaseDashFMeeee => '𖨍',
            BamumSupplement::BamumLetterPhaseDashFTaa => '𖨎',
            BamumSupplement::BamumLetterPhaseDashFNdaa => '𖨏',
            BamumSupplement::BamumLetterPhaseDashFNjaem => '𖨐',
            BamumSupplement::BamumLetterPhaseDashFM => '𖨑',
            BamumSupplement::BamumLetterPhaseDashFSuu => '𖨒',
            BamumSupplement::BamumLetterPhaseDashFShii => '𖨓',
            BamumSupplement::BamumLetterPhaseDashFSi => '𖨔',
            BamumSupplement::BamumLetterPhaseDashFSeux => '𖨕',
            BamumSupplement::BamumLetterPhaseDashFKyee => '𖨖',
            BamumSupplement::BamumLetterPhaseDashFKet => '𖨗',
            BamumSupplement::BamumLetterPhaseDashFNuae => '𖨘',
            BamumSupplement::BamumLetterPhaseDashFNu => '𖨙',
            BamumSupplement::BamumLetterPhaseDashFNjuae => '𖨚',
            BamumSupplement::BamumLetterPhaseDashFYoq => '𖨛',
            BamumSupplement::BamumLetterPhaseDashFShu => '𖨜',
            BamumSupplement::BamumLetterPhaseDashFYa => '𖨝',
            BamumSupplement::BamumLetterPhaseDashFNsha => '𖨞',
            BamumSupplement::BamumLetterPhaseDashFPeux => '𖨟',
            BamumSupplement::BamumLetterPhaseDashFNtee => '𖨠',
            BamumSupplement::BamumLetterPhaseDashFWue => '𖨡',
            BamumSupplement::BamumLetterPhaseDashFPee => '𖨢',
            BamumSupplement::BamumLetterPhaseDashFRu => '𖨣',
            BamumSupplement::BamumLetterPhaseDashFNi => '𖨤',
            BamumSupplement::BamumLetterPhaseDashFReux => '𖨥',
            BamumSupplement::BamumLetterPhaseDashFKen => '𖨦',
            BamumSupplement::BamumLetterPhaseDashFNgkwaen => '𖨧',
            BamumSupplement::BamumLetterPhaseDashFNgga => '𖨨',
            BamumSupplement::BamumLetterPhaseDashFSho => '𖨩',
            BamumSupplement::BamumLetterPhaseDashFPuae => '𖨪',
            BamumSupplement::BamumLetterPhaseDashFFom => '𖨫',
            BamumSupplement::BamumLetterPhaseDashFWa => '𖨬',
            BamumSupplement::BamumLetterPhaseDashFLi => '𖨭',
            BamumSupplement::BamumLetterPhaseDashFLoq => '𖨮',
            BamumSupplement::BamumLetterPhaseDashFKo => '𖨯',
            BamumSupplement::BamumLetterPhaseDashFMben => '𖨰',
            BamumSupplement::BamumLetterPhaseDashFRen => '𖨱',
            BamumSupplement::BamumLetterPhaseDashFMa => '𖨲',
            BamumSupplement::BamumLetterPhaseDashFMo => '𖨳',
            BamumSupplement::BamumLetterPhaseDashFMbaa => '𖨴',
            BamumSupplement::BamumLetterPhaseDashFTet => '𖨵',
            BamumSupplement::BamumLetterPhaseDashFKpa => '𖨶',
            BamumSupplement::BamumLetterPhaseDashFSamba => '𖨷',
            BamumSupplement::BamumLetterPhaseDashFVueq => '𖨸',
        }
    }
}

impl std::convert::TryFrom<char> for BamumSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𖠀' => Ok(BamumSupplement::BamumLetterPhaseDashANgkueMfon),
            '𖠁' => Ok(BamumSupplement::BamumLetterPhaseDashAGbieeFon),
            '𖠂' => Ok(BamumSupplement::BamumLetterPhaseDashAPonMfonPipaemgbiee),
            '𖠃' => Ok(BamumSupplement::BamumLetterPhaseDashAPonMfonPipaemba),
            '𖠄' => Ok(BamumSupplement::BamumLetterPhaseDashANaaMfon),
            '𖠅' => Ok(BamumSupplement::BamumLetterPhaseDashAShuenshuet),
            '𖠆' => Ok(BamumSupplement::BamumLetterPhaseDashATitaMfon),
            '𖠇' => Ok(BamumSupplement::BamumLetterPhaseDashANzaMfon),
            '𖠈' => Ok(BamumSupplement::BamumLetterPhaseDashAShindaPaNji),
            '𖠉' => Ok(BamumSupplement::BamumLetterPhaseDashAPonPaNjiPipaemgbiee),
            '𖠊' => Ok(BamumSupplement::BamumLetterPhaseDashAPonPaNjiPipaemba),
            '𖠋' => Ok(BamumSupplement::BamumLetterPhaseDashAMaembgbiee),
            '𖠌' => Ok(BamumSupplement::BamumLetterPhaseDashATuMaemba),
            '𖠍' => Ok(BamumSupplement::BamumLetterPhaseDashANgangu),
            '𖠎' => Ok(BamumSupplement::BamumLetterPhaseDashAMaemveux),
            '𖠏' => Ok(BamumSupplement::BamumLetterPhaseDashAMansuae),
            '𖠐' => Ok(BamumSupplement::BamumLetterPhaseDashAMveuaengam),
            '𖠑' => Ok(BamumSupplement::BamumLetterPhaseDashASeunyam),
            '𖠒' => Ok(BamumSupplement::BamumLetterPhaseDashANtoqpen),
            '𖠓' => Ok(BamumSupplement::BamumLetterPhaseDashAKeukeutnda),
            '𖠔' => Ok(BamumSupplement::BamumLetterPhaseDashANkindi),
            '𖠕' => Ok(BamumSupplement::BamumLetterPhaseDashASuu),
            '𖠖' => Ok(BamumSupplement::BamumLetterPhaseDashANgkuenzeum),
            '𖠗' => Ok(BamumSupplement::BamumLetterPhaseDashALapaq),
            '𖠘' => Ok(BamumSupplement::BamumLetterPhaseDashALetKut),
            '𖠙' => Ok(BamumSupplement::BamumLetterPhaseDashANtapMfaa),
            '𖠚' => Ok(BamumSupplement::BamumLetterPhaseDashAMaekeup),
            '𖠛' => Ok(BamumSupplement::BamumLetterPhaseDashAPashae),
            '𖠜' => Ok(BamumSupplement::BamumLetterPhaseDashAGheuaerae),
            '𖠝' => Ok(BamumSupplement::BamumLetterPhaseDashAPamshae),
            '𖠞' => Ok(BamumSupplement::BamumLetterPhaseDashAMonNggeuaet),
            '𖠟' => Ok(BamumSupplement::BamumLetterPhaseDashANzunMeut),
            '𖠠' => Ok(BamumSupplement::BamumLetterPhaseDashAUYuqNae),
            '𖠡' => Ok(BamumSupplement::BamumLetterPhaseDashAGheuaegheuae),
            '𖠢' => Ok(BamumSupplement::BamumLetterPhaseDashANtapNtaa),
            '𖠣' => Ok(BamumSupplement::BamumLetterPhaseDashASisa),
            '𖠤' => Ok(BamumSupplement::BamumLetterPhaseDashAMgbasa),
            '𖠥' => Ok(BamumSupplement::BamumLetterPhaseDashAMeunjomndeuq),
            '𖠦' => Ok(BamumSupplement::BamumLetterPhaseDashAMoompuq),
            '𖠧' => Ok(BamumSupplement::BamumLetterPhaseDashAKafa),
            '𖠨' => Ok(BamumSupplement::BamumLetterPhaseDashAPaLeeraewa),
            '𖠩' => Ok(BamumSupplement::BamumLetterPhaseDashANdaLeeraewa),
            '𖠪' => Ok(BamumSupplement::BamumLetterPhaseDashAPet),
            '𖠫' => Ok(BamumSupplement::BamumLetterPhaseDashAMaemkpen),
            '𖠬' => Ok(BamumSupplement::BamumLetterPhaseDashANika),
            '𖠭' => Ok(BamumSupplement::BamumLetterPhaseDashAPup),
            '𖠮' => Ok(BamumSupplement::BamumLetterPhaseDashATuaep),
            '𖠯' => Ok(BamumSupplement::BamumLetterPhaseDashALuaep),
            '𖠰' => Ok(BamumSupplement::BamumLetterPhaseDashASonjam),
            '𖠱' => Ok(BamumSupplement::BamumLetterPhaseDashATeuteuwen),
            '𖠲' => Ok(BamumSupplement::BamumLetterPhaseDashAMaenyi),
            '𖠳' => Ok(BamumSupplement::BamumLetterPhaseDashAKet),
            '𖠴' => Ok(BamumSupplement::BamumLetterPhaseDashANdaanggeuaet),
            '𖠵' => Ok(BamumSupplement::BamumLetterPhaseDashAKuoq),
            '𖠶' => Ok(BamumSupplement::BamumLetterPhaseDashAMoomeut),
            '𖠷' => Ok(BamumSupplement::BamumLetterPhaseDashAShum),
            '𖠸' => Ok(BamumSupplement::BamumLetterPhaseDashALommae),
            '𖠹' => Ok(BamumSupplement::BamumLetterPhaseDashAFiri),
            '𖠺' => Ok(BamumSupplement::BamumLetterPhaseDashARom),
            '𖠻' => Ok(BamumSupplement::BamumLetterPhaseDashAKpoq),
            '𖠼' => Ok(BamumSupplement::BamumLetterPhaseDashASoq),
            '𖠽' => Ok(BamumSupplement::BamumLetterPhaseDashAMapPieet),
            '𖠾' => Ok(BamumSupplement::BamumLetterPhaseDashAShirae),
            '𖠿' => Ok(BamumSupplement::BamumLetterPhaseDashANtap),
            '𖡀' => Ok(BamumSupplement::BamumLetterPhaseDashAShoqNshutYum),
            '𖡁' => Ok(BamumSupplement::BamumLetterPhaseDashANyitMongkeuaeq),
            '𖡂' => Ok(BamumSupplement::BamumLetterPhaseDashAPaarae),
            '𖡃' => Ok(BamumSupplement::BamumLetterPhaseDashANkaarae),
            '𖡄' => Ok(BamumSupplement::BamumLetterPhaseDashAUnknown),
            '𖡅' => Ok(BamumSupplement::BamumLetterPhaseDashANggen),
            '𖡆' => Ok(BamumSupplement::BamumLetterPhaseDashAMaesi),
            '𖡇' => Ok(BamumSupplement::BamumLetterPhaseDashANjam),
            '𖡈' => Ok(BamumSupplement::BamumLetterPhaseDashAMbanyi),
            '𖡉' => Ok(BamumSupplement::BamumLetterPhaseDashANyet),
            '𖡊' => Ok(BamumSupplement::BamumLetterPhaseDashATeuaen),
            '𖡋' => Ok(BamumSupplement::BamumLetterPhaseDashASot),
            '𖡌' => Ok(BamumSupplement::BamumLetterPhaseDashAPaam),
            '𖡍' => Ok(BamumSupplement::BamumLetterPhaseDashANshiee),
            '𖡎' => Ok(BamumSupplement::BamumLetterPhaseDashAMaem),
            '𖡏' => Ok(BamumSupplement::BamumLetterPhaseDashANyi),
            '𖡐' => Ok(BamumSupplement::BamumLetterPhaseDashAKaq),
            '𖡑' => Ok(BamumSupplement::BamumLetterPhaseDashANsha),
            '𖡒' => Ok(BamumSupplement::BamumLetterPhaseDashAVee),
            '𖡓' => Ok(BamumSupplement::BamumLetterPhaseDashALu),
            '𖡔' => Ok(BamumSupplement::BamumLetterPhaseDashANen),
            '𖡕' => Ok(BamumSupplement::BamumLetterPhaseDashANaq),
            '𖡖' => Ok(BamumSupplement::BamumLetterPhaseDashAMbaq),
            '𖡗' => Ok(BamumSupplement::BamumLetterPhaseDashBNshuet),
            '𖡘' => Ok(BamumSupplement::BamumLetterPhaseDashBTuMaemgbiee),
            '𖡙' => Ok(BamumSupplement::BamumLetterPhaseDashBSiee),
            '𖡚' => Ok(BamumSupplement::BamumLetterPhaseDashBSetTu),
            '𖡛' => Ok(BamumSupplement::BamumLetterPhaseDashBLomNteum),
            '𖡜' => Ok(BamumSupplement::BamumLetterPhaseDashBMbaMaelee),
            '𖡝' => Ok(BamumSupplement::BamumLetterPhaseDashBKieem),
            '𖡞' => Ok(BamumSupplement::BamumLetterPhaseDashBYeurae),
            '𖡟' => Ok(BamumSupplement::BamumLetterPhaseDashBMbaarae),
            '𖡠' => Ok(BamumSupplement::BamumLetterPhaseDashBKam),
            '𖡡' => Ok(BamumSupplement::BamumLetterPhaseDashBPeeshi),
            '𖡢' => Ok(BamumSupplement::BamumLetterPhaseDashBYafuLeeraewa),
            '𖡣' => Ok(BamumSupplement::BamumLetterPhaseDashBLamNshutNyam),
            '𖡤' => Ok(BamumSupplement::BamumLetterPhaseDashBNtieeSheuoq),
            '𖡥' => Ok(BamumSupplement::BamumLetterPhaseDashBNduNjaa),
            '𖡦' => Ok(BamumSupplement::BamumLetterPhaseDashBGheugheuaem),
            '𖡧' => Ok(BamumSupplement::BamumLetterPhaseDashBPit),
            '𖡨' => Ok(BamumSupplement::BamumLetterPhaseDashBTuNsiee),
            '𖡩' => Ok(BamumSupplement::BamumLetterPhaseDashBShetNjaq),
            '𖡪' => Ok(BamumSupplement::BamumLetterPhaseDashBSheuaeqtu),
            '𖡫' => Ok(BamumSupplement::BamumLetterPhaseDashBMfonTeuaeq),
            '𖡬' => Ok(BamumSupplement::BamumLetterPhaseDashBMbitMbaaket),
            '𖡭' => Ok(BamumSupplement::BamumLetterPhaseDashBNyiNteum),
            '𖡮' => Ok(BamumSupplement::BamumLetterPhaseDashBKeupuq),
            '𖡯' => Ok(BamumSupplement::BamumLetterPhaseDashBGheughen),
            '𖡰' => Ok(BamumSupplement::BamumLetterPhaseDashBKeuyeux),
            '𖡱' => Ok(BamumSupplement::BamumLetterPhaseDashBLaanae),
            '𖡲' => Ok(BamumSupplement::BamumLetterPhaseDashBParum),
            '𖡳' => Ok(BamumSupplement::BamumLetterPhaseDashBVeum),
            '𖡴' => Ok(BamumSupplement::BamumLetterPhaseDashBNgkindiMvop),
            '𖡵' => Ok(BamumSupplement::BamumLetterPhaseDashBNggeuMbu),
            '𖡶' => Ok(BamumSupplement::BamumLetterPhaseDashBWuaet),
            '𖡷' => Ok(BamumSupplement::BamumLetterPhaseDashBSakeuae),
            '𖡸' => Ok(BamumSupplement::BamumLetterPhaseDashBTaam),
            '𖡹' => Ok(BamumSupplement::BamumLetterPhaseDashBMeuq),
            '𖡺' => Ok(BamumSupplement::BamumLetterPhaseDashBNgguoq),
            '𖡻' => Ok(BamumSupplement::BamumLetterPhaseDashBNgguoqLarge),
            '𖡼' => Ok(BamumSupplement::BamumLetterPhaseDashBMfiyaq),
            '𖡽' => Ok(BamumSupplement::BamumLetterPhaseDashBSue),
            '𖡾' => Ok(BamumSupplement::BamumLetterPhaseDashBMbeuri),
            '𖡿' => Ok(BamumSupplement::BamumLetterPhaseDashBMontieen),
            '𖢀' => Ok(BamumSupplement::BamumLetterPhaseDashBNyaemae),
            '𖢁' => Ok(BamumSupplement::BamumLetterPhaseDashBPungaam),
            '𖢂' => Ok(BamumSupplement::BamumLetterPhaseDashBMeutNggeet),
            '𖢃' => Ok(BamumSupplement::BamumLetterPhaseDashBFeux),
            '𖢄' => Ok(BamumSupplement::BamumLetterPhaseDashBMbuoq),
            '𖢅' => Ok(BamumSupplement::BamumLetterPhaseDashBFee),
            '𖢆' => Ok(BamumSupplement::BamumLetterPhaseDashBKeuaem),
            '𖢇' => Ok(BamumSupplement::BamumLetterPhaseDashBMaNjeuaena),
            '𖢈' => Ok(BamumSupplement::BamumLetterPhaseDashBMaNjuqa),
            '𖢉' => Ok(BamumSupplement::BamumLetterPhaseDashBLet),
            '𖢊' => Ok(BamumSupplement::BamumLetterPhaseDashBNggaam),
            '𖢋' => Ok(BamumSupplement::BamumLetterPhaseDashBNsen),
            '𖢌' => Ok(BamumSupplement::BamumLetterPhaseDashBMa),
            '𖢍' => Ok(BamumSupplement::BamumLetterPhaseDashBKiq),
            '𖢎' => Ok(BamumSupplement::BamumLetterPhaseDashBNgom),
            '𖢏' => Ok(BamumSupplement::BamumLetterPhaseDashCNgkueMaemba),
            '𖢐' => Ok(BamumSupplement::BamumLetterPhaseDashCNza),
            '𖢑' => Ok(BamumSupplement::BamumLetterPhaseDashCYum),
            '𖢒' => Ok(BamumSupplement::BamumLetterPhaseDashCWangkuoq),
            '𖢓' => Ok(BamumSupplement::BamumLetterPhaseDashCNggen),
            '𖢔' => Ok(BamumSupplement::BamumLetterPhaseDashCNdeuaeree),
            '𖢕' => Ok(BamumSupplement::BamumLetterPhaseDashCNgkaq),
            '𖢖' => Ok(BamumSupplement::BamumLetterPhaseDashCGharae),
            '𖢗' => Ok(BamumSupplement::BamumLetterPhaseDashCMbeekeet),
            '𖢘' => Ok(BamumSupplement::BamumLetterPhaseDashCGbayi),
            '𖢙' => Ok(BamumSupplement::BamumLetterPhaseDashCNyirMkparaqMeun),
            '𖢚' => Ok(BamumSupplement::BamumLetterPhaseDashCNtuMbit),
            '𖢛' => Ok(BamumSupplement::BamumLetterPhaseDashCMbeum),
            '𖢜' => Ok(BamumSupplement::BamumLetterPhaseDashCPirieen),
            '𖢝' => Ok(BamumSupplement::BamumLetterPhaseDashCNdombu),
            '𖢞' => Ok(BamumSupplement::BamumLetterPhaseDashCMbaaCabbageDashTree),
            '𖢟' => Ok(BamumSupplement::BamumLetterPhaseDashCKeusheuaep),
            '𖢠' => Ok(BamumSupplement::BamumLetterPhaseDashCGhap),
            '𖢡' => Ok(BamumSupplement::BamumLetterPhaseDashCKeukaq),
            '𖢢' => Ok(BamumSupplement::BamumLetterPhaseDashCYuMuomae),
            '𖢣' => Ok(BamumSupplement::BamumLetterPhaseDashCNzeum),
            '𖢤' => Ok(BamumSupplement::BamumLetterPhaseDashCMbue),
            '𖢥' => Ok(BamumSupplement::BamumLetterPhaseDashCNseuaen),
            '𖢦' => Ok(BamumSupplement::BamumLetterPhaseDashCMbit),
            '𖢧' => Ok(BamumSupplement::BamumLetterPhaseDashCYeuq),
            '𖢨' => Ok(BamumSupplement::BamumLetterPhaseDashCKparaq),
            '𖢩' => Ok(BamumSupplement::BamumLetterPhaseDashCKaa),
            '𖢪' => Ok(BamumSupplement::BamumLetterPhaseDashCSeux),
            '𖢫' => Ok(BamumSupplement::BamumLetterPhaseDashCNdida),
            '𖢬' => Ok(BamumSupplement::BamumLetterPhaseDashCTaashae),
            '𖢭' => Ok(BamumSupplement::BamumLetterPhaseDashCNjueq),
            '𖢮' => Ok(BamumSupplement::BamumLetterPhaseDashCTitaYue),
            '𖢯' => Ok(BamumSupplement::BamumLetterPhaseDashCSuaet),
            '𖢰' => Ok(BamumSupplement::BamumLetterPhaseDashCNgguaenNyam),
            '𖢱' => Ok(BamumSupplement::BamumLetterPhaseDashCVeux),
            '𖢲' => Ok(BamumSupplement::BamumLetterPhaseDashCNansanaq),
            '𖢳' => Ok(BamumSupplement::BamumLetterPhaseDashCMaKeuaeri),
            '𖢴' => Ok(BamumSupplement::BamumLetterPhaseDashCNtaa),
            '𖢵' => Ok(BamumSupplement::BamumLetterPhaseDashCNgguon),
            '𖢶' => Ok(BamumSupplement::BamumLetterPhaseDashCLap),
            '𖢷' => Ok(BamumSupplement::BamumLetterPhaseDashCMbirieen),
            '𖢸' => Ok(BamumSupplement::BamumLetterPhaseDashCMgbasaq),
            '𖢹' => Ok(BamumSupplement::BamumLetterPhaseDashCNteungba),
            '𖢺' => Ok(BamumSupplement::BamumLetterPhaseDashCTeuteux),
            '𖢻' => Ok(BamumSupplement::BamumLetterPhaseDashCNggum),
            '𖢼' => Ok(BamumSupplement::BamumLetterPhaseDashCFue),
            '𖢽' => Ok(BamumSupplement::BamumLetterPhaseDashCNdeut),
            '𖢾' => Ok(BamumSupplement::BamumLetterPhaseDashCNsa),
            '𖢿' => Ok(BamumSupplement::BamumLetterPhaseDashCNshaq),
            '𖣀' => Ok(BamumSupplement::BamumLetterPhaseDashCBung),
            '𖣁' => Ok(BamumSupplement::BamumLetterPhaseDashCVeuaepen),
            '𖣂' => Ok(BamumSupplement::BamumLetterPhaseDashCMberae),
            '𖣃' => Ok(BamumSupplement::BamumLetterPhaseDashCRu),
            '𖣄' => Ok(BamumSupplement::BamumLetterPhaseDashCNjaem),
            '𖣅' => Ok(BamumSupplement::BamumLetterPhaseDashCLam),
            '𖣆' => Ok(BamumSupplement::BamumLetterPhaseDashCTituaep),
            '𖣇' => Ok(BamumSupplement::BamumLetterPhaseDashCNsuotNgom),
            '𖣈' => Ok(BamumSupplement::BamumLetterPhaseDashCNjeeee),
            '𖣉' => Ok(BamumSupplement::BamumLetterPhaseDashCKet),
            '𖣊' => Ok(BamumSupplement::BamumLetterPhaseDashCNggu),
            '𖣋' => Ok(BamumSupplement::BamumLetterPhaseDashCMaesi),
            '𖣌' => Ok(BamumSupplement::BamumLetterPhaseDashCMbuaem),
            '𖣍' => Ok(BamumSupplement::BamumLetterPhaseDashCLu),
            '𖣎' => Ok(BamumSupplement::BamumLetterPhaseDashCKut),
            '𖣏' => Ok(BamumSupplement::BamumLetterPhaseDashCNjam),
            '𖣐' => Ok(BamumSupplement::BamumLetterPhaseDashCNgom),
            '𖣑' => Ok(BamumSupplement::BamumLetterPhaseDashCWup),
            '𖣒' => Ok(BamumSupplement::BamumLetterPhaseDashCNggueet),
            '𖣓' => Ok(BamumSupplement::BamumLetterPhaseDashCNsom),
            '𖣔' => Ok(BamumSupplement::BamumLetterPhaseDashCNten),
            '𖣕' => Ok(BamumSupplement::BamumLetterPhaseDashCKuopNkaarae),
            '𖣖' => Ok(BamumSupplement::BamumLetterPhaseDashCNsun),
            '𖣗' => Ok(BamumSupplement::BamumLetterPhaseDashCNdam),
            '𖣘' => Ok(BamumSupplement::BamumLetterPhaseDashCMaNsiee),
            '𖣙' => Ok(BamumSupplement::BamumLetterPhaseDashCYaa),
            '𖣚' => Ok(BamumSupplement::BamumLetterPhaseDashCNdap),
            '𖣛' => Ok(BamumSupplement::BamumLetterPhaseDashCShueq),
            '𖣜' => Ok(BamumSupplement::BamumLetterPhaseDashCSetfon),
            '𖣝' => Ok(BamumSupplement::BamumLetterPhaseDashCMbi),
            '𖣞' => Ok(BamumSupplement::BamumLetterPhaseDashCMaemba),
            '𖣟' => Ok(BamumSupplement::BamumLetterPhaseDashCMbanyi),
            '𖣠' => Ok(BamumSupplement::BamumLetterPhaseDashCKeuseux),
            '𖣡' => Ok(BamumSupplement::BamumLetterPhaseDashCMbeux),
            '𖣢' => Ok(BamumSupplement::BamumLetterPhaseDashCKeum),
            '𖣣' => Ok(BamumSupplement::BamumLetterPhaseDashCMbaaPicket),
            '𖣤' => Ok(BamumSupplement::BamumLetterPhaseDashCYuwoq),
            '𖣥' => Ok(BamumSupplement::BamumLetterPhaseDashCNjeux),
            '𖣦' => Ok(BamumSupplement::BamumLetterPhaseDashCMiee),
            '𖣧' => Ok(BamumSupplement::BamumLetterPhaseDashCMuae),
            '𖣨' => Ok(BamumSupplement::BamumLetterPhaseDashCShiq),
            '𖣩' => Ok(BamumSupplement::BamumLetterPhaseDashCKenLaw),
            '𖣪' => Ok(BamumSupplement::BamumLetterPhaseDashCKenFatigue),
            '𖣫' => Ok(BamumSupplement::BamumLetterPhaseDashCNgaq),
            '𖣬' => Ok(BamumSupplement::BamumLetterPhaseDashCNaq),
            '𖣭' => Ok(BamumSupplement::BamumLetterPhaseDashCLiq),
            '𖣮' => Ok(BamumSupplement::BamumLetterPhaseDashCPin),
            '𖣯' => Ok(BamumSupplement::BamumLetterPhaseDashCPen),
            '𖣰' => Ok(BamumSupplement::BamumLetterPhaseDashCTet),
            '𖣱' => Ok(BamumSupplement::BamumLetterPhaseDashDMbuo),
            '𖣲' => Ok(BamumSupplement::BamumLetterPhaseDashDWap),
            '𖣳' => Ok(BamumSupplement::BamumLetterPhaseDashDNji),
            '𖣴' => Ok(BamumSupplement::BamumLetterPhaseDashDMfon),
            '𖣵' => Ok(BamumSupplement::BamumLetterPhaseDashDNjiee),
            '𖣶' => Ok(BamumSupplement::BamumLetterPhaseDashDLiee),
            '𖣷' => Ok(BamumSupplement::BamumLetterPhaseDashDNjeut),
            '𖣸' => Ok(BamumSupplement::BamumLetterPhaseDashDNshee),
            '𖣹' => Ok(BamumSupplement::BamumLetterPhaseDashDNggaamae),
            '𖣺' => Ok(BamumSupplement::BamumLetterPhaseDashDNyam),
            '𖣻' => Ok(BamumSupplement::BamumLetterPhaseDashDWuaen),
            '𖣼' => Ok(BamumSupplement::BamumLetterPhaseDashDNgkun),
            '𖣽' => Ok(BamumSupplement::BamumLetterPhaseDashDShee),
            '𖣾' => Ok(BamumSupplement::BamumLetterPhaseDashDNgkap),
            '𖣿' => Ok(BamumSupplement::BamumLetterPhaseDashDKeuaetmeun),
            '𖤀' => Ok(BamumSupplement::BamumLetterPhaseDashDTeut),
            '𖤁' => Ok(BamumSupplement::BamumLetterPhaseDashDSheuae),
            '𖤂' => Ok(BamumSupplement::BamumLetterPhaseDashDNjap),
            '𖤃' => Ok(BamumSupplement::BamumLetterPhaseDashDSue),
            '𖤄' => Ok(BamumSupplement::BamumLetterPhaseDashDKet),
            '𖤅' => Ok(BamumSupplement::BamumLetterPhaseDashDYaemmae),
            '𖤆' => Ok(BamumSupplement::BamumLetterPhaseDashDKuom),
            '𖤇' => Ok(BamumSupplement::BamumLetterPhaseDashDSap),
            '𖤈' => Ok(BamumSupplement::BamumLetterPhaseDashDMfeut),
            '𖤉' => Ok(BamumSupplement::BamumLetterPhaseDashDNdeux),
            '𖤊' => Ok(BamumSupplement::BamumLetterPhaseDashDMaleeri),
            '𖤋' => Ok(BamumSupplement::BamumLetterPhaseDashDMeut),
            '𖤌' => Ok(BamumSupplement::BamumLetterPhaseDashDSeuaeq),
            '𖤍' => Ok(BamumSupplement::BamumLetterPhaseDashDYen),
            '𖤎' => Ok(BamumSupplement::BamumLetterPhaseDashDNjeuaem),
            '𖤏' => Ok(BamumSupplement::BamumLetterPhaseDashDKeuotMbuae),
            '𖤐' => Ok(BamumSupplement::BamumLetterPhaseDashDNgkeuri),
            '𖤑' => Ok(BamumSupplement::BamumLetterPhaseDashDTu),
            '𖤒' => Ok(BamumSupplement::BamumLetterPhaseDashDGhaa),
            '𖤓' => Ok(BamumSupplement::BamumLetterPhaseDashDNgkyee),
            '𖤔' => Ok(BamumSupplement::BamumLetterPhaseDashDFeufeuaet),
            '𖤕' => Ok(BamumSupplement::BamumLetterPhaseDashDNdee),
            '𖤖' => Ok(BamumSupplement::BamumLetterPhaseDashDMgbofum),
            '𖤗' => Ok(BamumSupplement::BamumLetterPhaseDashDLeuaep),
            '𖤘' => Ok(BamumSupplement::BamumLetterPhaseDashDNdon),
            '𖤙' => Ok(BamumSupplement::BamumLetterPhaseDashDMoni),
            '𖤚' => Ok(BamumSupplement::BamumLetterPhaseDashDMgbeun),
            '𖤛' => Ok(BamumSupplement::BamumLetterPhaseDashDPuut),
            '𖤜' => Ok(BamumSupplement::BamumLetterPhaseDashDMgbiee),
            '𖤝' => Ok(BamumSupplement::BamumLetterPhaseDashDMfo),
            '𖤞' => Ok(BamumSupplement::BamumLetterPhaseDashDLum),
            '𖤟' => Ok(BamumSupplement::BamumLetterPhaseDashDNsieep),
            '𖤠' => Ok(BamumSupplement::BamumLetterPhaseDashDMbaa),
            '𖤡' => Ok(BamumSupplement::BamumLetterPhaseDashDKwaet),
            '𖤢' => Ok(BamumSupplement::BamumLetterPhaseDashDNyet),
            '𖤣' => Ok(BamumSupplement::BamumLetterPhaseDashDTeuaen),
            '𖤤' => Ok(BamumSupplement::BamumLetterPhaseDashDSot),
            '𖤥' => Ok(BamumSupplement::BamumLetterPhaseDashDYuwoq),
            '𖤦' => Ok(BamumSupplement::BamumLetterPhaseDashDKeum),
            '𖤧' => Ok(BamumSupplement::BamumLetterPhaseDashDRaem),
            '𖤨' => Ok(BamumSupplement::BamumLetterPhaseDashDTeeee),
            '𖤩' => Ok(BamumSupplement::BamumLetterPhaseDashDNgkeuaeq),
            '𖤪' => Ok(BamumSupplement::BamumLetterPhaseDashDMfeuae),
            '𖤫' => Ok(BamumSupplement::BamumLetterPhaseDashDNsieet),
            '𖤬' => Ok(BamumSupplement::BamumLetterPhaseDashDKeup),
            '𖤭' => Ok(BamumSupplement::BamumLetterPhaseDashDPip),
            '𖤮' => Ok(BamumSupplement::BamumLetterPhaseDashDPeutae),
            '𖤯' => Ok(BamumSupplement::BamumLetterPhaseDashDNyue),
            '𖤰' => Ok(BamumSupplement::BamumLetterPhaseDashDLet),
            '𖤱' => Ok(BamumSupplement::BamumLetterPhaseDashDNggaam),
            '𖤲' => Ok(BamumSupplement::BamumLetterPhaseDashDMfiee),
            '𖤳' => Ok(BamumSupplement::BamumLetterPhaseDashDNggwaen),
            '𖤴' => Ok(BamumSupplement::BamumLetterPhaseDashDYuom),
            '𖤵' => Ok(BamumSupplement::BamumLetterPhaseDashDPap),
            '𖤶' => Ok(BamumSupplement::BamumLetterPhaseDashDYuop),
            '𖤷' => Ok(BamumSupplement::BamumLetterPhaseDashDNdam),
            '𖤸' => Ok(BamumSupplement::BamumLetterPhaseDashDNteum),
            '𖤹' => Ok(BamumSupplement::BamumLetterPhaseDashDSuae),
            '𖤺' => Ok(BamumSupplement::BamumLetterPhaseDashDKun),
            '𖤻' => Ok(BamumSupplement::BamumLetterPhaseDashDNggeux),
            '𖤼' => Ok(BamumSupplement::BamumLetterPhaseDashDNgkiee),
            '𖤽' => Ok(BamumSupplement::BamumLetterPhaseDashDTuot),
            '𖤾' => Ok(BamumSupplement::BamumLetterPhaseDashDMeun),
            '𖤿' => Ok(BamumSupplement::BamumLetterPhaseDashDKuq),
            '𖥀' => Ok(BamumSupplement::BamumLetterPhaseDashDNsum),
            '𖥁' => Ok(BamumSupplement::BamumLetterPhaseDashDTeun),
            '𖥂' => Ok(BamumSupplement::BamumLetterPhaseDashDMaenjet),
            '𖥃' => Ok(BamumSupplement::BamumLetterPhaseDashDNggap),
            '𖥄' => Ok(BamumSupplement::BamumLetterPhaseDashDLeum),
            '𖥅' => Ok(BamumSupplement::BamumLetterPhaseDashDNgguom),
            '𖥆' => Ok(BamumSupplement::BamumLetterPhaseDashDNshut),
            '𖥇' => Ok(BamumSupplement::BamumLetterPhaseDashDNjueq),
            '𖥈' => Ok(BamumSupplement::BamumLetterPhaseDashDGheuae),
            '𖥉' => Ok(BamumSupplement::BamumLetterPhaseDashDKu),
            '𖥊' => Ok(BamumSupplement::BamumLetterPhaseDashDRenOld),
            '𖥋' => Ok(BamumSupplement::BamumLetterPhaseDashDTae),
            '𖥌' => Ok(BamumSupplement::BamumLetterPhaseDashDToq),
            '𖥍' => Ok(BamumSupplement::BamumLetterPhaseDashDNyi),
            '𖥎' => Ok(BamumSupplement::BamumLetterPhaseDashDRii),
            '𖥏' => Ok(BamumSupplement::BamumLetterPhaseDashDLeeee),
            '𖥐' => Ok(BamumSupplement::BamumLetterPhaseDashDMeeee),
            '𖥑' => Ok(BamumSupplement::BamumLetterPhaseDashDM),
            '𖥒' => Ok(BamumSupplement::BamumLetterPhaseDashDSuu),
            '𖥓' => Ok(BamumSupplement::BamumLetterPhaseDashDMu),
            '𖥔' => Ok(BamumSupplement::BamumLetterPhaseDashDShii),
            '𖥕' => Ok(BamumSupplement::BamumLetterPhaseDashDSheux),
            '𖥖' => Ok(BamumSupplement::BamumLetterPhaseDashDKyee),
            '𖥗' => Ok(BamumSupplement::BamumLetterPhaseDashDNu),
            '𖥘' => Ok(BamumSupplement::BamumLetterPhaseDashDShu),
            '𖥙' => Ok(BamumSupplement::BamumLetterPhaseDashDNtee),
            '𖥚' => Ok(BamumSupplement::BamumLetterPhaseDashDPee),
            '𖥛' => Ok(BamumSupplement::BamumLetterPhaseDashDNi),
            '𖥜' => Ok(BamumSupplement::BamumLetterPhaseDashDShoq),
            '𖥝' => Ok(BamumSupplement::BamumLetterPhaseDashDPuq),
            '𖥞' => Ok(BamumSupplement::BamumLetterPhaseDashDMvop),
            '𖥟' => Ok(BamumSupplement::BamumLetterPhaseDashDLoq),
            '𖥠' => Ok(BamumSupplement::BamumLetterPhaseDashDRenMuch),
            '𖥡' => Ok(BamumSupplement::BamumLetterPhaseDashDTi),
            '𖥢' => Ok(BamumSupplement::BamumLetterPhaseDashDNtuu),
            '𖥣' => Ok(BamumSupplement::BamumLetterPhaseDashDMbaaSeven),
            '𖥤' => Ok(BamumSupplement::BamumLetterPhaseDashDSaq),
            '𖥥' => Ok(BamumSupplement::BamumLetterPhaseDashDFaa),
            '𖥦' => Ok(BamumSupplement::BamumLetterPhaseDashENdap),
            '𖥧' => Ok(BamumSupplement::BamumLetterPhaseDashEToon),
            '𖥨' => Ok(BamumSupplement::BamumLetterPhaseDashEMbeum),
            '𖥩' => Ok(BamumSupplement::BamumLetterPhaseDashELap),
            '𖥪' => Ok(BamumSupplement::BamumLetterPhaseDashEVom),
            '𖥫' => Ok(BamumSupplement::BamumLetterPhaseDashELoon),
            '𖥬' => Ok(BamumSupplement::BamumLetterPhaseDashEPaa),
            '𖥭' => Ok(BamumSupplement::BamumLetterPhaseDashESom),
            '𖥮' => Ok(BamumSupplement::BamumLetterPhaseDashERaq),
            '𖥯' => Ok(BamumSupplement::BamumLetterPhaseDashENshuop),
            '𖥰' => Ok(BamumSupplement::BamumLetterPhaseDashENdun),
            '𖥱' => Ok(BamumSupplement::BamumLetterPhaseDashEPuae),
            '𖥲' => Ok(BamumSupplement::BamumLetterPhaseDashETam),
            '𖥳' => Ok(BamumSupplement::BamumLetterPhaseDashENgka),
            '𖥴' => Ok(BamumSupplement::BamumLetterPhaseDashEKpeux),
            '𖥵' => Ok(BamumSupplement::BamumLetterPhaseDashEWuo),
            '𖥶' => Ok(BamumSupplement::BamumLetterPhaseDashESee),
            '𖥷' => Ok(BamumSupplement::BamumLetterPhaseDashENggeuaet),
            '𖥸' => Ok(BamumSupplement::BamumLetterPhaseDashEPaam),
            '𖥹' => Ok(BamumSupplement::BamumLetterPhaseDashEToo),
            '𖥺' => Ok(BamumSupplement::BamumLetterPhaseDashEKuop),
            '𖥻' => Ok(BamumSupplement::BamumLetterPhaseDashELom),
            '𖥼' => Ok(BamumSupplement::BamumLetterPhaseDashENshiee),
            '𖥽' => Ok(BamumSupplement::BamumLetterPhaseDashENgop),
            '𖥾' => Ok(BamumSupplement::BamumLetterPhaseDashEMaem),
            '𖥿' => Ok(BamumSupplement::BamumLetterPhaseDashENgkeux),
            '𖦀' => Ok(BamumSupplement::BamumLetterPhaseDashENgoq),
            '𖦁' => Ok(BamumSupplement::BamumLetterPhaseDashENshue),
            '𖦂' => Ok(BamumSupplement::BamumLetterPhaseDashERimgba),
            '𖦃' => Ok(BamumSupplement::BamumLetterPhaseDashENjeux),
            '𖦄' => Ok(BamumSupplement::BamumLetterPhaseDashEPeem),
            '𖦅' => Ok(BamumSupplement::BamumLetterPhaseDashESaa),
            '𖦆' => Ok(BamumSupplement::BamumLetterPhaseDashENggurae),
            '𖦇' => Ok(BamumSupplement::BamumLetterPhaseDashEMgba),
            '𖦈' => Ok(BamumSupplement::BamumLetterPhaseDashEGheux),
            '𖦉' => Ok(BamumSupplement::BamumLetterPhaseDashENgkeuaem),
            '𖦊' => Ok(BamumSupplement::BamumLetterPhaseDashENjaemli),
            '𖦋' => Ok(BamumSupplement::BamumLetterPhaseDashEMap),
            '𖦌' => Ok(BamumSupplement::BamumLetterPhaseDashELoot),
            '𖦍' => Ok(BamumSupplement::BamumLetterPhaseDashENggeeee),
            '𖦎' => Ok(BamumSupplement::BamumLetterPhaseDashENdiq),
            '𖦏' => Ok(BamumSupplement::BamumLetterPhaseDashETaenNteum),
            '𖦐' => Ok(BamumSupplement::BamumLetterPhaseDashESet),
            '𖦑' => Ok(BamumSupplement::BamumLetterPhaseDashEPum),
            '𖦒' => Ok(BamumSupplement::BamumLetterPhaseDashENdaaSoftness),
            '𖦓' => Ok(BamumSupplement::BamumLetterPhaseDashENgguaeshaeNyam),
            '𖦔' => Ok(BamumSupplement::BamumLetterPhaseDashEYiee),
            '𖦕' => Ok(BamumSupplement::BamumLetterPhaseDashEGheun),
            '𖦖' => Ok(BamumSupplement::BamumLetterPhaseDashETuae),
            '𖦗' => Ok(BamumSupplement::BamumLetterPhaseDashEYeuae),
            '𖦘' => Ok(BamumSupplement::BamumLetterPhaseDashEPo),
            '𖦙' => Ok(BamumSupplement::BamumLetterPhaseDashETumae),
            '𖦚' => Ok(BamumSupplement::BamumLetterPhaseDashEKeuae),
            '𖦛' => Ok(BamumSupplement::BamumLetterPhaseDashESuaen),
            '𖦜' => Ok(BamumSupplement::BamumLetterPhaseDashETeuaeq),
            '𖦝' => Ok(BamumSupplement::BamumLetterPhaseDashEVeuae),
            '𖦞' => Ok(BamumSupplement::BamumLetterPhaseDashEWeux),
            '𖦟' => Ok(BamumSupplement::BamumLetterPhaseDashELaam),
            '𖦠' => Ok(BamumSupplement::BamumLetterPhaseDashEPu),
            '𖦡' => Ok(BamumSupplement::BamumLetterPhaseDashETaaq),
            '𖦢' => Ok(BamumSupplement::BamumLetterPhaseDashEGhaamae),
            '𖦣' => Ok(BamumSupplement::BamumLetterPhaseDashENgeureut),
            '𖦤' => Ok(BamumSupplement::BamumLetterPhaseDashESheuaeq),
            '𖦥' => Ok(BamumSupplement::BamumLetterPhaseDashEMgben),
            '𖦦' => Ok(BamumSupplement::BamumLetterPhaseDashEMbee),
            '𖦧' => Ok(BamumSupplement::BamumLetterPhaseDashENzaq),
            '𖦨' => Ok(BamumSupplement::BamumLetterPhaseDashENkom),
            '𖦩' => Ok(BamumSupplement::BamumLetterPhaseDashEGbet),
            '𖦪' => Ok(BamumSupplement::BamumLetterPhaseDashETum),
            '𖦫' => Ok(BamumSupplement::BamumLetterPhaseDashEKuet),
            '𖦬' => Ok(BamumSupplement::BamumLetterPhaseDashEYap),
            '𖦭' => Ok(BamumSupplement::BamumLetterPhaseDashENyiCleaver),
            '𖦮' => Ok(BamumSupplement::BamumLetterPhaseDashEYit),
            '𖦯' => Ok(BamumSupplement::BamumLetterPhaseDashEMfeuq),
            '𖦰' => Ok(BamumSupplement::BamumLetterPhaseDashENdiaq),
            '𖦱' => Ok(BamumSupplement::BamumLetterPhaseDashEPieeq),
            '𖦲' => Ok(BamumSupplement::BamumLetterPhaseDashEYueq),
            '𖦳' => Ok(BamumSupplement::BamumLetterPhaseDashELeuaem),
            '𖦴' => Ok(BamumSupplement::BamumLetterPhaseDashEFue),
            '𖦵' => Ok(BamumSupplement::BamumLetterPhaseDashEGbeux),
            '𖦶' => Ok(BamumSupplement::BamumLetterPhaseDashENgkup),
            '𖦷' => Ok(BamumSupplement::BamumLetterPhaseDashEKet),
            '𖦸' => Ok(BamumSupplement::BamumLetterPhaseDashEMae),
            '𖦹' => Ok(BamumSupplement::BamumLetterPhaseDashENgkaami),
            '𖦺' => Ok(BamumSupplement::BamumLetterPhaseDashEGhet),
            '𖦻' => Ok(BamumSupplement::BamumLetterPhaseDashEFa),
            '𖦼' => Ok(BamumSupplement::BamumLetterPhaseDashENtum),
            '𖦽' => Ok(BamumSupplement::BamumLetterPhaseDashEPeut),
            '𖦾' => Ok(BamumSupplement::BamumLetterPhaseDashEYeum),
            '𖦿' => Ok(BamumSupplement::BamumLetterPhaseDashENggeuae),
            '𖧀' => Ok(BamumSupplement::BamumLetterPhaseDashENyiBetween),
            '𖧁' => Ok(BamumSupplement::BamumLetterPhaseDashENzuq),
            '𖧂' => Ok(BamumSupplement::BamumLetterPhaseDashEPoon),
            '𖧃' => Ok(BamumSupplement::BamumLetterPhaseDashEMiee),
            '𖧄' => Ok(BamumSupplement::BamumLetterPhaseDashEFuet),
            '𖧅' => Ok(BamumSupplement::BamumLetterPhaseDashENae),
            '𖧆' => Ok(BamumSupplement::BamumLetterPhaseDashEMuae),
            '𖧇' => Ok(BamumSupplement::BamumLetterPhaseDashEGheuae),
            '𖧈' => Ok(BamumSupplement::BamumLetterPhaseDashEFuI),
            '𖧉' => Ok(BamumSupplement::BamumLetterPhaseDashEMvi),
            '𖧊' => Ok(BamumSupplement::BamumLetterPhaseDashEPuaq),
            '𖧋' => Ok(BamumSupplement::BamumLetterPhaseDashENgkum),
            '𖧌' => Ok(BamumSupplement::BamumLetterPhaseDashEKut),
            '𖧍' => Ok(BamumSupplement::BamumLetterPhaseDashEPiet),
            '𖧎' => Ok(BamumSupplement::BamumLetterPhaseDashENtap),
            '𖧏' => Ok(BamumSupplement::BamumLetterPhaseDashEYeuaet),
            '𖧐' => Ok(BamumSupplement::BamumLetterPhaseDashENggup),
            '𖧑' => Ok(BamumSupplement::BamumLetterPhaseDashEPaPeople),
            '𖧒' => Ok(BamumSupplement::BamumLetterPhaseDashEFuCall),
            '𖧓' => Ok(BamumSupplement::BamumLetterPhaseDashEFom),
            '𖧔' => Ok(BamumSupplement::BamumLetterPhaseDashENjee),
            '𖧕' => Ok(BamumSupplement::BamumLetterPhaseDashEA),
            '𖧖' => Ok(BamumSupplement::BamumLetterPhaseDashEToq),
            '𖧗' => Ok(BamumSupplement::BamumLetterPhaseDashEO),
            '𖧘' => Ok(BamumSupplement::BamumLetterPhaseDashEI),
            '𖧙' => Ok(BamumSupplement::BamumLetterPhaseDashELaq),
            '𖧚' => Ok(BamumSupplement::BamumLetterPhaseDashEPaPlural),
            '𖧛' => Ok(BamumSupplement::BamumLetterPhaseDashETaa),
            '𖧜' => Ok(BamumSupplement::BamumLetterPhaseDashETaq),
            '𖧝' => Ok(BamumSupplement::BamumLetterPhaseDashENdaaMyHouse),
            '𖧞' => Ok(BamumSupplement::BamumLetterPhaseDashEShiq),
            '𖧟' => Ok(BamumSupplement::BamumLetterPhaseDashEYeux),
            '𖧠' => Ok(BamumSupplement::BamumLetterPhaseDashENguae),
            '𖧡' => Ok(BamumSupplement::BamumLetterPhaseDashEYuaen),
            '𖧢' => Ok(BamumSupplement::BamumLetterPhaseDashEYoqSwimming),
            '𖧣' => Ok(BamumSupplement::BamumLetterPhaseDashEYoqCover),
            '𖧤' => Ok(BamumSupplement::BamumLetterPhaseDashEYuq),
            '𖧥' => Ok(BamumSupplement::BamumLetterPhaseDashEYun),
            '𖧦' => Ok(BamumSupplement::BamumLetterPhaseDashEKeux),
            '𖧧' => Ok(BamumSupplement::BamumLetterPhaseDashEPeux),
            '𖧨' => Ok(BamumSupplement::BamumLetterPhaseDashENjeeEpoch),
            '𖧩' => Ok(BamumSupplement::BamumLetterPhaseDashEPue),
            '𖧪' => Ok(BamumSupplement::BamumLetterPhaseDashEWue),
            '𖧫' => Ok(BamumSupplement::BamumLetterPhaseDashEFee),
            '𖧬' => Ok(BamumSupplement::BamumLetterPhaseDashEVee),
            '𖧭' => Ok(BamumSupplement::BamumLetterPhaseDashELu),
            '𖧮' => Ok(BamumSupplement::BamumLetterPhaseDashEMi),
            '𖧯' => Ok(BamumSupplement::BamumLetterPhaseDashEReux),
            '𖧰' => Ok(BamumSupplement::BamumLetterPhaseDashERae),
            '𖧱' => Ok(BamumSupplement::BamumLetterPhaseDashENguaet),
            '𖧲' => Ok(BamumSupplement::BamumLetterPhaseDashENga),
            '𖧳' => Ok(BamumSupplement::BamumLetterPhaseDashESho),
            '𖧴' => Ok(BamumSupplement::BamumLetterPhaseDashEShoq),
            '𖧵' => Ok(BamumSupplement::BamumLetterPhaseDashEFuRemedy),
            '𖧶' => Ok(BamumSupplement::BamumLetterPhaseDashENa),
            '𖧷' => Ok(BamumSupplement::BamumLetterPhaseDashEPi),
            '𖧸' => Ok(BamumSupplement::BamumLetterPhaseDashELoq),
            '𖧹' => Ok(BamumSupplement::BamumLetterPhaseDashEKo),
            '𖧺' => Ok(BamumSupplement::BamumLetterPhaseDashEMen),
            '𖧻' => Ok(BamumSupplement::BamumLetterPhaseDashEMa),
            '𖧼' => Ok(BamumSupplement::BamumLetterPhaseDashEMaq),
            '𖧽' => Ok(BamumSupplement::BamumLetterPhaseDashETeu),
            '𖧾' => Ok(BamumSupplement::BamumLetterPhaseDashEKi),
            '𖧿' => Ok(BamumSupplement::BamumLetterPhaseDashEMon),
            '𖨀' => Ok(BamumSupplement::BamumLetterPhaseDashETen),
            '𖨁' => Ok(BamumSupplement::BamumLetterPhaseDashEFaq),
            '𖨂' => Ok(BamumSupplement::BamumLetterPhaseDashEGhom),
            '𖨃' => Ok(BamumSupplement::BamumLetterPhaseDashFKa),
            '𖨄' => Ok(BamumSupplement::BamumLetterPhaseDashFU),
            '𖨅' => Ok(BamumSupplement::BamumLetterPhaseDashFKu),
            '𖨆' => Ok(BamumSupplement::BamumLetterPhaseDashFEe),
            '𖨇' => Ok(BamumSupplement::BamumLetterPhaseDashFRee),
            '𖨈' => Ok(BamumSupplement::BamumLetterPhaseDashFTae),
            '𖨉' => Ok(BamumSupplement::BamumLetterPhaseDashFNyi),
            '𖨊' => Ok(BamumSupplement::BamumLetterPhaseDashFLa),
            '𖨋' => Ok(BamumSupplement::BamumLetterPhaseDashFRii),
            '𖨌' => Ok(BamumSupplement::BamumLetterPhaseDashFRiee),
            '𖨍' => Ok(BamumSupplement::BamumLetterPhaseDashFMeeee),
            '𖨎' => Ok(BamumSupplement::BamumLetterPhaseDashFTaa),
            '𖨏' => Ok(BamumSupplement::BamumLetterPhaseDashFNdaa),
            '𖨐' => Ok(BamumSupplement::BamumLetterPhaseDashFNjaem),
            '𖨑' => Ok(BamumSupplement::BamumLetterPhaseDashFM),
            '𖨒' => Ok(BamumSupplement::BamumLetterPhaseDashFSuu),
            '𖨓' => Ok(BamumSupplement::BamumLetterPhaseDashFShii),
            '𖨔' => Ok(BamumSupplement::BamumLetterPhaseDashFSi),
            '𖨕' => Ok(BamumSupplement::BamumLetterPhaseDashFSeux),
            '𖨖' => Ok(BamumSupplement::BamumLetterPhaseDashFKyee),
            '𖨗' => Ok(BamumSupplement::BamumLetterPhaseDashFKet),
            '𖨘' => Ok(BamumSupplement::BamumLetterPhaseDashFNuae),
            '𖨙' => Ok(BamumSupplement::BamumLetterPhaseDashFNu),
            '𖨚' => Ok(BamumSupplement::BamumLetterPhaseDashFNjuae),
            '𖨛' => Ok(BamumSupplement::BamumLetterPhaseDashFYoq),
            '𖨜' => Ok(BamumSupplement::BamumLetterPhaseDashFShu),
            '𖨝' => Ok(BamumSupplement::BamumLetterPhaseDashFYa),
            '𖨞' => Ok(BamumSupplement::BamumLetterPhaseDashFNsha),
            '𖨟' => Ok(BamumSupplement::BamumLetterPhaseDashFPeux),
            '𖨠' => Ok(BamumSupplement::BamumLetterPhaseDashFNtee),
            '𖨡' => Ok(BamumSupplement::BamumLetterPhaseDashFWue),
            '𖨢' => Ok(BamumSupplement::BamumLetterPhaseDashFPee),
            '𖨣' => Ok(BamumSupplement::BamumLetterPhaseDashFRu),
            '𖨤' => Ok(BamumSupplement::BamumLetterPhaseDashFNi),
            '𖨥' => Ok(BamumSupplement::BamumLetterPhaseDashFReux),
            '𖨦' => Ok(BamumSupplement::BamumLetterPhaseDashFKen),
            '𖨧' => Ok(BamumSupplement::BamumLetterPhaseDashFNgkwaen),
            '𖨨' => Ok(BamumSupplement::BamumLetterPhaseDashFNgga),
            '𖨩' => Ok(BamumSupplement::BamumLetterPhaseDashFSho),
            '𖨪' => Ok(BamumSupplement::BamumLetterPhaseDashFPuae),
            '𖨫' => Ok(BamumSupplement::BamumLetterPhaseDashFFom),
            '𖨬' => Ok(BamumSupplement::BamumLetterPhaseDashFWa),
            '𖨭' => Ok(BamumSupplement::BamumLetterPhaseDashFLi),
            '𖨮' => Ok(BamumSupplement::BamumLetterPhaseDashFLoq),
            '𖨯' => Ok(BamumSupplement::BamumLetterPhaseDashFKo),
            '𖨰' => Ok(BamumSupplement::BamumLetterPhaseDashFMben),
            '𖨱' => Ok(BamumSupplement::BamumLetterPhaseDashFRen),
            '𖨲' => Ok(BamumSupplement::BamumLetterPhaseDashFMa),
            '𖨳' => Ok(BamumSupplement::BamumLetterPhaseDashFMo),
            '𖨴' => Ok(BamumSupplement::BamumLetterPhaseDashFMbaa),
            '𖨵' => Ok(BamumSupplement::BamumLetterPhaseDashFTet),
            '𖨶' => Ok(BamumSupplement::BamumLetterPhaseDashFKpa),
            '𖨷' => Ok(BamumSupplement::BamumLetterPhaseDashFSamba),
            '𖨸' => Ok(BamumSupplement::BamumLetterPhaseDashFVueq),
            _ => Err(()),
        }
    }
}

impl Into<u32> for BamumSupplement {
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

impl std::convert::TryFrom<u32> for BamumSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for BamumSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl BamumSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        BamumSupplement::BamumLetterPhaseDashANgkueMfon
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BamumSupplement::BamumLetterPhaseDashANgkueMfon => "bamum letter phase-a ngkue mfon",
            BamumSupplement::BamumLetterPhaseDashAGbieeFon => "bamum letter phase-a gbiee fon",
            BamumSupplement::BamumLetterPhaseDashAPonMfonPipaemgbiee => "bamum letter phase-a pon mfon pipaemgbiee",
            BamumSupplement::BamumLetterPhaseDashAPonMfonPipaemba => "bamum letter phase-a pon mfon pipaemba",
            BamumSupplement::BamumLetterPhaseDashANaaMfon => "bamum letter phase-a naa mfon",
            BamumSupplement::BamumLetterPhaseDashAShuenshuet => "bamum letter phase-a shuenshuet",
            BamumSupplement::BamumLetterPhaseDashATitaMfon => "bamum letter phase-a tita mfon",
            BamumSupplement::BamumLetterPhaseDashANzaMfon => "bamum letter phase-a nza mfon",
            BamumSupplement::BamumLetterPhaseDashAShindaPaNji => "bamum letter phase-a shinda pa nji",
            BamumSupplement::BamumLetterPhaseDashAPonPaNjiPipaemgbiee => "bamum letter phase-a pon pa nji pipaemgbiee",
            BamumSupplement::BamumLetterPhaseDashAPonPaNjiPipaemba => "bamum letter phase-a pon pa nji pipaemba",
            BamumSupplement::BamumLetterPhaseDashAMaembgbiee => "bamum letter phase-a maembgbiee",
            BamumSupplement::BamumLetterPhaseDashATuMaemba => "bamum letter phase-a tu maemba",
            BamumSupplement::BamumLetterPhaseDashANgangu => "bamum letter phase-a ngangu",
            BamumSupplement::BamumLetterPhaseDashAMaemveux => "bamum letter phase-a maemveux",
            BamumSupplement::BamumLetterPhaseDashAMansuae => "bamum letter phase-a mansuae",
            BamumSupplement::BamumLetterPhaseDashAMveuaengam => "bamum letter phase-a mveuaengam",
            BamumSupplement::BamumLetterPhaseDashASeunyam => "bamum letter phase-a seunyam",
            BamumSupplement::BamumLetterPhaseDashANtoqpen => "bamum letter phase-a ntoqpen",
            BamumSupplement::BamumLetterPhaseDashAKeukeutnda => "bamum letter phase-a keukeutnda",
            BamumSupplement::BamumLetterPhaseDashANkindi => "bamum letter phase-a nkindi",
            BamumSupplement::BamumLetterPhaseDashASuu => "bamum letter phase-a suu",
            BamumSupplement::BamumLetterPhaseDashANgkuenzeum => "bamum letter phase-a ngkuenzeum",
            BamumSupplement::BamumLetterPhaseDashALapaq => "bamum letter phase-a lapaq",
            BamumSupplement::BamumLetterPhaseDashALetKut => "bamum letter phase-a let kut",
            BamumSupplement::BamumLetterPhaseDashANtapMfaa => "bamum letter phase-a ntap mfaa",
            BamumSupplement::BamumLetterPhaseDashAMaekeup => "bamum letter phase-a maekeup",
            BamumSupplement::BamumLetterPhaseDashAPashae => "bamum letter phase-a pashae",
            BamumSupplement::BamumLetterPhaseDashAGheuaerae => "bamum letter phase-a gheuaerae",
            BamumSupplement::BamumLetterPhaseDashAPamshae => "bamum letter phase-a pamshae",
            BamumSupplement::BamumLetterPhaseDashAMonNggeuaet => "bamum letter phase-a mon nggeuaet",
            BamumSupplement::BamumLetterPhaseDashANzunMeut => "bamum letter phase-a nzun meut",
            BamumSupplement::BamumLetterPhaseDashAUYuqNae => "bamum letter phase-a u yuq nae",
            BamumSupplement::BamumLetterPhaseDashAGheuaegheuae => "bamum letter phase-a gheuaegheuae",
            BamumSupplement::BamumLetterPhaseDashANtapNtaa => "bamum letter phase-a ntap ntaa",
            BamumSupplement::BamumLetterPhaseDashASisa => "bamum letter phase-a sisa",
            BamumSupplement::BamumLetterPhaseDashAMgbasa => "bamum letter phase-a mgbasa",
            BamumSupplement::BamumLetterPhaseDashAMeunjomndeuq => "bamum letter phase-a meunjomndeuq",
            BamumSupplement::BamumLetterPhaseDashAMoompuq => "bamum letter phase-a moompuq",
            BamumSupplement::BamumLetterPhaseDashAKafa => "bamum letter phase-a kafa",
            BamumSupplement::BamumLetterPhaseDashAPaLeeraewa => "bamum letter phase-a pa leeraewa",
            BamumSupplement::BamumLetterPhaseDashANdaLeeraewa => "bamum letter phase-a nda leeraewa",
            BamumSupplement::BamumLetterPhaseDashAPet => "bamum letter phase-a pet",
            BamumSupplement::BamumLetterPhaseDashAMaemkpen => "bamum letter phase-a maemkpen",
            BamumSupplement::BamumLetterPhaseDashANika => "bamum letter phase-a nika",
            BamumSupplement::BamumLetterPhaseDashAPup => "bamum letter phase-a pup",
            BamumSupplement::BamumLetterPhaseDashATuaep => "bamum letter phase-a tuaep",
            BamumSupplement::BamumLetterPhaseDashALuaep => "bamum letter phase-a luaep",
            BamumSupplement::BamumLetterPhaseDashASonjam => "bamum letter phase-a sonjam",
            BamumSupplement::BamumLetterPhaseDashATeuteuwen => "bamum letter phase-a teuteuwen",
            BamumSupplement::BamumLetterPhaseDashAMaenyi => "bamum letter phase-a maenyi",
            BamumSupplement::BamumLetterPhaseDashAKet => "bamum letter phase-a ket",
            BamumSupplement::BamumLetterPhaseDashANdaanggeuaet => "bamum letter phase-a ndaanggeuaet",
            BamumSupplement::BamumLetterPhaseDashAKuoq => "bamum letter phase-a kuoq",
            BamumSupplement::BamumLetterPhaseDashAMoomeut => "bamum letter phase-a moomeut",
            BamumSupplement::BamumLetterPhaseDashAShum => "bamum letter phase-a shum",
            BamumSupplement::BamumLetterPhaseDashALommae => "bamum letter phase-a lommae",
            BamumSupplement::BamumLetterPhaseDashAFiri => "bamum letter phase-a firi",
            BamumSupplement::BamumLetterPhaseDashARom => "bamum letter phase-a rom",
            BamumSupplement::BamumLetterPhaseDashAKpoq => "bamum letter phase-a kpoq",
            BamumSupplement::BamumLetterPhaseDashASoq => "bamum letter phase-a soq",
            BamumSupplement::BamumLetterPhaseDashAMapPieet => "bamum letter phase-a map pieet",
            BamumSupplement::BamumLetterPhaseDashAShirae => "bamum letter phase-a shirae",
            BamumSupplement::BamumLetterPhaseDashANtap => "bamum letter phase-a ntap",
            BamumSupplement::BamumLetterPhaseDashAShoqNshutYum => "bamum letter phase-a shoq nshut yum",
            BamumSupplement::BamumLetterPhaseDashANyitMongkeuaeq => "bamum letter phase-a nyit mongkeuaeq",
            BamumSupplement::BamumLetterPhaseDashAPaarae => "bamum letter phase-a paarae",
            BamumSupplement::BamumLetterPhaseDashANkaarae => "bamum letter phase-a nkaarae",
            BamumSupplement::BamumLetterPhaseDashAUnknown => "bamum letter phase-a unknown",
            BamumSupplement::BamumLetterPhaseDashANggen => "bamum letter phase-a nggen",
            BamumSupplement::BamumLetterPhaseDashAMaesi => "bamum letter phase-a maesi",
            BamumSupplement::BamumLetterPhaseDashANjam => "bamum letter phase-a njam",
            BamumSupplement::BamumLetterPhaseDashAMbanyi => "bamum letter phase-a mbanyi",
            BamumSupplement::BamumLetterPhaseDashANyet => "bamum letter phase-a nyet",
            BamumSupplement::BamumLetterPhaseDashATeuaen => "bamum letter phase-a teuaen",
            BamumSupplement::BamumLetterPhaseDashASot => "bamum letter phase-a sot",
            BamumSupplement::BamumLetterPhaseDashAPaam => "bamum letter phase-a paam",
            BamumSupplement::BamumLetterPhaseDashANshiee => "bamum letter phase-a nshiee",
            BamumSupplement::BamumLetterPhaseDashAMaem => "bamum letter phase-a maem",
            BamumSupplement::BamumLetterPhaseDashANyi => "bamum letter phase-a nyi",
            BamumSupplement::BamumLetterPhaseDashAKaq => "bamum letter phase-a kaq",
            BamumSupplement::BamumLetterPhaseDashANsha => "bamum letter phase-a nsha",
            BamumSupplement::BamumLetterPhaseDashAVee => "bamum letter phase-a vee",
            BamumSupplement::BamumLetterPhaseDashALu => "bamum letter phase-a lu",
            BamumSupplement::BamumLetterPhaseDashANen => "bamum letter phase-a nen",
            BamumSupplement::BamumLetterPhaseDashANaq => "bamum letter phase-a naq",
            BamumSupplement::BamumLetterPhaseDashAMbaq => "bamum letter phase-a mbaq",
            BamumSupplement::BamumLetterPhaseDashBNshuet => "bamum letter phase-b nshuet",
            BamumSupplement::BamumLetterPhaseDashBTuMaemgbiee => "bamum letter phase-b tu maemgbiee",
            BamumSupplement::BamumLetterPhaseDashBSiee => "bamum letter phase-b siee",
            BamumSupplement::BamumLetterPhaseDashBSetTu => "bamum letter phase-b set tu",
            BamumSupplement::BamumLetterPhaseDashBLomNteum => "bamum letter phase-b lom nteum",
            BamumSupplement::BamumLetterPhaseDashBMbaMaelee => "bamum letter phase-b mba maelee",
            BamumSupplement::BamumLetterPhaseDashBKieem => "bamum letter phase-b kieem",
            BamumSupplement::BamumLetterPhaseDashBYeurae => "bamum letter phase-b yeurae",
            BamumSupplement::BamumLetterPhaseDashBMbaarae => "bamum letter phase-b mbaarae",
            BamumSupplement::BamumLetterPhaseDashBKam => "bamum letter phase-b kam",
            BamumSupplement::BamumLetterPhaseDashBPeeshi => "bamum letter phase-b peeshi",
            BamumSupplement::BamumLetterPhaseDashBYafuLeeraewa => "bamum letter phase-b yafu leeraewa",
            BamumSupplement::BamumLetterPhaseDashBLamNshutNyam => "bamum letter phase-b lam nshut nyam",
            BamumSupplement::BamumLetterPhaseDashBNtieeSheuoq => "bamum letter phase-b ntiee sheuoq",
            BamumSupplement::BamumLetterPhaseDashBNduNjaa => "bamum letter phase-b ndu njaa",
            BamumSupplement::BamumLetterPhaseDashBGheugheuaem => "bamum letter phase-b gheugheuaem",
            BamumSupplement::BamumLetterPhaseDashBPit => "bamum letter phase-b pit",
            BamumSupplement::BamumLetterPhaseDashBTuNsiee => "bamum letter phase-b tu nsiee",
            BamumSupplement::BamumLetterPhaseDashBShetNjaq => "bamum letter phase-b shet njaq",
            BamumSupplement::BamumLetterPhaseDashBSheuaeqtu => "bamum letter phase-b sheuaeqtu",
            BamumSupplement::BamumLetterPhaseDashBMfonTeuaeq => "bamum letter phase-b mfon teuaeq",
            BamumSupplement::BamumLetterPhaseDashBMbitMbaaket => "bamum letter phase-b mbit mbaaket",
            BamumSupplement::BamumLetterPhaseDashBNyiNteum => "bamum letter phase-b nyi nteum",
            BamumSupplement::BamumLetterPhaseDashBKeupuq => "bamum letter phase-b keupuq",
            BamumSupplement::BamumLetterPhaseDashBGheughen => "bamum letter phase-b gheughen",
            BamumSupplement::BamumLetterPhaseDashBKeuyeux => "bamum letter phase-b keuyeux",
            BamumSupplement::BamumLetterPhaseDashBLaanae => "bamum letter phase-b laanae",
            BamumSupplement::BamumLetterPhaseDashBParum => "bamum letter phase-b parum",
            BamumSupplement::BamumLetterPhaseDashBVeum => "bamum letter phase-b veum",
            BamumSupplement::BamumLetterPhaseDashBNgkindiMvop => "bamum letter phase-b ngkindi mvop",
            BamumSupplement::BamumLetterPhaseDashBNggeuMbu => "bamum letter phase-b nggeu mbu",
            BamumSupplement::BamumLetterPhaseDashBWuaet => "bamum letter phase-b wuaet",
            BamumSupplement::BamumLetterPhaseDashBSakeuae => "bamum letter phase-b sakeuae",
            BamumSupplement::BamumLetterPhaseDashBTaam => "bamum letter phase-b taam",
            BamumSupplement::BamumLetterPhaseDashBMeuq => "bamum letter phase-b meuq",
            BamumSupplement::BamumLetterPhaseDashBNgguoq => "bamum letter phase-b ngguoq",
            BamumSupplement::BamumLetterPhaseDashBNgguoqLarge => "bamum letter phase-b ngguoq large",
            BamumSupplement::BamumLetterPhaseDashBMfiyaq => "bamum letter phase-b mfiyaq",
            BamumSupplement::BamumLetterPhaseDashBSue => "bamum letter phase-b sue",
            BamumSupplement::BamumLetterPhaseDashBMbeuri => "bamum letter phase-b mbeuri",
            BamumSupplement::BamumLetterPhaseDashBMontieen => "bamum letter phase-b montieen",
            BamumSupplement::BamumLetterPhaseDashBNyaemae => "bamum letter phase-b nyaemae",
            BamumSupplement::BamumLetterPhaseDashBPungaam => "bamum letter phase-b pungaam",
            BamumSupplement::BamumLetterPhaseDashBMeutNggeet => "bamum letter phase-b meut nggeet",
            BamumSupplement::BamumLetterPhaseDashBFeux => "bamum letter phase-b feux",
            BamumSupplement::BamumLetterPhaseDashBMbuoq => "bamum letter phase-b mbuoq",
            BamumSupplement::BamumLetterPhaseDashBFee => "bamum letter phase-b fee",
            BamumSupplement::BamumLetterPhaseDashBKeuaem => "bamum letter phase-b keuaem",
            BamumSupplement::BamumLetterPhaseDashBMaNjeuaena => "bamum letter phase-b ma njeuaena",
            BamumSupplement::BamumLetterPhaseDashBMaNjuqa => "bamum letter phase-b ma njuqa",
            BamumSupplement::BamumLetterPhaseDashBLet => "bamum letter phase-b let",
            BamumSupplement::BamumLetterPhaseDashBNggaam => "bamum letter phase-b nggaam",
            BamumSupplement::BamumLetterPhaseDashBNsen => "bamum letter phase-b nsen",
            BamumSupplement::BamumLetterPhaseDashBMa => "bamum letter phase-b ma",
            BamumSupplement::BamumLetterPhaseDashBKiq => "bamum letter phase-b kiq",
            BamumSupplement::BamumLetterPhaseDashBNgom => "bamum letter phase-b ngom",
            BamumSupplement::BamumLetterPhaseDashCNgkueMaemba => "bamum letter phase-c ngkue maemba",
            BamumSupplement::BamumLetterPhaseDashCNza => "bamum letter phase-c nza",
            BamumSupplement::BamumLetterPhaseDashCYum => "bamum letter phase-c yum",
            BamumSupplement::BamumLetterPhaseDashCWangkuoq => "bamum letter phase-c wangkuoq",
            BamumSupplement::BamumLetterPhaseDashCNggen => "bamum letter phase-c nggen",
            BamumSupplement::BamumLetterPhaseDashCNdeuaeree => "bamum letter phase-c ndeuaeree",
            BamumSupplement::BamumLetterPhaseDashCNgkaq => "bamum letter phase-c ngkaq",
            BamumSupplement::BamumLetterPhaseDashCGharae => "bamum letter phase-c gharae",
            BamumSupplement::BamumLetterPhaseDashCMbeekeet => "bamum letter phase-c mbeekeet",
            BamumSupplement::BamumLetterPhaseDashCGbayi => "bamum letter phase-c gbayi",
            BamumSupplement::BamumLetterPhaseDashCNyirMkparaqMeun => "bamum letter phase-c nyir mkparaq meun",
            BamumSupplement::BamumLetterPhaseDashCNtuMbit => "bamum letter phase-c ntu mbit",
            BamumSupplement::BamumLetterPhaseDashCMbeum => "bamum letter phase-c mbeum",
            BamumSupplement::BamumLetterPhaseDashCPirieen => "bamum letter phase-c pirieen",
            BamumSupplement::BamumLetterPhaseDashCNdombu => "bamum letter phase-c ndombu",
            BamumSupplement::BamumLetterPhaseDashCMbaaCabbageDashTree => "bamum letter phase-c mbaa cabbage-tree",
            BamumSupplement::BamumLetterPhaseDashCKeusheuaep => "bamum letter phase-c keusheuaep",
            BamumSupplement::BamumLetterPhaseDashCGhap => "bamum letter phase-c ghap",
            BamumSupplement::BamumLetterPhaseDashCKeukaq => "bamum letter phase-c keukaq",
            BamumSupplement::BamumLetterPhaseDashCYuMuomae => "bamum letter phase-c yu muomae",
            BamumSupplement::BamumLetterPhaseDashCNzeum => "bamum letter phase-c nzeum",
            BamumSupplement::BamumLetterPhaseDashCMbue => "bamum letter phase-c mbue",
            BamumSupplement::BamumLetterPhaseDashCNseuaen => "bamum letter phase-c nseuaen",
            BamumSupplement::BamumLetterPhaseDashCMbit => "bamum letter phase-c mbit",
            BamumSupplement::BamumLetterPhaseDashCYeuq => "bamum letter phase-c yeuq",
            BamumSupplement::BamumLetterPhaseDashCKparaq => "bamum letter phase-c kparaq",
            BamumSupplement::BamumLetterPhaseDashCKaa => "bamum letter phase-c kaa",
            BamumSupplement::BamumLetterPhaseDashCSeux => "bamum letter phase-c seux",
            BamumSupplement::BamumLetterPhaseDashCNdida => "bamum letter phase-c ndida",
            BamumSupplement::BamumLetterPhaseDashCTaashae => "bamum letter phase-c taashae",
            BamumSupplement::BamumLetterPhaseDashCNjueq => "bamum letter phase-c njueq",
            BamumSupplement::BamumLetterPhaseDashCTitaYue => "bamum letter phase-c tita yue",
            BamumSupplement::BamumLetterPhaseDashCSuaet => "bamum letter phase-c suaet",
            BamumSupplement::BamumLetterPhaseDashCNgguaenNyam => "bamum letter phase-c ngguaen nyam",
            BamumSupplement::BamumLetterPhaseDashCVeux => "bamum letter phase-c veux",
            BamumSupplement::BamumLetterPhaseDashCNansanaq => "bamum letter phase-c nansanaq",
            BamumSupplement::BamumLetterPhaseDashCMaKeuaeri => "bamum letter phase-c ma keuaeri",
            BamumSupplement::BamumLetterPhaseDashCNtaa => "bamum letter phase-c ntaa",
            BamumSupplement::BamumLetterPhaseDashCNgguon => "bamum letter phase-c ngguon",
            BamumSupplement::BamumLetterPhaseDashCLap => "bamum letter phase-c lap",
            BamumSupplement::BamumLetterPhaseDashCMbirieen => "bamum letter phase-c mbirieen",
            BamumSupplement::BamumLetterPhaseDashCMgbasaq => "bamum letter phase-c mgbasaq",
            BamumSupplement::BamumLetterPhaseDashCNteungba => "bamum letter phase-c nteungba",
            BamumSupplement::BamumLetterPhaseDashCTeuteux => "bamum letter phase-c teuteux",
            BamumSupplement::BamumLetterPhaseDashCNggum => "bamum letter phase-c nggum",
            BamumSupplement::BamumLetterPhaseDashCFue => "bamum letter phase-c fue",
            BamumSupplement::BamumLetterPhaseDashCNdeut => "bamum letter phase-c ndeut",
            BamumSupplement::BamumLetterPhaseDashCNsa => "bamum letter phase-c nsa",
            BamumSupplement::BamumLetterPhaseDashCNshaq => "bamum letter phase-c nshaq",
            BamumSupplement::BamumLetterPhaseDashCBung => "bamum letter phase-c bung",
            BamumSupplement::BamumLetterPhaseDashCVeuaepen => "bamum letter phase-c veuaepen",
            BamumSupplement::BamumLetterPhaseDashCMberae => "bamum letter phase-c mberae",
            BamumSupplement::BamumLetterPhaseDashCRu => "bamum letter phase-c ru",
            BamumSupplement::BamumLetterPhaseDashCNjaem => "bamum letter phase-c njaem",
            BamumSupplement::BamumLetterPhaseDashCLam => "bamum letter phase-c lam",
            BamumSupplement::BamumLetterPhaseDashCTituaep => "bamum letter phase-c tituaep",
            BamumSupplement::BamumLetterPhaseDashCNsuotNgom => "bamum letter phase-c nsuot ngom",
            BamumSupplement::BamumLetterPhaseDashCNjeeee => "bamum letter phase-c njeeee",
            BamumSupplement::BamumLetterPhaseDashCKet => "bamum letter phase-c ket",
            BamumSupplement::BamumLetterPhaseDashCNggu => "bamum letter phase-c nggu",
            BamumSupplement::BamumLetterPhaseDashCMaesi => "bamum letter phase-c maesi",
            BamumSupplement::BamumLetterPhaseDashCMbuaem => "bamum letter phase-c mbuaem",
            BamumSupplement::BamumLetterPhaseDashCLu => "bamum letter phase-c lu",
            BamumSupplement::BamumLetterPhaseDashCKut => "bamum letter phase-c kut",
            BamumSupplement::BamumLetterPhaseDashCNjam => "bamum letter phase-c njam",
            BamumSupplement::BamumLetterPhaseDashCNgom => "bamum letter phase-c ngom",
            BamumSupplement::BamumLetterPhaseDashCWup => "bamum letter phase-c wup",
            BamumSupplement::BamumLetterPhaseDashCNggueet => "bamum letter phase-c nggueet",
            BamumSupplement::BamumLetterPhaseDashCNsom => "bamum letter phase-c nsom",
            BamumSupplement::BamumLetterPhaseDashCNten => "bamum letter phase-c nten",
            BamumSupplement::BamumLetterPhaseDashCKuopNkaarae => "bamum letter phase-c kuop nkaarae",
            BamumSupplement::BamumLetterPhaseDashCNsun => "bamum letter phase-c nsun",
            BamumSupplement::BamumLetterPhaseDashCNdam => "bamum letter phase-c ndam",
            BamumSupplement::BamumLetterPhaseDashCMaNsiee => "bamum letter phase-c ma nsiee",
            BamumSupplement::BamumLetterPhaseDashCYaa => "bamum letter phase-c yaa",
            BamumSupplement::BamumLetterPhaseDashCNdap => "bamum letter phase-c ndap",
            BamumSupplement::BamumLetterPhaseDashCShueq => "bamum letter phase-c shueq",
            BamumSupplement::BamumLetterPhaseDashCSetfon => "bamum letter phase-c setfon",
            BamumSupplement::BamumLetterPhaseDashCMbi => "bamum letter phase-c mbi",
            BamumSupplement::BamumLetterPhaseDashCMaemba => "bamum letter phase-c maemba",
            BamumSupplement::BamumLetterPhaseDashCMbanyi => "bamum letter phase-c mbanyi",
            BamumSupplement::BamumLetterPhaseDashCKeuseux => "bamum letter phase-c keuseux",
            BamumSupplement::BamumLetterPhaseDashCMbeux => "bamum letter phase-c mbeux",
            BamumSupplement::BamumLetterPhaseDashCKeum => "bamum letter phase-c keum",
            BamumSupplement::BamumLetterPhaseDashCMbaaPicket => "bamum letter phase-c mbaa picket",
            BamumSupplement::BamumLetterPhaseDashCYuwoq => "bamum letter phase-c yuwoq",
            BamumSupplement::BamumLetterPhaseDashCNjeux => "bamum letter phase-c njeux",
            BamumSupplement::BamumLetterPhaseDashCMiee => "bamum letter phase-c miee",
            BamumSupplement::BamumLetterPhaseDashCMuae => "bamum letter phase-c muae",
            BamumSupplement::BamumLetterPhaseDashCShiq => "bamum letter phase-c shiq",
            BamumSupplement::BamumLetterPhaseDashCKenLaw => "bamum letter phase-c ken law",
            BamumSupplement::BamumLetterPhaseDashCKenFatigue => "bamum letter phase-c ken fatigue",
            BamumSupplement::BamumLetterPhaseDashCNgaq => "bamum letter phase-c ngaq",
            BamumSupplement::BamumLetterPhaseDashCNaq => "bamum letter phase-c naq",
            BamumSupplement::BamumLetterPhaseDashCLiq => "bamum letter phase-c liq",
            BamumSupplement::BamumLetterPhaseDashCPin => "bamum letter phase-c pin",
            BamumSupplement::BamumLetterPhaseDashCPen => "bamum letter phase-c pen",
            BamumSupplement::BamumLetterPhaseDashCTet => "bamum letter phase-c tet",
            BamumSupplement::BamumLetterPhaseDashDMbuo => "bamum letter phase-d mbuo",
            BamumSupplement::BamumLetterPhaseDashDWap => "bamum letter phase-d wap",
            BamumSupplement::BamumLetterPhaseDashDNji => "bamum letter phase-d nji",
            BamumSupplement::BamumLetterPhaseDashDMfon => "bamum letter phase-d mfon",
            BamumSupplement::BamumLetterPhaseDashDNjiee => "bamum letter phase-d njiee",
            BamumSupplement::BamumLetterPhaseDashDLiee => "bamum letter phase-d liee",
            BamumSupplement::BamumLetterPhaseDashDNjeut => "bamum letter phase-d njeut",
            BamumSupplement::BamumLetterPhaseDashDNshee => "bamum letter phase-d nshee",
            BamumSupplement::BamumLetterPhaseDashDNggaamae => "bamum letter phase-d nggaamae",
            BamumSupplement::BamumLetterPhaseDashDNyam => "bamum letter phase-d nyam",
            BamumSupplement::BamumLetterPhaseDashDWuaen => "bamum letter phase-d wuaen",
            BamumSupplement::BamumLetterPhaseDashDNgkun => "bamum letter phase-d ngkun",
            BamumSupplement::BamumLetterPhaseDashDShee => "bamum letter phase-d shee",
            BamumSupplement::BamumLetterPhaseDashDNgkap => "bamum letter phase-d ngkap",
            BamumSupplement::BamumLetterPhaseDashDKeuaetmeun => "bamum letter phase-d keuaetmeun",
            BamumSupplement::BamumLetterPhaseDashDTeut => "bamum letter phase-d teut",
            BamumSupplement::BamumLetterPhaseDashDSheuae => "bamum letter phase-d sheuae",
            BamumSupplement::BamumLetterPhaseDashDNjap => "bamum letter phase-d njap",
            BamumSupplement::BamumLetterPhaseDashDSue => "bamum letter phase-d sue",
            BamumSupplement::BamumLetterPhaseDashDKet => "bamum letter phase-d ket",
            BamumSupplement::BamumLetterPhaseDashDYaemmae => "bamum letter phase-d yaemmae",
            BamumSupplement::BamumLetterPhaseDashDKuom => "bamum letter phase-d kuom",
            BamumSupplement::BamumLetterPhaseDashDSap => "bamum letter phase-d sap",
            BamumSupplement::BamumLetterPhaseDashDMfeut => "bamum letter phase-d mfeut",
            BamumSupplement::BamumLetterPhaseDashDNdeux => "bamum letter phase-d ndeux",
            BamumSupplement::BamumLetterPhaseDashDMaleeri => "bamum letter phase-d maleeri",
            BamumSupplement::BamumLetterPhaseDashDMeut => "bamum letter phase-d meut",
            BamumSupplement::BamumLetterPhaseDashDSeuaeq => "bamum letter phase-d seuaeq",
            BamumSupplement::BamumLetterPhaseDashDYen => "bamum letter phase-d yen",
            BamumSupplement::BamumLetterPhaseDashDNjeuaem => "bamum letter phase-d njeuaem",
            BamumSupplement::BamumLetterPhaseDashDKeuotMbuae => "bamum letter phase-d keuot mbuae",
            BamumSupplement::BamumLetterPhaseDashDNgkeuri => "bamum letter phase-d ngkeuri",
            BamumSupplement::BamumLetterPhaseDashDTu => "bamum letter phase-d tu",
            BamumSupplement::BamumLetterPhaseDashDGhaa => "bamum letter phase-d ghaa",
            BamumSupplement::BamumLetterPhaseDashDNgkyee => "bamum letter phase-d ngkyee",
            BamumSupplement::BamumLetterPhaseDashDFeufeuaet => "bamum letter phase-d feufeuaet",
            BamumSupplement::BamumLetterPhaseDashDNdee => "bamum letter phase-d ndee",
            BamumSupplement::BamumLetterPhaseDashDMgbofum => "bamum letter phase-d mgbofum",
            BamumSupplement::BamumLetterPhaseDashDLeuaep => "bamum letter phase-d leuaep",
            BamumSupplement::BamumLetterPhaseDashDNdon => "bamum letter phase-d ndon",
            BamumSupplement::BamumLetterPhaseDashDMoni => "bamum letter phase-d moni",
            BamumSupplement::BamumLetterPhaseDashDMgbeun => "bamum letter phase-d mgbeun",
            BamumSupplement::BamumLetterPhaseDashDPuut => "bamum letter phase-d puut",
            BamumSupplement::BamumLetterPhaseDashDMgbiee => "bamum letter phase-d mgbiee",
            BamumSupplement::BamumLetterPhaseDashDMfo => "bamum letter phase-d mfo",
            BamumSupplement::BamumLetterPhaseDashDLum => "bamum letter phase-d lum",
            BamumSupplement::BamumLetterPhaseDashDNsieep => "bamum letter phase-d nsieep",
            BamumSupplement::BamumLetterPhaseDashDMbaa => "bamum letter phase-d mbaa",
            BamumSupplement::BamumLetterPhaseDashDKwaet => "bamum letter phase-d kwaet",
            BamumSupplement::BamumLetterPhaseDashDNyet => "bamum letter phase-d nyet",
            BamumSupplement::BamumLetterPhaseDashDTeuaen => "bamum letter phase-d teuaen",
            BamumSupplement::BamumLetterPhaseDashDSot => "bamum letter phase-d sot",
            BamumSupplement::BamumLetterPhaseDashDYuwoq => "bamum letter phase-d yuwoq",
            BamumSupplement::BamumLetterPhaseDashDKeum => "bamum letter phase-d keum",
            BamumSupplement::BamumLetterPhaseDashDRaem => "bamum letter phase-d raem",
            BamumSupplement::BamumLetterPhaseDashDTeeee => "bamum letter phase-d teeee",
            BamumSupplement::BamumLetterPhaseDashDNgkeuaeq => "bamum letter phase-d ngkeuaeq",
            BamumSupplement::BamumLetterPhaseDashDMfeuae => "bamum letter phase-d mfeuae",
            BamumSupplement::BamumLetterPhaseDashDNsieet => "bamum letter phase-d nsieet",
            BamumSupplement::BamumLetterPhaseDashDKeup => "bamum letter phase-d keup",
            BamumSupplement::BamumLetterPhaseDashDPip => "bamum letter phase-d pip",
            BamumSupplement::BamumLetterPhaseDashDPeutae => "bamum letter phase-d peutae",
            BamumSupplement::BamumLetterPhaseDashDNyue => "bamum letter phase-d nyue",
            BamumSupplement::BamumLetterPhaseDashDLet => "bamum letter phase-d let",
            BamumSupplement::BamumLetterPhaseDashDNggaam => "bamum letter phase-d nggaam",
            BamumSupplement::BamumLetterPhaseDashDMfiee => "bamum letter phase-d mfiee",
            BamumSupplement::BamumLetterPhaseDashDNggwaen => "bamum letter phase-d nggwaen",
            BamumSupplement::BamumLetterPhaseDashDYuom => "bamum letter phase-d yuom",
            BamumSupplement::BamumLetterPhaseDashDPap => "bamum letter phase-d pap",
            BamumSupplement::BamumLetterPhaseDashDYuop => "bamum letter phase-d yuop",
            BamumSupplement::BamumLetterPhaseDashDNdam => "bamum letter phase-d ndam",
            BamumSupplement::BamumLetterPhaseDashDNteum => "bamum letter phase-d nteum",
            BamumSupplement::BamumLetterPhaseDashDSuae => "bamum letter phase-d suae",
            BamumSupplement::BamumLetterPhaseDashDKun => "bamum letter phase-d kun",
            BamumSupplement::BamumLetterPhaseDashDNggeux => "bamum letter phase-d nggeux",
            BamumSupplement::BamumLetterPhaseDashDNgkiee => "bamum letter phase-d ngkiee",
            BamumSupplement::BamumLetterPhaseDashDTuot => "bamum letter phase-d tuot",
            BamumSupplement::BamumLetterPhaseDashDMeun => "bamum letter phase-d meun",
            BamumSupplement::BamumLetterPhaseDashDKuq => "bamum letter phase-d kuq",
            BamumSupplement::BamumLetterPhaseDashDNsum => "bamum letter phase-d nsum",
            BamumSupplement::BamumLetterPhaseDashDTeun => "bamum letter phase-d teun",
            BamumSupplement::BamumLetterPhaseDashDMaenjet => "bamum letter phase-d maenjet",
            BamumSupplement::BamumLetterPhaseDashDNggap => "bamum letter phase-d nggap",
            BamumSupplement::BamumLetterPhaseDashDLeum => "bamum letter phase-d leum",
            BamumSupplement::BamumLetterPhaseDashDNgguom => "bamum letter phase-d ngguom",
            BamumSupplement::BamumLetterPhaseDashDNshut => "bamum letter phase-d nshut",
            BamumSupplement::BamumLetterPhaseDashDNjueq => "bamum letter phase-d njueq",
            BamumSupplement::BamumLetterPhaseDashDGheuae => "bamum letter phase-d gheuae",
            BamumSupplement::BamumLetterPhaseDashDKu => "bamum letter phase-d ku",
            BamumSupplement::BamumLetterPhaseDashDRenOld => "bamum letter phase-d ren old",
            BamumSupplement::BamumLetterPhaseDashDTae => "bamum letter phase-d tae",
            BamumSupplement::BamumLetterPhaseDashDToq => "bamum letter phase-d toq",
            BamumSupplement::BamumLetterPhaseDashDNyi => "bamum letter phase-d nyi",
            BamumSupplement::BamumLetterPhaseDashDRii => "bamum letter phase-d rii",
            BamumSupplement::BamumLetterPhaseDashDLeeee => "bamum letter phase-d leeee",
            BamumSupplement::BamumLetterPhaseDashDMeeee => "bamum letter phase-d meeee",
            BamumSupplement::BamumLetterPhaseDashDM => "bamum letter phase-d m",
            BamumSupplement::BamumLetterPhaseDashDSuu => "bamum letter phase-d suu",
            BamumSupplement::BamumLetterPhaseDashDMu => "bamum letter phase-d mu",
            BamumSupplement::BamumLetterPhaseDashDShii => "bamum letter phase-d shii",
            BamumSupplement::BamumLetterPhaseDashDSheux => "bamum letter phase-d sheux",
            BamumSupplement::BamumLetterPhaseDashDKyee => "bamum letter phase-d kyee",
            BamumSupplement::BamumLetterPhaseDashDNu => "bamum letter phase-d nu",
            BamumSupplement::BamumLetterPhaseDashDShu => "bamum letter phase-d shu",
            BamumSupplement::BamumLetterPhaseDashDNtee => "bamum letter phase-d ntee",
            BamumSupplement::BamumLetterPhaseDashDPee => "bamum letter phase-d pee",
            BamumSupplement::BamumLetterPhaseDashDNi => "bamum letter phase-d ni",
            BamumSupplement::BamumLetterPhaseDashDShoq => "bamum letter phase-d shoq",
            BamumSupplement::BamumLetterPhaseDashDPuq => "bamum letter phase-d puq",
            BamumSupplement::BamumLetterPhaseDashDMvop => "bamum letter phase-d mvop",
            BamumSupplement::BamumLetterPhaseDashDLoq => "bamum letter phase-d loq",
            BamumSupplement::BamumLetterPhaseDashDRenMuch => "bamum letter phase-d ren much",
            BamumSupplement::BamumLetterPhaseDashDTi => "bamum letter phase-d ti",
            BamumSupplement::BamumLetterPhaseDashDNtuu => "bamum letter phase-d ntuu",
            BamumSupplement::BamumLetterPhaseDashDMbaaSeven => "bamum letter phase-d mbaa seven",
            BamumSupplement::BamumLetterPhaseDashDSaq => "bamum letter phase-d saq",
            BamumSupplement::BamumLetterPhaseDashDFaa => "bamum letter phase-d faa",
            BamumSupplement::BamumLetterPhaseDashENdap => "bamum letter phase-e ndap",
            BamumSupplement::BamumLetterPhaseDashEToon => "bamum letter phase-e toon",
            BamumSupplement::BamumLetterPhaseDashEMbeum => "bamum letter phase-e mbeum",
            BamumSupplement::BamumLetterPhaseDashELap => "bamum letter phase-e lap",
            BamumSupplement::BamumLetterPhaseDashEVom => "bamum letter phase-e vom",
            BamumSupplement::BamumLetterPhaseDashELoon => "bamum letter phase-e loon",
            BamumSupplement::BamumLetterPhaseDashEPaa => "bamum letter phase-e paa",
            BamumSupplement::BamumLetterPhaseDashESom => "bamum letter phase-e som",
            BamumSupplement::BamumLetterPhaseDashERaq => "bamum letter phase-e raq",
            BamumSupplement::BamumLetterPhaseDashENshuop => "bamum letter phase-e nshuop",
            BamumSupplement::BamumLetterPhaseDashENdun => "bamum letter phase-e ndun",
            BamumSupplement::BamumLetterPhaseDashEPuae => "bamum letter phase-e puae",
            BamumSupplement::BamumLetterPhaseDashETam => "bamum letter phase-e tam",
            BamumSupplement::BamumLetterPhaseDashENgka => "bamum letter phase-e ngka",
            BamumSupplement::BamumLetterPhaseDashEKpeux => "bamum letter phase-e kpeux",
            BamumSupplement::BamumLetterPhaseDashEWuo => "bamum letter phase-e wuo",
            BamumSupplement::BamumLetterPhaseDashESee => "bamum letter phase-e see",
            BamumSupplement::BamumLetterPhaseDashENggeuaet => "bamum letter phase-e nggeuaet",
            BamumSupplement::BamumLetterPhaseDashEPaam => "bamum letter phase-e paam",
            BamumSupplement::BamumLetterPhaseDashEToo => "bamum letter phase-e too",
            BamumSupplement::BamumLetterPhaseDashEKuop => "bamum letter phase-e kuop",
            BamumSupplement::BamumLetterPhaseDashELom => "bamum letter phase-e lom",
            BamumSupplement::BamumLetterPhaseDashENshiee => "bamum letter phase-e nshiee",
            BamumSupplement::BamumLetterPhaseDashENgop => "bamum letter phase-e ngop",
            BamumSupplement::BamumLetterPhaseDashEMaem => "bamum letter phase-e maem",
            BamumSupplement::BamumLetterPhaseDashENgkeux => "bamum letter phase-e ngkeux",
            BamumSupplement::BamumLetterPhaseDashENgoq => "bamum letter phase-e ngoq",
            BamumSupplement::BamumLetterPhaseDashENshue => "bamum letter phase-e nshue",
            BamumSupplement::BamumLetterPhaseDashERimgba => "bamum letter phase-e rimgba",
            BamumSupplement::BamumLetterPhaseDashENjeux => "bamum letter phase-e njeux",
            BamumSupplement::BamumLetterPhaseDashEPeem => "bamum letter phase-e peem",
            BamumSupplement::BamumLetterPhaseDashESaa => "bamum letter phase-e saa",
            BamumSupplement::BamumLetterPhaseDashENggurae => "bamum letter phase-e nggurae",
            BamumSupplement::BamumLetterPhaseDashEMgba => "bamum letter phase-e mgba",
            BamumSupplement::BamumLetterPhaseDashEGheux => "bamum letter phase-e gheux",
            BamumSupplement::BamumLetterPhaseDashENgkeuaem => "bamum letter phase-e ngkeuaem",
            BamumSupplement::BamumLetterPhaseDashENjaemli => "bamum letter phase-e njaemli",
            BamumSupplement::BamumLetterPhaseDashEMap => "bamum letter phase-e map",
            BamumSupplement::BamumLetterPhaseDashELoot => "bamum letter phase-e loot",
            BamumSupplement::BamumLetterPhaseDashENggeeee => "bamum letter phase-e nggeeee",
            BamumSupplement::BamumLetterPhaseDashENdiq => "bamum letter phase-e ndiq",
            BamumSupplement::BamumLetterPhaseDashETaenNteum => "bamum letter phase-e taen nteum",
            BamumSupplement::BamumLetterPhaseDashESet => "bamum letter phase-e set",
            BamumSupplement::BamumLetterPhaseDashEPum => "bamum letter phase-e pum",
            BamumSupplement::BamumLetterPhaseDashENdaaSoftness => "bamum letter phase-e ndaa softness",
            BamumSupplement::BamumLetterPhaseDashENgguaeshaeNyam => "bamum letter phase-e ngguaeshae nyam",
            BamumSupplement::BamumLetterPhaseDashEYiee => "bamum letter phase-e yiee",
            BamumSupplement::BamumLetterPhaseDashEGheun => "bamum letter phase-e gheun",
            BamumSupplement::BamumLetterPhaseDashETuae => "bamum letter phase-e tuae",
            BamumSupplement::BamumLetterPhaseDashEYeuae => "bamum letter phase-e yeuae",
            BamumSupplement::BamumLetterPhaseDashEPo => "bamum letter phase-e po",
            BamumSupplement::BamumLetterPhaseDashETumae => "bamum letter phase-e tumae",
            BamumSupplement::BamumLetterPhaseDashEKeuae => "bamum letter phase-e keuae",
            BamumSupplement::BamumLetterPhaseDashESuaen => "bamum letter phase-e suaen",
            BamumSupplement::BamumLetterPhaseDashETeuaeq => "bamum letter phase-e teuaeq",
            BamumSupplement::BamumLetterPhaseDashEVeuae => "bamum letter phase-e veuae",
            BamumSupplement::BamumLetterPhaseDashEWeux => "bamum letter phase-e weux",
            BamumSupplement::BamumLetterPhaseDashELaam => "bamum letter phase-e laam",
            BamumSupplement::BamumLetterPhaseDashEPu => "bamum letter phase-e pu",
            BamumSupplement::BamumLetterPhaseDashETaaq => "bamum letter phase-e taaq",
            BamumSupplement::BamumLetterPhaseDashEGhaamae => "bamum letter phase-e ghaamae",
            BamumSupplement::BamumLetterPhaseDashENgeureut => "bamum letter phase-e ngeureut",
            BamumSupplement::BamumLetterPhaseDashESheuaeq => "bamum letter phase-e sheuaeq",
            BamumSupplement::BamumLetterPhaseDashEMgben => "bamum letter phase-e mgben",
            BamumSupplement::BamumLetterPhaseDashEMbee => "bamum letter phase-e mbee",
            BamumSupplement::BamumLetterPhaseDashENzaq => "bamum letter phase-e nzaq",
            BamumSupplement::BamumLetterPhaseDashENkom => "bamum letter phase-e nkom",
            BamumSupplement::BamumLetterPhaseDashEGbet => "bamum letter phase-e gbet",
            BamumSupplement::BamumLetterPhaseDashETum => "bamum letter phase-e tum",
            BamumSupplement::BamumLetterPhaseDashEKuet => "bamum letter phase-e kuet",
            BamumSupplement::BamumLetterPhaseDashEYap => "bamum letter phase-e yap",
            BamumSupplement::BamumLetterPhaseDashENyiCleaver => "bamum letter phase-e nyi cleaver",
            BamumSupplement::BamumLetterPhaseDashEYit => "bamum letter phase-e yit",
            BamumSupplement::BamumLetterPhaseDashEMfeuq => "bamum letter phase-e mfeuq",
            BamumSupplement::BamumLetterPhaseDashENdiaq => "bamum letter phase-e ndiaq",
            BamumSupplement::BamumLetterPhaseDashEPieeq => "bamum letter phase-e pieeq",
            BamumSupplement::BamumLetterPhaseDashEYueq => "bamum letter phase-e yueq",
            BamumSupplement::BamumLetterPhaseDashELeuaem => "bamum letter phase-e leuaem",
            BamumSupplement::BamumLetterPhaseDashEFue => "bamum letter phase-e fue",
            BamumSupplement::BamumLetterPhaseDashEGbeux => "bamum letter phase-e gbeux",
            BamumSupplement::BamumLetterPhaseDashENgkup => "bamum letter phase-e ngkup",
            BamumSupplement::BamumLetterPhaseDashEKet => "bamum letter phase-e ket",
            BamumSupplement::BamumLetterPhaseDashEMae => "bamum letter phase-e mae",
            BamumSupplement::BamumLetterPhaseDashENgkaami => "bamum letter phase-e ngkaami",
            BamumSupplement::BamumLetterPhaseDashEGhet => "bamum letter phase-e ghet",
            BamumSupplement::BamumLetterPhaseDashEFa => "bamum letter phase-e fa",
            BamumSupplement::BamumLetterPhaseDashENtum => "bamum letter phase-e ntum",
            BamumSupplement::BamumLetterPhaseDashEPeut => "bamum letter phase-e peut",
            BamumSupplement::BamumLetterPhaseDashEYeum => "bamum letter phase-e yeum",
            BamumSupplement::BamumLetterPhaseDashENggeuae => "bamum letter phase-e nggeuae",
            BamumSupplement::BamumLetterPhaseDashENyiBetween => "bamum letter phase-e nyi between",
            BamumSupplement::BamumLetterPhaseDashENzuq => "bamum letter phase-e nzuq",
            BamumSupplement::BamumLetterPhaseDashEPoon => "bamum letter phase-e poon",
            BamumSupplement::BamumLetterPhaseDashEMiee => "bamum letter phase-e miee",
            BamumSupplement::BamumLetterPhaseDashEFuet => "bamum letter phase-e fuet",
            BamumSupplement::BamumLetterPhaseDashENae => "bamum letter phase-e nae",
            BamumSupplement::BamumLetterPhaseDashEMuae => "bamum letter phase-e muae",
            BamumSupplement::BamumLetterPhaseDashEGheuae => "bamum letter phase-e gheuae",
            BamumSupplement::BamumLetterPhaseDashEFuI => "bamum letter phase-e fu i",
            BamumSupplement::BamumLetterPhaseDashEMvi => "bamum letter phase-e mvi",
            BamumSupplement::BamumLetterPhaseDashEPuaq => "bamum letter phase-e puaq",
            BamumSupplement::BamumLetterPhaseDashENgkum => "bamum letter phase-e ngkum",
            BamumSupplement::BamumLetterPhaseDashEKut => "bamum letter phase-e kut",
            BamumSupplement::BamumLetterPhaseDashEPiet => "bamum letter phase-e piet",
            BamumSupplement::BamumLetterPhaseDashENtap => "bamum letter phase-e ntap",
            BamumSupplement::BamumLetterPhaseDashEYeuaet => "bamum letter phase-e yeuaet",
            BamumSupplement::BamumLetterPhaseDashENggup => "bamum letter phase-e nggup",
            BamumSupplement::BamumLetterPhaseDashEPaPeople => "bamum letter phase-e pa people",
            BamumSupplement::BamumLetterPhaseDashEFuCall => "bamum letter phase-e fu call",
            BamumSupplement::BamumLetterPhaseDashEFom => "bamum letter phase-e fom",
            BamumSupplement::BamumLetterPhaseDashENjee => "bamum letter phase-e njee",
            BamumSupplement::BamumLetterPhaseDashEA => "bamum letter phase-e a",
            BamumSupplement::BamumLetterPhaseDashEToq => "bamum letter phase-e toq",
            BamumSupplement::BamumLetterPhaseDashEO => "bamum letter phase-e o",
            BamumSupplement::BamumLetterPhaseDashEI => "bamum letter phase-e i",
            BamumSupplement::BamumLetterPhaseDashELaq => "bamum letter phase-e laq",
            BamumSupplement::BamumLetterPhaseDashEPaPlural => "bamum letter phase-e pa plural",
            BamumSupplement::BamumLetterPhaseDashETaa => "bamum letter phase-e taa",
            BamumSupplement::BamumLetterPhaseDashETaq => "bamum letter phase-e taq",
            BamumSupplement::BamumLetterPhaseDashENdaaMyHouse => "bamum letter phase-e ndaa my house",
            BamumSupplement::BamumLetterPhaseDashEShiq => "bamum letter phase-e shiq",
            BamumSupplement::BamumLetterPhaseDashEYeux => "bamum letter phase-e yeux",
            BamumSupplement::BamumLetterPhaseDashENguae => "bamum letter phase-e nguae",
            BamumSupplement::BamumLetterPhaseDashEYuaen => "bamum letter phase-e yuaen",
            BamumSupplement::BamumLetterPhaseDashEYoqSwimming => "bamum letter phase-e yoq swimming",
            BamumSupplement::BamumLetterPhaseDashEYoqCover => "bamum letter phase-e yoq cover",
            BamumSupplement::BamumLetterPhaseDashEYuq => "bamum letter phase-e yuq",
            BamumSupplement::BamumLetterPhaseDashEYun => "bamum letter phase-e yun",
            BamumSupplement::BamumLetterPhaseDashEKeux => "bamum letter phase-e keux",
            BamumSupplement::BamumLetterPhaseDashEPeux => "bamum letter phase-e peux",
            BamumSupplement::BamumLetterPhaseDashENjeeEpoch => "bamum letter phase-e njee epoch",
            BamumSupplement::BamumLetterPhaseDashEPue => "bamum letter phase-e pue",
            BamumSupplement::BamumLetterPhaseDashEWue => "bamum letter phase-e wue",
            BamumSupplement::BamumLetterPhaseDashEFee => "bamum letter phase-e fee",
            BamumSupplement::BamumLetterPhaseDashEVee => "bamum letter phase-e vee",
            BamumSupplement::BamumLetterPhaseDashELu => "bamum letter phase-e lu",
            BamumSupplement::BamumLetterPhaseDashEMi => "bamum letter phase-e mi",
            BamumSupplement::BamumLetterPhaseDashEReux => "bamum letter phase-e reux",
            BamumSupplement::BamumLetterPhaseDashERae => "bamum letter phase-e rae",
            BamumSupplement::BamumLetterPhaseDashENguaet => "bamum letter phase-e nguaet",
            BamumSupplement::BamumLetterPhaseDashENga => "bamum letter phase-e nga",
            BamumSupplement::BamumLetterPhaseDashESho => "bamum letter phase-e sho",
            BamumSupplement::BamumLetterPhaseDashEShoq => "bamum letter phase-e shoq",
            BamumSupplement::BamumLetterPhaseDashEFuRemedy => "bamum letter phase-e fu remedy",
            BamumSupplement::BamumLetterPhaseDashENa => "bamum letter phase-e na",
            BamumSupplement::BamumLetterPhaseDashEPi => "bamum letter phase-e pi",
            BamumSupplement::BamumLetterPhaseDashELoq => "bamum letter phase-e loq",
            BamumSupplement::BamumLetterPhaseDashEKo => "bamum letter phase-e ko",
            BamumSupplement::BamumLetterPhaseDashEMen => "bamum letter phase-e men",
            BamumSupplement::BamumLetterPhaseDashEMa => "bamum letter phase-e ma",
            BamumSupplement::BamumLetterPhaseDashEMaq => "bamum letter phase-e maq",
            BamumSupplement::BamumLetterPhaseDashETeu => "bamum letter phase-e teu",
            BamumSupplement::BamumLetterPhaseDashEKi => "bamum letter phase-e ki",
            BamumSupplement::BamumLetterPhaseDashEMon => "bamum letter phase-e mon",
            BamumSupplement::BamumLetterPhaseDashETen => "bamum letter phase-e ten",
            BamumSupplement::BamumLetterPhaseDashEFaq => "bamum letter phase-e faq",
            BamumSupplement::BamumLetterPhaseDashEGhom => "bamum letter phase-e ghom",
            BamumSupplement::BamumLetterPhaseDashFKa => "bamum letter phase-f ka",
            BamumSupplement::BamumLetterPhaseDashFU => "bamum letter phase-f u",
            BamumSupplement::BamumLetterPhaseDashFKu => "bamum letter phase-f ku",
            BamumSupplement::BamumLetterPhaseDashFEe => "bamum letter phase-f ee",
            BamumSupplement::BamumLetterPhaseDashFRee => "bamum letter phase-f ree",
            BamumSupplement::BamumLetterPhaseDashFTae => "bamum letter phase-f tae",
            BamumSupplement::BamumLetterPhaseDashFNyi => "bamum letter phase-f nyi",
            BamumSupplement::BamumLetterPhaseDashFLa => "bamum letter phase-f la",
            BamumSupplement::BamumLetterPhaseDashFRii => "bamum letter phase-f rii",
            BamumSupplement::BamumLetterPhaseDashFRiee => "bamum letter phase-f riee",
            BamumSupplement::BamumLetterPhaseDashFMeeee => "bamum letter phase-f meeee",
            BamumSupplement::BamumLetterPhaseDashFTaa => "bamum letter phase-f taa",
            BamumSupplement::BamumLetterPhaseDashFNdaa => "bamum letter phase-f ndaa",
            BamumSupplement::BamumLetterPhaseDashFNjaem => "bamum letter phase-f njaem",
            BamumSupplement::BamumLetterPhaseDashFM => "bamum letter phase-f m",
            BamumSupplement::BamumLetterPhaseDashFSuu => "bamum letter phase-f suu",
            BamumSupplement::BamumLetterPhaseDashFShii => "bamum letter phase-f shii",
            BamumSupplement::BamumLetterPhaseDashFSi => "bamum letter phase-f si",
            BamumSupplement::BamumLetterPhaseDashFSeux => "bamum letter phase-f seux",
            BamumSupplement::BamumLetterPhaseDashFKyee => "bamum letter phase-f kyee",
            BamumSupplement::BamumLetterPhaseDashFKet => "bamum letter phase-f ket",
            BamumSupplement::BamumLetterPhaseDashFNuae => "bamum letter phase-f nuae",
            BamumSupplement::BamumLetterPhaseDashFNu => "bamum letter phase-f nu",
            BamumSupplement::BamumLetterPhaseDashFNjuae => "bamum letter phase-f njuae",
            BamumSupplement::BamumLetterPhaseDashFYoq => "bamum letter phase-f yoq",
            BamumSupplement::BamumLetterPhaseDashFShu => "bamum letter phase-f shu",
            BamumSupplement::BamumLetterPhaseDashFYa => "bamum letter phase-f ya",
            BamumSupplement::BamumLetterPhaseDashFNsha => "bamum letter phase-f nsha",
            BamumSupplement::BamumLetterPhaseDashFPeux => "bamum letter phase-f peux",
            BamumSupplement::BamumLetterPhaseDashFNtee => "bamum letter phase-f ntee",
            BamumSupplement::BamumLetterPhaseDashFWue => "bamum letter phase-f wue",
            BamumSupplement::BamumLetterPhaseDashFPee => "bamum letter phase-f pee",
            BamumSupplement::BamumLetterPhaseDashFRu => "bamum letter phase-f ru",
            BamumSupplement::BamumLetterPhaseDashFNi => "bamum letter phase-f ni",
            BamumSupplement::BamumLetterPhaseDashFReux => "bamum letter phase-f reux",
            BamumSupplement::BamumLetterPhaseDashFKen => "bamum letter phase-f ken",
            BamumSupplement::BamumLetterPhaseDashFNgkwaen => "bamum letter phase-f ngkwaen",
            BamumSupplement::BamumLetterPhaseDashFNgga => "bamum letter phase-f ngga",
            BamumSupplement::BamumLetterPhaseDashFSho => "bamum letter phase-f sho",
            BamumSupplement::BamumLetterPhaseDashFPuae => "bamum letter phase-f puae",
            BamumSupplement::BamumLetterPhaseDashFFom => "bamum letter phase-f fom",
            BamumSupplement::BamumLetterPhaseDashFWa => "bamum letter phase-f wa",
            BamumSupplement::BamumLetterPhaseDashFLi => "bamum letter phase-f li",
            BamumSupplement::BamumLetterPhaseDashFLoq => "bamum letter phase-f loq",
            BamumSupplement::BamumLetterPhaseDashFKo => "bamum letter phase-f ko",
            BamumSupplement::BamumLetterPhaseDashFMben => "bamum letter phase-f mben",
            BamumSupplement::BamumLetterPhaseDashFRen => "bamum letter phase-f ren",
            BamumSupplement::BamumLetterPhaseDashFMa => "bamum letter phase-f ma",
            BamumSupplement::BamumLetterPhaseDashFMo => "bamum letter phase-f mo",
            BamumSupplement::BamumLetterPhaseDashFMbaa => "bamum letter phase-f mbaa",
            BamumSupplement::BamumLetterPhaseDashFTet => "bamum letter phase-f tet",
            BamumSupplement::BamumLetterPhaseDashFKpa => "bamum letter phase-f kpa",
            BamumSupplement::BamumLetterPhaseDashFSamba => "bamum letter phase-f samba",
            BamumSupplement::BamumLetterPhaseDashFVueq => "bamum letter phase-f vueq",
        }
    }
}
