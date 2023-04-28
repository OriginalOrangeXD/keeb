use crate::{key_codes::KeyCode, NUM_COLS, NUM_ROWS};

#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::Exclaim, KeyCode::ForwardSlash, KeyCode::Minus, KeyCode::Empty,     KeyCode::Num1,      KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Empty, KeyCode::L,     KeyCode::S,     KeyCode::Z,         KeyCode::Num2,  KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::R,     KeyCode::N,     KeyCode::V,     KeyCode::Empty,     KeyCode::Num3,KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::C,     KeyCode::T,     KeyCode::W,     KeyCode::Empty,        KeyCode::Num4,       KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::G,     KeyCode::H,     KeyCode::M,     KeyCode::Empty,      KeyCode::Backspace,      KeyCode::Fn,   KeyCode::RightCtrl, KeyCode::Enter],
    [KeyCode::F,     KeyCode::D,     KeyCode::B,     KeyCode::RightShift, KeyCode::Fn,      KeyCode::Empty,   KeyCode::Empty, KeyCode::Empty],
];
pub const NORMAL_LAYER_MAPPING_arrow: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::Exclaim, KeyCode::ForwardSlash, KeyCode::Minus, KeyCode::Empty,     KeyCode::Empty,      KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Empty, KeyCode::L,     KeyCode::S,     KeyCode::Z,         KeyCode::Escape,  KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::R,     KeyCode::N,     KeyCode::V,     KeyCode::Right,     KeyCode::RightShift,KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::C,     KeyCode::T,     KeyCode::W,     KeyCode::Up,        KeyCode::Down,       KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::G,     KeyCode::H,     KeyCode::M,     KeyCode::Left,      KeyCode::Empty,      KeyCode::Fn,   KeyCode::LeftCtrl, KeyCode::Enter],
    [KeyCode::F,     KeyCode::D,     KeyCode::B,     KeyCode::Backspace, KeyCode::Empty,      KeyCode::Empty,   KeyCode::Empty, KeyCode::Empty],
];


#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::RightParen, KeyCode::RightSquareBracket, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::LeftParen, KeyCode::LeftSquareBracket, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::Fn, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::Equals, KeyCode::Empty, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
];

