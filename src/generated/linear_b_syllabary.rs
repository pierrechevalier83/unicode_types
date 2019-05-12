
/// An enum to represent all characters in the LinearBSyllabary block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LinearBSyllabary {
    /// \u{10000}: '𐀀'
    LinearBSyllableB008A,
    /// \u{10001}: '𐀁'
    LinearBSyllableB038E,
    /// \u{10002}: '𐀂'
    LinearBSyllableB028I,
    /// \u{10003}: '𐀃'
    LinearBSyllableB061O,
    /// \u{10004}: '𐀄'
    LinearBSyllableB010U,
    /// \u{10005}: '𐀅'
    LinearBSyllableB001Da,
    /// \u{10006}: '𐀆'
    LinearBSyllableB045De,
    /// \u{10007}: '𐀇'
    LinearBSyllableB007Di,
    /// \u{10008}: '𐀈'
    LinearBSyllableB014Do,
    /// \u{10009}: '𐀉'
    LinearBSyllableB051Du,
    /// \u{1000a}: '𐀊'
    LinearBSyllableB057Ja,
    /// \u{1000b}: '𐀋'
    LinearBSyllableB046Je,
    /// \u{1000d}: '𐀍'
    LinearBSyllableB036Jo,
    /// \u{1000e}: '𐀎'
    LinearBSyllableB065Ju,
    /// \u{1000f}: '𐀏'
    LinearBSyllableB077Ka,
    /// \u{10010}: '𐀐'
    LinearBSyllableB044Ke,
    /// \u{10011}: '𐀑'
    LinearBSyllableB067Ki,
    /// \u{10012}: '𐀒'
    LinearBSyllableB070Ko,
    /// \u{10013}: '𐀓'
    LinearBSyllableB081Ku,
    /// \u{10014}: '𐀔'
    LinearBSyllableB080Ma,
    /// \u{10015}: '𐀕'
    LinearBSyllableB013Me,
    /// \u{10016}: '𐀖'
    LinearBSyllableB073Mi,
    /// \u{10017}: '𐀗'
    LinearBSyllableB015Mo,
    /// \u{10018}: '𐀘'
    LinearBSyllableB023Mu,
    /// \u{10019}: '𐀙'
    LinearBSyllableB006Na,
    /// \u{1001a}: '𐀚'
    LinearBSyllableB024Ne,
    /// \u{1001b}: '𐀛'
    LinearBSyllableB030Ni,
    /// \u{1001c}: '𐀜'
    LinearBSyllableB052No,
    /// \u{1001d}: '𐀝'
    LinearBSyllableB055Nu,
    /// \u{1001e}: '𐀞'
    LinearBSyllableB003Pa,
    /// \u{1001f}: '𐀟'
    LinearBSyllableB072Pe,
    /// \u{10020}: '𐀠'
    LinearBSyllableB039Pi,
    /// \u{10021}: '𐀡'
    LinearBSyllableB011Po,
    /// \u{10022}: '𐀢'
    LinearBSyllableB050Pu,
    /// \u{10023}: '𐀣'
    LinearBSyllableB016Qa,
    /// \u{10024}: '𐀤'
    LinearBSyllableB078Qe,
    /// \u{10025}: '𐀥'
    LinearBSyllableB021Qi,
    /// \u{10026}: '𐀦'
    LinearBSyllableB032Qo,
    /// \u{10028}: '𐀨'
    LinearBSyllableB060Ra,
    /// \u{10029}: '𐀩'
    LinearBSyllableB027Re,
    /// \u{1002a}: '𐀪'
    LinearBSyllableB053Ri,
    /// \u{1002b}: '𐀫'
    LinearBSyllableB002Ro,
    /// \u{1002c}: '𐀬'
    LinearBSyllableB026Ru,
    /// \u{1002d}: '𐀭'
    LinearBSyllableB031Sa,
    /// \u{1002e}: '𐀮'
    LinearBSyllableB009Se,
    /// \u{1002f}: '𐀯'
    LinearBSyllableB041Si,
    /// \u{10030}: '𐀰'
    LinearBSyllableB012So,
    /// \u{10031}: '𐀱'
    LinearBSyllableB058Su,
    /// \u{10032}: '𐀲'
    LinearBSyllableB059Ta,
    /// \u{10033}: '𐀳'
    LinearBSyllableB004Te,
    /// \u{10034}: '𐀴'
    LinearBSyllableB037Ti,
    /// \u{10035}: '𐀵'
    LinearBSyllableB005To,
    /// \u{10036}: '𐀶'
    LinearBSyllableB069Tu,
    /// \u{10037}: '𐀷'
    LinearBSyllableB054Wa,
    /// \u{10038}: '𐀸'
    LinearBSyllableB075We,
    /// \u{10039}: '𐀹'
    LinearBSyllableB040Wi,
    /// \u{1003a}: '𐀺'
    LinearBSyllableB042Wo,
    /// \u{1003c}: '𐀼'
    LinearBSyllableB017Za,
    /// \u{1003d}: '𐀽'
    LinearBSyllableB074Ze,
    /// \u{1003f}: '𐀿'
    LinearBSyllableB020Zo,
    /// \u{10040}: '𐁀'
    LinearBSyllableB025A2,
    /// \u{10041}: '𐁁'
    LinearBSyllableB043A3,
    /// \u{10042}: '𐁂'
    LinearBSyllableB085Au,
    /// \u{10043}: '𐁃'
    LinearBSyllableB071Dwe,
    /// \u{10044}: '𐁄'
    LinearBSyllableB090Dwo,
    /// \u{10045}: '𐁅'
    LinearBSyllableB048Nwa,
    /// \u{10046}: '𐁆'
    LinearBSyllableB029Pu2,
    /// \u{10047}: '𐁇'
    LinearBSyllableB062Pte,
    /// \u{10048}: '𐁈'
    LinearBSyllableB076Ra2,
    /// \u{10049}: '𐁉'
    LinearBSyllableB033Ra3,
    /// \u{1004a}: '𐁊'
    LinearBSyllableB068Ro2,
    /// \u{1004b}: '𐁋'
    LinearBSyllableB066Ta2,
    /// \u{1004c}: '𐁌'
    LinearBSyllableB087Twe,
    /// \u{1004d}: '𐁍'
    LinearBSyllableB091Two,
    /// \u{10050}: '𐁐'
    LinearBSymbolB018,
    /// \u{10051}: '𐁑'
    LinearBSymbolB019,
    /// \u{10052}: '𐁒'
    LinearBSymbolB022,
    /// \u{10053}: '𐁓'
    LinearBSymbolB034,
    /// \u{10054}: '𐁔'
    LinearBSymbolB047,
    /// \u{10055}: '𐁕'
    LinearBSymbolB049,
    /// \u{10056}: '𐁖'
    LinearBSymbolB056,
    /// \u{10057}: '𐁗'
    LinearBSymbolB063,
    /// \u{10058}: '𐁘'
    LinearBSymbolB064,
    /// \u{10059}: '𐁙'
    LinearBSymbolB079,
    /// \u{1005a}: '𐁚'
    LinearBSymbolB082,
    /// \u{1005b}: '𐁛'
    LinearBSymbolB083,
    /// \u{1005c}: '𐁜'
    LinearBSymbolB086,
    /// \u{1005d}: '𐁝'
    LinearBSymbolB089,
}

impl Into<char> for LinearBSyllabary {
    fn into(self) -> char {
        match self {
            LinearBSyllabary::LinearBSyllableB008A => '𐀀',
            LinearBSyllabary::LinearBSyllableB038E => '𐀁',
            LinearBSyllabary::LinearBSyllableB028I => '𐀂',
            LinearBSyllabary::LinearBSyllableB061O => '𐀃',
            LinearBSyllabary::LinearBSyllableB010U => '𐀄',
            LinearBSyllabary::LinearBSyllableB001Da => '𐀅',
            LinearBSyllabary::LinearBSyllableB045De => '𐀆',
            LinearBSyllabary::LinearBSyllableB007Di => '𐀇',
            LinearBSyllabary::LinearBSyllableB014Do => '𐀈',
            LinearBSyllabary::LinearBSyllableB051Du => '𐀉',
            LinearBSyllabary::LinearBSyllableB057Ja => '𐀊',
            LinearBSyllabary::LinearBSyllableB046Je => '𐀋',
            LinearBSyllabary::LinearBSyllableB036Jo => '𐀍',
            LinearBSyllabary::LinearBSyllableB065Ju => '𐀎',
            LinearBSyllabary::LinearBSyllableB077Ka => '𐀏',
            LinearBSyllabary::LinearBSyllableB044Ke => '𐀐',
            LinearBSyllabary::LinearBSyllableB067Ki => '𐀑',
            LinearBSyllabary::LinearBSyllableB070Ko => '𐀒',
            LinearBSyllabary::LinearBSyllableB081Ku => '𐀓',
            LinearBSyllabary::LinearBSyllableB080Ma => '𐀔',
            LinearBSyllabary::LinearBSyllableB013Me => '𐀕',
            LinearBSyllabary::LinearBSyllableB073Mi => '𐀖',
            LinearBSyllabary::LinearBSyllableB015Mo => '𐀗',
            LinearBSyllabary::LinearBSyllableB023Mu => '𐀘',
            LinearBSyllabary::LinearBSyllableB006Na => '𐀙',
            LinearBSyllabary::LinearBSyllableB024Ne => '𐀚',
            LinearBSyllabary::LinearBSyllableB030Ni => '𐀛',
            LinearBSyllabary::LinearBSyllableB052No => '𐀜',
            LinearBSyllabary::LinearBSyllableB055Nu => '𐀝',
            LinearBSyllabary::LinearBSyllableB003Pa => '𐀞',
            LinearBSyllabary::LinearBSyllableB072Pe => '𐀟',
            LinearBSyllabary::LinearBSyllableB039Pi => '𐀠',
            LinearBSyllabary::LinearBSyllableB011Po => '𐀡',
            LinearBSyllabary::LinearBSyllableB050Pu => '𐀢',
            LinearBSyllabary::LinearBSyllableB016Qa => '𐀣',
            LinearBSyllabary::LinearBSyllableB078Qe => '𐀤',
            LinearBSyllabary::LinearBSyllableB021Qi => '𐀥',
            LinearBSyllabary::LinearBSyllableB032Qo => '𐀦',
            LinearBSyllabary::LinearBSyllableB060Ra => '𐀨',
            LinearBSyllabary::LinearBSyllableB027Re => '𐀩',
            LinearBSyllabary::LinearBSyllableB053Ri => '𐀪',
            LinearBSyllabary::LinearBSyllableB002Ro => '𐀫',
            LinearBSyllabary::LinearBSyllableB026Ru => '𐀬',
            LinearBSyllabary::LinearBSyllableB031Sa => '𐀭',
            LinearBSyllabary::LinearBSyllableB009Se => '𐀮',
            LinearBSyllabary::LinearBSyllableB041Si => '𐀯',
            LinearBSyllabary::LinearBSyllableB012So => '𐀰',
            LinearBSyllabary::LinearBSyllableB058Su => '𐀱',
            LinearBSyllabary::LinearBSyllableB059Ta => '𐀲',
            LinearBSyllabary::LinearBSyllableB004Te => '𐀳',
            LinearBSyllabary::LinearBSyllableB037Ti => '𐀴',
            LinearBSyllabary::LinearBSyllableB005To => '𐀵',
            LinearBSyllabary::LinearBSyllableB069Tu => '𐀶',
            LinearBSyllabary::LinearBSyllableB054Wa => '𐀷',
            LinearBSyllabary::LinearBSyllableB075We => '𐀸',
            LinearBSyllabary::LinearBSyllableB040Wi => '𐀹',
            LinearBSyllabary::LinearBSyllableB042Wo => '𐀺',
            LinearBSyllabary::LinearBSyllableB017Za => '𐀼',
            LinearBSyllabary::LinearBSyllableB074Ze => '𐀽',
            LinearBSyllabary::LinearBSyllableB020Zo => '𐀿',
            LinearBSyllabary::LinearBSyllableB025A2 => '𐁀',
            LinearBSyllabary::LinearBSyllableB043A3 => '𐁁',
            LinearBSyllabary::LinearBSyllableB085Au => '𐁂',
            LinearBSyllabary::LinearBSyllableB071Dwe => '𐁃',
            LinearBSyllabary::LinearBSyllableB090Dwo => '𐁄',
            LinearBSyllabary::LinearBSyllableB048Nwa => '𐁅',
            LinearBSyllabary::LinearBSyllableB029Pu2 => '𐁆',
            LinearBSyllabary::LinearBSyllableB062Pte => '𐁇',
            LinearBSyllabary::LinearBSyllableB076Ra2 => '𐁈',
            LinearBSyllabary::LinearBSyllableB033Ra3 => '𐁉',
            LinearBSyllabary::LinearBSyllableB068Ro2 => '𐁊',
            LinearBSyllabary::LinearBSyllableB066Ta2 => '𐁋',
            LinearBSyllabary::LinearBSyllableB087Twe => '𐁌',
            LinearBSyllabary::LinearBSyllableB091Two => '𐁍',
            LinearBSyllabary::LinearBSymbolB018 => '𐁐',
            LinearBSyllabary::LinearBSymbolB019 => '𐁑',
            LinearBSyllabary::LinearBSymbolB022 => '𐁒',
            LinearBSyllabary::LinearBSymbolB034 => '𐁓',
            LinearBSyllabary::LinearBSymbolB047 => '𐁔',
            LinearBSyllabary::LinearBSymbolB049 => '𐁕',
            LinearBSyllabary::LinearBSymbolB056 => '𐁖',
            LinearBSyllabary::LinearBSymbolB063 => '𐁗',
            LinearBSyllabary::LinearBSymbolB064 => '𐁘',
            LinearBSyllabary::LinearBSymbolB079 => '𐁙',
            LinearBSyllabary::LinearBSymbolB082 => '𐁚',
            LinearBSyllabary::LinearBSymbolB083 => '𐁛',
            LinearBSyllabary::LinearBSymbolB086 => '𐁜',
            LinearBSyllabary::LinearBSymbolB089 => '𐁝',
        }
    }
}

impl std::convert::TryFrom<char> for LinearBSyllabary {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐀀' => Ok(LinearBSyllabary::LinearBSyllableB008A),
            '𐀁' => Ok(LinearBSyllabary::LinearBSyllableB038E),
            '𐀂' => Ok(LinearBSyllabary::LinearBSyllableB028I),
            '𐀃' => Ok(LinearBSyllabary::LinearBSyllableB061O),
            '𐀄' => Ok(LinearBSyllabary::LinearBSyllableB010U),
            '𐀅' => Ok(LinearBSyllabary::LinearBSyllableB001Da),
            '𐀆' => Ok(LinearBSyllabary::LinearBSyllableB045De),
            '𐀇' => Ok(LinearBSyllabary::LinearBSyllableB007Di),
            '𐀈' => Ok(LinearBSyllabary::LinearBSyllableB014Do),
            '𐀉' => Ok(LinearBSyllabary::LinearBSyllableB051Du),
            '𐀊' => Ok(LinearBSyllabary::LinearBSyllableB057Ja),
            '𐀋' => Ok(LinearBSyllabary::LinearBSyllableB046Je),
            '𐀍' => Ok(LinearBSyllabary::LinearBSyllableB036Jo),
            '𐀎' => Ok(LinearBSyllabary::LinearBSyllableB065Ju),
            '𐀏' => Ok(LinearBSyllabary::LinearBSyllableB077Ka),
            '𐀐' => Ok(LinearBSyllabary::LinearBSyllableB044Ke),
            '𐀑' => Ok(LinearBSyllabary::LinearBSyllableB067Ki),
            '𐀒' => Ok(LinearBSyllabary::LinearBSyllableB070Ko),
            '𐀓' => Ok(LinearBSyllabary::LinearBSyllableB081Ku),
            '𐀔' => Ok(LinearBSyllabary::LinearBSyllableB080Ma),
            '𐀕' => Ok(LinearBSyllabary::LinearBSyllableB013Me),
            '𐀖' => Ok(LinearBSyllabary::LinearBSyllableB073Mi),
            '𐀗' => Ok(LinearBSyllabary::LinearBSyllableB015Mo),
            '𐀘' => Ok(LinearBSyllabary::LinearBSyllableB023Mu),
            '𐀙' => Ok(LinearBSyllabary::LinearBSyllableB006Na),
            '𐀚' => Ok(LinearBSyllabary::LinearBSyllableB024Ne),
            '𐀛' => Ok(LinearBSyllabary::LinearBSyllableB030Ni),
            '𐀜' => Ok(LinearBSyllabary::LinearBSyllableB052No),
            '𐀝' => Ok(LinearBSyllabary::LinearBSyllableB055Nu),
            '𐀞' => Ok(LinearBSyllabary::LinearBSyllableB003Pa),
            '𐀟' => Ok(LinearBSyllabary::LinearBSyllableB072Pe),
            '𐀠' => Ok(LinearBSyllabary::LinearBSyllableB039Pi),
            '𐀡' => Ok(LinearBSyllabary::LinearBSyllableB011Po),
            '𐀢' => Ok(LinearBSyllabary::LinearBSyllableB050Pu),
            '𐀣' => Ok(LinearBSyllabary::LinearBSyllableB016Qa),
            '𐀤' => Ok(LinearBSyllabary::LinearBSyllableB078Qe),
            '𐀥' => Ok(LinearBSyllabary::LinearBSyllableB021Qi),
            '𐀦' => Ok(LinearBSyllabary::LinearBSyllableB032Qo),
            '𐀨' => Ok(LinearBSyllabary::LinearBSyllableB060Ra),
            '𐀩' => Ok(LinearBSyllabary::LinearBSyllableB027Re),
            '𐀪' => Ok(LinearBSyllabary::LinearBSyllableB053Ri),
            '𐀫' => Ok(LinearBSyllabary::LinearBSyllableB002Ro),
            '𐀬' => Ok(LinearBSyllabary::LinearBSyllableB026Ru),
            '𐀭' => Ok(LinearBSyllabary::LinearBSyllableB031Sa),
            '𐀮' => Ok(LinearBSyllabary::LinearBSyllableB009Se),
            '𐀯' => Ok(LinearBSyllabary::LinearBSyllableB041Si),
            '𐀰' => Ok(LinearBSyllabary::LinearBSyllableB012So),
            '𐀱' => Ok(LinearBSyllabary::LinearBSyllableB058Su),
            '𐀲' => Ok(LinearBSyllabary::LinearBSyllableB059Ta),
            '𐀳' => Ok(LinearBSyllabary::LinearBSyllableB004Te),
            '𐀴' => Ok(LinearBSyllabary::LinearBSyllableB037Ti),
            '𐀵' => Ok(LinearBSyllabary::LinearBSyllableB005To),
            '𐀶' => Ok(LinearBSyllabary::LinearBSyllableB069Tu),
            '𐀷' => Ok(LinearBSyllabary::LinearBSyllableB054Wa),
            '𐀸' => Ok(LinearBSyllabary::LinearBSyllableB075We),
            '𐀹' => Ok(LinearBSyllabary::LinearBSyllableB040Wi),
            '𐀺' => Ok(LinearBSyllabary::LinearBSyllableB042Wo),
            '𐀼' => Ok(LinearBSyllabary::LinearBSyllableB017Za),
            '𐀽' => Ok(LinearBSyllabary::LinearBSyllableB074Ze),
            '𐀿' => Ok(LinearBSyllabary::LinearBSyllableB020Zo),
            '𐁀' => Ok(LinearBSyllabary::LinearBSyllableB025A2),
            '𐁁' => Ok(LinearBSyllabary::LinearBSyllableB043A3),
            '𐁂' => Ok(LinearBSyllabary::LinearBSyllableB085Au),
            '𐁃' => Ok(LinearBSyllabary::LinearBSyllableB071Dwe),
            '𐁄' => Ok(LinearBSyllabary::LinearBSyllableB090Dwo),
            '𐁅' => Ok(LinearBSyllabary::LinearBSyllableB048Nwa),
            '𐁆' => Ok(LinearBSyllabary::LinearBSyllableB029Pu2),
            '𐁇' => Ok(LinearBSyllabary::LinearBSyllableB062Pte),
            '𐁈' => Ok(LinearBSyllabary::LinearBSyllableB076Ra2),
            '𐁉' => Ok(LinearBSyllabary::LinearBSyllableB033Ra3),
            '𐁊' => Ok(LinearBSyllabary::LinearBSyllableB068Ro2),
            '𐁋' => Ok(LinearBSyllabary::LinearBSyllableB066Ta2),
            '𐁌' => Ok(LinearBSyllabary::LinearBSyllableB087Twe),
            '𐁍' => Ok(LinearBSyllabary::LinearBSyllableB091Two),
            '𐁐' => Ok(LinearBSyllabary::LinearBSymbolB018),
            '𐁑' => Ok(LinearBSyllabary::LinearBSymbolB019),
            '𐁒' => Ok(LinearBSyllabary::LinearBSymbolB022),
            '𐁓' => Ok(LinearBSyllabary::LinearBSymbolB034),
            '𐁔' => Ok(LinearBSyllabary::LinearBSymbolB047),
            '𐁕' => Ok(LinearBSyllabary::LinearBSymbolB049),
            '𐁖' => Ok(LinearBSyllabary::LinearBSymbolB056),
            '𐁗' => Ok(LinearBSyllabary::LinearBSymbolB063),
            '𐁘' => Ok(LinearBSyllabary::LinearBSymbolB064),
            '𐁙' => Ok(LinearBSyllabary::LinearBSymbolB079),
            '𐁚' => Ok(LinearBSyllabary::LinearBSymbolB082),
            '𐁛' => Ok(LinearBSyllabary::LinearBSymbolB083),
            '𐁜' => Ok(LinearBSyllabary::LinearBSymbolB086),
            '𐁝' => Ok(LinearBSyllabary::LinearBSymbolB089),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LinearBSyllabary {
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

impl std::convert::TryFrom<u32> for LinearBSyllabary {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LinearBSyllabary {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LinearBSyllabary {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LinearBSyllabary::LinearBSyllableB008A
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            LinearBSyllabary::LinearBSyllableB008A => "linear b syllable b008 a",
            LinearBSyllabary::LinearBSyllableB038E => "linear b syllable b038 e",
            LinearBSyllabary::LinearBSyllableB028I => "linear b syllable b028 i",
            LinearBSyllabary::LinearBSyllableB061O => "linear b syllable b061 o",
            LinearBSyllabary::LinearBSyllableB010U => "linear b syllable b010 u",
            LinearBSyllabary::LinearBSyllableB001Da => "linear b syllable b001 da",
            LinearBSyllabary::LinearBSyllableB045De => "linear b syllable b045 de",
            LinearBSyllabary::LinearBSyllableB007Di => "linear b syllable b007 di",
            LinearBSyllabary::LinearBSyllableB014Do => "linear b syllable b014 do",
            LinearBSyllabary::LinearBSyllableB051Du => "linear b syllable b051 du",
            LinearBSyllabary::LinearBSyllableB057Ja => "linear b syllable b057 ja",
            LinearBSyllabary::LinearBSyllableB046Je => "linear b syllable b046 je",
            LinearBSyllabary::LinearBSyllableB036Jo => "linear b syllable b036 jo",
            LinearBSyllabary::LinearBSyllableB065Ju => "linear b syllable b065 ju",
            LinearBSyllabary::LinearBSyllableB077Ka => "linear b syllable b077 ka",
            LinearBSyllabary::LinearBSyllableB044Ke => "linear b syllable b044 ke",
            LinearBSyllabary::LinearBSyllableB067Ki => "linear b syllable b067 ki",
            LinearBSyllabary::LinearBSyllableB070Ko => "linear b syllable b070 ko",
            LinearBSyllabary::LinearBSyllableB081Ku => "linear b syllable b081 ku",
            LinearBSyllabary::LinearBSyllableB080Ma => "linear b syllable b080 ma",
            LinearBSyllabary::LinearBSyllableB013Me => "linear b syllable b013 me",
            LinearBSyllabary::LinearBSyllableB073Mi => "linear b syllable b073 mi",
            LinearBSyllabary::LinearBSyllableB015Mo => "linear b syllable b015 mo",
            LinearBSyllabary::LinearBSyllableB023Mu => "linear b syllable b023 mu",
            LinearBSyllabary::LinearBSyllableB006Na => "linear b syllable b006 na",
            LinearBSyllabary::LinearBSyllableB024Ne => "linear b syllable b024 ne",
            LinearBSyllabary::LinearBSyllableB030Ni => "linear b syllable b030 ni",
            LinearBSyllabary::LinearBSyllableB052No => "linear b syllable b052 no",
            LinearBSyllabary::LinearBSyllableB055Nu => "linear b syllable b055 nu",
            LinearBSyllabary::LinearBSyllableB003Pa => "linear b syllable b003 pa",
            LinearBSyllabary::LinearBSyllableB072Pe => "linear b syllable b072 pe",
            LinearBSyllabary::LinearBSyllableB039Pi => "linear b syllable b039 pi",
            LinearBSyllabary::LinearBSyllableB011Po => "linear b syllable b011 po",
            LinearBSyllabary::LinearBSyllableB050Pu => "linear b syllable b050 pu",
            LinearBSyllabary::LinearBSyllableB016Qa => "linear b syllable b016 qa",
            LinearBSyllabary::LinearBSyllableB078Qe => "linear b syllable b078 qe",
            LinearBSyllabary::LinearBSyllableB021Qi => "linear b syllable b021 qi",
            LinearBSyllabary::LinearBSyllableB032Qo => "linear b syllable b032 qo",
            LinearBSyllabary::LinearBSyllableB060Ra => "linear b syllable b060 ra",
            LinearBSyllabary::LinearBSyllableB027Re => "linear b syllable b027 re",
            LinearBSyllabary::LinearBSyllableB053Ri => "linear b syllable b053 ri",
            LinearBSyllabary::LinearBSyllableB002Ro => "linear b syllable b002 ro",
            LinearBSyllabary::LinearBSyllableB026Ru => "linear b syllable b026 ru",
            LinearBSyllabary::LinearBSyllableB031Sa => "linear b syllable b031 sa",
            LinearBSyllabary::LinearBSyllableB009Se => "linear b syllable b009 se",
            LinearBSyllabary::LinearBSyllableB041Si => "linear b syllable b041 si",
            LinearBSyllabary::LinearBSyllableB012So => "linear b syllable b012 so",
            LinearBSyllabary::LinearBSyllableB058Su => "linear b syllable b058 su",
            LinearBSyllabary::LinearBSyllableB059Ta => "linear b syllable b059 ta",
            LinearBSyllabary::LinearBSyllableB004Te => "linear b syllable b004 te",
            LinearBSyllabary::LinearBSyllableB037Ti => "linear b syllable b037 ti",
            LinearBSyllabary::LinearBSyllableB005To => "linear b syllable b005 to",
            LinearBSyllabary::LinearBSyllableB069Tu => "linear b syllable b069 tu",
            LinearBSyllabary::LinearBSyllableB054Wa => "linear b syllable b054 wa",
            LinearBSyllabary::LinearBSyllableB075We => "linear b syllable b075 we",
            LinearBSyllabary::LinearBSyllableB040Wi => "linear b syllable b040 wi",
            LinearBSyllabary::LinearBSyllableB042Wo => "linear b syllable b042 wo",
            LinearBSyllabary::LinearBSyllableB017Za => "linear b syllable b017 za",
            LinearBSyllabary::LinearBSyllableB074Ze => "linear b syllable b074 ze",
            LinearBSyllabary::LinearBSyllableB020Zo => "linear b syllable b020 zo",
            LinearBSyllabary::LinearBSyllableB025A2 => "linear b syllable b025 a2",
            LinearBSyllabary::LinearBSyllableB043A3 => "linear b syllable b043 a3",
            LinearBSyllabary::LinearBSyllableB085Au => "linear b syllable b085 au",
            LinearBSyllabary::LinearBSyllableB071Dwe => "linear b syllable b071 dwe",
            LinearBSyllabary::LinearBSyllableB090Dwo => "linear b syllable b090 dwo",
            LinearBSyllabary::LinearBSyllableB048Nwa => "linear b syllable b048 nwa",
            LinearBSyllabary::LinearBSyllableB029Pu2 => "linear b syllable b029 pu2",
            LinearBSyllabary::LinearBSyllableB062Pte => "linear b syllable b062 pte",
            LinearBSyllabary::LinearBSyllableB076Ra2 => "linear b syllable b076 ra2",
            LinearBSyllabary::LinearBSyllableB033Ra3 => "linear b syllable b033 ra3",
            LinearBSyllabary::LinearBSyllableB068Ro2 => "linear b syllable b068 ro2",
            LinearBSyllabary::LinearBSyllableB066Ta2 => "linear b syllable b066 ta2",
            LinearBSyllabary::LinearBSyllableB087Twe => "linear b syllable b087 twe",
            LinearBSyllabary::LinearBSyllableB091Two => "linear b syllable b091 two",
            LinearBSyllabary::LinearBSymbolB018 => "linear b symbol b018",
            LinearBSyllabary::LinearBSymbolB019 => "linear b symbol b019",
            LinearBSyllabary::LinearBSymbolB022 => "linear b symbol b022",
            LinearBSyllabary::LinearBSymbolB034 => "linear b symbol b034",
            LinearBSyllabary::LinearBSymbolB047 => "linear b symbol b047",
            LinearBSyllabary::LinearBSymbolB049 => "linear b symbol b049",
            LinearBSyllabary::LinearBSymbolB056 => "linear b symbol b056",
            LinearBSyllabary::LinearBSymbolB063 => "linear b symbol b063",
            LinearBSyllabary::LinearBSymbolB064 => "linear b symbol b064",
            LinearBSyllabary::LinearBSymbolB079 => "linear b symbol b079",
            LinearBSyllabary::LinearBSymbolB082 => "linear b symbol b082",
            LinearBSyllabary::LinearBSymbolB083 => "linear b symbol b083",
            LinearBSyllabary::LinearBSymbolB086 => "linear b symbol b086",
            LinearBSyllabary::LinearBSymbolB089 => "linear b symbol b089",
        }
    }
}
