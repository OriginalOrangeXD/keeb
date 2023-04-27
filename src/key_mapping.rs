use crate::{key_codes::KeyCode, NUM_COLS, NUM_ROWS};

#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::Empty, KeyCode::Tilde, KeyCode::Empty, KeyCode::Empty,     KeyCode::Empty,     KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Empty, KeyCode::SingleQuote,     KeyCode::A,  KeyCode::Semicolon, KeyCode::Empty,     KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Comma, KeyCode::O,     KeyCode::Q,            KeyCode::Left,      KeyCode::Empty,     KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Period,KeyCode::E,     KeyCode::J,            KeyCode::Up,        KeyCode::Down,      KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::P,     KeyCode::U,     KeyCode::K,       KeyCode::Right,          KeyCode::Escape,        KeyCode::RightCmd, KeyCode::RightCtrl, KeyCode::Space],
    [KeyCode::Y,     KeyCode::I,     KeyCode::X,        KeyCode::Tab,           KeyCode::Fn,KeyCode::RightCtrl, KeyCode::Empty, KeyCode::Empty],// Fix rightctrl
];


#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::Num1, KeyCode::Num4, KeyCode::Num1, KeyCode::Num1, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::Num7, KeyCode::Num4, KeyCode::Num1, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
    [KeyCode::Num8, KeyCode::Num5, KeyCode::Num2, KeyCode::Num0, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Fn, KeyCode::Backspace],
    [KeyCode::Num9, KeyCode::Num6, KeyCode::Num3, KeyCode::R, KeyCode::Backspace, KeyCode::RightCmd, KeyCode::Fn, KeyCode::Backspace],
    [KeyCode::F, KeyCode::G, KeyCode::C, KeyCode::R, KeyCode::Fn, KeyCode::RightCmd, KeyCode::Backspace, KeyCode::Backspace],
];

