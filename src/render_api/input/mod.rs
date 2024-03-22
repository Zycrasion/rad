pub enum MouseButton {
    Middle,
    Left,
    Right,
}

pub enum InputAction {
    Press,
    Release,
    Repeat,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ArrowDirection
{
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Numpad(u8),
    Semicolon,
    Equal,
    Key(char),
    LeftBracket,
    RightBracket,
    Backslash,
    Escape,
    Enter,
    Tab,
    Backspace,
    Arrow(ArrowDirection),
    Function(u8),
    CapsLock,
    NumLock,
    LeftShift,
    LeftControl,
    LeftAlt,
    RightShift,
    RightControl,
    RightAlt,
    Unknown
}

impl Key
{
}

impl InputAction
{

}