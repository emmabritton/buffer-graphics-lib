pub const CHAR_WIDTH: usize = 6;
pub const CHAR_HEIGHT: usize = 7;

pub const fn get_px_ascii(code: u8) -> &'static [bool] {
    match code {
        65 | 97 => &A,
        66 | 98 => &B,
        67 | 99 => &C,
        68 | 100=> &D,
        69 | 101=> &E,
        70 | 102=> &F,
        71 |103=> &G,
        72 |104=> &H,
        73 |105=> &I,
        74 |106=> &J,
        75 |107=> &K,
        76 |108=> &L,
        77 |109=> &M,
        78 |110=> &N,
        79 |111=> &O,
        80 |112=> &P,
        81 |113=> &Q,
        82 |114=> &R,
        83 |115=> &S,
        84 |116=> &T,
        85 |117=> &U,
        86 |118=> &V,
        87 |119=> &W,
        88 |120=> &X,
        89 |121=> &Y,
        90 |122=> &Z,
        33 => &EXCLAIM,
        34 => &DOUBLE_QUOTE,
        35 => &HASH,
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
        31 => &ELLIPSIS,
        _ => &UNKNOWN
    }
}

pub const fn get_px(chr: char) -> &'static [bool] {
    match chr.to_ascii_uppercase() {
        'A' => &A,
        'B' => &B,
        'C' => &C,
        'D' => &D,
        'E' => &E,
        'F' => &F,
        'G' => &G,
        'H' => &H,
        'I' => &I,
        'J' => &J,
        'K' => &K,
        'L' => &L,
        'M' => &M,
        'N' => &N,
        'O' => &O,
        'P' => &P,
        'Q' => &Q,
        'R' => &R,
        'S' => &S,
        'T' => &T,
        'U' => &U,
        'V' => &V,
        'W' => &W,
        'X' => &X,
        'Y' => &Y,
        'Z' => &Z,
        '.' => &PERIOD,
        '=' => &EQUALS,
        '-' => &MINUS,
        '+' => &PLUS,
        ',' => &COMMA,
        '_' => &UNDERSCORE,
        '"' => &DOUBLE_QUOTE,
        '\'' => &QUOTE,
        '!' => &EXCLAIM,
        ':' => &COLON,
        ';' => &SEMICOLON,
        '?' => &QUESTION,
        '/' => &SLASH,
        '\\' => &BACKSLASH,
        '%' => &PERCENT,
        '#' => &HASH,
        '@' => &AT,
        '(' => &PAREN_L,
        ')' => &PAREN_R,
        '[' => &SQUARE_L,
        ']' => &SQUARE_R,
        '<' => &ANGLE_L,
        '>' => &ANGLE_R,
        '{' => &CURLY_L,
        '}' => &CURLY_R,
        '~' => &TILDE,
        '^' => &POWER,
        '`' => &BACKTICK,
        '&' => &AMPERSAND,
        '*' => &ASTERISK,
        '0' => &ZERO,
        '1' => &ONE,
        '2' => &TWO,
        '3' => &THREE,
        '4' => &FOUR,
        '5' => &FIVE,
        '6' => &SIX,
        '7' => &SEVEN,
        '8' => &EIGHT,
        '9' => &NINE,
        '…' => &ELLIPSIS,
        '£' => &POUND,
        _ => &UNKNOWN,
    }
}

const LETTER_PX_COUNT: usize = 6 * 7;

const AT: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,true,false,
    true,false,false,false,false,true,
    true,false,false,false,false,true,
    true,false,true,true,false,true,
    true,false,true,true,true,true,
    true,false,false,false,false,false,
    false,true,true,true,true,true,
];
const A: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,true,true,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
];
const B: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,true,true,false,false,
];
const C: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const D: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,true,true,false,false,
];
const E: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,true,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,true,true,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,true,true,true,false,
];
const F: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,true,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,true,true,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
];
const G: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,false,false,
    false,true,false,true,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const H: [bool; LETTER_PX_COUNT] = [
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,true,true,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
];
const I: [bool; LETTER_PX_COUNT] = [
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
];
const J: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,true,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,true,false,true,false,false,
    false,false,true,false,false,false,
];
const K: [bool; LETTER_PX_COUNT] = [
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,true,false,false,
    false,true,true,false,false,false,
    false,true,false,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
];
const L: [bool; LETTER_PX_COUNT] = [
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,true,true,true,false,
];
const M: [bool; LETTER_PX_COUNT] = [
    true,false,false,false,true,false,
    true,true,false,true,true,false,
    true,false,true,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
];
const N: [bool; LETTER_PX_COUNT] = [
    true,false,false,false,false,true,
    true,true,false,false,false,true,
    true,false,true,false,false,true,
    true,false,false,true,false,true,
    true,false,false,false,true,true,
    true,false,false,false,false,true,
    true,false,false,false,false,true,
];
const O: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const P: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,true,true,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
];
const R: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,true,true,false,false,
    false,true,true,false,false,false,
    false,true,false,true,false,false,
    false,true,false,false,true,false,
];
const S: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,false,false,
    false,false,true,true,false,false,
    false,false,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const T: [bool; LETTER_PX_COUNT] = [
    true,true,true,true,true,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
];
const U: [bool; LETTER_PX_COUNT] = [
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const Q: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,false,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,true,false,true,false,
    true,false,false,true,false,false,
    false,true,true,false,true,false,
];
const W: [bool; LETTER_PX_COUNT] = [
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,true,false,true,false,
    true,false,true,false,true,false,
    false,true,false,true,false,false,
];
const V: [bool; LETTER_PX_COUNT] = [
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    true,false,false,false,true,false,
    false,true,false,true,false,false,
    false,false,true,false,false,false,
];
const X: [bool; LETTER_PX_COUNT] = [
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    false,false,true,false,false,false,
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    false,true,false,true,false,false,
];
const Y: [bool; LETTER_PX_COUNT] = [
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
];
const Z: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,true,false,
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,true,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,true,true,true,false,
];
const AMPERSAND: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,true,false,false,
    false,false,true,false,false,false,
    false,true,false,true,false,true,
    false,true,false,false,true,false,
    false,false,true,true,false,true,
];
const EXCLAIM: [bool; LETTER_PX_COUNT] = [
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,false,false,false,false,
    false,false,true,false,false,false,
];
const PERIOD: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,true,false,false,false,
];
const COMMA: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
];
const COLON: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,true,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,true,false,false,false,
    false,false,false,false,false,false,
];
const SEMICOLON: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,true,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,false,false,false,false,
];
const PLUS: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,true,false,false,
    false,false,true,true,true,false,
    false,false,false,true,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
];
const MINUS: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,true,true,true,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
];
const EQUALS: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,true,true,true,false,false,
    false,false,false,false,false,false,
    false,true,true,true,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
];
const SQUARE_L: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,true,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,true,true,false,
];
const SQUARE_R: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,true,true,true,false,false,
];
const PAREN_L: [bool; LETTER_PX_COUNT] = [
    false,false,false,true,true,false,
    false,false,true,true,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,true,false,false,
    false,false,false,true,true,false,
];
const PAREN_R: [bool; LETTER_PX_COUNT] = [
    false,true,true,false,false,false,
    false,false,true,true,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,true,true,false,false,
    false,true,true,false,false,false,
];
const ANGLE_L: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,true,false,false,false,
    false,true,false,false,false,false,
    false,false,true,false,false,false,
    false,false,false,true,false,false,
    false,false,false,false,true,false,
];
const ANGLE_R: [bool; LETTER_PX_COUNT] = [
    false,true,false,false,false,false,
    false,false,true,false,false,false,
    false,false,false,true,false,false,
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,true,false,false,false,
    false,true,false,false,false,false,
];
const DOUBLE_QUOTE: [bool; LETTER_PX_COUNT] = [
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
];
const QUOTE: [bool; LETTER_PX_COUNT] = [
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
];
const QUESTION: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,false,false,false,true,false,
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,false,false,false,false,
    false,false,false,true,false,false,
];
const SLASH: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,true,
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,true,false,false,false,
    false,true,false,false,false,false,
    true,false,false,false,false,false,
];
const BACKSLASH: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    true,false,false,false,false,false,
    false,true,false,false,false,false,
    false,false,true,false,false,false,
    false,false,false,true,false,false,
    false,false,false,false,true,false,
    false,false,false,false,false,true,
];
const ASTERISK: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    true,false,true,false,true,false,
    false,true,true,true,false,false,
    true,true,true,true,true,false,
    false,true,true,true,false,false,
    true,false,true,false,true,false,
    false,false,false,false,false,false,
];
const PERCENT: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    true,false,false,false,false,true,
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,true,false,false,false,
    false,true,false,false,false,false,
    true,false,false,false,false,true,
];
const ZERO: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const ONE: [bool; LETTER_PX_COUNT] = [
    false,false,true,false,false,false,
    false,true,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,true,true,true,false,false,
];
const TWO: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,true,false,false,false,
    false,true,false,false,false,false,
    false,true,true,true,true,false,
];
const THREE: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,false,false,false,true,false,
    false,false,true,true,false,false,
    false,false,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const FOUR: [bool; LETTER_PX_COUNT] = [
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    false,true,true,true,true,false,
    false,false,false,true,false,false,
];
const FIVE: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,true,true,false,false,
    false,false,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const SIX: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,false,false,
    false,true,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const SEVEN: [bool; LETTER_PX_COUNT] = [
    false,true,true,true,true,false,
    false,false,false,false,true,false,
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
];
const EIGHT: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const NINE: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,true,false,
    false,false,false,false,true,false,
    false,true,false,false,true,false,
    false,false,true,true,false,false,
];
const UNDERSCORE: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    true,true,true,true,true,false,
];
const UNKNOWN: [bool; LETTER_PX_COUNT] = [
    true,true,true,true,true,true,
    true,false,false,false,false,true,
    true,false,false,false,false,true,
    true,false,false,false,false,true,
    true,false,false,false,false,true,
    true,false,false,false,false,true,
    true,true,true,true,true,true,
];
const HASH: [bool; LETTER_PX_COUNT] = [
    false,true,false,true,false,false,
    true,true,true,true,true,false,
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    false,true,false,true,false,false,
    true,true,true,true,true,false,
    false,true,false,true,false,false,
];
const ELLIPSIS: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,true,false,true,false,true,
];
const POUND: [bool; LETTER_PX_COUNT] = [
    false,false,true,true,false,false,
    false,true,false,false,true,false,
    false,true,false,false,false,false,
    true,true,true,false,false,false,
    false,true,false,false,false,false,
    false,true,false,false,false,false,
    false,true,true,true,true,false,
];
const POWER: [bool; LETTER_PX_COUNT] = [
    false,false,true,false,false,false,
    false,true,false,true,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
];
const CURLY_L: [bool; LETTER_PX_COUNT] = [
    false,false,false,true,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,true,false,false,false,false,
    false,false,true,false,false,false,
    false,false,true,false,false,false,
    false,false,false,true,false,false,
];
const CURLY_R: [bool; LETTER_PX_COUNT] = [
    false,false,true,false,false,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,false,false,true,false,
    false,false,false,true,false,false,
    false,false,false,true,false,false,
    false,false,true,false,false,false,
];
const BACKTICK: [bool; LETTER_PX_COUNT] = [
    false,false,true,false,false,false,
    false,false,false,true,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
];
const TILDE: [bool; LETTER_PX_COUNT] = [
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,true,false,true,false,
    false,true,false,true,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
    false,false,false,false,false,false,
];