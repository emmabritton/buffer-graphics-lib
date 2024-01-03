use crate::text::{
    ASCII_CENT, ASCII_CHECK, ASCII_CURRENCY, ASCII_DEGREE, ASCII_ELLIPSIS, ASCII_POUND, ASCII_YEN,
};

pub const CHAR_WIDTH: usize = 4;
pub const CHAR_HEIGHT: usize = 5;

pub const fn get_px_ascii(code: u8) -> &'static [bool] {
    match code {
        65 | 97 => &A,
        66 | 98 => &B,
        67 | 99 => &C,
        68 | 100 => &D,
        69 | 101 => &E,
        70 | 102 => &F,
        71 | 103 => &G,
        72 | 104 => &H,
        73 | 105 => &I,
        74 | 106 => &J,
        75 | 107 => &K,
        76 | 108 => &L,
        77 | 109 => &M,
        78 | 110 => &N,
        79 | 111 => &O,
        80 | 112 => &P,
        81 | 113 => &Q,
        82 | 114 => &R,
        83 | 115 => &S,
        84 | 116 => &T,
        85 | 117 => &U,
        86 | 118 => &V,
        87 | 119 => &W,
        88 | 120 => &X,
        89 | 121 => &Y,
        90 | 122 => &Z,
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
        91 => &SQUARE_L,
        92 => &BACKSLASH,
        93 => &SQUARE_R,
        95 => &UNDERSCORE,
        94 => &POWER,
        96 => &BACKTICK,
        123 => &CURLY_L,
        125 => &CURLY_R,
        126 => &TILDE,
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
    true, true, true, true, true, false, false, true, true, false, true, true, true, false, false,
    false, true, true, true, true,
];
const A: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, true, true, true, true, true, false, false,
    true, true, false, false, true,
];
const B: [bool; LETTER_PX_COUNT] = [
    true, true, true, false, true, false, false, true, true, true, true, false, true, false, false,
    true, true, true, true, false,
];
const C: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, true, false, false, false, true, false,
    false, true, false, true, true, false,
];
const D: [bool; LETTER_PX_COUNT] = [
    true, true, true, false, true, false, false, true, true, false, false, true, true, false,
    false, true, true, true, true, false,
];
const E: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, true, false, false, false, true, true, true, false, true, false, false,
    false, true, true, true, true,
];
const F: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, true, false, false, false, true, true, true, false, true, false, false,
    false, true, false, false, false,
];
const G: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, false, true, false, true, true, true, false,
    false, true, false, true, true, false,
];
const H: [bool; LETTER_PX_COUNT] = [
    true, false, false, true, true, false, false, true, true, true, true, true, true, false, false,
    true, true, false, false, true,
];
const I: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, true, false, false, false, true, false, false, false,
    true, false, false, false, true, false,
];
const J: [bool; LETTER_PX_COUNT] = [
    false, false, false, true, false, false, false, true, false, false, false, true, true, false,
    false, true, false, true, true, false,
];
const K: [bool; LETTER_PX_COUNT] = [
    true, false, false, true, true, false, true, false, true, true, false, false, true, false,
    true, false, true, false, false, true,
];
const L: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, false, false, true, false, false, false, true, false,
    false, false, true, true, true, true,
];
const M: [bool; LETTER_PX_COUNT] = [
    true, false, false, true, true, true, true, true, true, false, false, true, true, false, false,
    true, true, false, false, true,
];
const N: [bool; LETTER_PX_COUNT] = [
    true, false, false, true, true, true, false, true, true, false, true, true, true, false, false,
    true, true, false, false, true,
];
const O: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, true, false, false, true, true, false,
    false, true, false, true, true, false,
];
const P: [bool; LETTER_PX_COUNT] = [
    true, true, true, false, true, false, false, true, true, true, true, false, true, false, false,
    false, true, false, false, false,
];
const R: [bool; LETTER_PX_COUNT] = [
    true, true, true, false, true, false, false, true, true, true, true, false, true, false, true,
    false, true, false, false, true,
];
const S: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, true, false, false, false, false, true, true, false, false, false,
    false, true, true, true, true, false,
];
const T: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, false, false, true, false, false, false, true, false, false, false,
    true, false, false, false, true, false,
];
const U: [bool; LETTER_PX_COUNT] = [
    true, false, false, true, true, false, false, true, true, false, false, true, true, false,
    false, true, false, true, true, false,
];
const Q: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, true, false, false, true, true, false,
    true, true, false, true, true, true,
];
const W: [bool; LETTER_PX_COUNT] = [
    true, false, false, true, true, false, false, true, true, false, false, true, true, true, true,
    true, true, false, false, true,
];
const V: [bool; LETTER_PX_COUNT] = [
    true, false, true, false, true, false, true, false, true, false, true, false, true, false,
    true, false, false, true, false, false,
];
const X: [bool; LETTER_PX_COUNT] = [
    true, false, true, false, true, false, true, false, false, true, false, false, true, false,
    true, false, true, false, true, false,
];
const Y: [bool; LETTER_PX_COUNT] = [
    true, false, true, false, true, false, true, false, false, true, false, false, false, true,
    false, false, false, true, false, false,
];
const Z: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, false, false, false, true, false, true, true, false, true, false,
    false, false, true, true, true, true,
];
const AMPERSAND: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, false, false, true, false, true, true, false,
    true, false, false, true, false, true,
];
const EXCLAIM: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, false, true, false, false, false, true, false, false, false,
    false, false, false, false, true, false,
];
const PERIOD: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, false, false,
];
const COMMA: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, false, true, false, false,
];
const COLON: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, true, false, false, false, false, false, false, false, true,
    false, false, false, false, false, false,
];
const SEMICOLON: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, true, false, false, false, false, false, false, false, true,
    false, false, false, true, false, false,
];
const PLUS: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, true, false, false, true, true, true, false, false, true,
    false, false, false, false, false, false,
];
const MINUS: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, true, true, true, false, false, false,
    false, false, false, false, false, false,
];
const EQUALS: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, true, true, true, false, false, false, false, false, true, true,
    true, false, false, false, false, false,
];
const SQUARE_L: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, false, true, false, false, false, true, false, false, false, true,
    false, false, false, true, true, false,
];
const SQUARE_R: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, false, false, true, false, false, false, true, false, false, false,
    true, false, false, true, true, false,
];
const PAREN_L: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, true, false, false, false, true, false, false, false, true,
    false, false, false, false, true, false,
];
const PAREN_R: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, true, false, false, false, true, false, false, false,
    true, false, false, true, false, false,
];
const ANGLE_L: [bool; LETTER_PX_COUNT] = [
    false, false, false, true, false, false, true, false, false, true, false, false, false, false,
    true, false, false, false, false, true,
];
const ANGLE_R: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, false, true, false, false, false, false, true, false, false, true,
    false, false, true, false, false, false,
];
const DOUBLE_QUOTE: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, true, false, true, false, true, false, true, false, false,
    false, false, false, false, false, false,
];
const QUOTE: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, true, false, false, false, true, false, false, false, false,
    false, false, false, false, false, false,
];
const QUESTION: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, false, false, false, true, false, false,
    true, false, false, true, false, false,
];
const SLASH: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, true, false, false, true, false, false, true,
    false, false, true, false, false, false,
];
const BACKSLASH: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, true, false, false, false, false, true, false, false, false, false,
    true, false, false, false, false, true,
];
const ASTERISK: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, true, false, true, false, false, true, false, false, true,
    false, true, false, false, false, false,
];
const PERCENT: [bool; LETTER_PX_COUNT] = [
    true, false, false, true, false, false, true, false, false, true, false, false, true, false,
    false, true, false, false, false, false,
];
const ZERO: [bool; LETTER_PX_COUNT] = [
    false, false, true, false,
    false, true, false, true,
    false, true, false, true,
    false, true, false, true,
    false, false, true, false,
];
const ONE: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, true, true, false, false, false, true, false, false, false,
    true, false, false, true, true, true,
];
const TWO: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, false, false, true, false, false, true,
    false, false, true, true, true, true,
];
const THREE: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, false, false, false, true, false, true, true, false, false, false,
    false, true, false, true, true, false,
];
const FOUR: [bool; LETTER_PX_COUNT] = [
    true, false, false, false, true, false, false, false, true, false, true, false, true, true,
    true, true, false, false, true, false,
];
const FIVE: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, true, false, false, false, true, true, true, false, false, false,
    false, true, true, true, true, false,
];
const SIX: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, false, true, true, true, false, true, false,
    false, true, false, true, true, false,
];
const SEVEN: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, false, false, false, true, false, false, true, false, false, true,
    false, false, false, true, false, false,
];
const EIGHT: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, false, true, true, false, true, false,
    false, true, false, true, true, false,
];
const NINE: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, false, true, true, true, false, false,
    false, true, false, true, true, false,
];
const UNDERSCORE: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, true, true, true,
];
const UNKNOWN: [bool; LETTER_PX_COUNT] = [
    true, true, true, true, true, false, false, true, true, false, false, true, true, false, false,
    true, true, true, true, true,
];
const HASH: [bool; LETTER_PX_COUNT] = [
    false, true, false, true, true, true, true, true, false, true, false, true, true, true, true,
    true, false, true, false, true,
];
const ELLIPSIS: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, false, true,
];
const POUND: [bool; LETTER_PX_COUNT] = [
    false, true, true, false, true, false, false, true, true, true, false, false, true, false,
    false, false, true, true, true, true,
];
const POWER: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, true, false, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false,
];
const CURLY_L: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, true, false, false, true, true, false, false, false, true,
    false, false, false, false, true, false,
];
const CURLY_R: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, true, false, false, false, true, true, false, false,
    true, false, false, true, false, false,
];
const BACKTICK: [bool; LETTER_PX_COUNT] = [
    false, true, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false,
];
const TILDE: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, true, false, true, true, false, true, false, false, false,
    false, false, false, false, false, false,
];
const DEGREE: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, true, false, true, false, false, true, false, false, false,
    false, false, false, false, false, false,
];
const YEN: [bool; LETTER_PX_COUNT] = [
    false, true, false, true, false, true, false, true, false, false, true, false, false, true,
    true, true, false, false, true, false,
];
const CURRENCY: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, true, false, false, true, false, true, true, false, false, true,
    true, false, true, false, false, true,
];
const DOLLAR: [bool; LETTER_PX_COUNT] = [
    false, true, true, true, true, false, true, false, false, true, true, false, false, true,
    false, true, true, true, true, false,
];
const CENT: [bool; LETTER_PX_COUNT] = [
    false, false, true, false, false, true, true, true, false, true, false, false, false, true,
    true, true, false, false, true, false,
];
const CHECK: [bool; LETTER_PX_COUNT] = [
    false, false, false, false, false, false, false, true, true, false, true, false, false, true,
    false, false, false, false, false, false,
];
