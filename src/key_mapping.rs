use crate::{key_codes::KeyCode, NUM_COLS, NUM_ROWS};

#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty,     KeyCode::Empty,      KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Empty, KeyCode::L,     KeyCode::S,     KeyCode::Z,         KeyCode::Backspace,  KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::R,     KeyCode::N,     KeyCode::V,     KeyCode::Right,     KeyCode::RightShift, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::C,     KeyCode::T,     KeyCode::W,     KeyCode::Up,        KeyCode::Down,       KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::G,     KeyCode::H,     KeyCode::M,     KeyCode::Left,      KeyCode::Empty,      KeyCode::Tab,   KeyCode::RightCmd, KeyCode::Enter],
    [KeyCode::F,     KeyCode::D,     KeyCode::B,     KeyCode::Backspace, KeyCode::Empty,      KeyCode::Tab,   KeyCode::Empty, KeyCode::Empty],
];


#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
];

