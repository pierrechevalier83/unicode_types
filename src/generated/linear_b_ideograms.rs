
/// An enum to represent all characters in the LinearBIdeograms block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LinearBIdeograms {
    /// \u{10080}: '𐂀'
    LinearBIdeogramB100Man,
    /// \u{10081}: '𐂁'
    LinearBIdeogramB102Woman,
    /// \u{10082}: '𐂂'
    LinearBIdeogramB104Deer,
    /// \u{10083}: '𐂃'
    LinearBIdeogramB105Equid,
    /// \u{10084}: '𐂄'
    LinearBIdeogramB105fMare,
    /// \u{10085}: '𐂅'
    LinearBIdeogramB105mStallion,
    /// \u{10086}: '𐂆'
    LinearBIdeogramB106fEwe,
    /// \u{10087}: '𐂇'
    LinearBIdeogramB106mRam,
    /// \u{10088}: '𐂈'
    LinearBIdeogramB107fSheDashGoat,
    /// \u{10089}: '𐂉'
    LinearBIdeogramB107mHeDashGoat,
    /// \u{1008a}: '𐂊'
    LinearBIdeogramB108fSow,
    /// \u{1008b}: '𐂋'
    LinearBIdeogramB108mBoar,
    /// \u{1008c}: '𐂌'
    LinearBIdeogramB109fCow,
    /// \u{1008d}: '𐂍'
    LinearBIdeogramB109mBull,
    /// \u{1008e}: '𐂎'
    LinearBIdeogramB120Wheat,
    /// \u{1008f}: '𐂏'
    LinearBIdeogramB121Barley,
    /// \u{10090}: '𐂐'
    LinearBIdeogramB122Olive,
    /// \u{10091}: '𐂑'
    LinearBIdeogramB123Spice,
    /// \u{10092}: '𐂒'
    LinearBIdeogramB125Cyperus,
    /// \u{10093}: '𐂓'
    LinearBMonogramB127Kapo,
    /// \u{10094}: '𐂔'
    LinearBMonogramB128Kanako,
    /// \u{10095}: '𐂕'
    LinearBIdeogramB130Oil,
    /// \u{10096}: '𐂖'
    LinearBIdeogramB131Wine,
    /// \u{10097}: '𐂗'
    LinearBIdeogramB132,
    /// \u{10098}: '𐂘'
    LinearBMonogramB133Arepa,
    /// \u{10099}: '𐂙'
    LinearBMonogramB135Meri,
    /// \u{1009a}: '𐂚'
    LinearBIdeogramB140Bronze,
    /// \u{1009b}: '𐂛'
    LinearBIdeogramB141Gold,
    /// \u{1009c}: '𐂜'
    LinearBIdeogramB142,
    /// \u{1009d}: '𐂝'
    LinearBIdeogramB145Wool,
    /// \u{1009e}: '𐂞'
    LinearBIdeogramB146,
    /// \u{1009f}: '𐂟'
    LinearBIdeogramB150,
    /// \u{100a0}: '𐂠'
    LinearBIdeogramB151Horn,
    /// \u{100a1}: '𐂡'
    LinearBIdeogramB152,
    /// \u{100a2}: '𐂢'
    LinearBIdeogramB153,
    /// \u{100a3}: '𐂣'
    LinearBIdeogramB154,
    /// \u{100a4}: '𐂤'
    LinearBMonogramB156Turo2,
    /// \u{100a5}: '𐂥'
    LinearBIdeogramB157,
    /// \u{100a6}: '𐂦'
    LinearBIdeogramB158,
    /// \u{100a7}: '𐂧'
    LinearBIdeogramB159Cloth,
    /// \u{100a8}: '𐂨'
    LinearBIdeogramB160,
    /// \u{100a9}: '𐂩'
    LinearBIdeogramB161,
    /// \u{100aa}: '𐂪'
    LinearBIdeogramB162Garment,
    /// \u{100ab}: '𐂫'
    LinearBIdeogramB163Armour,
    /// \u{100ac}: '𐂬'
    LinearBIdeogramB164,
    /// \u{100ad}: '𐂭'
    LinearBIdeogramB165,
    /// \u{100ae}: '𐂮'
    LinearBIdeogramB166,
    /// \u{100af}: '𐂯'
    LinearBIdeogramB167,
    /// \u{100b0}: '𐂰'
    LinearBIdeogramB168,
    /// \u{100b1}: '𐂱'
    LinearBIdeogramB169,
    /// \u{100b2}: '𐂲'
    LinearBIdeogramB170,
    /// \u{100b3}: '𐂳'
    LinearBIdeogramB171,
    /// \u{100b4}: '𐂴'
    LinearBIdeogramB172,
    /// \u{100b5}: '𐂵'
    LinearBIdeogramB173Month,
    /// \u{100b6}: '𐂶'
    LinearBIdeogramB174,
    /// \u{100b7}: '𐂷'
    LinearBIdeogramB176Tree,
    /// \u{100b8}: '𐂸'
    LinearBIdeogramB177,
    /// \u{100b9}: '𐂹'
    LinearBIdeogramB178,
    /// \u{100ba}: '𐂺'
    LinearBIdeogramB179,
    /// \u{100bb}: '𐂻'
    LinearBIdeogramB180,
    /// \u{100bc}: '𐂼'
    LinearBIdeogramB181,
    /// \u{100bd}: '𐂽'
    LinearBIdeogramB182,
    /// \u{100be}: '𐂾'
    LinearBIdeogramB183,
    /// \u{100bf}: '𐂿'
    LinearBIdeogramB184,
    /// \u{100c0}: '𐃀'
    LinearBIdeogramB185,
    /// \u{100c1}: '𐃁'
    LinearBIdeogramB189,
    /// \u{100c2}: '𐃂'
    LinearBIdeogramB190,
    /// \u{100c3}: '𐃃'
    LinearBIdeogramB191Helmet,
    /// \u{100c4}: '𐃄'
    LinearBIdeogramB220Footstool,
    /// \u{100c5}: '𐃅'
    LinearBIdeogramB225Bathtub,
    /// \u{100c6}: '𐃆'
    LinearBIdeogramB230Spear,
    /// \u{100c7}: '𐃇'
    LinearBIdeogramB231Arrow,
    /// \u{100c8}: '𐃈'
    LinearBIdeogramB232,
    /// \u{100c9}: '𐃉'
    LinearBIdeogramB233Sword,
    /// \u{100ca}: '𐃊'
    LinearBIdeogramB234,
    /// \u{100cb}: '𐃋'
    LinearBIdeogramB236,
    /// \u{100cc}: '𐃌'
    LinearBIdeogramB240WheeledChariot,
    /// \u{100cd}: '𐃍'
    LinearBIdeogramB241Chariot,
    /// \u{100ce}: '𐃎'
    LinearBIdeogramB242ChariotFrame,
    /// \u{100cf}: '𐃏'
    LinearBIdeogramB243Wheel,
    /// \u{100d0}: '𐃐'
    LinearBIdeogramB245,
    /// \u{100d1}: '𐃑'
    LinearBIdeogramB246,
    /// \u{100d2}: '𐃒'
    LinearBMonogramB247Dipte,
    /// \u{100d3}: '𐃓'
    LinearBIdeogramB248,
    /// \u{100d4}: '𐃔'
    LinearBIdeogramB249,
    /// \u{100d5}: '𐃕'
    LinearBIdeogramB251,
    /// \u{100d6}: '𐃖'
    LinearBIdeogramB252,
    /// \u{100d7}: '𐃗'
    LinearBIdeogramB253,
    /// \u{100d8}: '𐃘'
    LinearBIdeogramB254Dart,
    /// \u{100d9}: '𐃙'
    LinearBIdeogramB255,
    /// \u{100da}: '𐃚'
    LinearBIdeogramB256,
    /// \u{100db}: '𐃛'
    LinearBIdeogramB257,
    /// \u{100dc}: '𐃜'
    LinearBIdeogramB258,
    /// \u{100dd}: '𐃝'
    LinearBIdeogramB259,
    /// \u{100de}: '𐃞'
    LinearBIdeogramVesselB155,
    /// \u{100df}: '𐃟'
    LinearBIdeogramVesselB200,
    /// \u{100e0}: '𐃠'
    LinearBIdeogramVesselB201,
    /// \u{100e1}: '𐃡'
    LinearBIdeogramVesselB202,
    /// \u{100e2}: '𐃢'
    LinearBIdeogramVesselB203,
    /// \u{100e3}: '𐃣'
    LinearBIdeogramVesselB204,
    /// \u{100e4}: '𐃤'
    LinearBIdeogramVesselB205,
    /// \u{100e5}: '𐃥'
    LinearBIdeogramVesselB206,
    /// \u{100e6}: '𐃦'
    LinearBIdeogramVesselB207,
    /// \u{100e7}: '𐃧'
    LinearBIdeogramVesselB208,
    /// \u{100e8}: '𐃨'
    LinearBIdeogramVesselB209,
    /// \u{100e9}: '𐃩'
    LinearBIdeogramVesselB210,
    /// \u{100ea}: '𐃪'
    LinearBIdeogramVesselB211,
    /// \u{100eb}: '𐃫'
    LinearBIdeogramVesselB212,
    /// \u{100ec}: '𐃬'
    LinearBIdeogramVesselB213,
    /// \u{100ed}: '𐃭'
    LinearBIdeogramVesselB214,
    /// \u{100ee}: '𐃮'
    LinearBIdeogramVesselB215,
    /// \u{100ef}: '𐃯'
    LinearBIdeogramVesselB216,
    /// \u{100f0}: '𐃰'
    LinearBIdeogramVesselB217,
    /// \u{100f1}: '𐃱'
    LinearBIdeogramVesselB218,
    /// \u{100f2}: '𐃲'
    LinearBIdeogramVesselB219,
    /// \u{100f3}: '𐃳'
    LinearBIdeogramVesselB221,
    /// \u{100f4}: '𐃴'
    LinearBIdeogramVesselB222,
    /// \u{100f5}: '𐃵'
    LinearBIdeogramVesselB226,
    /// \u{100f6}: '𐃶'
    LinearBIdeogramVesselB227,
    /// \u{100f7}: '𐃷'
    LinearBIdeogramVesselB228,
    /// \u{100f8}: '𐃸'
    LinearBIdeogramVesselB229,
    /// \u{100f9}: '𐃹'
    LinearBIdeogramVesselB250,
    /// \u{100fa}: '𐃺'
    LinearBIdeogramVesselB305,
}

impl Into<char> for LinearBIdeograms {
    fn into(self) -> char {
        match self {
            LinearBIdeograms::LinearBIdeogramB100Man => '𐂀',
            LinearBIdeograms::LinearBIdeogramB102Woman => '𐂁',
            LinearBIdeograms::LinearBIdeogramB104Deer => '𐂂',
            LinearBIdeograms::LinearBIdeogramB105Equid => '𐂃',
            LinearBIdeograms::LinearBIdeogramB105fMare => '𐂄',
            LinearBIdeograms::LinearBIdeogramB105mStallion => '𐂅',
            LinearBIdeograms::LinearBIdeogramB106fEwe => '𐂆',
            LinearBIdeograms::LinearBIdeogramB106mRam => '𐂇',
            LinearBIdeograms::LinearBIdeogramB107fSheDashGoat => '𐂈',
            LinearBIdeograms::LinearBIdeogramB107mHeDashGoat => '𐂉',
            LinearBIdeograms::LinearBIdeogramB108fSow => '𐂊',
            LinearBIdeograms::LinearBIdeogramB108mBoar => '𐂋',
            LinearBIdeograms::LinearBIdeogramB109fCow => '𐂌',
            LinearBIdeograms::LinearBIdeogramB109mBull => '𐂍',
            LinearBIdeograms::LinearBIdeogramB120Wheat => '𐂎',
            LinearBIdeograms::LinearBIdeogramB121Barley => '𐂏',
            LinearBIdeograms::LinearBIdeogramB122Olive => '𐂐',
            LinearBIdeograms::LinearBIdeogramB123Spice => '𐂑',
            LinearBIdeograms::LinearBIdeogramB125Cyperus => '𐂒',
            LinearBIdeograms::LinearBMonogramB127Kapo => '𐂓',
            LinearBIdeograms::LinearBMonogramB128Kanako => '𐂔',
            LinearBIdeograms::LinearBIdeogramB130Oil => '𐂕',
            LinearBIdeograms::LinearBIdeogramB131Wine => '𐂖',
            LinearBIdeograms::LinearBIdeogramB132 => '𐂗',
            LinearBIdeograms::LinearBMonogramB133Arepa => '𐂘',
            LinearBIdeograms::LinearBMonogramB135Meri => '𐂙',
            LinearBIdeograms::LinearBIdeogramB140Bronze => '𐂚',
            LinearBIdeograms::LinearBIdeogramB141Gold => '𐂛',
            LinearBIdeograms::LinearBIdeogramB142 => '𐂜',
            LinearBIdeograms::LinearBIdeogramB145Wool => '𐂝',
            LinearBIdeograms::LinearBIdeogramB146 => '𐂞',
            LinearBIdeograms::LinearBIdeogramB150 => '𐂟',
            LinearBIdeograms::LinearBIdeogramB151Horn => '𐂠',
            LinearBIdeograms::LinearBIdeogramB152 => '𐂡',
            LinearBIdeograms::LinearBIdeogramB153 => '𐂢',
            LinearBIdeograms::LinearBIdeogramB154 => '𐂣',
            LinearBIdeograms::LinearBMonogramB156Turo2 => '𐂤',
            LinearBIdeograms::LinearBIdeogramB157 => '𐂥',
            LinearBIdeograms::LinearBIdeogramB158 => '𐂦',
            LinearBIdeograms::LinearBIdeogramB159Cloth => '𐂧',
            LinearBIdeograms::LinearBIdeogramB160 => '𐂨',
            LinearBIdeograms::LinearBIdeogramB161 => '𐂩',
            LinearBIdeograms::LinearBIdeogramB162Garment => '𐂪',
            LinearBIdeograms::LinearBIdeogramB163Armour => '𐂫',
            LinearBIdeograms::LinearBIdeogramB164 => '𐂬',
            LinearBIdeograms::LinearBIdeogramB165 => '𐂭',
            LinearBIdeograms::LinearBIdeogramB166 => '𐂮',
            LinearBIdeograms::LinearBIdeogramB167 => '𐂯',
            LinearBIdeograms::LinearBIdeogramB168 => '𐂰',
            LinearBIdeograms::LinearBIdeogramB169 => '𐂱',
            LinearBIdeograms::LinearBIdeogramB170 => '𐂲',
            LinearBIdeograms::LinearBIdeogramB171 => '𐂳',
            LinearBIdeograms::LinearBIdeogramB172 => '𐂴',
            LinearBIdeograms::LinearBIdeogramB173Month => '𐂵',
            LinearBIdeograms::LinearBIdeogramB174 => '𐂶',
            LinearBIdeograms::LinearBIdeogramB176Tree => '𐂷',
            LinearBIdeograms::LinearBIdeogramB177 => '𐂸',
            LinearBIdeograms::LinearBIdeogramB178 => '𐂹',
            LinearBIdeograms::LinearBIdeogramB179 => '𐂺',
            LinearBIdeograms::LinearBIdeogramB180 => '𐂻',
            LinearBIdeograms::LinearBIdeogramB181 => '𐂼',
            LinearBIdeograms::LinearBIdeogramB182 => '𐂽',
            LinearBIdeograms::LinearBIdeogramB183 => '𐂾',
            LinearBIdeograms::LinearBIdeogramB184 => '𐂿',
            LinearBIdeograms::LinearBIdeogramB185 => '𐃀',
            LinearBIdeograms::LinearBIdeogramB189 => '𐃁',
            LinearBIdeograms::LinearBIdeogramB190 => '𐃂',
            LinearBIdeograms::LinearBIdeogramB191Helmet => '𐃃',
            LinearBIdeograms::LinearBIdeogramB220Footstool => '𐃄',
            LinearBIdeograms::LinearBIdeogramB225Bathtub => '𐃅',
            LinearBIdeograms::LinearBIdeogramB230Spear => '𐃆',
            LinearBIdeograms::LinearBIdeogramB231Arrow => '𐃇',
            LinearBIdeograms::LinearBIdeogramB232 => '𐃈',
            LinearBIdeograms::LinearBIdeogramB233Sword => '𐃉',
            LinearBIdeograms::LinearBIdeogramB234 => '𐃊',
            LinearBIdeograms::LinearBIdeogramB236 => '𐃋',
            LinearBIdeograms::LinearBIdeogramB240WheeledChariot => '𐃌',
            LinearBIdeograms::LinearBIdeogramB241Chariot => '𐃍',
            LinearBIdeograms::LinearBIdeogramB242ChariotFrame => '𐃎',
            LinearBIdeograms::LinearBIdeogramB243Wheel => '𐃏',
            LinearBIdeograms::LinearBIdeogramB245 => '𐃐',
            LinearBIdeograms::LinearBIdeogramB246 => '𐃑',
            LinearBIdeograms::LinearBMonogramB247Dipte => '𐃒',
            LinearBIdeograms::LinearBIdeogramB248 => '𐃓',
            LinearBIdeograms::LinearBIdeogramB249 => '𐃔',
            LinearBIdeograms::LinearBIdeogramB251 => '𐃕',
            LinearBIdeograms::LinearBIdeogramB252 => '𐃖',
            LinearBIdeograms::LinearBIdeogramB253 => '𐃗',
            LinearBIdeograms::LinearBIdeogramB254Dart => '𐃘',
            LinearBIdeograms::LinearBIdeogramB255 => '𐃙',
            LinearBIdeograms::LinearBIdeogramB256 => '𐃚',
            LinearBIdeograms::LinearBIdeogramB257 => '𐃛',
            LinearBIdeograms::LinearBIdeogramB258 => '𐃜',
            LinearBIdeograms::LinearBIdeogramB259 => '𐃝',
            LinearBIdeograms::LinearBIdeogramVesselB155 => '𐃞',
            LinearBIdeograms::LinearBIdeogramVesselB200 => '𐃟',
            LinearBIdeograms::LinearBIdeogramVesselB201 => '𐃠',
            LinearBIdeograms::LinearBIdeogramVesselB202 => '𐃡',
            LinearBIdeograms::LinearBIdeogramVesselB203 => '𐃢',
            LinearBIdeograms::LinearBIdeogramVesselB204 => '𐃣',
            LinearBIdeograms::LinearBIdeogramVesselB205 => '𐃤',
            LinearBIdeograms::LinearBIdeogramVesselB206 => '𐃥',
            LinearBIdeograms::LinearBIdeogramVesselB207 => '𐃦',
            LinearBIdeograms::LinearBIdeogramVesselB208 => '𐃧',
            LinearBIdeograms::LinearBIdeogramVesselB209 => '𐃨',
            LinearBIdeograms::LinearBIdeogramVesselB210 => '𐃩',
            LinearBIdeograms::LinearBIdeogramVesselB211 => '𐃪',
            LinearBIdeograms::LinearBIdeogramVesselB212 => '𐃫',
            LinearBIdeograms::LinearBIdeogramVesselB213 => '𐃬',
            LinearBIdeograms::LinearBIdeogramVesselB214 => '𐃭',
            LinearBIdeograms::LinearBIdeogramVesselB215 => '𐃮',
            LinearBIdeograms::LinearBIdeogramVesselB216 => '𐃯',
            LinearBIdeograms::LinearBIdeogramVesselB217 => '𐃰',
            LinearBIdeograms::LinearBIdeogramVesselB218 => '𐃱',
            LinearBIdeograms::LinearBIdeogramVesselB219 => '𐃲',
            LinearBIdeograms::LinearBIdeogramVesselB221 => '𐃳',
            LinearBIdeograms::LinearBIdeogramVesselB222 => '𐃴',
            LinearBIdeograms::LinearBIdeogramVesselB226 => '𐃵',
            LinearBIdeograms::LinearBIdeogramVesselB227 => '𐃶',
            LinearBIdeograms::LinearBIdeogramVesselB228 => '𐃷',
            LinearBIdeograms::LinearBIdeogramVesselB229 => '𐃸',
            LinearBIdeograms::LinearBIdeogramVesselB250 => '𐃹',
            LinearBIdeograms::LinearBIdeogramVesselB305 => '𐃺',
        }
    }
}

impl std::convert::TryFrom<char> for LinearBIdeograms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐂀' => Ok(LinearBIdeograms::LinearBIdeogramB100Man),
            '𐂁' => Ok(LinearBIdeograms::LinearBIdeogramB102Woman),
            '𐂂' => Ok(LinearBIdeograms::LinearBIdeogramB104Deer),
            '𐂃' => Ok(LinearBIdeograms::LinearBIdeogramB105Equid),
            '𐂄' => Ok(LinearBIdeograms::LinearBIdeogramB105fMare),
            '𐂅' => Ok(LinearBIdeograms::LinearBIdeogramB105mStallion),
            '𐂆' => Ok(LinearBIdeograms::LinearBIdeogramB106fEwe),
            '𐂇' => Ok(LinearBIdeograms::LinearBIdeogramB106mRam),
            '𐂈' => Ok(LinearBIdeograms::LinearBIdeogramB107fSheDashGoat),
            '𐂉' => Ok(LinearBIdeograms::LinearBIdeogramB107mHeDashGoat),
            '𐂊' => Ok(LinearBIdeograms::LinearBIdeogramB108fSow),
            '𐂋' => Ok(LinearBIdeograms::LinearBIdeogramB108mBoar),
            '𐂌' => Ok(LinearBIdeograms::LinearBIdeogramB109fCow),
            '𐂍' => Ok(LinearBIdeograms::LinearBIdeogramB109mBull),
            '𐂎' => Ok(LinearBIdeograms::LinearBIdeogramB120Wheat),
            '𐂏' => Ok(LinearBIdeograms::LinearBIdeogramB121Barley),
            '𐂐' => Ok(LinearBIdeograms::LinearBIdeogramB122Olive),
            '𐂑' => Ok(LinearBIdeograms::LinearBIdeogramB123Spice),
            '𐂒' => Ok(LinearBIdeograms::LinearBIdeogramB125Cyperus),
            '𐂓' => Ok(LinearBIdeograms::LinearBMonogramB127Kapo),
            '𐂔' => Ok(LinearBIdeograms::LinearBMonogramB128Kanako),
            '𐂕' => Ok(LinearBIdeograms::LinearBIdeogramB130Oil),
            '𐂖' => Ok(LinearBIdeograms::LinearBIdeogramB131Wine),
            '𐂗' => Ok(LinearBIdeograms::LinearBIdeogramB132),
            '𐂘' => Ok(LinearBIdeograms::LinearBMonogramB133Arepa),
            '𐂙' => Ok(LinearBIdeograms::LinearBMonogramB135Meri),
            '𐂚' => Ok(LinearBIdeograms::LinearBIdeogramB140Bronze),
            '𐂛' => Ok(LinearBIdeograms::LinearBIdeogramB141Gold),
            '𐂜' => Ok(LinearBIdeograms::LinearBIdeogramB142),
            '𐂝' => Ok(LinearBIdeograms::LinearBIdeogramB145Wool),
            '𐂞' => Ok(LinearBIdeograms::LinearBIdeogramB146),
            '𐂟' => Ok(LinearBIdeograms::LinearBIdeogramB150),
            '𐂠' => Ok(LinearBIdeograms::LinearBIdeogramB151Horn),
            '𐂡' => Ok(LinearBIdeograms::LinearBIdeogramB152),
            '𐂢' => Ok(LinearBIdeograms::LinearBIdeogramB153),
            '𐂣' => Ok(LinearBIdeograms::LinearBIdeogramB154),
            '𐂤' => Ok(LinearBIdeograms::LinearBMonogramB156Turo2),
            '𐂥' => Ok(LinearBIdeograms::LinearBIdeogramB157),
            '𐂦' => Ok(LinearBIdeograms::LinearBIdeogramB158),
            '𐂧' => Ok(LinearBIdeograms::LinearBIdeogramB159Cloth),
            '𐂨' => Ok(LinearBIdeograms::LinearBIdeogramB160),
            '𐂩' => Ok(LinearBIdeograms::LinearBIdeogramB161),
            '𐂪' => Ok(LinearBIdeograms::LinearBIdeogramB162Garment),
            '𐂫' => Ok(LinearBIdeograms::LinearBIdeogramB163Armour),
            '𐂬' => Ok(LinearBIdeograms::LinearBIdeogramB164),
            '𐂭' => Ok(LinearBIdeograms::LinearBIdeogramB165),
            '𐂮' => Ok(LinearBIdeograms::LinearBIdeogramB166),
            '𐂯' => Ok(LinearBIdeograms::LinearBIdeogramB167),
            '𐂰' => Ok(LinearBIdeograms::LinearBIdeogramB168),
            '𐂱' => Ok(LinearBIdeograms::LinearBIdeogramB169),
            '𐂲' => Ok(LinearBIdeograms::LinearBIdeogramB170),
            '𐂳' => Ok(LinearBIdeograms::LinearBIdeogramB171),
            '𐂴' => Ok(LinearBIdeograms::LinearBIdeogramB172),
            '𐂵' => Ok(LinearBIdeograms::LinearBIdeogramB173Month),
            '𐂶' => Ok(LinearBIdeograms::LinearBIdeogramB174),
            '𐂷' => Ok(LinearBIdeograms::LinearBIdeogramB176Tree),
            '𐂸' => Ok(LinearBIdeograms::LinearBIdeogramB177),
            '𐂹' => Ok(LinearBIdeograms::LinearBIdeogramB178),
            '𐂺' => Ok(LinearBIdeograms::LinearBIdeogramB179),
            '𐂻' => Ok(LinearBIdeograms::LinearBIdeogramB180),
            '𐂼' => Ok(LinearBIdeograms::LinearBIdeogramB181),
            '𐂽' => Ok(LinearBIdeograms::LinearBIdeogramB182),
            '𐂾' => Ok(LinearBIdeograms::LinearBIdeogramB183),
            '𐂿' => Ok(LinearBIdeograms::LinearBIdeogramB184),
            '𐃀' => Ok(LinearBIdeograms::LinearBIdeogramB185),
            '𐃁' => Ok(LinearBIdeograms::LinearBIdeogramB189),
            '𐃂' => Ok(LinearBIdeograms::LinearBIdeogramB190),
            '𐃃' => Ok(LinearBIdeograms::LinearBIdeogramB191Helmet),
            '𐃄' => Ok(LinearBIdeograms::LinearBIdeogramB220Footstool),
            '𐃅' => Ok(LinearBIdeograms::LinearBIdeogramB225Bathtub),
            '𐃆' => Ok(LinearBIdeograms::LinearBIdeogramB230Spear),
            '𐃇' => Ok(LinearBIdeograms::LinearBIdeogramB231Arrow),
            '𐃈' => Ok(LinearBIdeograms::LinearBIdeogramB232),
            '𐃉' => Ok(LinearBIdeograms::LinearBIdeogramB233Sword),
            '𐃊' => Ok(LinearBIdeograms::LinearBIdeogramB234),
            '𐃋' => Ok(LinearBIdeograms::LinearBIdeogramB236),
            '𐃌' => Ok(LinearBIdeograms::LinearBIdeogramB240WheeledChariot),
            '𐃍' => Ok(LinearBIdeograms::LinearBIdeogramB241Chariot),
            '𐃎' => Ok(LinearBIdeograms::LinearBIdeogramB242ChariotFrame),
            '𐃏' => Ok(LinearBIdeograms::LinearBIdeogramB243Wheel),
            '𐃐' => Ok(LinearBIdeograms::LinearBIdeogramB245),
            '𐃑' => Ok(LinearBIdeograms::LinearBIdeogramB246),
            '𐃒' => Ok(LinearBIdeograms::LinearBMonogramB247Dipte),
            '𐃓' => Ok(LinearBIdeograms::LinearBIdeogramB248),
            '𐃔' => Ok(LinearBIdeograms::LinearBIdeogramB249),
            '𐃕' => Ok(LinearBIdeograms::LinearBIdeogramB251),
            '𐃖' => Ok(LinearBIdeograms::LinearBIdeogramB252),
            '𐃗' => Ok(LinearBIdeograms::LinearBIdeogramB253),
            '𐃘' => Ok(LinearBIdeograms::LinearBIdeogramB254Dart),
            '𐃙' => Ok(LinearBIdeograms::LinearBIdeogramB255),
            '𐃚' => Ok(LinearBIdeograms::LinearBIdeogramB256),
            '𐃛' => Ok(LinearBIdeograms::LinearBIdeogramB257),
            '𐃜' => Ok(LinearBIdeograms::LinearBIdeogramB258),
            '𐃝' => Ok(LinearBIdeograms::LinearBIdeogramB259),
            '𐃞' => Ok(LinearBIdeograms::LinearBIdeogramVesselB155),
            '𐃟' => Ok(LinearBIdeograms::LinearBIdeogramVesselB200),
            '𐃠' => Ok(LinearBIdeograms::LinearBIdeogramVesselB201),
            '𐃡' => Ok(LinearBIdeograms::LinearBIdeogramVesselB202),
            '𐃢' => Ok(LinearBIdeograms::LinearBIdeogramVesselB203),
            '𐃣' => Ok(LinearBIdeograms::LinearBIdeogramVesselB204),
            '𐃤' => Ok(LinearBIdeograms::LinearBIdeogramVesselB205),
            '𐃥' => Ok(LinearBIdeograms::LinearBIdeogramVesselB206),
            '𐃦' => Ok(LinearBIdeograms::LinearBIdeogramVesselB207),
            '𐃧' => Ok(LinearBIdeograms::LinearBIdeogramVesselB208),
            '𐃨' => Ok(LinearBIdeograms::LinearBIdeogramVesselB209),
            '𐃩' => Ok(LinearBIdeograms::LinearBIdeogramVesselB210),
            '𐃪' => Ok(LinearBIdeograms::LinearBIdeogramVesselB211),
            '𐃫' => Ok(LinearBIdeograms::LinearBIdeogramVesselB212),
            '𐃬' => Ok(LinearBIdeograms::LinearBIdeogramVesselB213),
            '𐃭' => Ok(LinearBIdeograms::LinearBIdeogramVesselB214),
            '𐃮' => Ok(LinearBIdeograms::LinearBIdeogramVesselB215),
            '𐃯' => Ok(LinearBIdeograms::LinearBIdeogramVesselB216),
            '𐃰' => Ok(LinearBIdeograms::LinearBIdeogramVesselB217),
            '𐃱' => Ok(LinearBIdeograms::LinearBIdeogramVesselB218),
            '𐃲' => Ok(LinearBIdeograms::LinearBIdeogramVesselB219),
            '𐃳' => Ok(LinearBIdeograms::LinearBIdeogramVesselB221),
            '𐃴' => Ok(LinearBIdeograms::LinearBIdeogramVesselB222),
            '𐃵' => Ok(LinearBIdeograms::LinearBIdeogramVesselB226),
            '𐃶' => Ok(LinearBIdeograms::LinearBIdeogramVesselB227),
            '𐃷' => Ok(LinearBIdeograms::LinearBIdeogramVesselB228),
            '𐃸' => Ok(LinearBIdeograms::LinearBIdeogramVesselB229),
            '𐃹' => Ok(LinearBIdeograms::LinearBIdeogramVesselB250),
            '𐃺' => Ok(LinearBIdeograms::LinearBIdeogramVesselB305),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LinearBIdeograms {
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

impl std::convert::TryFrom<u32> for LinearBIdeograms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LinearBIdeograms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LinearBIdeograms {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LinearBIdeograms::LinearBIdeogramB100Man
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("LinearBIdeograms{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
