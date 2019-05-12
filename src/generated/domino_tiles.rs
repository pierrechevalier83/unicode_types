
/// An enum to represent all characters in the DominoTiles block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DominoTiles {
    /// \u{1f030}: '🀰'
    DominoTileHorizontalBack,
    /// \u{1f031}: '🀱'
    DominoTileHorizontalDash00Dash00,
    /// \u{1f032}: '🀲'
    DominoTileHorizontalDash00Dash01,
    /// \u{1f033}: '🀳'
    DominoTileHorizontalDash00Dash02,
    /// \u{1f034}: '🀴'
    DominoTileHorizontalDash00Dash03,
    /// \u{1f035}: '🀵'
    DominoTileHorizontalDash00Dash04,
    /// \u{1f036}: '🀶'
    DominoTileHorizontalDash00Dash05,
    /// \u{1f037}: '🀷'
    DominoTileHorizontalDash00Dash06,
    /// \u{1f038}: '🀸'
    DominoTileHorizontalDash01Dash00,
    /// \u{1f039}: '🀹'
    DominoTileHorizontalDash01Dash01,
    /// \u{1f03a}: '🀺'
    DominoTileHorizontalDash01Dash02,
    /// \u{1f03b}: '🀻'
    DominoTileHorizontalDash01Dash03,
    /// \u{1f03c}: '🀼'
    DominoTileHorizontalDash01Dash04,
    /// \u{1f03d}: '🀽'
    DominoTileHorizontalDash01Dash05,
    /// \u{1f03e}: '🀾'
    DominoTileHorizontalDash01Dash06,
    /// \u{1f03f}: '🀿'
    DominoTileHorizontalDash02Dash00,
    /// \u{1f040}: '🁀'
    DominoTileHorizontalDash02Dash01,
    /// \u{1f041}: '🁁'
    DominoTileHorizontalDash02Dash02,
    /// \u{1f042}: '🁂'
    DominoTileHorizontalDash02Dash03,
    /// \u{1f043}: '🁃'
    DominoTileHorizontalDash02Dash04,
    /// \u{1f044}: '🁄'
    DominoTileHorizontalDash02Dash05,
    /// \u{1f045}: '🁅'
    DominoTileHorizontalDash02Dash06,
    /// \u{1f046}: '🁆'
    DominoTileHorizontalDash03Dash00,
    /// \u{1f047}: '🁇'
    DominoTileHorizontalDash03Dash01,
    /// \u{1f048}: '🁈'
    DominoTileHorizontalDash03Dash02,
    /// \u{1f049}: '🁉'
    DominoTileHorizontalDash03Dash03,
    /// \u{1f04a}: '🁊'
    DominoTileHorizontalDash03Dash04,
    /// \u{1f04b}: '🁋'
    DominoTileHorizontalDash03Dash05,
    /// \u{1f04c}: '🁌'
    DominoTileHorizontalDash03Dash06,
    /// \u{1f04d}: '🁍'
    DominoTileHorizontalDash04Dash00,
    /// \u{1f04e}: '🁎'
    DominoTileHorizontalDash04Dash01,
    /// \u{1f04f}: '🁏'
    DominoTileHorizontalDash04Dash02,
    /// \u{1f050}: '🁐'
    DominoTileHorizontalDash04Dash03,
    /// \u{1f051}: '🁑'
    DominoTileHorizontalDash04Dash04,
    /// \u{1f052}: '🁒'
    DominoTileHorizontalDash04Dash05,
    /// \u{1f053}: '🁓'
    DominoTileHorizontalDash04Dash06,
    /// \u{1f054}: '🁔'
    DominoTileHorizontalDash05Dash00,
    /// \u{1f055}: '🁕'
    DominoTileHorizontalDash05Dash01,
    /// \u{1f056}: '🁖'
    DominoTileHorizontalDash05Dash02,
    /// \u{1f057}: '🁗'
    DominoTileHorizontalDash05Dash03,
    /// \u{1f058}: '🁘'
    DominoTileHorizontalDash05Dash04,
    /// \u{1f059}: '🁙'
    DominoTileHorizontalDash05Dash05,
    /// \u{1f05a}: '🁚'
    DominoTileHorizontalDash05Dash06,
    /// \u{1f05b}: '🁛'
    DominoTileHorizontalDash06Dash00,
    /// \u{1f05c}: '🁜'
    DominoTileHorizontalDash06Dash01,
    /// \u{1f05d}: '🁝'
    DominoTileHorizontalDash06Dash02,
    /// \u{1f05e}: '🁞'
    DominoTileHorizontalDash06Dash03,
    /// \u{1f05f}: '🁟'
    DominoTileHorizontalDash06Dash04,
    /// \u{1f060}: '🁠'
    DominoTileHorizontalDash06Dash05,
    /// \u{1f061}: '🁡'
    DominoTileHorizontalDash06Dash06,
    /// \u{1f062}: '🁢'
    DominoTileVerticalBack,
    /// \u{1f063}: '🁣'
    DominoTileVerticalDash00Dash00,
    /// \u{1f064}: '🁤'
    DominoTileVerticalDash00Dash01,
    /// \u{1f065}: '🁥'
    DominoTileVerticalDash00Dash02,
    /// \u{1f066}: '🁦'
    DominoTileVerticalDash00Dash03,
    /// \u{1f067}: '🁧'
    DominoTileVerticalDash00Dash04,
    /// \u{1f068}: '🁨'
    DominoTileVerticalDash00Dash05,
    /// \u{1f069}: '🁩'
    DominoTileVerticalDash00Dash06,
    /// \u{1f06a}: '🁪'
    DominoTileVerticalDash01Dash00,
    /// \u{1f06b}: '🁫'
    DominoTileVerticalDash01Dash01,
    /// \u{1f06c}: '🁬'
    DominoTileVerticalDash01Dash02,
    /// \u{1f06d}: '🁭'
    DominoTileVerticalDash01Dash03,
    /// \u{1f06e}: '🁮'
    DominoTileVerticalDash01Dash04,
    /// \u{1f06f}: '🁯'
    DominoTileVerticalDash01Dash05,
    /// \u{1f070}: '🁰'
    DominoTileVerticalDash01Dash06,
    /// \u{1f071}: '🁱'
    DominoTileVerticalDash02Dash00,
    /// \u{1f072}: '🁲'
    DominoTileVerticalDash02Dash01,
    /// \u{1f073}: '🁳'
    DominoTileVerticalDash02Dash02,
    /// \u{1f074}: '🁴'
    DominoTileVerticalDash02Dash03,
    /// \u{1f075}: '🁵'
    DominoTileVerticalDash02Dash04,
    /// \u{1f076}: '🁶'
    DominoTileVerticalDash02Dash05,
    /// \u{1f077}: '🁷'
    DominoTileVerticalDash02Dash06,
    /// \u{1f078}: '🁸'
    DominoTileVerticalDash03Dash00,
    /// \u{1f079}: '🁹'
    DominoTileVerticalDash03Dash01,
    /// \u{1f07a}: '🁺'
    DominoTileVerticalDash03Dash02,
    /// \u{1f07b}: '🁻'
    DominoTileVerticalDash03Dash03,
    /// \u{1f07c}: '🁼'
    DominoTileVerticalDash03Dash04,
    /// \u{1f07d}: '🁽'
    DominoTileVerticalDash03Dash05,
    /// \u{1f07e}: '🁾'
    DominoTileVerticalDash03Dash06,
    /// \u{1f07f}: '🁿'
    DominoTileVerticalDash04Dash00,
    /// \u{1f080}: '🂀'
    DominoTileVerticalDash04Dash01,
    /// \u{1f081}: '🂁'
    DominoTileVerticalDash04Dash02,
    /// \u{1f082}: '🂂'
    DominoTileVerticalDash04Dash03,
    /// \u{1f083}: '🂃'
    DominoTileVerticalDash04Dash04,
    /// \u{1f084}: '🂄'
    DominoTileVerticalDash04Dash05,
    /// \u{1f085}: '🂅'
    DominoTileVerticalDash04Dash06,
    /// \u{1f086}: '🂆'
    DominoTileVerticalDash05Dash00,
    /// \u{1f087}: '🂇'
    DominoTileVerticalDash05Dash01,
    /// \u{1f088}: '🂈'
    DominoTileVerticalDash05Dash02,
    /// \u{1f089}: '🂉'
    DominoTileVerticalDash05Dash03,
    /// \u{1f08a}: '🂊'
    DominoTileVerticalDash05Dash04,
    /// \u{1f08b}: '🂋'
    DominoTileVerticalDash05Dash05,
    /// \u{1f08c}: '🂌'
    DominoTileVerticalDash05Dash06,
    /// \u{1f08d}: '🂍'
    DominoTileVerticalDash06Dash00,
    /// \u{1f08e}: '🂎'
    DominoTileVerticalDash06Dash01,
    /// \u{1f08f}: '🂏'
    DominoTileVerticalDash06Dash02,
    /// \u{1f090}: '🂐'
    DominoTileVerticalDash06Dash03,
    /// \u{1f091}: '🂑'
    DominoTileVerticalDash06Dash04,
    /// \u{1f092}: '🂒'
    DominoTileVerticalDash06Dash05,
    /// \u{1f093}: '🂓'
    DominoTileVerticalDash06Dash06,
}

impl Into<char> for DominoTiles {
    fn into(self) -> char {
        match self {
            DominoTiles::DominoTileHorizontalBack => '🀰',
            DominoTiles::DominoTileHorizontalDash00Dash00 => '🀱',
            DominoTiles::DominoTileHorizontalDash00Dash01 => '🀲',
            DominoTiles::DominoTileHorizontalDash00Dash02 => '🀳',
            DominoTiles::DominoTileHorizontalDash00Dash03 => '🀴',
            DominoTiles::DominoTileHorizontalDash00Dash04 => '🀵',
            DominoTiles::DominoTileHorizontalDash00Dash05 => '🀶',
            DominoTiles::DominoTileHorizontalDash00Dash06 => '🀷',
            DominoTiles::DominoTileHorizontalDash01Dash00 => '🀸',
            DominoTiles::DominoTileHorizontalDash01Dash01 => '🀹',
            DominoTiles::DominoTileHorizontalDash01Dash02 => '🀺',
            DominoTiles::DominoTileHorizontalDash01Dash03 => '🀻',
            DominoTiles::DominoTileHorizontalDash01Dash04 => '🀼',
            DominoTiles::DominoTileHorizontalDash01Dash05 => '🀽',
            DominoTiles::DominoTileHorizontalDash01Dash06 => '🀾',
            DominoTiles::DominoTileHorizontalDash02Dash00 => '🀿',
            DominoTiles::DominoTileHorizontalDash02Dash01 => '🁀',
            DominoTiles::DominoTileHorizontalDash02Dash02 => '🁁',
            DominoTiles::DominoTileHorizontalDash02Dash03 => '🁂',
            DominoTiles::DominoTileHorizontalDash02Dash04 => '🁃',
            DominoTiles::DominoTileHorizontalDash02Dash05 => '🁄',
            DominoTiles::DominoTileHorizontalDash02Dash06 => '🁅',
            DominoTiles::DominoTileHorizontalDash03Dash00 => '🁆',
            DominoTiles::DominoTileHorizontalDash03Dash01 => '🁇',
            DominoTiles::DominoTileHorizontalDash03Dash02 => '🁈',
            DominoTiles::DominoTileHorizontalDash03Dash03 => '🁉',
            DominoTiles::DominoTileHorizontalDash03Dash04 => '🁊',
            DominoTiles::DominoTileHorizontalDash03Dash05 => '🁋',
            DominoTiles::DominoTileHorizontalDash03Dash06 => '🁌',
            DominoTiles::DominoTileHorizontalDash04Dash00 => '🁍',
            DominoTiles::DominoTileHorizontalDash04Dash01 => '🁎',
            DominoTiles::DominoTileHorizontalDash04Dash02 => '🁏',
            DominoTiles::DominoTileHorizontalDash04Dash03 => '🁐',
            DominoTiles::DominoTileHorizontalDash04Dash04 => '🁑',
            DominoTiles::DominoTileHorizontalDash04Dash05 => '🁒',
            DominoTiles::DominoTileHorizontalDash04Dash06 => '🁓',
            DominoTiles::DominoTileHorizontalDash05Dash00 => '🁔',
            DominoTiles::DominoTileHorizontalDash05Dash01 => '🁕',
            DominoTiles::DominoTileHorizontalDash05Dash02 => '🁖',
            DominoTiles::DominoTileHorizontalDash05Dash03 => '🁗',
            DominoTiles::DominoTileHorizontalDash05Dash04 => '🁘',
            DominoTiles::DominoTileHorizontalDash05Dash05 => '🁙',
            DominoTiles::DominoTileHorizontalDash05Dash06 => '🁚',
            DominoTiles::DominoTileHorizontalDash06Dash00 => '🁛',
            DominoTiles::DominoTileHorizontalDash06Dash01 => '🁜',
            DominoTiles::DominoTileHorizontalDash06Dash02 => '🁝',
            DominoTiles::DominoTileHorizontalDash06Dash03 => '🁞',
            DominoTiles::DominoTileHorizontalDash06Dash04 => '🁟',
            DominoTiles::DominoTileHorizontalDash06Dash05 => '🁠',
            DominoTiles::DominoTileHorizontalDash06Dash06 => '🁡',
            DominoTiles::DominoTileVerticalBack => '🁢',
            DominoTiles::DominoTileVerticalDash00Dash00 => '🁣',
            DominoTiles::DominoTileVerticalDash00Dash01 => '🁤',
            DominoTiles::DominoTileVerticalDash00Dash02 => '🁥',
            DominoTiles::DominoTileVerticalDash00Dash03 => '🁦',
            DominoTiles::DominoTileVerticalDash00Dash04 => '🁧',
            DominoTiles::DominoTileVerticalDash00Dash05 => '🁨',
            DominoTiles::DominoTileVerticalDash00Dash06 => '🁩',
            DominoTiles::DominoTileVerticalDash01Dash00 => '🁪',
            DominoTiles::DominoTileVerticalDash01Dash01 => '🁫',
            DominoTiles::DominoTileVerticalDash01Dash02 => '🁬',
            DominoTiles::DominoTileVerticalDash01Dash03 => '🁭',
            DominoTiles::DominoTileVerticalDash01Dash04 => '🁮',
            DominoTiles::DominoTileVerticalDash01Dash05 => '🁯',
            DominoTiles::DominoTileVerticalDash01Dash06 => '🁰',
            DominoTiles::DominoTileVerticalDash02Dash00 => '🁱',
            DominoTiles::DominoTileVerticalDash02Dash01 => '🁲',
            DominoTiles::DominoTileVerticalDash02Dash02 => '🁳',
            DominoTiles::DominoTileVerticalDash02Dash03 => '🁴',
            DominoTiles::DominoTileVerticalDash02Dash04 => '🁵',
            DominoTiles::DominoTileVerticalDash02Dash05 => '🁶',
            DominoTiles::DominoTileVerticalDash02Dash06 => '🁷',
            DominoTiles::DominoTileVerticalDash03Dash00 => '🁸',
            DominoTiles::DominoTileVerticalDash03Dash01 => '🁹',
            DominoTiles::DominoTileVerticalDash03Dash02 => '🁺',
            DominoTiles::DominoTileVerticalDash03Dash03 => '🁻',
            DominoTiles::DominoTileVerticalDash03Dash04 => '🁼',
            DominoTiles::DominoTileVerticalDash03Dash05 => '🁽',
            DominoTiles::DominoTileVerticalDash03Dash06 => '🁾',
            DominoTiles::DominoTileVerticalDash04Dash00 => '🁿',
            DominoTiles::DominoTileVerticalDash04Dash01 => '🂀',
            DominoTiles::DominoTileVerticalDash04Dash02 => '🂁',
            DominoTiles::DominoTileVerticalDash04Dash03 => '🂂',
            DominoTiles::DominoTileVerticalDash04Dash04 => '🂃',
            DominoTiles::DominoTileVerticalDash04Dash05 => '🂄',
            DominoTiles::DominoTileVerticalDash04Dash06 => '🂅',
            DominoTiles::DominoTileVerticalDash05Dash00 => '🂆',
            DominoTiles::DominoTileVerticalDash05Dash01 => '🂇',
            DominoTiles::DominoTileVerticalDash05Dash02 => '🂈',
            DominoTiles::DominoTileVerticalDash05Dash03 => '🂉',
            DominoTiles::DominoTileVerticalDash05Dash04 => '🂊',
            DominoTiles::DominoTileVerticalDash05Dash05 => '🂋',
            DominoTiles::DominoTileVerticalDash05Dash06 => '🂌',
            DominoTiles::DominoTileVerticalDash06Dash00 => '🂍',
            DominoTiles::DominoTileVerticalDash06Dash01 => '🂎',
            DominoTiles::DominoTileVerticalDash06Dash02 => '🂏',
            DominoTiles::DominoTileVerticalDash06Dash03 => '🂐',
            DominoTiles::DominoTileVerticalDash06Dash04 => '🂑',
            DominoTiles::DominoTileVerticalDash06Dash05 => '🂒',
            DominoTiles::DominoTileVerticalDash06Dash06 => '🂓',
        }
    }
}

impl std::convert::TryFrom<char> for DominoTiles {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🀰' => Ok(DominoTiles::DominoTileHorizontalBack),
            '🀱' => Ok(DominoTiles::DominoTileHorizontalDash00Dash00),
            '🀲' => Ok(DominoTiles::DominoTileHorizontalDash00Dash01),
            '🀳' => Ok(DominoTiles::DominoTileHorizontalDash00Dash02),
            '🀴' => Ok(DominoTiles::DominoTileHorizontalDash00Dash03),
            '🀵' => Ok(DominoTiles::DominoTileHorizontalDash00Dash04),
            '🀶' => Ok(DominoTiles::DominoTileHorizontalDash00Dash05),
            '🀷' => Ok(DominoTiles::DominoTileHorizontalDash00Dash06),
            '🀸' => Ok(DominoTiles::DominoTileHorizontalDash01Dash00),
            '🀹' => Ok(DominoTiles::DominoTileHorizontalDash01Dash01),
            '🀺' => Ok(DominoTiles::DominoTileHorizontalDash01Dash02),
            '🀻' => Ok(DominoTiles::DominoTileHorizontalDash01Dash03),
            '🀼' => Ok(DominoTiles::DominoTileHorizontalDash01Dash04),
            '🀽' => Ok(DominoTiles::DominoTileHorizontalDash01Dash05),
            '🀾' => Ok(DominoTiles::DominoTileHorizontalDash01Dash06),
            '🀿' => Ok(DominoTiles::DominoTileHorizontalDash02Dash00),
            '🁀' => Ok(DominoTiles::DominoTileHorizontalDash02Dash01),
            '🁁' => Ok(DominoTiles::DominoTileHorizontalDash02Dash02),
            '🁂' => Ok(DominoTiles::DominoTileHorizontalDash02Dash03),
            '🁃' => Ok(DominoTiles::DominoTileHorizontalDash02Dash04),
            '🁄' => Ok(DominoTiles::DominoTileHorizontalDash02Dash05),
            '🁅' => Ok(DominoTiles::DominoTileHorizontalDash02Dash06),
            '🁆' => Ok(DominoTiles::DominoTileHorizontalDash03Dash00),
            '🁇' => Ok(DominoTiles::DominoTileHorizontalDash03Dash01),
            '🁈' => Ok(DominoTiles::DominoTileHorizontalDash03Dash02),
            '🁉' => Ok(DominoTiles::DominoTileHorizontalDash03Dash03),
            '🁊' => Ok(DominoTiles::DominoTileHorizontalDash03Dash04),
            '🁋' => Ok(DominoTiles::DominoTileHorizontalDash03Dash05),
            '🁌' => Ok(DominoTiles::DominoTileHorizontalDash03Dash06),
            '🁍' => Ok(DominoTiles::DominoTileHorizontalDash04Dash00),
            '🁎' => Ok(DominoTiles::DominoTileHorizontalDash04Dash01),
            '🁏' => Ok(DominoTiles::DominoTileHorizontalDash04Dash02),
            '🁐' => Ok(DominoTiles::DominoTileHorizontalDash04Dash03),
            '🁑' => Ok(DominoTiles::DominoTileHorizontalDash04Dash04),
            '🁒' => Ok(DominoTiles::DominoTileHorizontalDash04Dash05),
            '🁓' => Ok(DominoTiles::DominoTileHorizontalDash04Dash06),
            '🁔' => Ok(DominoTiles::DominoTileHorizontalDash05Dash00),
            '🁕' => Ok(DominoTiles::DominoTileHorizontalDash05Dash01),
            '🁖' => Ok(DominoTiles::DominoTileHorizontalDash05Dash02),
            '🁗' => Ok(DominoTiles::DominoTileHorizontalDash05Dash03),
            '🁘' => Ok(DominoTiles::DominoTileHorizontalDash05Dash04),
            '🁙' => Ok(DominoTiles::DominoTileHorizontalDash05Dash05),
            '🁚' => Ok(DominoTiles::DominoTileHorizontalDash05Dash06),
            '🁛' => Ok(DominoTiles::DominoTileHorizontalDash06Dash00),
            '🁜' => Ok(DominoTiles::DominoTileHorizontalDash06Dash01),
            '🁝' => Ok(DominoTiles::DominoTileHorizontalDash06Dash02),
            '🁞' => Ok(DominoTiles::DominoTileHorizontalDash06Dash03),
            '🁟' => Ok(DominoTiles::DominoTileHorizontalDash06Dash04),
            '🁠' => Ok(DominoTiles::DominoTileHorizontalDash06Dash05),
            '🁡' => Ok(DominoTiles::DominoTileHorizontalDash06Dash06),
            '🁢' => Ok(DominoTiles::DominoTileVerticalBack),
            '🁣' => Ok(DominoTiles::DominoTileVerticalDash00Dash00),
            '🁤' => Ok(DominoTiles::DominoTileVerticalDash00Dash01),
            '🁥' => Ok(DominoTiles::DominoTileVerticalDash00Dash02),
            '🁦' => Ok(DominoTiles::DominoTileVerticalDash00Dash03),
            '🁧' => Ok(DominoTiles::DominoTileVerticalDash00Dash04),
            '🁨' => Ok(DominoTiles::DominoTileVerticalDash00Dash05),
            '🁩' => Ok(DominoTiles::DominoTileVerticalDash00Dash06),
            '🁪' => Ok(DominoTiles::DominoTileVerticalDash01Dash00),
            '🁫' => Ok(DominoTiles::DominoTileVerticalDash01Dash01),
            '🁬' => Ok(DominoTiles::DominoTileVerticalDash01Dash02),
            '🁭' => Ok(DominoTiles::DominoTileVerticalDash01Dash03),
            '🁮' => Ok(DominoTiles::DominoTileVerticalDash01Dash04),
            '🁯' => Ok(DominoTiles::DominoTileVerticalDash01Dash05),
            '🁰' => Ok(DominoTiles::DominoTileVerticalDash01Dash06),
            '🁱' => Ok(DominoTiles::DominoTileVerticalDash02Dash00),
            '🁲' => Ok(DominoTiles::DominoTileVerticalDash02Dash01),
            '🁳' => Ok(DominoTiles::DominoTileVerticalDash02Dash02),
            '🁴' => Ok(DominoTiles::DominoTileVerticalDash02Dash03),
            '🁵' => Ok(DominoTiles::DominoTileVerticalDash02Dash04),
            '🁶' => Ok(DominoTiles::DominoTileVerticalDash02Dash05),
            '🁷' => Ok(DominoTiles::DominoTileVerticalDash02Dash06),
            '🁸' => Ok(DominoTiles::DominoTileVerticalDash03Dash00),
            '🁹' => Ok(DominoTiles::DominoTileVerticalDash03Dash01),
            '🁺' => Ok(DominoTiles::DominoTileVerticalDash03Dash02),
            '🁻' => Ok(DominoTiles::DominoTileVerticalDash03Dash03),
            '🁼' => Ok(DominoTiles::DominoTileVerticalDash03Dash04),
            '🁽' => Ok(DominoTiles::DominoTileVerticalDash03Dash05),
            '🁾' => Ok(DominoTiles::DominoTileVerticalDash03Dash06),
            '🁿' => Ok(DominoTiles::DominoTileVerticalDash04Dash00),
            '🂀' => Ok(DominoTiles::DominoTileVerticalDash04Dash01),
            '🂁' => Ok(DominoTiles::DominoTileVerticalDash04Dash02),
            '🂂' => Ok(DominoTiles::DominoTileVerticalDash04Dash03),
            '🂃' => Ok(DominoTiles::DominoTileVerticalDash04Dash04),
            '🂄' => Ok(DominoTiles::DominoTileVerticalDash04Dash05),
            '🂅' => Ok(DominoTiles::DominoTileVerticalDash04Dash06),
            '🂆' => Ok(DominoTiles::DominoTileVerticalDash05Dash00),
            '🂇' => Ok(DominoTiles::DominoTileVerticalDash05Dash01),
            '🂈' => Ok(DominoTiles::DominoTileVerticalDash05Dash02),
            '🂉' => Ok(DominoTiles::DominoTileVerticalDash05Dash03),
            '🂊' => Ok(DominoTiles::DominoTileVerticalDash05Dash04),
            '🂋' => Ok(DominoTiles::DominoTileVerticalDash05Dash05),
            '🂌' => Ok(DominoTiles::DominoTileVerticalDash05Dash06),
            '🂍' => Ok(DominoTiles::DominoTileVerticalDash06Dash00),
            '🂎' => Ok(DominoTiles::DominoTileVerticalDash06Dash01),
            '🂏' => Ok(DominoTiles::DominoTileVerticalDash06Dash02),
            '🂐' => Ok(DominoTiles::DominoTileVerticalDash06Dash03),
            '🂑' => Ok(DominoTiles::DominoTileVerticalDash06Dash04),
            '🂒' => Ok(DominoTiles::DominoTileVerticalDash06Dash05),
            '🂓' => Ok(DominoTiles::DominoTileVerticalDash06Dash06),
            _ => Err(()),
        }
    }
}

impl Into<u32> for DominoTiles {
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

impl std::convert::TryFrom<u32> for DominoTiles {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for DominoTiles {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl DominoTiles {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        DominoTiles::DominoTileHorizontalBack
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("DominoTiles{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
