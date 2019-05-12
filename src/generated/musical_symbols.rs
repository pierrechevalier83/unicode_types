/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{1d100}: '𝄀'
    pub const MUSICAL_SYMBOL_SINGLE_BARLINE: char = '𝄀';
    /// \u{1d101}: '𝄁'
    pub const MUSICAL_SYMBOL_DOUBLE_BARLINE: char = '𝄁';
    /// \u{1d102}: '𝄂'
    pub const MUSICAL_SYMBOL_FINAL_BARLINE: char = '𝄂';
    /// \u{1d103}: '𝄃'
    pub const MUSICAL_SYMBOL_REVERSE_FINAL_BARLINE: char = '𝄃';
    /// \u{1d104}: '𝄄'
    pub const MUSICAL_SYMBOL_DASHED_BARLINE: char = '𝄄';
    /// \u{1d105}: '𝄅'
    pub const MUSICAL_SYMBOL_SHORT_BARLINE: char = '𝄅';
    /// \u{1d106}: '𝄆'
    pub const MUSICAL_SYMBOL_LEFT_REPEAT_SIGN: char = '𝄆';
    /// \u{1d107}: '𝄇'
    pub const MUSICAL_SYMBOL_RIGHT_REPEAT_SIGN: char = '𝄇';
    /// \u{1d108}: '𝄈'
    pub const MUSICAL_SYMBOL_REPEAT_DOTS: char = '𝄈';
    /// \u{1d109}: '𝄉'
    pub const MUSICAL_SYMBOL_DAL_SEGNO: char = '𝄉';
    /// \u{1d10a}: '𝄊'
    pub const MUSICAL_SYMBOL_DA_CAPO: char = '𝄊';
    /// \u{1d10b}: '𝄋'
    pub const MUSICAL_SYMBOL_SEGNO: char = '𝄋';
    /// \u{1d10c}: '𝄌'
    pub const MUSICAL_SYMBOL_CODA: char = '𝄌';
    /// \u{1d10d}: '𝄍'
    pub const MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_1: char = '𝄍';
    /// \u{1d10e}: '𝄎'
    pub const MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_2: char = '𝄎';
    /// \u{1d10f}: '𝄏'
    pub const MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_3: char = '𝄏';
    /// \u{1d110}: '𝄐'
    pub const MUSICAL_SYMBOL_FERMATA: char = '𝄐';
    /// \u{1d111}: '𝄑'
    pub const MUSICAL_SYMBOL_FERMATA_BELOW: char = '𝄑';
    /// \u{1d112}: '𝄒'
    pub const MUSICAL_SYMBOL_BREATH_MARK: char = '𝄒';
    /// \u{1d113}: '𝄓'
    pub const MUSICAL_SYMBOL_CAESURA: char = '𝄓';
    /// \u{1d114}: '𝄔'
    pub const MUSICAL_SYMBOL_BRACE: char = '𝄔';
    /// \u{1d115}: '𝄕'
    pub const MUSICAL_SYMBOL_BRACKET: char = '𝄕';
    /// \u{1d116}: '𝄖'
    pub const MUSICAL_SYMBOL_ONE_DASH_LINE_STAFF: char = '𝄖';
    /// \u{1d117}: '𝄗'
    pub const MUSICAL_SYMBOL_TWO_DASH_LINE_STAFF: char = '𝄗';
    /// \u{1d118}: '𝄘'
    pub const MUSICAL_SYMBOL_THREE_DASH_LINE_STAFF: char = '𝄘';
    /// \u{1d119}: '𝄙'
    pub const MUSICAL_SYMBOL_FOUR_DASH_LINE_STAFF: char = '𝄙';
    /// \u{1d11a}: '𝄚'
    pub const MUSICAL_SYMBOL_FIVE_DASH_LINE_STAFF: char = '𝄚';
    /// \u{1d11b}: '𝄛'
    pub const MUSICAL_SYMBOL_SIX_DASH_LINE_STAFF: char = '𝄛';
    /// \u{1d11c}: '𝄜'
    pub const MUSICAL_SYMBOL_SIX_DASH_STRING_FRETBOARD: char = '𝄜';
    /// \u{1d11d}: '𝄝'
    pub const MUSICAL_SYMBOL_FOUR_DASH_STRING_FRETBOARD: char = '𝄝';
    /// \u{1d11e}: '𝄞'
    pub const MUSICAL_SYMBOL_G_CLEF: char = '𝄞';
    /// \u{1d11f}: '𝄟'
    pub const MUSICAL_SYMBOL_G_CLEF_OTTAVA_ALTA: char = '𝄟';
    /// \u{1d120}: '𝄠'
    pub const MUSICAL_SYMBOL_G_CLEF_OTTAVA_BASSA: char = '𝄠';
    /// \u{1d121}: '𝄡'
    pub const MUSICAL_SYMBOL_C_CLEF: char = '𝄡';
    /// \u{1d122}: '𝄢'
    pub const MUSICAL_SYMBOL_F_CLEF: char = '𝄢';
    /// \u{1d123}: '𝄣'
    pub const MUSICAL_SYMBOL_F_CLEF_OTTAVA_ALTA: char = '𝄣';
    /// \u{1d124}: '𝄤'
    pub const MUSICAL_SYMBOL_F_CLEF_OTTAVA_BASSA: char = '𝄤';
    /// \u{1d125}: '𝄥'
    pub const MUSICAL_SYMBOL_DRUM_CLEF_DASH_1: char = '𝄥';
    /// \u{1d126}: '𝄦'
    pub const MUSICAL_SYMBOL_DRUM_CLEF_DASH_2: char = '𝄦';
    /// \u{1d129}: '𝄩'
    pub const MUSICAL_SYMBOL_MULTIPLE_MEASURE_REST: char = '𝄩';
    /// \u{1d12a}: '𝄪'
    pub const MUSICAL_SYMBOL_DOUBLE_SHARP: char = '𝄪';
    /// \u{1d12b}: '𝄫'
    pub const MUSICAL_SYMBOL_DOUBLE_FLAT: char = '𝄫';
    /// \u{1d12c}: '𝄬'
    pub const MUSICAL_SYMBOL_FLAT_UP: char = '𝄬';
    /// \u{1d12d}: '𝄭'
    pub const MUSICAL_SYMBOL_FLAT_DOWN: char = '𝄭';
    /// \u{1d12e}: '𝄮'
    pub const MUSICAL_SYMBOL_NATURAL_UP: char = '𝄮';
    /// \u{1d12f}: '𝄯'
    pub const MUSICAL_SYMBOL_NATURAL_DOWN: char = '𝄯';
    /// \u{1d130}: '𝄰'
    pub const MUSICAL_SYMBOL_SHARP_UP: char = '𝄰';
    /// \u{1d131}: '𝄱'
    pub const MUSICAL_SYMBOL_SHARP_DOWN: char = '𝄱';
    /// \u{1d132}: '𝄲'
    pub const MUSICAL_SYMBOL_QUARTER_TONE_SHARP: char = '𝄲';
    /// \u{1d133}: '𝄳'
    pub const MUSICAL_SYMBOL_QUARTER_TONE_FLAT: char = '𝄳';
    /// \u{1d134}: '𝄴'
    pub const MUSICAL_SYMBOL_COMMON_TIME: char = '𝄴';
    /// \u{1d135}: '𝄵'
    pub const MUSICAL_SYMBOL_CUT_TIME: char = '𝄵';
    /// \u{1d136}: '𝄶'
    pub const MUSICAL_SYMBOL_OTTAVA_ALTA: char = '𝄶';
    /// \u{1d137}: '𝄷'
    pub const MUSICAL_SYMBOL_OTTAVA_BASSA: char = '𝄷';
    /// \u{1d138}: '𝄸'
    pub const MUSICAL_SYMBOL_QUINDICESIMA_ALTA: char = '𝄸';
    /// \u{1d139}: '𝄹'
    pub const MUSICAL_SYMBOL_QUINDICESIMA_BASSA: char = '𝄹';
    /// \u{1d13a}: '𝄺'
    pub const MUSICAL_SYMBOL_MULTI_REST: char = '𝄺';
    /// \u{1d13b}: '𝄻'
    pub const MUSICAL_SYMBOL_WHOLE_REST: char = '𝄻';
    /// \u{1d13c}: '𝄼'
    pub const MUSICAL_SYMBOL_HALF_REST: char = '𝄼';
    /// \u{1d13d}: '𝄽'
    pub const MUSICAL_SYMBOL_QUARTER_REST: char = '𝄽';
    /// \u{1d13e}: '𝄾'
    pub const MUSICAL_SYMBOL_EIGHTH_REST: char = '𝄾';
    /// \u{1d13f}: '𝄿'
    pub const MUSICAL_SYMBOL_SIXTEENTH_REST: char = '𝄿';
    /// \u{1d140}: '𝅀'
    pub const MUSICAL_SYMBOL_THIRTY_DASH_SECOND_REST: char = '𝅀';
    /// \u{1d141}: '𝅁'
    pub const MUSICAL_SYMBOL_SIXTY_DASH_FOURTH_REST: char = '𝅁';
    /// \u{1d142}: '𝅂'
    pub const MUSICAL_SYMBOL_ONE_HUNDRED_TWENTY_DASH_EIGHTH_REST: char = '𝅂';
    /// \u{1d143}: '𝅃'
    pub const MUSICAL_SYMBOL_X_NOTEHEAD: char = '𝅃';
    /// \u{1d144}: '𝅄'
    pub const MUSICAL_SYMBOL_PLUS_NOTEHEAD: char = '𝅄';
    /// \u{1d145}: '𝅅'
    pub const MUSICAL_SYMBOL_CIRCLE_X_NOTEHEAD: char = '𝅅';
    /// \u{1d146}: '𝅆'
    pub const MUSICAL_SYMBOL_SQUARE_NOTEHEAD_WHITE: char = '𝅆';
    /// \u{1d147}: '𝅇'
    pub const MUSICAL_SYMBOL_SQUARE_NOTEHEAD_BLACK: char = '𝅇';
    /// \u{1d148}: '𝅈'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_WHITE: char = '𝅈';
    /// \u{1d149}: '𝅉'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_BLACK: char = '𝅉';
    /// \u{1d14a}: '𝅊'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_LEFT_WHITE: char = '𝅊';
    /// \u{1d14b}: '𝅋'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_LEFT_BLACK: char = '𝅋';
    /// \u{1d14c}: '𝅌'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_RIGHT_WHITE: char = '𝅌';
    /// \u{1d14d}: '𝅍'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_RIGHT_BLACK: char = '𝅍';
    /// \u{1d14e}: '𝅎'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_DOWN_WHITE: char = '𝅎';
    /// \u{1d14f}: '𝅏'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_DOWN_BLACK: char = '𝅏';
    /// \u{1d150}: '𝅐'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_RIGHT_WHITE: char = '𝅐';
    /// \u{1d151}: '𝅑'
    pub const MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_RIGHT_BLACK: char = '𝅑';
    /// \u{1d152}: '𝅒'
    pub const MUSICAL_SYMBOL_MOON_NOTEHEAD_WHITE: char = '𝅒';
    /// \u{1d153}: '𝅓'
    pub const MUSICAL_SYMBOL_MOON_NOTEHEAD_BLACK: char = '𝅓';
    /// \u{1d154}: '𝅔'
    pub const MUSICAL_SYMBOL_TRIANGLE_DASH_ROUND_NOTEHEAD_DOWN_WHITE: char = '𝅔';
    /// \u{1d155}: '𝅕'
    pub const MUSICAL_SYMBOL_TRIANGLE_DASH_ROUND_NOTEHEAD_DOWN_BLACK: char = '𝅕';
    /// \u{1d156}: '𝅖'
    pub const MUSICAL_SYMBOL_PARENTHESIS_NOTEHEAD: char = '𝅖';
    /// \u{1d157}: '𝅗'
    pub const MUSICAL_SYMBOL_VOID_NOTEHEAD: char = '𝅗';
    /// \u{1d158}: '𝅘'
    pub const MUSICAL_SYMBOL_NOTEHEAD_BLACK: char = '𝅘';
    /// \u{1d159}: '𝅙'
    pub const MUSICAL_SYMBOL_NULL_NOTEHEAD: char = '𝅙';
    /// \u{1d15a}: '𝅚'
    pub const MUSICAL_SYMBOL_CLUSTER_NOTEHEAD_WHITE: char = '𝅚';
    /// \u{1d15b}: '𝅛'
    pub const MUSICAL_SYMBOL_CLUSTER_NOTEHEAD_BLACK: char = '𝅛';
    /// \u{1d15c}: '𝅜'
    pub const MUSICAL_SYMBOL_BREVE: char = '𝅜';
    /// \u{1d15d}: '𝅝'
    pub const MUSICAL_SYMBOL_WHOLE_NOTE: char = '𝅝';
    /// \u{1d15e}: '𝅗𝅥'
    pub const MUSICAL_SYMBOL_HALF_NOTE: char = '𝅗𝅥';
    /// \u{1d15f}: '𝅘𝅥'
    pub const MUSICAL_SYMBOL_QUARTER_NOTE: char = '𝅘𝅥';
    /// \u{1d160}: '𝅘𝅥𝅮'
    pub const MUSICAL_SYMBOL_EIGHTH_NOTE: char = '𝅘𝅥𝅮';
    /// \u{1d161}: '𝅘𝅥𝅯'
    pub const MUSICAL_SYMBOL_SIXTEENTH_NOTE: char = '𝅘𝅥𝅯';
    /// \u{1d162}: '𝅘𝅥𝅰'
    pub const MUSICAL_SYMBOL_THIRTY_DASH_SECOND_NOTE: char = '𝅘𝅥𝅰';
    /// \u{1d163}: '𝅘𝅥𝅱'
    pub const MUSICAL_SYMBOL_SIXTY_DASH_FOURTH_NOTE: char = '𝅘𝅥𝅱';
    /// \u{1d164}: '𝅘𝅥𝅲'
    pub const MUSICAL_SYMBOL_ONE_HUNDRED_TWENTY_DASH_EIGHTH_NOTE: char = '𝅘𝅥𝅲';
    /// \u{1d165}: '𝅥'
    pub const MUSICAL_SYMBOL_COMBINING_STEM: char = '𝅥';
    /// \u{1d166}: '𝅦'
    pub const MUSICAL_SYMBOL_COMBINING_SPRECHGESANG_STEM: char = '𝅦';
    /// \u{1d167}: '𝅧'
    pub const MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_1: char = '𝅧';
    /// \u{1d168}: '𝅨'
    pub const MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_2: char = '𝅨';
    /// \u{1d169}: '𝅩'
    pub const MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_3: char = '𝅩';
    /// \u{1d16a}: '𝅪'
    pub const MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_1: char = '𝅪';
    /// \u{1d16b}: '𝅫'
    pub const MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_2: char = '𝅫';
    /// \u{1d16c}: '𝅬'
    pub const MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_3: char = '𝅬';
    /// \u{1d16d}: '𝅭'
    pub const MUSICAL_SYMBOL_COMBINING_AUGMENTATION_DOT: char = '𝅭';
    /// \u{1d16e}: '𝅮'
    pub const MUSICAL_SYMBOL_COMBINING_FLAG_DASH_1: char = '𝅮';
    /// \u{1d16f}: '𝅯'
    pub const MUSICAL_SYMBOL_COMBINING_FLAG_DASH_2: char = '𝅯';
    /// \u{1d170}: '𝅰'
    pub const MUSICAL_SYMBOL_COMBINING_FLAG_DASH_3: char = '𝅰';
    /// \u{1d171}: '𝅱'
    pub const MUSICAL_SYMBOL_COMBINING_FLAG_DASH_4: char = '𝅱';
    /// \u{1d172}: '𝅲'
    pub const MUSICAL_SYMBOL_COMBINING_FLAG_DASH_5: char = '𝅲';
    /// \u{1d173}: '𝅳'
    pub const MUSICAL_SYMBOL_BEGIN_BEAM: char = '𝅳';
    /// \u{1d174}: '𝅴'
    pub const MUSICAL_SYMBOL_END_BEAM: char = '𝅴';
    /// \u{1d175}: '𝅵'
    pub const MUSICAL_SYMBOL_BEGIN_TIE: char = '𝅵';
    /// \u{1d176}: '𝅶'
    pub const MUSICAL_SYMBOL_END_TIE: char = '𝅶';
    /// \u{1d177}: '𝅷'
    pub const MUSICAL_SYMBOL_BEGIN_SLUR: char = '𝅷';
    /// \u{1d178}: '𝅸'
    pub const MUSICAL_SYMBOL_END_SLUR: char = '𝅸';
    /// \u{1d179}: '𝅹'
    pub const MUSICAL_SYMBOL_BEGIN_PHRASE: char = '𝅹';
    /// \u{1d17a}: '𝅺'
    pub const MUSICAL_SYMBOL_END_PHRASE: char = '𝅺';
    /// \u{1d17b}: '𝅻'
    pub const MUSICAL_SYMBOL_COMBINING_ACCENT: char = '𝅻';
    /// \u{1d17c}: '𝅼'
    pub const MUSICAL_SYMBOL_COMBINING_STACCATO: char = '𝅼';
    /// \u{1d17d}: '𝅽'
    pub const MUSICAL_SYMBOL_COMBINING_TENUTO: char = '𝅽';
    /// \u{1d17e}: '𝅾'
    pub const MUSICAL_SYMBOL_COMBINING_STACCATISSIMO: char = '𝅾';
    /// \u{1d17f}: '𝅿'
    pub const MUSICAL_SYMBOL_COMBINING_MARCATO: char = '𝅿';
    /// \u{1d180}: '𝆀'
    pub const MUSICAL_SYMBOL_COMBINING_MARCATO_DASH_STACCATO: char = '𝆀';
    /// \u{1d181}: '𝆁'
    pub const MUSICAL_SYMBOL_COMBINING_ACCENT_DASH_STACCATO: char = '𝆁';
    /// \u{1d182}: '𝆂'
    pub const MUSICAL_SYMBOL_COMBINING_LOURE: char = '𝆂';
    /// \u{1d183}: '𝆃'
    pub const MUSICAL_SYMBOL_ARPEGGIATO_UP: char = '𝆃';
    /// \u{1d184}: '𝆄'
    pub const MUSICAL_SYMBOL_ARPEGGIATO_DOWN: char = '𝆄';
    /// \u{1d185}: '𝆅'
    pub const MUSICAL_SYMBOL_COMBINING_DOIT: char = '𝆅';
    /// \u{1d186}: '𝆆'
    pub const MUSICAL_SYMBOL_COMBINING_RIP: char = '𝆆';
    /// \u{1d187}: '𝆇'
    pub const MUSICAL_SYMBOL_COMBINING_FLIP: char = '𝆇';
    /// \u{1d188}: '𝆈'
    pub const MUSICAL_SYMBOL_COMBINING_SMEAR: char = '𝆈';
    /// \u{1d189}: '𝆉'
    pub const MUSICAL_SYMBOL_COMBINING_BEND: char = '𝆉';
    /// \u{1d18a}: '𝆊'
    pub const MUSICAL_SYMBOL_COMBINING_DOUBLE_TONGUE: char = '𝆊';
    /// \u{1d18b}: '𝆋'
    pub const MUSICAL_SYMBOL_COMBINING_TRIPLE_TONGUE: char = '𝆋';
    /// \u{1d18c}: '𝆌'
    pub const MUSICAL_SYMBOL_RINFORZANDO: char = '𝆌';
    /// \u{1d18d}: '𝆍'
    pub const MUSICAL_SYMBOL_SUBITO: char = '𝆍';
    /// \u{1d18e}: '𝆎'
    pub const MUSICAL_SYMBOL_Z: char = '𝆎';
    /// \u{1d18f}: '𝆏'
    pub const MUSICAL_SYMBOL_PIANO: char = '𝆏';
    /// \u{1d190}: '𝆐'
    pub const MUSICAL_SYMBOL_MEZZO: char = '𝆐';
    /// \u{1d191}: '𝆑'
    pub const MUSICAL_SYMBOL_FORTE: char = '𝆑';
    /// \u{1d192}: '𝆒'
    pub const MUSICAL_SYMBOL_CRESCENDO: char = '𝆒';
    /// \u{1d193}: '𝆓'
    pub const MUSICAL_SYMBOL_DECRESCENDO: char = '𝆓';
    /// \u{1d194}: '𝆔'
    pub const MUSICAL_SYMBOL_GRACE_NOTE_SLASH: char = '𝆔';
    /// \u{1d195}: '𝆕'
    pub const MUSICAL_SYMBOL_GRACE_NOTE_NO_SLASH: char = '𝆕';
    /// \u{1d196}: '𝆖'
    pub const MUSICAL_SYMBOL_TR: char = '𝆖';
    /// \u{1d197}: '𝆗'
    pub const MUSICAL_SYMBOL_TURN: char = '𝆗';
    /// \u{1d198}: '𝆘'
    pub const MUSICAL_SYMBOL_INVERTED_TURN: char = '𝆘';
    /// \u{1d199}: '𝆙'
    pub const MUSICAL_SYMBOL_TURN_SLASH: char = '𝆙';
    /// \u{1d19a}: '𝆚'
    pub const MUSICAL_SYMBOL_TURN_UP: char = '𝆚';
    /// \u{1d19b}: '𝆛'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_1: char = '𝆛';
    /// \u{1d19c}: '𝆜'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_2: char = '𝆜';
    /// \u{1d19d}: '𝆝'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_3: char = '𝆝';
    /// \u{1d19e}: '𝆞'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_4: char = '𝆞';
    /// \u{1d19f}: '𝆟'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_5: char = '𝆟';
    /// \u{1d1a0}: '𝆠'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_6: char = '𝆠';
    /// \u{1d1a1}: '𝆡'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_7: char = '𝆡';
    /// \u{1d1a2}: '𝆢'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_8: char = '𝆢';
    /// \u{1d1a3}: '𝆣'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_9: char = '𝆣';
    /// \u{1d1a4}: '𝆤'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_10: char = '𝆤';
    /// \u{1d1a5}: '𝆥'
    pub const MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_11: char = '𝆥';
    /// \u{1d1a6}: '𝆦'
    pub const MUSICAL_SYMBOL_HAUPTSTIMME: char = '𝆦';
    /// \u{1d1a7}: '𝆧'
    pub const MUSICAL_SYMBOL_NEBENSTIMME: char = '𝆧';
    /// \u{1d1a8}: '𝆨'
    pub const MUSICAL_SYMBOL_END_OF_STIMME: char = '𝆨';
    /// \u{1d1a9}: '𝆩'
    pub const MUSICAL_SYMBOL_DEGREE_SLASH: char = '𝆩';
    /// \u{1d1aa}: '𝆪'
    pub const MUSICAL_SYMBOL_COMBINING_DOWN_BOW: char = '𝆪';
    /// \u{1d1ab}: '𝆫'
    pub const MUSICAL_SYMBOL_COMBINING_UP_BOW: char = '𝆫';
    /// \u{1d1ac}: '𝆬'
    pub const MUSICAL_SYMBOL_COMBINING_HARMONIC: char = '𝆬';
    /// \u{1d1ad}: '𝆭'
    pub const MUSICAL_SYMBOL_COMBINING_SNAP_PIZZICATO: char = '𝆭';
    /// \u{1d1ae}: '𝆮'
    pub const MUSICAL_SYMBOL_PEDAL_MARK: char = '𝆮';
    /// \u{1d1af}: '𝆯'
    pub const MUSICAL_SYMBOL_PEDAL_UP_MARK: char = '𝆯';
    /// \u{1d1b0}: '𝆰'
    pub const MUSICAL_SYMBOL_HALF_PEDAL_MARK: char = '𝆰';
    /// \u{1d1b1}: '𝆱'
    pub const MUSICAL_SYMBOL_GLISSANDO_UP: char = '𝆱';
    /// \u{1d1b2}: '𝆲'
    pub const MUSICAL_SYMBOL_GLISSANDO_DOWN: char = '𝆲';
    /// \u{1d1b3}: '𝆳'
    pub const MUSICAL_SYMBOL_WITH_FINGERNAILS: char = '𝆳';
    /// \u{1d1b4}: '𝆴'
    pub const MUSICAL_SYMBOL_DAMP: char = '𝆴';
    /// \u{1d1b5}: '𝆵'
    pub const MUSICAL_SYMBOL_DAMP_ALL: char = '𝆵';
    /// \u{1d1b6}: '𝆶'
    pub const MUSICAL_SYMBOL_MAXIMA: char = '𝆶';
    /// \u{1d1b7}: '𝆷'
    pub const MUSICAL_SYMBOL_LONGA: char = '𝆷';
    /// \u{1d1b8}: '𝆸'
    pub const MUSICAL_SYMBOL_BREVIS: char = '𝆸';
    /// \u{1d1b9}: '𝆹'
    pub const MUSICAL_SYMBOL_SEMIBREVIS_WHITE: char = '𝆹';
    /// \u{1d1ba}: '𝆺'
    pub const MUSICAL_SYMBOL_SEMIBREVIS_BLACK: char = '𝆺';
    /// \u{1d1bb}: '𝆹𝅥'
    pub const MUSICAL_SYMBOL_MINIMA: char = '𝆹𝅥';
    /// \u{1d1bc}: '𝆺𝅥'
    pub const MUSICAL_SYMBOL_MINIMA_BLACK: char = '𝆺𝅥';
    /// \u{1d1bd}: '𝆹𝅥𝅮'
    pub const MUSICAL_SYMBOL_SEMIMINIMA_WHITE: char = '𝆹𝅥𝅮';
    /// \u{1d1be}: '𝆺𝅥𝅮'
    pub const MUSICAL_SYMBOL_SEMIMINIMA_BLACK: char = '𝆺𝅥𝅮';
    /// \u{1d1bf}: '𝆹𝅥𝅯'
    pub const MUSICAL_SYMBOL_FUSA_WHITE: char = '𝆹𝅥𝅯';
    /// \u{1d1c0}: '𝆺𝅥𝅯'
    pub const MUSICAL_SYMBOL_FUSA_BLACK: char = '𝆺𝅥𝅯';
    /// \u{1d1c1}: '𝇁'
    pub const MUSICAL_SYMBOL_LONGA_PERFECTA_REST: char = '𝇁';
    /// \u{1d1c2}: '𝇂'
    pub const MUSICAL_SYMBOL_LONGA_IMPERFECTA_REST: char = '𝇂';
    /// \u{1d1c3}: '𝇃'
    pub const MUSICAL_SYMBOL_BREVIS_REST: char = '𝇃';
    /// \u{1d1c4}: '𝇄'
    pub const MUSICAL_SYMBOL_SEMIBREVIS_REST: char = '𝇄';
    /// \u{1d1c5}: '𝇅'
    pub const MUSICAL_SYMBOL_MINIMA_REST: char = '𝇅';
    /// \u{1d1c6}: '𝇆'
    pub const MUSICAL_SYMBOL_SEMIMINIMA_REST: char = '𝇆';
    /// \u{1d1c7}: '𝇇'
    pub const MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_PERFECTA: char = '𝇇';
    /// \u{1d1c8}: '𝇈'
    pub const MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_IMPERFECTA: char = '𝇈';
    /// \u{1d1c9}: '𝇉'
    pub const MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_PERFECTA_DIMINUTION_DASH_1: char = '𝇉';
    /// \u{1d1ca}: '𝇊'
    pub const MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_PERFECTA: char = '𝇊';
    /// \u{1d1cb}: '𝇋'
    pub const MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA: char = '𝇋';
    /// \u{1d1cc}: '𝇌'
    pub const MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_1: char = '𝇌';
    /// \u{1d1cd}: '𝇍'
    pub const MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_2: char = '𝇍';
    /// \u{1d1ce}: '𝇎'
    pub const MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_3: char = '𝇎';
    /// \u{1d1cf}: '𝇏'
    pub const MUSICAL_SYMBOL_CROIX: char = '𝇏';
    /// \u{1d1d0}: '𝇐'
    pub const MUSICAL_SYMBOL_GREGORIAN_C_CLEF: char = '𝇐';
    /// \u{1d1d1}: '𝇑'
    pub const MUSICAL_SYMBOL_GREGORIAN_F_CLEF: char = '𝇑';
    /// \u{1d1d2}: '𝇒'
    pub const MUSICAL_SYMBOL_SQUARE_B: char = '𝇒';
    /// \u{1d1d3}: '𝇓'
    pub const MUSICAL_SYMBOL_VIRGA: char = '𝇓';
    /// \u{1d1d4}: '𝇔'
    pub const MUSICAL_SYMBOL_PODATUS: char = '𝇔';
    /// \u{1d1d5}: '𝇕'
    pub const MUSICAL_SYMBOL_CLIVIS: char = '𝇕';
    /// \u{1d1d6}: '𝇖'
    pub const MUSICAL_SYMBOL_SCANDICUS: char = '𝇖';
    /// \u{1d1d7}: '𝇗'
    pub const MUSICAL_SYMBOL_CLIMACUS: char = '𝇗';
    /// \u{1d1d8}: '𝇘'
    pub const MUSICAL_SYMBOL_TORCULUS: char = '𝇘';
    /// \u{1d1d9}: '𝇙'
    pub const MUSICAL_SYMBOL_PORRECTUS: char = '𝇙';
    /// \u{1d1da}: '𝇚'
    pub const MUSICAL_SYMBOL_PORRECTUS_FLEXUS: char = '𝇚';
    /// \u{1d1db}: '𝇛'
    pub const MUSICAL_SYMBOL_SCANDICUS_FLEXUS: char = '𝇛';
    /// \u{1d1dc}: '𝇜'
    pub const MUSICAL_SYMBOL_TORCULUS_RESUPINUS: char = '𝇜';
    /// \u{1d1dd}: '𝇝'
    pub const MUSICAL_SYMBOL_PES_SUBPUNCTIS: char = '𝇝';
    /// \u{1d1de}: '𝇞'
    pub const MUSICAL_SYMBOL_KIEVAN_C_CLEF: char = '𝇞';
    /// \u{1d1df}: '𝇟'
    pub const MUSICAL_SYMBOL_KIEVAN_END_OF_PIECE: char = '𝇟';
    /// \u{1d1e0}: '𝇠'
    pub const MUSICAL_SYMBOL_KIEVAN_FINAL_NOTE: char = '𝇠';
    /// \u{1d1e1}: '𝇡'
    pub const MUSICAL_SYMBOL_KIEVAN_RECITATIVE_MARK: char = '𝇡';
    /// \u{1d1e2}: '𝇢'
    pub const MUSICAL_SYMBOL_KIEVAN_WHOLE_NOTE: char = '𝇢';
    /// \u{1d1e3}: '𝇣'
    pub const MUSICAL_SYMBOL_KIEVAN_HALF_NOTE: char = '𝇣';
    /// \u{1d1e4}: '𝇤'
    pub const MUSICAL_SYMBOL_KIEVAN_QUARTER_NOTE_STEM_DOWN: char = '𝇤';
    /// \u{1d1e5}: '𝇥'
    pub const MUSICAL_SYMBOL_KIEVAN_QUARTER_NOTE_STEM_UP: char = '𝇥';
    /// \u{1d1e6}: '𝇦'
    pub const MUSICAL_SYMBOL_KIEVAN_EIGHTH_NOTE_STEM_DOWN: char = '𝇦';
    /// \u{1d1e7}: '𝇧'
    pub const MUSICAL_SYMBOL_KIEVAN_EIGHTH_NOTE_STEM_UP: char = '𝇧';
    /// \u{1d1e8}: '𝇨'
    pub const MUSICAL_SYMBOL_KIEVAN_FLAT_SIGN: char = '𝇨';
}

/// An enum to represent all characters in the MusicalSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MusicalSymbols {
    /// \u{1d100}: '𝄀'
    MusicalSymbolSingleBarline,
    /// \u{1d101}: '𝄁'
    MusicalSymbolDoubleBarline,
    /// \u{1d102}: '𝄂'
    MusicalSymbolFinalBarline,
    /// \u{1d103}: '𝄃'
    MusicalSymbolReverseFinalBarline,
    /// \u{1d104}: '𝄄'
    MusicalSymbolDashedBarline,
    /// \u{1d105}: '𝄅'
    MusicalSymbolShortBarline,
    /// \u{1d106}: '𝄆'
    MusicalSymbolLeftRepeatSign,
    /// \u{1d107}: '𝄇'
    MusicalSymbolRightRepeatSign,
    /// \u{1d108}: '𝄈'
    MusicalSymbolRepeatDots,
    /// \u{1d109}: '𝄉'
    MusicalSymbolDalSegno,
    /// \u{1d10a}: '𝄊'
    MusicalSymbolDaCapo,
    /// \u{1d10b}: '𝄋'
    MusicalSymbolSegno,
    /// \u{1d10c}: '𝄌'
    MusicalSymbolCoda,
    /// \u{1d10d}: '𝄍'
    MusicalSymbolRepeatedFigureDash1,
    /// \u{1d10e}: '𝄎'
    MusicalSymbolRepeatedFigureDash2,
    /// \u{1d10f}: '𝄏'
    MusicalSymbolRepeatedFigureDash3,
    /// \u{1d110}: '𝄐'
    MusicalSymbolFermata,
    /// \u{1d111}: '𝄑'
    MusicalSymbolFermataBelow,
    /// \u{1d112}: '𝄒'
    MusicalSymbolBreathMark,
    /// \u{1d113}: '𝄓'
    MusicalSymbolCaesura,
    /// \u{1d114}: '𝄔'
    MusicalSymbolBrace,
    /// \u{1d115}: '𝄕'
    MusicalSymbolBracket,
    /// \u{1d116}: '𝄖'
    MusicalSymbolOneDashLineStaff,
    /// \u{1d117}: '𝄗'
    MusicalSymbolTwoDashLineStaff,
    /// \u{1d118}: '𝄘'
    MusicalSymbolThreeDashLineStaff,
    /// \u{1d119}: '𝄙'
    MusicalSymbolFourDashLineStaff,
    /// \u{1d11a}: '𝄚'
    MusicalSymbolFiveDashLineStaff,
    /// \u{1d11b}: '𝄛'
    MusicalSymbolSixDashLineStaff,
    /// \u{1d11c}: '𝄜'
    MusicalSymbolSixDashStringFretboard,
    /// \u{1d11d}: '𝄝'
    MusicalSymbolFourDashStringFretboard,
    /// \u{1d11e}: '𝄞'
    MusicalSymbolGClef,
    /// \u{1d11f}: '𝄟'
    MusicalSymbolGClefOttavaAlta,
    /// \u{1d120}: '𝄠'
    MusicalSymbolGClefOttavaBassa,
    /// \u{1d121}: '𝄡'
    MusicalSymbolCClef,
    /// \u{1d122}: '𝄢'
    MusicalSymbolFClef,
    /// \u{1d123}: '𝄣'
    MusicalSymbolFClefOttavaAlta,
    /// \u{1d124}: '𝄤'
    MusicalSymbolFClefOttavaBassa,
    /// \u{1d125}: '𝄥'
    MusicalSymbolDrumClefDash1,
    /// \u{1d126}: '𝄦'
    MusicalSymbolDrumClefDash2,
    /// \u{1d129}: '𝄩'
    MusicalSymbolMultipleMeasureRest,
    /// \u{1d12a}: '𝄪'
    MusicalSymbolDoubleSharp,
    /// \u{1d12b}: '𝄫'
    MusicalSymbolDoubleFlat,
    /// \u{1d12c}: '𝄬'
    MusicalSymbolFlatUp,
    /// \u{1d12d}: '𝄭'
    MusicalSymbolFlatDown,
    /// \u{1d12e}: '𝄮'
    MusicalSymbolNaturalUp,
    /// \u{1d12f}: '𝄯'
    MusicalSymbolNaturalDown,
    /// \u{1d130}: '𝄰'
    MusicalSymbolSharpUp,
    /// \u{1d131}: '𝄱'
    MusicalSymbolSharpDown,
    /// \u{1d132}: '𝄲'
    MusicalSymbolQuarterToneSharp,
    /// \u{1d133}: '𝄳'
    MusicalSymbolQuarterToneFlat,
    /// \u{1d134}: '𝄴'
    MusicalSymbolCommonTime,
    /// \u{1d135}: '𝄵'
    MusicalSymbolCutTime,
    /// \u{1d136}: '𝄶'
    MusicalSymbolOttavaAlta,
    /// \u{1d137}: '𝄷'
    MusicalSymbolOttavaBassa,
    /// \u{1d138}: '𝄸'
    MusicalSymbolQuindicesimaAlta,
    /// \u{1d139}: '𝄹'
    MusicalSymbolQuindicesimaBassa,
    /// \u{1d13a}: '𝄺'
    MusicalSymbolMultiRest,
    /// \u{1d13b}: '𝄻'
    MusicalSymbolWholeRest,
    /// \u{1d13c}: '𝄼'
    MusicalSymbolHalfRest,
    /// \u{1d13d}: '𝄽'
    MusicalSymbolQuarterRest,
    /// \u{1d13e}: '𝄾'
    MusicalSymbolEighthRest,
    /// \u{1d13f}: '𝄿'
    MusicalSymbolSixteenthRest,
    /// \u{1d140}: '𝅀'
    MusicalSymbolThirtyDashSecondRest,
    /// \u{1d141}: '𝅁'
    MusicalSymbolSixtyDashFourthRest,
    /// \u{1d142}: '𝅂'
    MusicalSymbolOneHundredTwentyDashEighthRest,
    /// \u{1d143}: '𝅃'
    MusicalSymbolXNotehead,
    /// \u{1d144}: '𝅄'
    MusicalSymbolPlusNotehead,
    /// \u{1d145}: '𝅅'
    MusicalSymbolCircleXNotehead,
    /// \u{1d146}: '𝅆'
    MusicalSymbolSquareNoteheadWhite,
    /// \u{1d147}: '𝅇'
    MusicalSymbolSquareNoteheadBlack,
    /// \u{1d148}: '𝅈'
    MusicalSymbolTriangleNoteheadUpWhite,
    /// \u{1d149}: '𝅉'
    MusicalSymbolTriangleNoteheadUpBlack,
    /// \u{1d14a}: '𝅊'
    MusicalSymbolTriangleNoteheadLeftWhite,
    /// \u{1d14b}: '𝅋'
    MusicalSymbolTriangleNoteheadLeftBlack,
    /// \u{1d14c}: '𝅌'
    MusicalSymbolTriangleNoteheadRightWhite,
    /// \u{1d14d}: '𝅍'
    MusicalSymbolTriangleNoteheadRightBlack,
    /// \u{1d14e}: '𝅎'
    MusicalSymbolTriangleNoteheadDownWhite,
    /// \u{1d14f}: '𝅏'
    MusicalSymbolTriangleNoteheadDownBlack,
    /// \u{1d150}: '𝅐'
    MusicalSymbolTriangleNoteheadUpRightWhite,
    /// \u{1d151}: '𝅑'
    MusicalSymbolTriangleNoteheadUpRightBlack,
    /// \u{1d152}: '𝅒'
    MusicalSymbolMoonNoteheadWhite,
    /// \u{1d153}: '𝅓'
    MusicalSymbolMoonNoteheadBlack,
    /// \u{1d154}: '𝅔'
    MusicalSymbolTriangleDashRoundNoteheadDownWhite,
    /// \u{1d155}: '𝅕'
    MusicalSymbolTriangleDashRoundNoteheadDownBlack,
    /// \u{1d156}: '𝅖'
    MusicalSymbolParenthesisNotehead,
    /// \u{1d157}: '𝅗'
    MusicalSymbolVoidNotehead,
    /// \u{1d158}: '𝅘'
    MusicalSymbolNoteheadBlack,
    /// \u{1d159}: '𝅙'
    MusicalSymbolNullNotehead,
    /// \u{1d15a}: '𝅚'
    MusicalSymbolClusterNoteheadWhite,
    /// \u{1d15b}: '𝅛'
    MusicalSymbolClusterNoteheadBlack,
    /// \u{1d15c}: '𝅜'
    MusicalSymbolBreve,
    /// \u{1d15d}: '𝅝'
    MusicalSymbolWholeNote,
    /// \u{1d15e}: '𝅗𝅥'
    MusicalSymbolHalfNote,
    /// \u{1d15f}: '𝅘𝅥'
    MusicalSymbolQuarterNote,
    /// \u{1d160}: '𝅘𝅥𝅮'
    MusicalSymbolEighthNote,
    /// \u{1d161}: '𝅘𝅥𝅯'
    MusicalSymbolSixteenthNote,
    /// \u{1d162}: '𝅘𝅥𝅰'
    MusicalSymbolThirtyDashSecondNote,
    /// \u{1d163}: '𝅘𝅥𝅱'
    MusicalSymbolSixtyDashFourthNote,
    /// \u{1d164}: '𝅘𝅥𝅲'
    MusicalSymbolOneHundredTwentyDashEighthNote,
    /// \u{1d165}: '𝅥'
    MusicalSymbolCombiningStem,
    /// \u{1d166}: '𝅦'
    MusicalSymbolCombiningSprechgesangStem,
    /// \u{1d167}: '𝅧'
    MusicalSymbolCombiningTremoloDash1,
    /// \u{1d168}: '𝅨'
    MusicalSymbolCombiningTremoloDash2,
    /// \u{1d169}: '𝅩'
    MusicalSymbolCombiningTremoloDash3,
    /// \u{1d16a}: '𝅪'
    MusicalSymbolFingeredTremoloDash1,
    /// \u{1d16b}: '𝅫'
    MusicalSymbolFingeredTremoloDash2,
    /// \u{1d16c}: '𝅬'
    MusicalSymbolFingeredTremoloDash3,
    /// \u{1d16d}: '𝅭'
    MusicalSymbolCombiningAugmentationDot,
    /// \u{1d16e}: '𝅮'
    MusicalSymbolCombiningFlagDash1,
    /// \u{1d16f}: '𝅯'
    MusicalSymbolCombiningFlagDash2,
    /// \u{1d170}: '𝅰'
    MusicalSymbolCombiningFlagDash3,
    /// \u{1d171}: '𝅱'
    MusicalSymbolCombiningFlagDash4,
    /// \u{1d172}: '𝅲'
    MusicalSymbolCombiningFlagDash5,
    /// \u{1d173}: '𝅳'
    MusicalSymbolBeginBeam,
    /// \u{1d174}: '𝅴'
    MusicalSymbolEndBeam,
    /// \u{1d175}: '𝅵'
    MusicalSymbolBeginTie,
    /// \u{1d176}: '𝅶'
    MusicalSymbolEndTie,
    /// \u{1d177}: '𝅷'
    MusicalSymbolBeginSlur,
    /// \u{1d178}: '𝅸'
    MusicalSymbolEndSlur,
    /// \u{1d179}: '𝅹'
    MusicalSymbolBeginPhrase,
    /// \u{1d17a}: '𝅺'
    MusicalSymbolEndPhrase,
    /// \u{1d17b}: '𝅻'
    MusicalSymbolCombiningAccent,
    /// \u{1d17c}: '𝅼'
    MusicalSymbolCombiningStaccato,
    /// \u{1d17d}: '𝅽'
    MusicalSymbolCombiningTenuto,
    /// \u{1d17e}: '𝅾'
    MusicalSymbolCombiningStaccatissimo,
    /// \u{1d17f}: '𝅿'
    MusicalSymbolCombiningMarcato,
    /// \u{1d180}: '𝆀'
    MusicalSymbolCombiningMarcatoDashStaccato,
    /// \u{1d181}: '𝆁'
    MusicalSymbolCombiningAccentDashStaccato,
    /// \u{1d182}: '𝆂'
    MusicalSymbolCombiningLoure,
    /// \u{1d183}: '𝆃'
    MusicalSymbolArpeggiatoUp,
    /// \u{1d184}: '𝆄'
    MusicalSymbolArpeggiatoDown,
    /// \u{1d185}: '𝆅'
    MusicalSymbolCombiningDoit,
    /// \u{1d186}: '𝆆'
    MusicalSymbolCombiningRip,
    /// \u{1d187}: '𝆇'
    MusicalSymbolCombiningFlip,
    /// \u{1d188}: '𝆈'
    MusicalSymbolCombiningSmear,
    /// \u{1d189}: '𝆉'
    MusicalSymbolCombiningBend,
    /// \u{1d18a}: '𝆊'
    MusicalSymbolCombiningDoubleTongue,
    /// \u{1d18b}: '𝆋'
    MusicalSymbolCombiningTripleTongue,
    /// \u{1d18c}: '𝆌'
    MusicalSymbolRinforzando,
    /// \u{1d18d}: '𝆍'
    MusicalSymbolSubito,
    /// \u{1d18e}: '𝆎'
    MusicalSymbolZ,
    /// \u{1d18f}: '𝆏'
    MusicalSymbolPiano,
    /// \u{1d190}: '𝆐'
    MusicalSymbolMezzo,
    /// \u{1d191}: '𝆑'
    MusicalSymbolForte,
    /// \u{1d192}: '𝆒'
    MusicalSymbolCrescendo,
    /// \u{1d193}: '𝆓'
    MusicalSymbolDecrescendo,
    /// \u{1d194}: '𝆔'
    MusicalSymbolGraceNoteSlash,
    /// \u{1d195}: '𝆕'
    MusicalSymbolGraceNoteNoSlash,
    /// \u{1d196}: '𝆖'
    MusicalSymbolTr,
    /// \u{1d197}: '𝆗'
    MusicalSymbolTurn,
    /// \u{1d198}: '𝆘'
    MusicalSymbolInvertedTurn,
    /// \u{1d199}: '𝆙'
    MusicalSymbolTurnSlash,
    /// \u{1d19a}: '𝆚'
    MusicalSymbolTurnUp,
    /// \u{1d19b}: '𝆛'
    MusicalSymbolOrnamentStrokeDash1,
    /// \u{1d19c}: '𝆜'
    MusicalSymbolOrnamentStrokeDash2,
    /// \u{1d19d}: '𝆝'
    MusicalSymbolOrnamentStrokeDash3,
    /// \u{1d19e}: '𝆞'
    MusicalSymbolOrnamentStrokeDash4,
    /// \u{1d19f}: '𝆟'
    MusicalSymbolOrnamentStrokeDash5,
    /// \u{1d1a0}: '𝆠'
    MusicalSymbolOrnamentStrokeDash6,
    /// \u{1d1a1}: '𝆡'
    MusicalSymbolOrnamentStrokeDash7,
    /// \u{1d1a2}: '𝆢'
    MusicalSymbolOrnamentStrokeDash8,
    /// \u{1d1a3}: '𝆣'
    MusicalSymbolOrnamentStrokeDash9,
    /// \u{1d1a4}: '𝆤'
    MusicalSymbolOrnamentStrokeDash10,
    /// \u{1d1a5}: '𝆥'
    MusicalSymbolOrnamentStrokeDash11,
    /// \u{1d1a6}: '𝆦'
    MusicalSymbolHauptstimme,
    /// \u{1d1a7}: '𝆧'
    MusicalSymbolNebenstimme,
    /// \u{1d1a8}: '𝆨'
    MusicalSymbolEndOfStimme,
    /// \u{1d1a9}: '𝆩'
    MusicalSymbolDegreeSlash,
    /// \u{1d1aa}: '𝆪'
    MusicalSymbolCombiningDownBow,
    /// \u{1d1ab}: '𝆫'
    MusicalSymbolCombiningUpBow,
    /// \u{1d1ac}: '𝆬'
    MusicalSymbolCombiningHarmonic,
    /// \u{1d1ad}: '𝆭'
    MusicalSymbolCombiningSnapPizzicato,
    /// \u{1d1ae}: '𝆮'
    MusicalSymbolPedalMark,
    /// \u{1d1af}: '𝆯'
    MusicalSymbolPedalUpMark,
    /// \u{1d1b0}: '𝆰'
    MusicalSymbolHalfPedalMark,
    /// \u{1d1b1}: '𝆱'
    MusicalSymbolGlissandoUp,
    /// \u{1d1b2}: '𝆲'
    MusicalSymbolGlissandoDown,
    /// \u{1d1b3}: '𝆳'
    MusicalSymbolWithFingernails,
    /// \u{1d1b4}: '𝆴'
    MusicalSymbolDamp,
    /// \u{1d1b5}: '𝆵'
    MusicalSymbolDampAll,
    /// \u{1d1b6}: '𝆶'
    MusicalSymbolMaxima,
    /// \u{1d1b7}: '𝆷'
    MusicalSymbolLonga,
    /// \u{1d1b8}: '𝆸'
    MusicalSymbolBrevis,
    /// \u{1d1b9}: '𝆹'
    MusicalSymbolSemibrevisWhite,
    /// \u{1d1ba}: '𝆺'
    MusicalSymbolSemibrevisBlack,
    /// \u{1d1bb}: '𝆹𝅥'
    MusicalSymbolMinima,
    /// \u{1d1bc}: '𝆺𝅥'
    MusicalSymbolMinimaBlack,
    /// \u{1d1bd}: '𝆹𝅥𝅮'
    MusicalSymbolSemiminimaWhite,
    /// \u{1d1be}: '𝆺𝅥𝅮'
    MusicalSymbolSemiminimaBlack,
    /// \u{1d1bf}: '𝆹𝅥𝅯'
    MusicalSymbolFusaWhite,
    /// \u{1d1c0}: '𝆺𝅥𝅯'
    MusicalSymbolFusaBlack,
    /// \u{1d1c1}: '𝇁'
    MusicalSymbolLongaPerfectaRest,
    /// \u{1d1c2}: '𝇂'
    MusicalSymbolLongaImperfectaRest,
    /// \u{1d1c3}: '𝇃'
    MusicalSymbolBrevisRest,
    /// \u{1d1c4}: '𝇄'
    MusicalSymbolSemibrevisRest,
    /// \u{1d1c5}: '𝇅'
    MusicalSymbolMinimaRest,
    /// \u{1d1c6}: '𝇆'
    MusicalSymbolSemiminimaRest,
    /// \u{1d1c7}: '𝇇'
    MusicalSymbolTempusPerfectumCumProlationePerfecta,
    /// \u{1d1c8}: '𝇈'
    MusicalSymbolTempusPerfectumCumProlationeImperfecta,
    /// \u{1d1c9}: '𝇉'
    MusicalSymbolTempusPerfectumCumProlationePerfectaDiminutionDash1,
    /// \u{1d1ca}: '𝇊'
    MusicalSymbolTempusImperfectumCumProlationePerfecta,
    /// \u{1d1cb}: '𝇋'
    MusicalSymbolTempusImperfectumCumProlationeImperfecta,
    /// \u{1d1cc}: '𝇌'
    MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash1,
    /// \u{1d1cd}: '𝇍'
    MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash2,
    /// \u{1d1ce}: '𝇎'
    MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash3,
    /// \u{1d1cf}: '𝇏'
    MusicalSymbolCroix,
    /// \u{1d1d0}: '𝇐'
    MusicalSymbolGregorianCClef,
    /// \u{1d1d1}: '𝇑'
    MusicalSymbolGregorianFClef,
    /// \u{1d1d2}: '𝇒'
    MusicalSymbolSquareB,
    /// \u{1d1d3}: '𝇓'
    MusicalSymbolVirga,
    /// \u{1d1d4}: '𝇔'
    MusicalSymbolPodatus,
    /// \u{1d1d5}: '𝇕'
    MusicalSymbolClivis,
    /// \u{1d1d6}: '𝇖'
    MusicalSymbolScandicus,
    /// \u{1d1d7}: '𝇗'
    MusicalSymbolClimacus,
    /// \u{1d1d8}: '𝇘'
    MusicalSymbolTorculus,
    /// \u{1d1d9}: '𝇙'
    MusicalSymbolPorrectus,
    /// \u{1d1da}: '𝇚'
    MusicalSymbolPorrectusFlexus,
    /// \u{1d1db}: '𝇛'
    MusicalSymbolScandicusFlexus,
    /// \u{1d1dc}: '𝇜'
    MusicalSymbolTorculusResupinus,
    /// \u{1d1dd}: '𝇝'
    MusicalSymbolPesSubpunctis,
    /// \u{1d1de}: '𝇞'
    MusicalSymbolKievanCClef,
    /// \u{1d1df}: '𝇟'
    MusicalSymbolKievanEndOfPiece,
    /// \u{1d1e0}: '𝇠'
    MusicalSymbolKievanFinalNote,
    /// \u{1d1e1}: '𝇡'
    MusicalSymbolKievanRecitativeMark,
    /// \u{1d1e2}: '𝇢'
    MusicalSymbolKievanWholeNote,
    /// \u{1d1e3}: '𝇣'
    MusicalSymbolKievanHalfNote,
    /// \u{1d1e4}: '𝇤'
    MusicalSymbolKievanQuarterNoteStemDown,
    /// \u{1d1e5}: '𝇥'
    MusicalSymbolKievanQuarterNoteStemUp,
    /// \u{1d1e6}: '𝇦'
    MusicalSymbolKievanEighthNoteStemDown,
    /// \u{1d1e7}: '𝇧'
    MusicalSymbolKievanEighthNoteStemUp,
    /// \u{1d1e8}: '𝇨'
    MusicalSymbolKievanFlatSign,
}

impl Into<char> for MusicalSymbols {
    fn into(self) -> char {
        use constants::*;
        match self {
            MusicalSymbols::MusicalSymbolSingleBarline => MUSICAL_SYMBOL_SINGLE_BARLINE,
            MusicalSymbols::MusicalSymbolDoubleBarline => MUSICAL_SYMBOL_DOUBLE_BARLINE,
            MusicalSymbols::MusicalSymbolFinalBarline => MUSICAL_SYMBOL_FINAL_BARLINE,
            MusicalSymbols::MusicalSymbolReverseFinalBarline => MUSICAL_SYMBOL_REVERSE_FINAL_BARLINE,
            MusicalSymbols::MusicalSymbolDashedBarline => MUSICAL_SYMBOL_DASHED_BARLINE,
            MusicalSymbols::MusicalSymbolShortBarline => MUSICAL_SYMBOL_SHORT_BARLINE,
            MusicalSymbols::MusicalSymbolLeftRepeatSign => MUSICAL_SYMBOL_LEFT_REPEAT_SIGN,
            MusicalSymbols::MusicalSymbolRightRepeatSign => MUSICAL_SYMBOL_RIGHT_REPEAT_SIGN,
            MusicalSymbols::MusicalSymbolRepeatDots => MUSICAL_SYMBOL_REPEAT_DOTS,
            MusicalSymbols::MusicalSymbolDalSegno => MUSICAL_SYMBOL_DAL_SEGNO,
            MusicalSymbols::MusicalSymbolDaCapo => MUSICAL_SYMBOL_DA_CAPO,
            MusicalSymbols::MusicalSymbolSegno => MUSICAL_SYMBOL_SEGNO,
            MusicalSymbols::MusicalSymbolCoda => MUSICAL_SYMBOL_CODA,
            MusicalSymbols::MusicalSymbolRepeatedFigureDash1 => MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_1,
            MusicalSymbols::MusicalSymbolRepeatedFigureDash2 => MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_2,
            MusicalSymbols::MusicalSymbolRepeatedFigureDash3 => MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_3,
            MusicalSymbols::MusicalSymbolFermata => MUSICAL_SYMBOL_FERMATA,
            MusicalSymbols::MusicalSymbolFermataBelow => MUSICAL_SYMBOL_FERMATA_BELOW,
            MusicalSymbols::MusicalSymbolBreathMark => MUSICAL_SYMBOL_BREATH_MARK,
            MusicalSymbols::MusicalSymbolCaesura => MUSICAL_SYMBOL_CAESURA,
            MusicalSymbols::MusicalSymbolBrace => MUSICAL_SYMBOL_BRACE,
            MusicalSymbols::MusicalSymbolBracket => MUSICAL_SYMBOL_BRACKET,
            MusicalSymbols::MusicalSymbolOneDashLineStaff => MUSICAL_SYMBOL_ONE_DASH_LINE_STAFF,
            MusicalSymbols::MusicalSymbolTwoDashLineStaff => MUSICAL_SYMBOL_TWO_DASH_LINE_STAFF,
            MusicalSymbols::MusicalSymbolThreeDashLineStaff => MUSICAL_SYMBOL_THREE_DASH_LINE_STAFF,
            MusicalSymbols::MusicalSymbolFourDashLineStaff => MUSICAL_SYMBOL_FOUR_DASH_LINE_STAFF,
            MusicalSymbols::MusicalSymbolFiveDashLineStaff => MUSICAL_SYMBOL_FIVE_DASH_LINE_STAFF,
            MusicalSymbols::MusicalSymbolSixDashLineStaff => MUSICAL_SYMBOL_SIX_DASH_LINE_STAFF,
            MusicalSymbols::MusicalSymbolSixDashStringFretboard => MUSICAL_SYMBOL_SIX_DASH_STRING_FRETBOARD,
            MusicalSymbols::MusicalSymbolFourDashStringFretboard => MUSICAL_SYMBOL_FOUR_DASH_STRING_FRETBOARD,
            MusicalSymbols::MusicalSymbolGClef => MUSICAL_SYMBOL_G_CLEF,
            MusicalSymbols::MusicalSymbolGClefOttavaAlta => MUSICAL_SYMBOL_G_CLEF_OTTAVA_ALTA,
            MusicalSymbols::MusicalSymbolGClefOttavaBassa => MUSICAL_SYMBOL_G_CLEF_OTTAVA_BASSA,
            MusicalSymbols::MusicalSymbolCClef => MUSICAL_SYMBOL_C_CLEF,
            MusicalSymbols::MusicalSymbolFClef => MUSICAL_SYMBOL_F_CLEF,
            MusicalSymbols::MusicalSymbolFClefOttavaAlta => MUSICAL_SYMBOL_F_CLEF_OTTAVA_ALTA,
            MusicalSymbols::MusicalSymbolFClefOttavaBassa => MUSICAL_SYMBOL_F_CLEF_OTTAVA_BASSA,
            MusicalSymbols::MusicalSymbolDrumClefDash1 => MUSICAL_SYMBOL_DRUM_CLEF_DASH_1,
            MusicalSymbols::MusicalSymbolDrumClefDash2 => MUSICAL_SYMBOL_DRUM_CLEF_DASH_2,
            MusicalSymbols::MusicalSymbolMultipleMeasureRest => MUSICAL_SYMBOL_MULTIPLE_MEASURE_REST,
            MusicalSymbols::MusicalSymbolDoubleSharp => MUSICAL_SYMBOL_DOUBLE_SHARP,
            MusicalSymbols::MusicalSymbolDoubleFlat => MUSICAL_SYMBOL_DOUBLE_FLAT,
            MusicalSymbols::MusicalSymbolFlatUp => MUSICAL_SYMBOL_FLAT_UP,
            MusicalSymbols::MusicalSymbolFlatDown => MUSICAL_SYMBOL_FLAT_DOWN,
            MusicalSymbols::MusicalSymbolNaturalUp => MUSICAL_SYMBOL_NATURAL_UP,
            MusicalSymbols::MusicalSymbolNaturalDown => MUSICAL_SYMBOL_NATURAL_DOWN,
            MusicalSymbols::MusicalSymbolSharpUp => MUSICAL_SYMBOL_SHARP_UP,
            MusicalSymbols::MusicalSymbolSharpDown => MUSICAL_SYMBOL_SHARP_DOWN,
            MusicalSymbols::MusicalSymbolQuarterToneSharp => MUSICAL_SYMBOL_QUARTER_TONE_SHARP,
            MusicalSymbols::MusicalSymbolQuarterToneFlat => MUSICAL_SYMBOL_QUARTER_TONE_FLAT,
            MusicalSymbols::MusicalSymbolCommonTime => MUSICAL_SYMBOL_COMMON_TIME,
            MusicalSymbols::MusicalSymbolCutTime => MUSICAL_SYMBOL_CUT_TIME,
            MusicalSymbols::MusicalSymbolOttavaAlta => MUSICAL_SYMBOL_OTTAVA_ALTA,
            MusicalSymbols::MusicalSymbolOttavaBassa => MUSICAL_SYMBOL_OTTAVA_BASSA,
            MusicalSymbols::MusicalSymbolQuindicesimaAlta => MUSICAL_SYMBOL_QUINDICESIMA_ALTA,
            MusicalSymbols::MusicalSymbolQuindicesimaBassa => MUSICAL_SYMBOL_QUINDICESIMA_BASSA,
            MusicalSymbols::MusicalSymbolMultiRest => MUSICAL_SYMBOL_MULTI_REST,
            MusicalSymbols::MusicalSymbolWholeRest => MUSICAL_SYMBOL_WHOLE_REST,
            MusicalSymbols::MusicalSymbolHalfRest => MUSICAL_SYMBOL_HALF_REST,
            MusicalSymbols::MusicalSymbolQuarterRest => MUSICAL_SYMBOL_QUARTER_REST,
            MusicalSymbols::MusicalSymbolEighthRest => MUSICAL_SYMBOL_EIGHTH_REST,
            MusicalSymbols::MusicalSymbolSixteenthRest => MUSICAL_SYMBOL_SIXTEENTH_REST,
            MusicalSymbols::MusicalSymbolThirtyDashSecondRest => MUSICAL_SYMBOL_THIRTY_DASH_SECOND_REST,
            MusicalSymbols::MusicalSymbolSixtyDashFourthRest => MUSICAL_SYMBOL_SIXTY_DASH_FOURTH_REST,
            MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthRest => MUSICAL_SYMBOL_ONE_HUNDRED_TWENTY_DASH_EIGHTH_REST,
            MusicalSymbols::MusicalSymbolXNotehead => MUSICAL_SYMBOL_X_NOTEHEAD,
            MusicalSymbols::MusicalSymbolPlusNotehead => MUSICAL_SYMBOL_PLUS_NOTEHEAD,
            MusicalSymbols::MusicalSymbolCircleXNotehead => MUSICAL_SYMBOL_CIRCLE_X_NOTEHEAD,
            MusicalSymbols::MusicalSymbolSquareNoteheadWhite => MUSICAL_SYMBOL_SQUARE_NOTEHEAD_WHITE,
            MusicalSymbols::MusicalSymbolSquareNoteheadBlack => MUSICAL_SYMBOL_SQUARE_NOTEHEAD_BLACK,
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpWhite => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_WHITE,
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpBlack => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_BLACK,
            MusicalSymbols::MusicalSymbolTriangleNoteheadLeftWhite => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_LEFT_WHITE,
            MusicalSymbols::MusicalSymbolTriangleNoteheadLeftBlack => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_LEFT_BLACK,
            MusicalSymbols::MusicalSymbolTriangleNoteheadRightWhite => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_RIGHT_WHITE,
            MusicalSymbols::MusicalSymbolTriangleNoteheadRightBlack => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_RIGHT_BLACK,
            MusicalSymbols::MusicalSymbolTriangleNoteheadDownWhite => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_DOWN_WHITE,
            MusicalSymbols::MusicalSymbolTriangleNoteheadDownBlack => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_DOWN_BLACK,
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightWhite => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_RIGHT_WHITE,
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightBlack => MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_RIGHT_BLACK,
            MusicalSymbols::MusicalSymbolMoonNoteheadWhite => MUSICAL_SYMBOL_MOON_NOTEHEAD_WHITE,
            MusicalSymbols::MusicalSymbolMoonNoteheadBlack => MUSICAL_SYMBOL_MOON_NOTEHEAD_BLACK,
            MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownWhite => MUSICAL_SYMBOL_TRIANGLE_DASH_ROUND_NOTEHEAD_DOWN_WHITE,
            MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownBlack => MUSICAL_SYMBOL_TRIANGLE_DASH_ROUND_NOTEHEAD_DOWN_BLACK,
            MusicalSymbols::MusicalSymbolParenthesisNotehead => MUSICAL_SYMBOL_PARENTHESIS_NOTEHEAD,
            MusicalSymbols::MusicalSymbolVoidNotehead => MUSICAL_SYMBOL_VOID_NOTEHEAD,
            MusicalSymbols::MusicalSymbolNoteheadBlack => MUSICAL_SYMBOL_NOTEHEAD_BLACK,
            MusicalSymbols::MusicalSymbolNullNotehead => MUSICAL_SYMBOL_NULL_NOTEHEAD,
            MusicalSymbols::MusicalSymbolClusterNoteheadWhite => MUSICAL_SYMBOL_CLUSTER_NOTEHEAD_WHITE,
            MusicalSymbols::MusicalSymbolClusterNoteheadBlack => MUSICAL_SYMBOL_CLUSTER_NOTEHEAD_BLACK,
            MusicalSymbols::MusicalSymbolBreve => MUSICAL_SYMBOL_BREVE,
            MusicalSymbols::MusicalSymbolWholeNote => MUSICAL_SYMBOL_WHOLE_NOTE,
            MusicalSymbols::MusicalSymbolHalfNote => MUSICAL_SYMBOL_HALF_NOTE,
            MusicalSymbols::MusicalSymbolQuarterNote => MUSICAL_SYMBOL_QUARTER_NOTE,
            MusicalSymbols::MusicalSymbolEighthNote => MUSICAL_SYMBOL_EIGHTH_NOTE,
            MusicalSymbols::MusicalSymbolSixteenthNote => MUSICAL_SYMBOL_SIXTEENTH_NOTE,
            MusicalSymbols::MusicalSymbolThirtyDashSecondNote => MUSICAL_SYMBOL_THIRTY_DASH_SECOND_NOTE,
            MusicalSymbols::MusicalSymbolSixtyDashFourthNote => MUSICAL_SYMBOL_SIXTY_DASH_FOURTH_NOTE,
            MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthNote => MUSICAL_SYMBOL_ONE_HUNDRED_TWENTY_DASH_EIGHTH_NOTE,
            MusicalSymbols::MusicalSymbolCombiningStem => MUSICAL_SYMBOL_COMBINING_STEM,
            MusicalSymbols::MusicalSymbolCombiningSprechgesangStem => MUSICAL_SYMBOL_COMBINING_SPRECHGESANG_STEM,
            MusicalSymbols::MusicalSymbolCombiningTremoloDash1 => MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_1,
            MusicalSymbols::MusicalSymbolCombiningTremoloDash2 => MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_2,
            MusicalSymbols::MusicalSymbolCombiningTremoloDash3 => MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_3,
            MusicalSymbols::MusicalSymbolFingeredTremoloDash1 => MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_1,
            MusicalSymbols::MusicalSymbolFingeredTremoloDash2 => MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_2,
            MusicalSymbols::MusicalSymbolFingeredTremoloDash3 => MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_3,
            MusicalSymbols::MusicalSymbolCombiningAugmentationDot => MUSICAL_SYMBOL_COMBINING_AUGMENTATION_DOT,
            MusicalSymbols::MusicalSymbolCombiningFlagDash1 => MUSICAL_SYMBOL_COMBINING_FLAG_DASH_1,
            MusicalSymbols::MusicalSymbolCombiningFlagDash2 => MUSICAL_SYMBOL_COMBINING_FLAG_DASH_2,
            MusicalSymbols::MusicalSymbolCombiningFlagDash3 => MUSICAL_SYMBOL_COMBINING_FLAG_DASH_3,
            MusicalSymbols::MusicalSymbolCombiningFlagDash4 => MUSICAL_SYMBOL_COMBINING_FLAG_DASH_4,
            MusicalSymbols::MusicalSymbolCombiningFlagDash5 => MUSICAL_SYMBOL_COMBINING_FLAG_DASH_5,
            MusicalSymbols::MusicalSymbolBeginBeam => MUSICAL_SYMBOL_BEGIN_BEAM,
            MusicalSymbols::MusicalSymbolEndBeam => MUSICAL_SYMBOL_END_BEAM,
            MusicalSymbols::MusicalSymbolBeginTie => MUSICAL_SYMBOL_BEGIN_TIE,
            MusicalSymbols::MusicalSymbolEndTie => MUSICAL_SYMBOL_END_TIE,
            MusicalSymbols::MusicalSymbolBeginSlur => MUSICAL_SYMBOL_BEGIN_SLUR,
            MusicalSymbols::MusicalSymbolEndSlur => MUSICAL_SYMBOL_END_SLUR,
            MusicalSymbols::MusicalSymbolBeginPhrase => MUSICAL_SYMBOL_BEGIN_PHRASE,
            MusicalSymbols::MusicalSymbolEndPhrase => MUSICAL_SYMBOL_END_PHRASE,
            MusicalSymbols::MusicalSymbolCombiningAccent => MUSICAL_SYMBOL_COMBINING_ACCENT,
            MusicalSymbols::MusicalSymbolCombiningStaccato => MUSICAL_SYMBOL_COMBINING_STACCATO,
            MusicalSymbols::MusicalSymbolCombiningTenuto => MUSICAL_SYMBOL_COMBINING_TENUTO,
            MusicalSymbols::MusicalSymbolCombiningStaccatissimo => MUSICAL_SYMBOL_COMBINING_STACCATISSIMO,
            MusicalSymbols::MusicalSymbolCombiningMarcato => MUSICAL_SYMBOL_COMBINING_MARCATO,
            MusicalSymbols::MusicalSymbolCombiningMarcatoDashStaccato => MUSICAL_SYMBOL_COMBINING_MARCATO_DASH_STACCATO,
            MusicalSymbols::MusicalSymbolCombiningAccentDashStaccato => MUSICAL_SYMBOL_COMBINING_ACCENT_DASH_STACCATO,
            MusicalSymbols::MusicalSymbolCombiningLoure => MUSICAL_SYMBOL_COMBINING_LOURE,
            MusicalSymbols::MusicalSymbolArpeggiatoUp => MUSICAL_SYMBOL_ARPEGGIATO_UP,
            MusicalSymbols::MusicalSymbolArpeggiatoDown => MUSICAL_SYMBOL_ARPEGGIATO_DOWN,
            MusicalSymbols::MusicalSymbolCombiningDoit => MUSICAL_SYMBOL_COMBINING_DOIT,
            MusicalSymbols::MusicalSymbolCombiningRip => MUSICAL_SYMBOL_COMBINING_RIP,
            MusicalSymbols::MusicalSymbolCombiningFlip => MUSICAL_SYMBOL_COMBINING_FLIP,
            MusicalSymbols::MusicalSymbolCombiningSmear => MUSICAL_SYMBOL_COMBINING_SMEAR,
            MusicalSymbols::MusicalSymbolCombiningBend => MUSICAL_SYMBOL_COMBINING_BEND,
            MusicalSymbols::MusicalSymbolCombiningDoubleTongue => MUSICAL_SYMBOL_COMBINING_DOUBLE_TONGUE,
            MusicalSymbols::MusicalSymbolCombiningTripleTongue => MUSICAL_SYMBOL_COMBINING_TRIPLE_TONGUE,
            MusicalSymbols::MusicalSymbolRinforzando => MUSICAL_SYMBOL_RINFORZANDO,
            MusicalSymbols::MusicalSymbolSubito => MUSICAL_SYMBOL_SUBITO,
            MusicalSymbols::MusicalSymbolZ => MUSICAL_SYMBOL_Z,
            MusicalSymbols::MusicalSymbolPiano => MUSICAL_SYMBOL_PIANO,
            MusicalSymbols::MusicalSymbolMezzo => MUSICAL_SYMBOL_MEZZO,
            MusicalSymbols::MusicalSymbolForte => MUSICAL_SYMBOL_FORTE,
            MusicalSymbols::MusicalSymbolCrescendo => MUSICAL_SYMBOL_CRESCENDO,
            MusicalSymbols::MusicalSymbolDecrescendo => MUSICAL_SYMBOL_DECRESCENDO,
            MusicalSymbols::MusicalSymbolGraceNoteSlash => MUSICAL_SYMBOL_GRACE_NOTE_SLASH,
            MusicalSymbols::MusicalSymbolGraceNoteNoSlash => MUSICAL_SYMBOL_GRACE_NOTE_NO_SLASH,
            MusicalSymbols::MusicalSymbolTr => MUSICAL_SYMBOL_TR,
            MusicalSymbols::MusicalSymbolTurn => MUSICAL_SYMBOL_TURN,
            MusicalSymbols::MusicalSymbolInvertedTurn => MUSICAL_SYMBOL_INVERTED_TURN,
            MusicalSymbols::MusicalSymbolTurnSlash => MUSICAL_SYMBOL_TURN_SLASH,
            MusicalSymbols::MusicalSymbolTurnUp => MUSICAL_SYMBOL_TURN_UP,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash1 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_1,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash2 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_2,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash3 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_3,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash4 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_4,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash5 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_5,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash6 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_6,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash7 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_7,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash8 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_8,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash9 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_9,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash10 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_10,
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash11 => MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_11,
            MusicalSymbols::MusicalSymbolHauptstimme => MUSICAL_SYMBOL_HAUPTSTIMME,
            MusicalSymbols::MusicalSymbolNebenstimme => MUSICAL_SYMBOL_NEBENSTIMME,
            MusicalSymbols::MusicalSymbolEndOfStimme => MUSICAL_SYMBOL_END_OF_STIMME,
            MusicalSymbols::MusicalSymbolDegreeSlash => MUSICAL_SYMBOL_DEGREE_SLASH,
            MusicalSymbols::MusicalSymbolCombiningDownBow => MUSICAL_SYMBOL_COMBINING_DOWN_BOW,
            MusicalSymbols::MusicalSymbolCombiningUpBow => MUSICAL_SYMBOL_COMBINING_UP_BOW,
            MusicalSymbols::MusicalSymbolCombiningHarmonic => MUSICAL_SYMBOL_COMBINING_HARMONIC,
            MusicalSymbols::MusicalSymbolCombiningSnapPizzicato => MUSICAL_SYMBOL_COMBINING_SNAP_PIZZICATO,
            MusicalSymbols::MusicalSymbolPedalMark => MUSICAL_SYMBOL_PEDAL_MARK,
            MusicalSymbols::MusicalSymbolPedalUpMark => MUSICAL_SYMBOL_PEDAL_UP_MARK,
            MusicalSymbols::MusicalSymbolHalfPedalMark => MUSICAL_SYMBOL_HALF_PEDAL_MARK,
            MusicalSymbols::MusicalSymbolGlissandoUp => MUSICAL_SYMBOL_GLISSANDO_UP,
            MusicalSymbols::MusicalSymbolGlissandoDown => MUSICAL_SYMBOL_GLISSANDO_DOWN,
            MusicalSymbols::MusicalSymbolWithFingernails => MUSICAL_SYMBOL_WITH_FINGERNAILS,
            MusicalSymbols::MusicalSymbolDamp => MUSICAL_SYMBOL_DAMP,
            MusicalSymbols::MusicalSymbolDampAll => MUSICAL_SYMBOL_DAMP_ALL,
            MusicalSymbols::MusicalSymbolMaxima => MUSICAL_SYMBOL_MAXIMA,
            MusicalSymbols::MusicalSymbolLonga => MUSICAL_SYMBOL_LONGA,
            MusicalSymbols::MusicalSymbolBrevis => MUSICAL_SYMBOL_BREVIS,
            MusicalSymbols::MusicalSymbolSemibrevisWhite => MUSICAL_SYMBOL_SEMIBREVIS_WHITE,
            MusicalSymbols::MusicalSymbolSemibrevisBlack => MUSICAL_SYMBOL_SEMIBREVIS_BLACK,
            MusicalSymbols::MusicalSymbolMinima => MUSICAL_SYMBOL_MINIMA,
            MusicalSymbols::MusicalSymbolMinimaBlack => MUSICAL_SYMBOL_MINIMA_BLACK,
            MusicalSymbols::MusicalSymbolSemiminimaWhite => MUSICAL_SYMBOL_SEMIMINIMA_WHITE,
            MusicalSymbols::MusicalSymbolSemiminimaBlack => MUSICAL_SYMBOL_SEMIMINIMA_BLACK,
            MusicalSymbols::MusicalSymbolFusaWhite => MUSICAL_SYMBOL_FUSA_WHITE,
            MusicalSymbols::MusicalSymbolFusaBlack => MUSICAL_SYMBOL_FUSA_BLACK,
            MusicalSymbols::MusicalSymbolLongaPerfectaRest => MUSICAL_SYMBOL_LONGA_PERFECTA_REST,
            MusicalSymbols::MusicalSymbolLongaImperfectaRest => MUSICAL_SYMBOL_LONGA_IMPERFECTA_REST,
            MusicalSymbols::MusicalSymbolBrevisRest => MUSICAL_SYMBOL_BREVIS_REST,
            MusicalSymbols::MusicalSymbolSemibrevisRest => MUSICAL_SYMBOL_SEMIBREVIS_REST,
            MusicalSymbols::MusicalSymbolMinimaRest => MUSICAL_SYMBOL_MINIMA_REST,
            MusicalSymbols::MusicalSymbolSemiminimaRest => MUSICAL_SYMBOL_SEMIMINIMA_REST,
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfecta => MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_PERFECTA,
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationeImperfecta => MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_IMPERFECTA,
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfectaDiminutionDash1 => MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_PERFECTA_DIMINUTION_DASH_1,
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationePerfecta => MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_PERFECTA,
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfecta => MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA,
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash1 => MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_1,
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash2 => MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_2,
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash3 => MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_3,
            MusicalSymbols::MusicalSymbolCroix => MUSICAL_SYMBOL_CROIX,
            MusicalSymbols::MusicalSymbolGregorianCClef => MUSICAL_SYMBOL_GREGORIAN_C_CLEF,
            MusicalSymbols::MusicalSymbolGregorianFClef => MUSICAL_SYMBOL_GREGORIAN_F_CLEF,
            MusicalSymbols::MusicalSymbolSquareB => MUSICAL_SYMBOL_SQUARE_B,
            MusicalSymbols::MusicalSymbolVirga => MUSICAL_SYMBOL_VIRGA,
            MusicalSymbols::MusicalSymbolPodatus => MUSICAL_SYMBOL_PODATUS,
            MusicalSymbols::MusicalSymbolClivis => MUSICAL_SYMBOL_CLIVIS,
            MusicalSymbols::MusicalSymbolScandicus => MUSICAL_SYMBOL_SCANDICUS,
            MusicalSymbols::MusicalSymbolClimacus => MUSICAL_SYMBOL_CLIMACUS,
            MusicalSymbols::MusicalSymbolTorculus => MUSICAL_SYMBOL_TORCULUS,
            MusicalSymbols::MusicalSymbolPorrectus => MUSICAL_SYMBOL_PORRECTUS,
            MusicalSymbols::MusicalSymbolPorrectusFlexus => MUSICAL_SYMBOL_PORRECTUS_FLEXUS,
            MusicalSymbols::MusicalSymbolScandicusFlexus => MUSICAL_SYMBOL_SCANDICUS_FLEXUS,
            MusicalSymbols::MusicalSymbolTorculusResupinus => MUSICAL_SYMBOL_TORCULUS_RESUPINUS,
            MusicalSymbols::MusicalSymbolPesSubpunctis => MUSICAL_SYMBOL_PES_SUBPUNCTIS,
            MusicalSymbols::MusicalSymbolKievanCClef => MUSICAL_SYMBOL_KIEVAN_C_CLEF,
            MusicalSymbols::MusicalSymbolKievanEndOfPiece => MUSICAL_SYMBOL_KIEVAN_END_OF_PIECE,
            MusicalSymbols::MusicalSymbolKievanFinalNote => MUSICAL_SYMBOL_KIEVAN_FINAL_NOTE,
            MusicalSymbols::MusicalSymbolKievanRecitativeMark => MUSICAL_SYMBOL_KIEVAN_RECITATIVE_MARK,
            MusicalSymbols::MusicalSymbolKievanWholeNote => MUSICAL_SYMBOL_KIEVAN_WHOLE_NOTE,
            MusicalSymbols::MusicalSymbolKievanHalfNote => MUSICAL_SYMBOL_KIEVAN_HALF_NOTE,
            MusicalSymbols::MusicalSymbolKievanQuarterNoteStemDown => MUSICAL_SYMBOL_KIEVAN_QUARTER_NOTE_STEM_DOWN,
            MusicalSymbols::MusicalSymbolKievanQuarterNoteStemUp => MUSICAL_SYMBOL_KIEVAN_QUARTER_NOTE_STEM_UP,
            MusicalSymbols::MusicalSymbolKievanEighthNoteStemDown => MUSICAL_SYMBOL_KIEVAN_EIGHTH_NOTE_STEM_DOWN,
            MusicalSymbols::MusicalSymbolKievanEighthNoteStemUp => MUSICAL_SYMBOL_KIEVAN_EIGHTH_NOTE_STEM_UP,
            MusicalSymbols::MusicalSymbolKievanFlatSign => MUSICAL_SYMBOL_KIEVAN_FLAT_SIGN,
        }
    }
}

impl std::convert::TryFrom<char> for MusicalSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MUSICAL_SYMBOL_SINGLE_BARLINE => Ok(MusicalSymbols::MusicalSymbolSingleBarline),
            MUSICAL_SYMBOL_DOUBLE_BARLINE => Ok(MusicalSymbols::MusicalSymbolDoubleBarline),
            MUSICAL_SYMBOL_FINAL_BARLINE => Ok(MusicalSymbols::MusicalSymbolFinalBarline),
            MUSICAL_SYMBOL_REVERSE_FINAL_BARLINE => Ok(MusicalSymbols::MusicalSymbolReverseFinalBarline),
            MUSICAL_SYMBOL_DASHED_BARLINE => Ok(MusicalSymbols::MusicalSymbolDashedBarline),
            MUSICAL_SYMBOL_SHORT_BARLINE => Ok(MusicalSymbols::MusicalSymbolShortBarline),
            MUSICAL_SYMBOL_LEFT_REPEAT_SIGN => Ok(MusicalSymbols::MusicalSymbolLeftRepeatSign),
            MUSICAL_SYMBOL_RIGHT_REPEAT_SIGN => Ok(MusicalSymbols::MusicalSymbolRightRepeatSign),
            MUSICAL_SYMBOL_REPEAT_DOTS => Ok(MusicalSymbols::MusicalSymbolRepeatDots),
            MUSICAL_SYMBOL_DAL_SEGNO => Ok(MusicalSymbols::MusicalSymbolDalSegno),
            MUSICAL_SYMBOL_DA_CAPO => Ok(MusicalSymbols::MusicalSymbolDaCapo),
            MUSICAL_SYMBOL_SEGNO => Ok(MusicalSymbols::MusicalSymbolSegno),
            MUSICAL_SYMBOL_CODA => Ok(MusicalSymbols::MusicalSymbolCoda),
            MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_1 => Ok(MusicalSymbols::MusicalSymbolRepeatedFigureDash1),
            MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_2 => Ok(MusicalSymbols::MusicalSymbolRepeatedFigureDash2),
            MUSICAL_SYMBOL_REPEATED_FIGURE_DASH_3 => Ok(MusicalSymbols::MusicalSymbolRepeatedFigureDash3),
            MUSICAL_SYMBOL_FERMATA => Ok(MusicalSymbols::MusicalSymbolFermata),
            MUSICAL_SYMBOL_FERMATA_BELOW => Ok(MusicalSymbols::MusicalSymbolFermataBelow),
            MUSICAL_SYMBOL_BREATH_MARK => Ok(MusicalSymbols::MusicalSymbolBreathMark),
            MUSICAL_SYMBOL_CAESURA => Ok(MusicalSymbols::MusicalSymbolCaesura),
            MUSICAL_SYMBOL_BRACE => Ok(MusicalSymbols::MusicalSymbolBrace),
            MUSICAL_SYMBOL_BRACKET => Ok(MusicalSymbols::MusicalSymbolBracket),
            MUSICAL_SYMBOL_ONE_DASH_LINE_STAFF => Ok(MusicalSymbols::MusicalSymbolOneDashLineStaff),
            MUSICAL_SYMBOL_TWO_DASH_LINE_STAFF => Ok(MusicalSymbols::MusicalSymbolTwoDashLineStaff),
            MUSICAL_SYMBOL_THREE_DASH_LINE_STAFF => Ok(MusicalSymbols::MusicalSymbolThreeDashLineStaff),
            MUSICAL_SYMBOL_FOUR_DASH_LINE_STAFF => Ok(MusicalSymbols::MusicalSymbolFourDashLineStaff),
            MUSICAL_SYMBOL_FIVE_DASH_LINE_STAFF => Ok(MusicalSymbols::MusicalSymbolFiveDashLineStaff),
            MUSICAL_SYMBOL_SIX_DASH_LINE_STAFF => Ok(MusicalSymbols::MusicalSymbolSixDashLineStaff),
            MUSICAL_SYMBOL_SIX_DASH_STRING_FRETBOARD => Ok(MusicalSymbols::MusicalSymbolSixDashStringFretboard),
            MUSICAL_SYMBOL_FOUR_DASH_STRING_FRETBOARD => Ok(MusicalSymbols::MusicalSymbolFourDashStringFretboard),
            MUSICAL_SYMBOL_G_CLEF => Ok(MusicalSymbols::MusicalSymbolGClef),
            MUSICAL_SYMBOL_G_CLEF_OTTAVA_ALTA => Ok(MusicalSymbols::MusicalSymbolGClefOttavaAlta),
            MUSICAL_SYMBOL_G_CLEF_OTTAVA_BASSA => Ok(MusicalSymbols::MusicalSymbolGClefOttavaBassa),
            MUSICAL_SYMBOL_C_CLEF => Ok(MusicalSymbols::MusicalSymbolCClef),
            MUSICAL_SYMBOL_F_CLEF => Ok(MusicalSymbols::MusicalSymbolFClef),
            MUSICAL_SYMBOL_F_CLEF_OTTAVA_ALTA => Ok(MusicalSymbols::MusicalSymbolFClefOttavaAlta),
            MUSICAL_SYMBOL_F_CLEF_OTTAVA_BASSA => Ok(MusicalSymbols::MusicalSymbolFClefOttavaBassa),
            MUSICAL_SYMBOL_DRUM_CLEF_DASH_1 => Ok(MusicalSymbols::MusicalSymbolDrumClefDash1),
            MUSICAL_SYMBOL_DRUM_CLEF_DASH_2 => Ok(MusicalSymbols::MusicalSymbolDrumClefDash2),
            MUSICAL_SYMBOL_MULTIPLE_MEASURE_REST => Ok(MusicalSymbols::MusicalSymbolMultipleMeasureRest),
            MUSICAL_SYMBOL_DOUBLE_SHARP => Ok(MusicalSymbols::MusicalSymbolDoubleSharp),
            MUSICAL_SYMBOL_DOUBLE_FLAT => Ok(MusicalSymbols::MusicalSymbolDoubleFlat),
            MUSICAL_SYMBOL_FLAT_UP => Ok(MusicalSymbols::MusicalSymbolFlatUp),
            MUSICAL_SYMBOL_FLAT_DOWN => Ok(MusicalSymbols::MusicalSymbolFlatDown),
            MUSICAL_SYMBOL_NATURAL_UP => Ok(MusicalSymbols::MusicalSymbolNaturalUp),
            MUSICAL_SYMBOL_NATURAL_DOWN => Ok(MusicalSymbols::MusicalSymbolNaturalDown),
            MUSICAL_SYMBOL_SHARP_UP => Ok(MusicalSymbols::MusicalSymbolSharpUp),
            MUSICAL_SYMBOL_SHARP_DOWN => Ok(MusicalSymbols::MusicalSymbolSharpDown),
            MUSICAL_SYMBOL_QUARTER_TONE_SHARP => Ok(MusicalSymbols::MusicalSymbolQuarterToneSharp),
            MUSICAL_SYMBOL_QUARTER_TONE_FLAT => Ok(MusicalSymbols::MusicalSymbolQuarterToneFlat),
            MUSICAL_SYMBOL_COMMON_TIME => Ok(MusicalSymbols::MusicalSymbolCommonTime),
            MUSICAL_SYMBOL_CUT_TIME => Ok(MusicalSymbols::MusicalSymbolCutTime),
            MUSICAL_SYMBOL_OTTAVA_ALTA => Ok(MusicalSymbols::MusicalSymbolOttavaAlta),
            MUSICAL_SYMBOL_OTTAVA_BASSA => Ok(MusicalSymbols::MusicalSymbolOttavaBassa),
            MUSICAL_SYMBOL_QUINDICESIMA_ALTA => Ok(MusicalSymbols::MusicalSymbolQuindicesimaAlta),
            MUSICAL_SYMBOL_QUINDICESIMA_BASSA => Ok(MusicalSymbols::MusicalSymbolQuindicesimaBassa),
            MUSICAL_SYMBOL_MULTI_REST => Ok(MusicalSymbols::MusicalSymbolMultiRest),
            MUSICAL_SYMBOL_WHOLE_REST => Ok(MusicalSymbols::MusicalSymbolWholeRest),
            MUSICAL_SYMBOL_HALF_REST => Ok(MusicalSymbols::MusicalSymbolHalfRest),
            MUSICAL_SYMBOL_QUARTER_REST => Ok(MusicalSymbols::MusicalSymbolQuarterRest),
            MUSICAL_SYMBOL_EIGHTH_REST => Ok(MusicalSymbols::MusicalSymbolEighthRest),
            MUSICAL_SYMBOL_SIXTEENTH_REST => Ok(MusicalSymbols::MusicalSymbolSixteenthRest),
            MUSICAL_SYMBOL_THIRTY_DASH_SECOND_REST => Ok(MusicalSymbols::MusicalSymbolThirtyDashSecondRest),
            MUSICAL_SYMBOL_SIXTY_DASH_FOURTH_REST => Ok(MusicalSymbols::MusicalSymbolSixtyDashFourthRest),
            MUSICAL_SYMBOL_ONE_HUNDRED_TWENTY_DASH_EIGHTH_REST => Ok(MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthRest),
            MUSICAL_SYMBOL_X_NOTEHEAD => Ok(MusicalSymbols::MusicalSymbolXNotehead),
            MUSICAL_SYMBOL_PLUS_NOTEHEAD => Ok(MusicalSymbols::MusicalSymbolPlusNotehead),
            MUSICAL_SYMBOL_CIRCLE_X_NOTEHEAD => Ok(MusicalSymbols::MusicalSymbolCircleXNotehead),
            MUSICAL_SYMBOL_SQUARE_NOTEHEAD_WHITE => Ok(MusicalSymbols::MusicalSymbolSquareNoteheadWhite),
            MUSICAL_SYMBOL_SQUARE_NOTEHEAD_BLACK => Ok(MusicalSymbols::MusicalSymbolSquareNoteheadBlack),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_WHITE => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadUpWhite),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_BLACK => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadUpBlack),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_LEFT_WHITE => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadLeftWhite),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_LEFT_BLACK => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadLeftBlack),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_RIGHT_WHITE => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadRightWhite),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_RIGHT_BLACK => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadRightBlack),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_DOWN_WHITE => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadDownWhite),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_DOWN_BLACK => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadDownBlack),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_RIGHT_WHITE => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightWhite),
            MUSICAL_SYMBOL_TRIANGLE_NOTEHEAD_UP_RIGHT_BLACK => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightBlack),
            MUSICAL_SYMBOL_MOON_NOTEHEAD_WHITE => Ok(MusicalSymbols::MusicalSymbolMoonNoteheadWhite),
            MUSICAL_SYMBOL_MOON_NOTEHEAD_BLACK => Ok(MusicalSymbols::MusicalSymbolMoonNoteheadBlack),
            MUSICAL_SYMBOL_TRIANGLE_DASH_ROUND_NOTEHEAD_DOWN_WHITE => Ok(MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownWhite),
            MUSICAL_SYMBOL_TRIANGLE_DASH_ROUND_NOTEHEAD_DOWN_BLACK => Ok(MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownBlack),
            MUSICAL_SYMBOL_PARENTHESIS_NOTEHEAD => Ok(MusicalSymbols::MusicalSymbolParenthesisNotehead),
            MUSICAL_SYMBOL_VOID_NOTEHEAD => Ok(MusicalSymbols::MusicalSymbolVoidNotehead),
            MUSICAL_SYMBOL_NOTEHEAD_BLACK => Ok(MusicalSymbols::MusicalSymbolNoteheadBlack),
            MUSICAL_SYMBOL_NULL_NOTEHEAD => Ok(MusicalSymbols::MusicalSymbolNullNotehead),
            MUSICAL_SYMBOL_CLUSTER_NOTEHEAD_WHITE => Ok(MusicalSymbols::MusicalSymbolClusterNoteheadWhite),
            MUSICAL_SYMBOL_CLUSTER_NOTEHEAD_BLACK => Ok(MusicalSymbols::MusicalSymbolClusterNoteheadBlack),
            MUSICAL_SYMBOL_BREVE => Ok(MusicalSymbols::MusicalSymbolBreve),
            MUSICAL_SYMBOL_WHOLE_NOTE => Ok(MusicalSymbols::MusicalSymbolWholeNote),
            MUSICAL_SYMBOL_HALF_NOTE => Ok(MusicalSymbols::MusicalSymbolHalfNote),
            MUSICAL_SYMBOL_QUARTER_NOTE => Ok(MusicalSymbols::MusicalSymbolQuarterNote),
            MUSICAL_SYMBOL_EIGHTH_NOTE => Ok(MusicalSymbols::MusicalSymbolEighthNote),
            MUSICAL_SYMBOL_SIXTEENTH_NOTE => Ok(MusicalSymbols::MusicalSymbolSixteenthNote),
            MUSICAL_SYMBOL_THIRTY_DASH_SECOND_NOTE => Ok(MusicalSymbols::MusicalSymbolThirtyDashSecondNote),
            MUSICAL_SYMBOL_SIXTY_DASH_FOURTH_NOTE => Ok(MusicalSymbols::MusicalSymbolSixtyDashFourthNote),
            MUSICAL_SYMBOL_ONE_HUNDRED_TWENTY_DASH_EIGHTH_NOTE => Ok(MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthNote),
            MUSICAL_SYMBOL_COMBINING_STEM => Ok(MusicalSymbols::MusicalSymbolCombiningStem),
            MUSICAL_SYMBOL_COMBINING_SPRECHGESANG_STEM => Ok(MusicalSymbols::MusicalSymbolCombiningSprechgesangStem),
            MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_1 => Ok(MusicalSymbols::MusicalSymbolCombiningTremoloDash1),
            MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_2 => Ok(MusicalSymbols::MusicalSymbolCombiningTremoloDash2),
            MUSICAL_SYMBOL_COMBINING_TREMOLO_DASH_3 => Ok(MusicalSymbols::MusicalSymbolCombiningTremoloDash3),
            MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_1 => Ok(MusicalSymbols::MusicalSymbolFingeredTremoloDash1),
            MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_2 => Ok(MusicalSymbols::MusicalSymbolFingeredTremoloDash2),
            MUSICAL_SYMBOL_FINGERED_TREMOLO_DASH_3 => Ok(MusicalSymbols::MusicalSymbolFingeredTremoloDash3),
            MUSICAL_SYMBOL_COMBINING_AUGMENTATION_DOT => Ok(MusicalSymbols::MusicalSymbolCombiningAugmentationDot),
            MUSICAL_SYMBOL_COMBINING_FLAG_DASH_1 => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash1),
            MUSICAL_SYMBOL_COMBINING_FLAG_DASH_2 => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash2),
            MUSICAL_SYMBOL_COMBINING_FLAG_DASH_3 => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash3),
            MUSICAL_SYMBOL_COMBINING_FLAG_DASH_4 => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash4),
            MUSICAL_SYMBOL_COMBINING_FLAG_DASH_5 => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash5),
            MUSICAL_SYMBOL_BEGIN_BEAM => Ok(MusicalSymbols::MusicalSymbolBeginBeam),
            MUSICAL_SYMBOL_END_BEAM => Ok(MusicalSymbols::MusicalSymbolEndBeam),
            MUSICAL_SYMBOL_BEGIN_TIE => Ok(MusicalSymbols::MusicalSymbolBeginTie),
            MUSICAL_SYMBOL_END_TIE => Ok(MusicalSymbols::MusicalSymbolEndTie),
            MUSICAL_SYMBOL_BEGIN_SLUR => Ok(MusicalSymbols::MusicalSymbolBeginSlur),
            MUSICAL_SYMBOL_END_SLUR => Ok(MusicalSymbols::MusicalSymbolEndSlur),
            MUSICAL_SYMBOL_BEGIN_PHRASE => Ok(MusicalSymbols::MusicalSymbolBeginPhrase),
            MUSICAL_SYMBOL_END_PHRASE => Ok(MusicalSymbols::MusicalSymbolEndPhrase),
            MUSICAL_SYMBOL_COMBINING_ACCENT => Ok(MusicalSymbols::MusicalSymbolCombiningAccent),
            MUSICAL_SYMBOL_COMBINING_STACCATO => Ok(MusicalSymbols::MusicalSymbolCombiningStaccato),
            MUSICAL_SYMBOL_COMBINING_TENUTO => Ok(MusicalSymbols::MusicalSymbolCombiningTenuto),
            MUSICAL_SYMBOL_COMBINING_STACCATISSIMO => Ok(MusicalSymbols::MusicalSymbolCombiningStaccatissimo),
            MUSICAL_SYMBOL_COMBINING_MARCATO => Ok(MusicalSymbols::MusicalSymbolCombiningMarcato),
            MUSICAL_SYMBOL_COMBINING_MARCATO_DASH_STACCATO => Ok(MusicalSymbols::MusicalSymbolCombiningMarcatoDashStaccato),
            MUSICAL_SYMBOL_COMBINING_ACCENT_DASH_STACCATO => Ok(MusicalSymbols::MusicalSymbolCombiningAccentDashStaccato),
            MUSICAL_SYMBOL_COMBINING_LOURE => Ok(MusicalSymbols::MusicalSymbolCombiningLoure),
            MUSICAL_SYMBOL_ARPEGGIATO_UP => Ok(MusicalSymbols::MusicalSymbolArpeggiatoUp),
            MUSICAL_SYMBOL_ARPEGGIATO_DOWN => Ok(MusicalSymbols::MusicalSymbolArpeggiatoDown),
            MUSICAL_SYMBOL_COMBINING_DOIT => Ok(MusicalSymbols::MusicalSymbolCombiningDoit),
            MUSICAL_SYMBOL_COMBINING_RIP => Ok(MusicalSymbols::MusicalSymbolCombiningRip),
            MUSICAL_SYMBOL_COMBINING_FLIP => Ok(MusicalSymbols::MusicalSymbolCombiningFlip),
            MUSICAL_SYMBOL_COMBINING_SMEAR => Ok(MusicalSymbols::MusicalSymbolCombiningSmear),
            MUSICAL_SYMBOL_COMBINING_BEND => Ok(MusicalSymbols::MusicalSymbolCombiningBend),
            MUSICAL_SYMBOL_COMBINING_DOUBLE_TONGUE => Ok(MusicalSymbols::MusicalSymbolCombiningDoubleTongue),
            MUSICAL_SYMBOL_COMBINING_TRIPLE_TONGUE => Ok(MusicalSymbols::MusicalSymbolCombiningTripleTongue),
            MUSICAL_SYMBOL_RINFORZANDO => Ok(MusicalSymbols::MusicalSymbolRinforzando),
            MUSICAL_SYMBOL_SUBITO => Ok(MusicalSymbols::MusicalSymbolSubito),
            MUSICAL_SYMBOL_Z => Ok(MusicalSymbols::MusicalSymbolZ),
            MUSICAL_SYMBOL_PIANO => Ok(MusicalSymbols::MusicalSymbolPiano),
            MUSICAL_SYMBOL_MEZZO => Ok(MusicalSymbols::MusicalSymbolMezzo),
            MUSICAL_SYMBOL_FORTE => Ok(MusicalSymbols::MusicalSymbolForte),
            MUSICAL_SYMBOL_CRESCENDO => Ok(MusicalSymbols::MusicalSymbolCrescendo),
            MUSICAL_SYMBOL_DECRESCENDO => Ok(MusicalSymbols::MusicalSymbolDecrescendo),
            MUSICAL_SYMBOL_GRACE_NOTE_SLASH => Ok(MusicalSymbols::MusicalSymbolGraceNoteSlash),
            MUSICAL_SYMBOL_GRACE_NOTE_NO_SLASH => Ok(MusicalSymbols::MusicalSymbolGraceNoteNoSlash),
            MUSICAL_SYMBOL_TR => Ok(MusicalSymbols::MusicalSymbolTr),
            MUSICAL_SYMBOL_TURN => Ok(MusicalSymbols::MusicalSymbolTurn),
            MUSICAL_SYMBOL_INVERTED_TURN => Ok(MusicalSymbols::MusicalSymbolInvertedTurn),
            MUSICAL_SYMBOL_TURN_SLASH => Ok(MusicalSymbols::MusicalSymbolTurnSlash),
            MUSICAL_SYMBOL_TURN_UP => Ok(MusicalSymbols::MusicalSymbolTurnUp),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_1 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash1),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_2 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash2),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_3 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash3),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_4 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash4),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_5 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash5),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_6 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash6),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_7 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash7),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_8 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash8),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_9 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash9),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_10 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash10),
            MUSICAL_SYMBOL_ORNAMENT_STROKE_DASH_11 => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash11),
            MUSICAL_SYMBOL_HAUPTSTIMME => Ok(MusicalSymbols::MusicalSymbolHauptstimme),
            MUSICAL_SYMBOL_NEBENSTIMME => Ok(MusicalSymbols::MusicalSymbolNebenstimme),
            MUSICAL_SYMBOL_END_OF_STIMME => Ok(MusicalSymbols::MusicalSymbolEndOfStimme),
            MUSICAL_SYMBOL_DEGREE_SLASH => Ok(MusicalSymbols::MusicalSymbolDegreeSlash),
            MUSICAL_SYMBOL_COMBINING_DOWN_BOW => Ok(MusicalSymbols::MusicalSymbolCombiningDownBow),
            MUSICAL_SYMBOL_COMBINING_UP_BOW => Ok(MusicalSymbols::MusicalSymbolCombiningUpBow),
            MUSICAL_SYMBOL_COMBINING_HARMONIC => Ok(MusicalSymbols::MusicalSymbolCombiningHarmonic),
            MUSICAL_SYMBOL_COMBINING_SNAP_PIZZICATO => Ok(MusicalSymbols::MusicalSymbolCombiningSnapPizzicato),
            MUSICAL_SYMBOL_PEDAL_MARK => Ok(MusicalSymbols::MusicalSymbolPedalMark),
            MUSICAL_SYMBOL_PEDAL_UP_MARK => Ok(MusicalSymbols::MusicalSymbolPedalUpMark),
            MUSICAL_SYMBOL_HALF_PEDAL_MARK => Ok(MusicalSymbols::MusicalSymbolHalfPedalMark),
            MUSICAL_SYMBOL_GLISSANDO_UP => Ok(MusicalSymbols::MusicalSymbolGlissandoUp),
            MUSICAL_SYMBOL_GLISSANDO_DOWN => Ok(MusicalSymbols::MusicalSymbolGlissandoDown),
            MUSICAL_SYMBOL_WITH_FINGERNAILS => Ok(MusicalSymbols::MusicalSymbolWithFingernails),
            MUSICAL_SYMBOL_DAMP => Ok(MusicalSymbols::MusicalSymbolDamp),
            MUSICAL_SYMBOL_DAMP_ALL => Ok(MusicalSymbols::MusicalSymbolDampAll),
            MUSICAL_SYMBOL_MAXIMA => Ok(MusicalSymbols::MusicalSymbolMaxima),
            MUSICAL_SYMBOL_LONGA => Ok(MusicalSymbols::MusicalSymbolLonga),
            MUSICAL_SYMBOL_BREVIS => Ok(MusicalSymbols::MusicalSymbolBrevis),
            MUSICAL_SYMBOL_SEMIBREVIS_WHITE => Ok(MusicalSymbols::MusicalSymbolSemibrevisWhite),
            MUSICAL_SYMBOL_SEMIBREVIS_BLACK => Ok(MusicalSymbols::MusicalSymbolSemibrevisBlack),
            MUSICAL_SYMBOL_MINIMA => Ok(MusicalSymbols::MusicalSymbolMinima),
            MUSICAL_SYMBOL_MINIMA_BLACK => Ok(MusicalSymbols::MusicalSymbolMinimaBlack),
            MUSICAL_SYMBOL_SEMIMINIMA_WHITE => Ok(MusicalSymbols::MusicalSymbolSemiminimaWhite),
            MUSICAL_SYMBOL_SEMIMINIMA_BLACK => Ok(MusicalSymbols::MusicalSymbolSemiminimaBlack),
            MUSICAL_SYMBOL_FUSA_WHITE => Ok(MusicalSymbols::MusicalSymbolFusaWhite),
            MUSICAL_SYMBOL_FUSA_BLACK => Ok(MusicalSymbols::MusicalSymbolFusaBlack),
            MUSICAL_SYMBOL_LONGA_PERFECTA_REST => Ok(MusicalSymbols::MusicalSymbolLongaPerfectaRest),
            MUSICAL_SYMBOL_LONGA_IMPERFECTA_REST => Ok(MusicalSymbols::MusicalSymbolLongaImperfectaRest),
            MUSICAL_SYMBOL_BREVIS_REST => Ok(MusicalSymbols::MusicalSymbolBrevisRest),
            MUSICAL_SYMBOL_SEMIBREVIS_REST => Ok(MusicalSymbols::MusicalSymbolSemibrevisRest),
            MUSICAL_SYMBOL_MINIMA_REST => Ok(MusicalSymbols::MusicalSymbolMinimaRest),
            MUSICAL_SYMBOL_SEMIMINIMA_REST => Ok(MusicalSymbols::MusicalSymbolSemiminimaRest),
            MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_PERFECTA => Ok(MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfecta),
            MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_IMPERFECTA => Ok(MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationeImperfecta),
            MUSICAL_SYMBOL_TEMPUS_PERFECTUM_CUM_PROLATIONE_PERFECTA_DIMINUTION_DASH_1 => Ok(MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfectaDiminutionDash1),
            MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_PERFECTA => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationePerfecta),
            MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfecta),
            MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_1 => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash1),
            MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_2 => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash2),
            MUSICAL_SYMBOL_TEMPUS_IMPERFECTUM_CUM_PROLATIONE_IMPERFECTA_DIMINUTION_DASH_3 => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash3),
            MUSICAL_SYMBOL_CROIX => Ok(MusicalSymbols::MusicalSymbolCroix),
            MUSICAL_SYMBOL_GREGORIAN_C_CLEF => Ok(MusicalSymbols::MusicalSymbolGregorianCClef),
            MUSICAL_SYMBOL_GREGORIAN_F_CLEF => Ok(MusicalSymbols::MusicalSymbolGregorianFClef),
            MUSICAL_SYMBOL_SQUARE_B => Ok(MusicalSymbols::MusicalSymbolSquareB),
            MUSICAL_SYMBOL_VIRGA => Ok(MusicalSymbols::MusicalSymbolVirga),
            MUSICAL_SYMBOL_PODATUS => Ok(MusicalSymbols::MusicalSymbolPodatus),
            MUSICAL_SYMBOL_CLIVIS => Ok(MusicalSymbols::MusicalSymbolClivis),
            MUSICAL_SYMBOL_SCANDICUS => Ok(MusicalSymbols::MusicalSymbolScandicus),
            MUSICAL_SYMBOL_CLIMACUS => Ok(MusicalSymbols::MusicalSymbolClimacus),
            MUSICAL_SYMBOL_TORCULUS => Ok(MusicalSymbols::MusicalSymbolTorculus),
            MUSICAL_SYMBOL_PORRECTUS => Ok(MusicalSymbols::MusicalSymbolPorrectus),
            MUSICAL_SYMBOL_PORRECTUS_FLEXUS => Ok(MusicalSymbols::MusicalSymbolPorrectusFlexus),
            MUSICAL_SYMBOL_SCANDICUS_FLEXUS => Ok(MusicalSymbols::MusicalSymbolScandicusFlexus),
            MUSICAL_SYMBOL_TORCULUS_RESUPINUS => Ok(MusicalSymbols::MusicalSymbolTorculusResupinus),
            MUSICAL_SYMBOL_PES_SUBPUNCTIS => Ok(MusicalSymbols::MusicalSymbolPesSubpunctis),
            MUSICAL_SYMBOL_KIEVAN_C_CLEF => Ok(MusicalSymbols::MusicalSymbolKievanCClef),
            MUSICAL_SYMBOL_KIEVAN_END_OF_PIECE => Ok(MusicalSymbols::MusicalSymbolKievanEndOfPiece),
            MUSICAL_SYMBOL_KIEVAN_FINAL_NOTE => Ok(MusicalSymbols::MusicalSymbolKievanFinalNote),
            MUSICAL_SYMBOL_KIEVAN_RECITATIVE_MARK => Ok(MusicalSymbols::MusicalSymbolKievanRecitativeMark),
            MUSICAL_SYMBOL_KIEVAN_WHOLE_NOTE => Ok(MusicalSymbols::MusicalSymbolKievanWholeNote),
            MUSICAL_SYMBOL_KIEVAN_HALF_NOTE => Ok(MusicalSymbols::MusicalSymbolKievanHalfNote),
            MUSICAL_SYMBOL_KIEVAN_QUARTER_NOTE_STEM_DOWN => Ok(MusicalSymbols::MusicalSymbolKievanQuarterNoteStemDown),
            MUSICAL_SYMBOL_KIEVAN_QUARTER_NOTE_STEM_UP => Ok(MusicalSymbols::MusicalSymbolKievanQuarterNoteStemUp),
            MUSICAL_SYMBOL_KIEVAN_EIGHTH_NOTE_STEM_DOWN => Ok(MusicalSymbols::MusicalSymbolKievanEighthNoteStemDown),
            MUSICAL_SYMBOL_KIEVAN_EIGHTH_NOTE_STEM_UP => Ok(MusicalSymbols::MusicalSymbolKievanEighthNoteStemUp),
            MUSICAL_SYMBOL_KIEVAN_FLAT_SIGN => Ok(MusicalSymbols::MusicalSymbolKievanFlatSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MusicalSymbols {
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

impl std::convert::TryFrom<u32> for MusicalSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MusicalSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MusicalSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MusicalSymbols::MusicalSymbolSingleBarline
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MusicalSymbols::MusicalSymbolSingleBarline => "musical symbol single barline",
            MusicalSymbols::MusicalSymbolDoubleBarline => "musical symbol double barline",
            MusicalSymbols::MusicalSymbolFinalBarline => "musical symbol final barline",
            MusicalSymbols::MusicalSymbolReverseFinalBarline => "musical symbol reverse final barline",
            MusicalSymbols::MusicalSymbolDashedBarline => "musical symbol dashed barline",
            MusicalSymbols::MusicalSymbolShortBarline => "musical symbol short barline",
            MusicalSymbols::MusicalSymbolLeftRepeatSign => "musical symbol left repeat sign",
            MusicalSymbols::MusicalSymbolRightRepeatSign => "musical symbol right repeat sign",
            MusicalSymbols::MusicalSymbolRepeatDots => "musical symbol repeat dots",
            MusicalSymbols::MusicalSymbolDalSegno => "musical symbol dal segno",
            MusicalSymbols::MusicalSymbolDaCapo => "musical symbol da capo",
            MusicalSymbols::MusicalSymbolSegno => "musical symbol segno",
            MusicalSymbols::MusicalSymbolCoda => "musical symbol coda",
            MusicalSymbols::MusicalSymbolRepeatedFigureDash1 => "musical symbol repeated figure-1",
            MusicalSymbols::MusicalSymbolRepeatedFigureDash2 => "musical symbol repeated figure-2",
            MusicalSymbols::MusicalSymbolRepeatedFigureDash3 => "musical symbol repeated figure-3",
            MusicalSymbols::MusicalSymbolFermata => "musical symbol fermata",
            MusicalSymbols::MusicalSymbolFermataBelow => "musical symbol fermata below",
            MusicalSymbols::MusicalSymbolBreathMark => "musical symbol breath mark",
            MusicalSymbols::MusicalSymbolCaesura => "musical symbol caesura",
            MusicalSymbols::MusicalSymbolBrace => "musical symbol brace",
            MusicalSymbols::MusicalSymbolBracket => "musical symbol bracket",
            MusicalSymbols::MusicalSymbolOneDashLineStaff => "musical symbol one-line staff",
            MusicalSymbols::MusicalSymbolTwoDashLineStaff => "musical symbol two-line staff",
            MusicalSymbols::MusicalSymbolThreeDashLineStaff => "musical symbol three-line staff",
            MusicalSymbols::MusicalSymbolFourDashLineStaff => "musical symbol four-line staff",
            MusicalSymbols::MusicalSymbolFiveDashLineStaff => "musical symbol five-line staff",
            MusicalSymbols::MusicalSymbolSixDashLineStaff => "musical symbol six-line staff",
            MusicalSymbols::MusicalSymbolSixDashStringFretboard => "musical symbol six-string fretboard",
            MusicalSymbols::MusicalSymbolFourDashStringFretboard => "musical symbol four-string fretboard",
            MusicalSymbols::MusicalSymbolGClef => "musical symbol g clef",
            MusicalSymbols::MusicalSymbolGClefOttavaAlta => "musical symbol g clef ottava alta",
            MusicalSymbols::MusicalSymbolGClefOttavaBassa => "musical symbol g clef ottava bassa",
            MusicalSymbols::MusicalSymbolCClef => "musical symbol c clef",
            MusicalSymbols::MusicalSymbolFClef => "musical symbol f clef",
            MusicalSymbols::MusicalSymbolFClefOttavaAlta => "musical symbol f clef ottava alta",
            MusicalSymbols::MusicalSymbolFClefOttavaBassa => "musical symbol f clef ottava bassa",
            MusicalSymbols::MusicalSymbolDrumClefDash1 => "musical symbol drum clef-1",
            MusicalSymbols::MusicalSymbolDrumClefDash2 => "musical symbol drum clef-2",
            MusicalSymbols::MusicalSymbolMultipleMeasureRest => "musical symbol multiple measure rest",
            MusicalSymbols::MusicalSymbolDoubleSharp => "musical symbol double sharp",
            MusicalSymbols::MusicalSymbolDoubleFlat => "musical symbol double flat",
            MusicalSymbols::MusicalSymbolFlatUp => "musical symbol flat up",
            MusicalSymbols::MusicalSymbolFlatDown => "musical symbol flat down",
            MusicalSymbols::MusicalSymbolNaturalUp => "musical symbol natural up",
            MusicalSymbols::MusicalSymbolNaturalDown => "musical symbol natural down",
            MusicalSymbols::MusicalSymbolSharpUp => "musical symbol sharp up",
            MusicalSymbols::MusicalSymbolSharpDown => "musical symbol sharp down",
            MusicalSymbols::MusicalSymbolQuarterToneSharp => "musical symbol quarter tone sharp",
            MusicalSymbols::MusicalSymbolQuarterToneFlat => "musical symbol quarter tone flat",
            MusicalSymbols::MusicalSymbolCommonTime => "musical symbol common time",
            MusicalSymbols::MusicalSymbolCutTime => "musical symbol cut time",
            MusicalSymbols::MusicalSymbolOttavaAlta => "musical symbol ottava alta",
            MusicalSymbols::MusicalSymbolOttavaBassa => "musical symbol ottava bassa",
            MusicalSymbols::MusicalSymbolQuindicesimaAlta => "musical symbol quindicesima alta",
            MusicalSymbols::MusicalSymbolQuindicesimaBassa => "musical symbol quindicesima bassa",
            MusicalSymbols::MusicalSymbolMultiRest => "musical symbol multi rest",
            MusicalSymbols::MusicalSymbolWholeRest => "musical symbol whole rest",
            MusicalSymbols::MusicalSymbolHalfRest => "musical symbol half rest",
            MusicalSymbols::MusicalSymbolQuarterRest => "musical symbol quarter rest",
            MusicalSymbols::MusicalSymbolEighthRest => "musical symbol eighth rest",
            MusicalSymbols::MusicalSymbolSixteenthRest => "musical symbol sixteenth rest",
            MusicalSymbols::MusicalSymbolThirtyDashSecondRest => "musical symbol thirty-second rest",
            MusicalSymbols::MusicalSymbolSixtyDashFourthRest => "musical symbol sixty-fourth rest",
            MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthRest => "musical symbol one hundred twenty-eighth rest",
            MusicalSymbols::MusicalSymbolXNotehead => "musical symbol x notehead",
            MusicalSymbols::MusicalSymbolPlusNotehead => "musical symbol plus notehead",
            MusicalSymbols::MusicalSymbolCircleXNotehead => "musical symbol circle x notehead",
            MusicalSymbols::MusicalSymbolSquareNoteheadWhite => "musical symbol square notehead white",
            MusicalSymbols::MusicalSymbolSquareNoteheadBlack => "musical symbol square notehead black",
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpWhite => "musical symbol triangle notehead up white",
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpBlack => "musical symbol triangle notehead up black",
            MusicalSymbols::MusicalSymbolTriangleNoteheadLeftWhite => "musical symbol triangle notehead left white",
            MusicalSymbols::MusicalSymbolTriangleNoteheadLeftBlack => "musical symbol triangle notehead left black",
            MusicalSymbols::MusicalSymbolTriangleNoteheadRightWhite => "musical symbol triangle notehead right white",
            MusicalSymbols::MusicalSymbolTriangleNoteheadRightBlack => "musical symbol triangle notehead right black",
            MusicalSymbols::MusicalSymbolTriangleNoteheadDownWhite => "musical symbol triangle notehead down white",
            MusicalSymbols::MusicalSymbolTriangleNoteheadDownBlack => "musical symbol triangle notehead down black",
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightWhite => "musical symbol triangle notehead up right white",
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightBlack => "musical symbol triangle notehead up right black",
            MusicalSymbols::MusicalSymbolMoonNoteheadWhite => "musical symbol moon notehead white",
            MusicalSymbols::MusicalSymbolMoonNoteheadBlack => "musical symbol moon notehead black",
            MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownWhite => "musical symbol triangle-round notehead down white",
            MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownBlack => "musical symbol triangle-round notehead down black",
            MusicalSymbols::MusicalSymbolParenthesisNotehead => "musical symbol parenthesis notehead",
            MusicalSymbols::MusicalSymbolVoidNotehead => "musical symbol void notehead",
            MusicalSymbols::MusicalSymbolNoteheadBlack => "musical symbol notehead black",
            MusicalSymbols::MusicalSymbolNullNotehead => "musical symbol null notehead",
            MusicalSymbols::MusicalSymbolClusterNoteheadWhite => "musical symbol cluster notehead white",
            MusicalSymbols::MusicalSymbolClusterNoteheadBlack => "musical symbol cluster notehead black",
            MusicalSymbols::MusicalSymbolBreve => "musical symbol breve",
            MusicalSymbols::MusicalSymbolWholeNote => "musical symbol whole note",
            MusicalSymbols::MusicalSymbolHalfNote => "musical symbol half note",
            MusicalSymbols::MusicalSymbolQuarterNote => "musical symbol quarter note",
            MusicalSymbols::MusicalSymbolEighthNote => "musical symbol eighth note",
            MusicalSymbols::MusicalSymbolSixteenthNote => "musical symbol sixteenth note",
            MusicalSymbols::MusicalSymbolThirtyDashSecondNote => "musical symbol thirty-second note",
            MusicalSymbols::MusicalSymbolSixtyDashFourthNote => "musical symbol sixty-fourth note",
            MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthNote => "musical symbol one hundred twenty-eighth note",
            MusicalSymbols::MusicalSymbolCombiningStem => "musical symbol combining stem",
            MusicalSymbols::MusicalSymbolCombiningSprechgesangStem => "musical symbol combining sprechgesang stem",
            MusicalSymbols::MusicalSymbolCombiningTremoloDash1 => "musical symbol combining tremolo-1",
            MusicalSymbols::MusicalSymbolCombiningTremoloDash2 => "musical symbol combining tremolo-2",
            MusicalSymbols::MusicalSymbolCombiningTremoloDash3 => "musical symbol combining tremolo-3",
            MusicalSymbols::MusicalSymbolFingeredTremoloDash1 => "musical symbol fingered tremolo-1",
            MusicalSymbols::MusicalSymbolFingeredTremoloDash2 => "musical symbol fingered tremolo-2",
            MusicalSymbols::MusicalSymbolFingeredTremoloDash3 => "musical symbol fingered tremolo-3",
            MusicalSymbols::MusicalSymbolCombiningAugmentationDot => "musical symbol combining augmentation dot",
            MusicalSymbols::MusicalSymbolCombiningFlagDash1 => "musical symbol combining flag-1",
            MusicalSymbols::MusicalSymbolCombiningFlagDash2 => "musical symbol combining flag-2",
            MusicalSymbols::MusicalSymbolCombiningFlagDash3 => "musical symbol combining flag-3",
            MusicalSymbols::MusicalSymbolCombiningFlagDash4 => "musical symbol combining flag-4",
            MusicalSymbols::MusicalSymbolCombiningFlagDash5 => "musical symbol combining flag-5",
            MusicalSymbols::MusicalSymbolBeginBeam => "musical symbol begin beam",
            MusicalSymbols::MusicalSymbolEndBeam => "musical symbol end beam",
            MusicalSymbols::MusicalSymbolBeginTie => "musical symbol begin tie",
            MusicalSymbols::MusicalSymbolEndTie => "musical symbol end tie",
            MusicalSymbols::MusicalSymbolBeginSlur => "musical symbol begin slur",
            MusicalSymbols::MusicalSymbolEndSlur => "musical symbol end slur",
            MusicalSymbols::MusicalSymbolBeginPhrase => "musical symbol begin phrase",
            MusicalSymbols::MusicalSymbolEndPhrase => "musical symbol end phrase",
            MusicalSymbols::MusicalSymbolCombiningAccent => "musical symbol combining accent",
            MusicalSymbols::MusicalSymbolCombiningStaccato => "musical symbol combining staccato",
            MusicalSymbols::MusicalSymbolCombiningTenuto => "musical symbol combining tenuto",
            MusicalSymbols::MusicalSymbolCombiningStaccatissimo => "musical symbol combining staccatissimo",
            MusicalSymbols::MusicalSymbolCombiningMarcato => "musical symbol combining marcato",
            MusicalSymbols::MusicalSymbolCombiningMarcatoDashStaccato => "musical symbol combining marcato-staccato",
            MusicalSymbols::MusicalSymbolCombiningAccentDashStaccato => "musical symbol combining accent-staccato",
            MusicalSymbols::MusicalSymbolCombiningLoure => "musical symbol combining loure",
            MusicalSymbols::MusicalSymbolArpeggiatoUp => "musical symbol arpeggiato up",
            MusicalSymbols::MusicalSymbolArpeggiatoDown => "musical symbol arpeggiato down",
            MusicalSymbols::MusicalSymbolCombiningDoit => "musical symbol combining doit",
            MusicalSymbols::MusicalSymbolCombiningRip => "musical symbol combining rip",
            MusicalSymbols::MusicalSymbolCombiningFlip => "musical symbol combining flip",
            MusicalSymbols::MusicalSymbolCombiningSmear => "musical symbol combining smear",
            MusicalSymbols::MusicalSymbolCombiningBend => "musical symbol combining bend",
            MusicalSymbols::MusicalSymbolCombiningDoubleTongue => "musical symbol combining double tongue",
            MusicalSymbols::MusicalSymbolCombiningTripleTongue => "musical symbol combining triple tongue",
            MusicalSymbols::MusicalSymbolRinforzando => "musical symbol rinforzando",
            MusicalSymbols::MusicalSymbolSubito => "musical symbol subito",
            MusicalSymbols::MusicalSymbolZ => "musical symbol z",
            MusicalSymbols::MusicalSymbolPiano => "musical symbol piano",
            MusicalSymbols::MusicalSymbolMezzo => "musical symbol mezzo",
            MusicalSymbols::MusicalSymbolForte => "musical symbol forte",
            MusicalSymbols::MusicalSymbolCrescendo => "musical symbol crescendo",
            MusicalSymbols::MusicalSymbolDecrescendo => "musical symbol decrescendo",
            MusicalSymbols::MusicalSymbolGraceNoteSlash => "musical symbol grace note slash",
            MusicalSymbols::MusicalSymbolGraceNoteNoSlash => "musical symbol grace note no slash",
            MusicalSymbols::MusicalSymbolTr => "musical symbol tr",
            MusicalSymbols::MusicalSymbolTurn => "musical symbol turn",
            MusicalSymbols::MusicalSymbolInvertedTurn => "musical symbol inverted turn",
            MusicalSymbols::MusicalSymbolTurnSlash => "musical symbol turn slash",
            MusicalSymbols::MusicalSymbolTurnUp => "musical symbol turn up",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash1 => "musical symbol ornament stroke-1",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash2 => "musical symbol ornament stroke-2",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash3 => "musical symbol ornament stroke-3",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash4 => "musical symbol ornament stroke-4",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash5 => "musical symbol ornament stroke-5",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash6 => "musical symbol ornament stroke-6",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash7 => "musical symbol ornament stroke-7",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash8 => "musical symbol ornament stroke-8",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash9 => "musical symbol ornament stroke-9",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash10 => "musical symbol ornament stroke-10",
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash11 => "musical symbol ornament stroke-11",
            MusicalSymbols::MusicalSymbolHauptstimme => "musical symbol hauptstimme",
            MusicalSymbols::MusicalSymbolNebenstimme => "musical symbol nebenstimme",
            MusicalSymbols::MusicalSymbolEndOfStimme => "musical symbol end of stimme",
            MusicalSymbols::MusicalSymbolDegreeSlash => "musical symbol degree slash",
            MusicalSymbols::MusicalSymbolCombiningDownBow => "musical symbol combining down bow",
            MusicalSymbols::MusicalSymbolCombiningUpBow => "musical symbol combining up bow",
            MusicalSymbols::MusicalSymbolCombiningHarmonic => "musical symbol combining harmonic",
            MusicalSymbols::MusicalSymbolCombiningSnapPizzicato => "musical symbol combining snap pizzicato",
            MusicalSymbols::MusicalSymbolPedalMark => "musical symbol pedal mark",
            MusicalSymbols::MusicalSymbolPedalUpMark => "musical symbol pedal up mark",
            MusicalSymbols::MusicalSymbolHalfPedalMark => "musical symbol half pedal mark",
            MusicalSymbols::MusicalSymbolGlissandoUp => "musical symbol glissando up",
            MusicalSymbols::MusicalSymbolGlissandoDown => "musical symbol glissando down",
            MusicalSymbols::MusicalSymbolWithFingernails => "musical symbol with fingernails",
            MusicalSymbols::MusicalSymbolDamp => "musical symbol damp",
            MusicalSymbols::MusicalSymbolDampAll => "musical symbol damp all",
            MusicalSymbols::MusicalSymbolMaxima => "musical symbol maxima",
            MusicalSymbols::MusicalSymbolLonga => "musical symbol longa",
            MusicalSymbols::MusicalSymbolBrevis => "musical symbol brevis",
            MusicalSymbols::MusicalSymbolSemibrevisWhite => "musical symbol semibrevis white",
            MusicalSymbols::MusicalSymbolSemibrevisBlack => "musical symbol semibrevis black",
            MusicalSymbols::MusicalSymbolMinima => "musical symbol minima",
            MusicalSymbols::MusicalSymbolMinimaBlack => "musical symbol minima black",
            MusicalSymbols::MusicalSymbolSemiminimaWhite => "musical symbol semiminima white",
            MusicalSymbols::MusicalSymbolSemiminimaBlack => "musical symbol semiminima black",
            MusicalSymbols::MusicalSymbolFusaWhite => "musical symbol fusa white",
            MusicalSymbols::MusicalSymbolFusaBlack => "musical symbol fusa black",
            MusicalSymbols::MusicalSymbolLongaPerfectaRest => "musical symbol longa perfecta rest",
            MusicalSymbols::MusicalSymbolLongaImperfectaRest => "musical symbol longa imperfecta rest",
            MusicalSymbols::MusicalSymbolBrevisRest => "musical symbol brevis rest",
            MusicalSymbols::MusicalSymbolSemibrevisRest => "musical symbol semibrevis rest",
            MusicalSymbols::MusicalSymbolMinimaRest => "musical symbol minima rest",
            MusicalSymbols::MusicalSymbolSemiminimaRest => "musical symbol semiminima rest",
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfecta => "musical symbol tempus perfectum cum prolatione perfecta",
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationeImperfecta => "musical symbol tempus perfectum cum prolatione imperfecta",
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfectaDiminutionDash1 => "musical symbol tempus perfectum cum prolatione perfecta diminution-1",
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationePerfecta => "musical symbol tempus imperfectum cum prolatione perfecta",
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfecta => "musical symbol tempus imperfectum cum prolatione imperfecta",
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash1 => "musical symbol tempus imperfectum cum prolatione imperfecta diminution-1",
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash2 => "musical symbol tempus imperfectum cum prolatione imperfecta diminution-2",
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash3 => "musical symbol tempus imperfectum cum prolatione imperfecta diminution-3",
            MusicalSymbols::MusicalSymbolCroix => "musical symbol croix",
            MusicalSymbols::MusicalSymbolGregorianCClef => "musical symbol gregorian c clef",
            MusicalSymbols::MusicalSymbolGregorianFClef => "musical symbol gregorian f clef",
            MusicalSymbols::MusicalSymbolSquareB => "musical symbol square b",
            MusicalSymbols::MusicalSymbolVirga => "musical symbol virga",
            MusicalSymbols::MusicalSymbolPodatus => "musical symbol podatus",
            MusicalSymbols::MusicalSymbolClivis => "musical symbol clivis",
            MusicalSymbols::MusicalSymbolScandicus => "musical symbol scandicus",
            MusicalSymbols::MusicalSymbolClimacus => "musical symbol climacus",
            MusicalSymbols::MusicalSymbolTorculus => "musical symbol torculus",
            MusicalSymbols::MusicalSymbolPorrectus => "musical symbol porrectus",
            MusicalSymbols::MusicalSymbolPorrectusFlexus => "musical symbol porrectus flexus",
            MusicalSymbols::MusicalSymbolScandicusFlexus => "musical symbol scandicus flexus",
            MusicalSymbols::MusicalSymbolTorculusResupinus => "musical symbol torculus resupinus",
            MusicalSymbols::MusicalSymbolPesSubpunctis => "musical symbol pes subpunctis",
            MusicalSymbols::MusicalSymbolKievanCClef => "musical symbol kievan c clef",
            MusicalSymbols::MusicalSymbolKievanEndOfPiece => "musical symbol kievan end of piece",
            MusicalSymbols::MusicalSymbolKievanFinalNote => "musical symbol kievan final note",
            MusicalSymbols::MusicalSymbolKievanRecitativeMark => "musical symbol kievan recitative mark",
            MusicalSymbols::MusicalSymbolKievanWholeNote => "musical symbol kievan whole note",
            MusicalSymbols::MusicalSymbolKievanHalfNote => "musical symbol kievan half note",
            MusicalSymbols::MusicalSymbolKievanQuarterNoteStemDown => "musical symbol kievan quarter note stem down",
            MusicalSymbols::MusicalSymbolKievanQuarterNoteStemUp => "musical symbol kievan quarter note stem up",
            MusicalSymbols::MusicalSymbolKievanEighthNoteStemDown => "musical symbol kievan eighth note stem down",
            MusicalSymbols::MusicalSymbolKievanEighthNoteStemUp => "musical symbol kievan eighth note stem up",
            MusicalSymbols::MusicalSymbolKievanFlatSign => "musical symbol kievan flat sign",
        }
    }
}
