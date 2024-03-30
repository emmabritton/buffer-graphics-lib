use crate::text::{
    ASCII_CENT, ASCII_CHECK, ASCII_CURRENCY, ASCII_DEGREE, ASCII_ELLIPSIS, ASCII_EURO, ASCII_POUND,
    ASCII_YEN,
};

pub const CHAR_WIDTH: usize = 6;
pub const CHAR_HEIGHT: usize = 7;

pub const fn get_px_ascii(code: u8) -> &'static [bool] {
    match code {
        33 => &EXCLAIM,
        34 => &DOUBLE_QUOTE,
        35 => &HASH,
        36 => &DOLLAR,
        37 => &PERCENT,
        38 => &AMPERSAND,
        39 => &QUOTE,
        40 => &PAREN_L,
        41 => &PAREN_R,
        42 => &ASTERISK,
        43 => &PLUS,
        44 => &COMMA,
        45 => &MINUS,
        46 => &PERIOD,
        47 => &SLASH,
        48 => &ZERO,
        49 => &ONE,
        50 => &TWO,
        51 => &THREE,
        52 => &FOUR,
        53 => &FIVE,
        54 => &SIX,
        55 => &SEVEN,
        56 => &EIGHT,
        57 => &NINE,
        58 => &COLON,
        59 => &SEMICOLON,
        60 => &ANGLE_L,
        61 => &EQUALS,
        62 => &ANGLE_R,
        63 => &QUESTION,
        64 => &AT,
        65 => &A,
        66 => &B,
        67 => &C,
        68 => &D,
        69 => &E,
        70 => &F,
        71 => &G,
        72 => &H,
        73 => &I,
        74 => &J,
        75 => &K,
        76 => &L,
        77 => &M,
        78 => &N,
        79 => &O,
        80 => &P,
        81 => &Q,
        82 => &R,
        83 => &S,
        84 => &T,
        85 => &U,
        86 => &V,
        87 => &W,
        88 => &X,
        89 => &Y,
        90 => &Z,
        91 => &SQUARE_L,
        92 => &BACKSLASH,
        93 => &SQUARE_R,
        95 => &UNDERSCORE,
        94 => &POWER,
        96 => &BACKTICK,
        97 => &LOWER_A,
        98 => &LOWER_B,
        99 => &LOWER_C,
        100 => &LOWER_D,
        101 => &LOWER_E,
        102 => &LOWER_F,
        103 => &LOWER_G,
        104 => &LOWER_H,
        105 => &LOWER_I,
        106 => &LOWER_J,
        107 => &LOWER_K,
        108 => &LOWER_L,
        109 => &LOWER_M,
        110 => &LOWER_N,
        111 => &LOWER_O,
        112 => &LOWER_P,
        113 => &LOWER_Q,
        114 => &LOWER_R,
        115 => &LOWER_S,
        116 => &LOWER_T,
        117 => &LOWER_U,
        118 => &LOWER_V,
        119 => &LOWER_W,
        120 => &LOWER_X,
        121 => &LOWER_Y,
        122 => &LOWER_Z,
        123 => &CURLY_L,
        124 => &PIPE,
        125 => &CURLY_R,
        126 => &TILDE,
        ASCII_EURO => &EURO,
        ASCII_ELLIPSIS => &ELLIPSIS,
        ASCII_DEGREE => &DEGREE,
        ASCII_POUND => &POUND,
        ASCII_CURRENCY => &CURRENCY,
        ASCII_YEN => &YEN,
        ASCII_CENT => &CENT,
        ASCII_CHECK => &CHECK,
        _ => &UNKNOWN,
    }
}

pub const LETTER_PX_COUNT: usize = CHAR_WIDTH * CHAR_HEIGHT;

const AT: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, true, false, true, false, false, false, false, true, true, false,
    false, false, false, true, true, false, true, true, false, true, true, false, true, true, true,
    true, true, false, false, false, false, false, false, true, true, true, true, true,
];
const A: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, true, true, true, true, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, false, false, false, false,
];
const B: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, false, false, false, true, false, false, true, false, false, true,
    true, true, false, false, false, true, false, false, true, false, false, true, false, false,
    true, false, false, true, true, true, false, false, false, false, false, false, false, false,
];
const C: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, false, false, false, true, false, false, false, false, false, true, false, false,
    true, false, false, false, true, true, false, false, false, false, false, false, false, false,
];
const D: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, true, false, false, true, false, false, true, false, false,
    true, false, false, true, true, true, false, false, false, false, false, false, false, false,
];
const E: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, true, false, false, true, false, false, false, false, false, true,
    true, true, false, false, false, true, false, false, false, false, false, true, false, false,
    false, false, false, true, true, true, true, false, false, false, false, false, false, false,
];
const F: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, true, false, false, true, false, false, false, false, false, true,
    true, true, false, false, false, true, false, false, false, false, false, true, false, false,
    false, false, false, true, false, false, false, false, false, false, false, false, false,
    false,
];
const G: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, false, false, false, true, false, true, true, false, false, true, false, false,
    true, false, false, false, true, true, false, false, false, false, false, false, false, false,
];
const H: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, true, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, true, true, true, true, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, false, false, false, false,
];
const I: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, true, false,
    false, false, true, true, true, true, true, false, false, false, false, false, false, false,
];
const J: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, true, false, true, false,
    false, false, true, true, true, false, false, false, false, false, false, false, false, false,
];
const K: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, true, false, false, true, false, true, false, false, false, true,
    true, false, false, false, false, true, false, true, false, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, false, false, false, false,
];
const L: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, false, true, false, false, false, false, false, true,
    false, false, false, false, false, true, false, false, false, false, false, true, false, false,
    false, false, false, true, true, true, true, false, false, false, false, false, false, false,
];
const M: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, true, true, false, true, true, false, true, false,
    true, false, true, false, true, false, false, false, true, false, true, false, false, false,
    true, false, true, false, false, false, true, false, false, false, false, false, false, false,
];
const N: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, true, true, false, false, true, false, true, false,
    true, false, true, false, true, false, false, true, true, false, true, false, false, false,
    true, false, true, false, false, false, true, false, false, false, false, false, false, false,
];
const O: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, false, false, true, false, false, false, true, false, true, false,
    false, false, true, false, true, false, false, false, true, false, true, false, false, false,
    true, false, false, true, true, true, false, false, false, false, false, false, false, false,
];
const P: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, true, true, true, false, false, false, true, false, false,
    false, false, false, true, false, false, false, false, false, false, false, false, false,
    false,
];
const R: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, true, true, true, false, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, false, false, false, false,
];
const S: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, false,
    true, false, false, false, false, false, false, true, false, false, false, true, false, false,
    true, false, false, false, true, true, false, false, false, false, false, false, false, false,
];
const T: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, true, false,
    false, false, false, false, true, false, false, false, false, false, false, false, false,
    false,
];
const U: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, true, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, true, false, false, true, false, false, true, false, false,
    true, false, false, false, true, true, false, false, false, false, false, false, false, false,
];
const Q: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, false, false, true, false, false, false, true, false, true, false,
    false, false, true, false, true, false, true, false, true, false, true, false, false, true,
    false, false, false, true, true, false, true, false, false, false, false, false, false, false,
];
const W: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, true, false, false, false, true, false, true, false,
    false, false, true, false, true, false, false, false, true, false, true, false, true, false,
    true, false, false, true, false, true, false, false, false, false, false, false, false, false,
];
const V: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, true, false, false, false, true, false, true, false,
    false, false, true, false, true, false, false, false, true, false, false, true, false, true,
    false, false, false, false, true, false, false, false, false, false, false, false, false,
    false,
];
const X: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, true, false, false, false, true, false, false, true,
    false, true, false, false, false, false, true, false, false, false, false, true, false, true,
    false, false, true, false, false, false, true, false, false, false, false, false, false, false,
];
const Y: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, true, false, false, false, true, false, false, true,
    false, true, false, false, false, false, true, false, false, false, false, false, true, false,
    false, false, false, false, true, false, false, false, false, false, false, false, false,
    false,
];
const Z: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, true, false, false, false, false, false, true, false, false, false,
    false, true, false, false, false, false, true, false, false, false, false, true, false, false,
    false, false, false, true, true, true, true, false, false, false, false, false, false, false,
];
const AMPERSAND: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, true, false, false, false, false, true, false, false, false, false, true, false, true,
    false, true, false, true, false, false, true, false, false, false, true, true, false, true,
];
const EXCLAIM: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, true, false,
    false, false, false, false, false, false, false, false, false, false, true, false, false,
    false,
];
const PERIOD: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, true,
    false, false, false,
];
const COMMA: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, false, false, false, false, false, true, false,
    false, false,
];
const COLON: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, false, false, false, false, false, false,
    false, false, false,
];
const SEMICOLON: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false,
];
const PLUS: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true, false, false, false, false, true, true, true, false, false, false, false,
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];
const MINUS: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, true, true, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];
const EQUALS: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, true, true, false, false, false, false, false, false, false, false, false, true, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];
const SQUARE_L: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, true, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, true, false,
    false, false, false, false, true, false, false, false, false, false, true, true, true, false,
];
const SQUARE_R: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, false, false, false, false, false, true, false, false, false, false,
    false, true, false, false, false, false, false, true, false, false, false, false, false, true,
    false, false, false, false, false, true, false, false, false, true, true, true, false, false,
];
const PAREN_L: [bool; LETTER_PX_COUNT] = [
    false, false, false, true, true, false, false, false, true, true, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, true, false,
    false, false, false, false, true, true, false, false, false, false, false, true, true, false,
];
const PAREN_R: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, false, false, false, false, true, true, false, false, false, false,
    false, true, false, false, false, false, false, true, false, false, false, false, false, true,
    false, false, false, false, true, true, false, false, false, true, true, false, false, false,
];
const ANGLE_L: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, true, false, false, false, false, true, false, false, false, false,
    true, false, false, false, false, true, false, false, false, false, false, false, true, false,
    false, false, false, false, false, true, false, false, false, false, false, false, true, false,
];
const ANGLE_R: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, false, false, true, false, false, false, false, false,
    false, true, false, false, false, false, false, false, true, false, false, false, false, true,
    false, false, false, false, true, false, false, false, false, true, false, false, false, false,
];
const DOUBLE_QUOTE: [bool; LETTER_PX_COUNT] = [
    false, true, false, true, false, false, false, true, false, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];
const QUOTE: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, false, false, true, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];
const QUESTION: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, false,
    false, false, true, false, false, false, false, false, true, false, false, false, false, true,
    false, false, false, false, false, false, false, false, false, false, false, true, false,
    false,
];
const SLASH: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, true, false,
    false, false, false, true, false, false, false, false, true, false, false, false, false, true,
    false, false, false, false, true, false, false, false, false, true, false, false, false, false,
    false,
];
const BACKSLASH: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, true, false, false, false, false, false, false, true,
    false, false, false, false, false, false, true, false, false, false, false, false, false, true,
    false, false, false, false, false, false, true, false, false, false, false, false, false, true,
];
const ASTERISK: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, true, false, true, false, true, false, false, true,
    true, true, false, false, true, true, true, true, true, false, false, true, true, true, false,
    false, true, false, true, false, true, false, false, false, false, false, false, false,
];
const PERCENT: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, true, false, false, false, false, true, false, false,
    false, false, true, false, false, false, false, true, false, false, false, false, true, false,
    false, false, false, true, false, false, false, false, true, false, false, false, false, true,
];
const ZERO: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, true, false, false, true, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, true, true, false, false,
];
const ONE: [bool; LETTER_PX_COUNT] = [
    false, false, false, true, false, false, false, false, true, true, false, false, false, false,
    false, true, false, false, false, false, false, true, false, false, false, false, false, true,
    false, false, false, false, false, true, false, false, false, false, true, true, true, false,
];
const TWO: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, false,
    false, false, true, false, false, false, false, true, false, false, false, false, true, false,
    false, false, false, true, false, false, false, false, false, true, true, true, true, false,
];
const THREE: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, false,
    false, false, true, false, false, false, true, true, false, false, false, false, false, false,
    true, false, false, true, false, false, true, false, false, false, true, true, false, false,
];
const FOUR: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, false, true, false, false, false, false, false, true,
    false, false, false, false, false, true, false, true, false, false, false, true, false, true,
    false, false, false, true, true, true, true, false, false, false, false, true, false, false,
];
const FIVE: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, true, false, false, true, false, false, false, false, false, true,
    false, false, false, false, false, true, true, true, false, false, false, false, false, false,
    true, false, false, true, false, false, true, false, false, false, true, true, false, false,
];
const SIX: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, false, false, false, true, true, true, false, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, true, true, false, false,
];
const SEVEN: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, true, false, false, false, false, false, true, false, false, false,
    false, false, true, false, false, false, false, true, false, false, false, false, false, true,
    false, false, false, false, true, false, false, false, false, false, true, false, false, false,
];
const EIGHT: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, false, true, true, false, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, true, true, false, false,
];
const NINE: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, false, true, true, true, false, false, false, false, false,
    true, false, false, true, false, false, true, false, false, false, true, true, false, false,
];
const UNDERSCORE: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, true, true, true,
    true, false,
];
const UNKNOWN: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, true, true, true, false, false, false, false, true, true, false, false,
    false, false, true, true, false, false, false, false, true, true, false, false, false, false,
    true, true, false, false, false, false, true, true, true, true, true, true, true,
];
const HASH: [bool; LETTER_PX_COUNT] = [
    false, true, false, true, false, false, true, true, true, true, true, false, false, true,
    false, true, false, false, false, true, false, true, false, false, false, true, false, true,
    false, false, true, true, true, true, true, false, false, true, false, true, false, false,
];
const ELLIPSIS: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, true, false, true,
    false, true,
];
const POUND: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, false, false, true, true, true, false, false, false, false, true, false, false,
    false, false, false, true, false, false, false, false, false, true, true, true, true, false,
];
const POWER: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, false, true, false, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];
const CURLY_L: [bool; LETTER_PX_COUNT] = [
    false, false, false, true, false, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, true, false, false, false, false, false, false, true, false,
    false, false, false, false, true, false, false, false, false, false, false, true, false, false,
];
const CURLY_R: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, false, false, false, true, false, false, false, false,
    false, true, false, false, false, false, false, false, true, false, false, false, false, true,
    false, false, false, false, false, true, false, false, false, false, true, false, false, false,
];
const BACKTICK: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, false, false, false, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];
const TILDE: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, false, true, false, false, true, false, true, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];
const DEGREE: [bool; LETTER_PX_COUNT] = [
    false, false, false, true, false, false, false, false, true, false, true, false, false, false,
    false, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false,
];

const DOLLAR: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, false, true, true, true, true, false, false, true,
    false, false, false, false, false, false, true, true, false, false, false, false, false, false,
    true, false, false, true, true, true, true, false, false, false, true, false, false, false,
];
const YEN: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, false, true, false, true, false, false, false, false,
    true, false, false, false, false, true, true, true, false, false, false, false, true, false,
    false, false, false, true, true, true, false, false, false, false, true, false, false, false,
];
const CURRENCY: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, true, false, false, false, true, false, false,
    true, true, true, false, false, false, true, false, true, false, false, false, true, true,
    true, false, false, true, false, false, false, true, false, false, false, false, false, false,
];
const CENT: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, true, false, false, false, false, true,
    true, true, false, false, false, true, false, false, false, false, false, true, true, true,
    false, false, false, false, true, false, false, false, false, false, false, false, false,
    false,
];
const CHECK: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, true, false, false, false, true, false, false, true, false,
    true, false, false, false, false, true, false, false, false, false, false, false, false, false,
    false,
];
const LOWER_A: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, true, false, false, true, false, false, true, false, false, true, false,
    false, true, false, false, false, true, true, true, false, false, false, false, false, false,
    false,
];
const LOWER_B: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, false, true, false, false, false, false, false, true,
    true, true, false, false, false, true, false, false, true, false, false, true, false, false,
    true, false, false, true, true, true, false, false, false, false, false, false, false, false,
];
const LOWER_C: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, true, false, false, true, false, false, false, false, false, true, false,
    false, false, false, false, false, true, true, true, false, false, false, false, false, false,
    false,
];
const LOWER_D: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, true, false, false, false, false, false, true, false, false, false,
    true, true, true, false, false, true, false, false, true, false, false, true, false, false,
    true, false, false, false, true, true, true, false, false, false, false, false, false, false,
];
const LOWER_E: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, true, true, true, true, false, false, true, false,
    false, false, false, false, false, true, true, true, false, false, false, false, false, false,
    false,
];
const LOWER_F: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, false, false, true, true, true, false, false, false, false, true, false, false,
    false, false, false, true, false, false, false, false, false, false, false, false, false,
    false,
];
const LOWER_G: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, true, false, true, false, false, false, false, true,
    true, false, false, false, false, false, true, false, false, false, true, true, false, false,
    false,
];
const LOWER_H: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, false, true, false, false, false, false, false, true,
    true, true, false, false, false, true, false, false, true, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, false, false, false, false,
];
const LOWER_I: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, true, false, false, false, false,
    false, false, false, false, false, false, false, true, false, false, false, false, false, true,
    false, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false,
];
const LOWER_J: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, true, false, false, false,
    false, false, false, false, false, false, false, false, true, false, false, false, false,
    false, true, false, false, false, false, false, true, false, false, false, true, true, false,
    false, false,
];
const LOWER_K: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, false, true, false, true, false, false, false, true,
    true, false, false, false, false, true, false, true, false, false, false, true, false, false,
    true, false, false, true, false, false, true, false, false, false, false, false, false, false,
];
const LOWER_L: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, false, false, true, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, true, false,
    false, false, false, false, false, true, false, false, false, false, false, false, false,
    false,
];
const LOWER_M: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, false, true, false, false, true, false, true, false, true, false, true, false,
    false, false, true, false, true, false, false, false, true, false, false, false, false, false,
    false,
];
const LOWER_N: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, true, false, false, true, false, false, true, false,
    false, true, false, false, true, false, false, true, false, false, false, false, false, false,
    false,
];
const LOWER_O: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, true, false, false, true, false, false, true, false,
    false, true, false, false, false, true, true, false, false, false, false, false, false, false,
    false,
];
const LOWER_P: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, true, false, false, false, false, true, false, true, false, false, false, true, true,
    false, false, false, false, true, false, false, false, false, false, true, false, false, false,
    false,
];
const LOWER_Q: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, true, false, true, false, false, false, false, true,
    true, false, false, false, false, false, true, false, false, false, false, false, true, true,
    false,
];
const LOWER_R: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, true, false, false, false, false, false, true, false,
    false, false, false, false, true, false, false, false, false, false, false, false, false,
    false, false,
];
const LOWER_S: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, true, true, true, false, false, false, false, false,
    false, true, false, false, true, true, true, false, false, false, false, false, false, false,
    false,
];
const LOWER_T: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, true, false, false, false, false, false, true,
    true, false, false, false, false, true, false, false, false, false, false, true, false, false,
    true, false, false, false, true, true, false, false, false, false, false, false, false, false,
];
const LOWER_U: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, true, false, false, true, false, false, true, false, false, true, false,
    false, true, false, false, false, true, true, false, false, false, false, false, false, false,
    false,
];
const LOWER_V: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, true,
    false, false, false, true, false, true, false, false, false, true, false, false, true, false,
    true, false, false, false, false, true, false, false, false, false, false, false, false, false,
    false,
];
const LOWER_W: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, true,
    false, false, false, true, false, true, false, true, false, true, false, true, false, true,
    false, true, false, false, true, false, true, false, false, false, false, false, false, false,
    false,
];
const LOWER_X: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, true, false, false, true, false, false, true, false, false, false, true,
    true, false, false, false, true, false, false, true, false, false, false, false, false, false,
    false,
];
const LOWER_Y: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, true, false, false, true, false, false, true, false, false, false, true,
    true, true, false, false, false, false, false, true, false, false, false, true, true, false,
    false,
];
const LOWER_Z: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, true, true, true, false, false, false, false, true, false, false, false, false, true,
    false, false, false, false, true, true, true, true, false, false, false, false, false, false,
    false,
];
const EURO: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, true, false, false, true, false, true, true,
    true, false, false, false, false, true, false, false, false, false, true, true, true, false,
    false, false, false, true, false, false, true, false, false, false, true, true, false, false,
];
const PIPE: [bool; LETTER_PX_COUNT] = [
    false, false, true, true, false, false, false, false, true, true, false, false, false, false,
    true, true, false, false, false, false, true, true, false, false, false, false, true, true,
    false, false, false, false, true, true, false, false, false, false, true, true, false, false,
];
