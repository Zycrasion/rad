use glfw::Action;

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
    pub fn from_glfw_key(key : glfw::Key) -> Key
    {
        match key
        {
            glfw::Key::Space => Key::Space,
            glfw::Key::Apostrophe => Key::Apostrophe,
            glfw::Key::Comma => Key::Comma,
            glfw::Key::Minus => Key::Minus,
            glfw::Key::Period => Key::Period,
            glfw::Key::Slash => Key::Slash,
            glfw::Key::Num0 => Key::Numpad(0),
            glfw::Key::Num1 => Key::Numpad(1),
            glfw::Key::Num2 => Key::Numpad(2),
            glfw::Key::Num3 => Key::Numpad(3),
            glfw::Key::Num4 => Key::Numpad(4),
            glfw::Key::Num5 => Key::Numpad(5),
            glfw::Key::Num6 => Key::Numpad(6),
            glfw::Key::Num7 => Key::Numpad(7),
            glfw::Key::Num8 => Key::Numpad(8),
            glfw::Key::Num9 => Key::Numpad(9),
            glfw::Key::Semicolon => Key::Semicolon,
            glfw::Key::Equal => Key::Equal,
            glfw::Key::A => Key::Key('A'),
            glfw::Key::B => Key::Key('B'),
            glfw::Key::C => Key::Key('C'),
            glfw::Key::D => Key::Key('D'),
            glfw::Key::E => Key::Key('E'),
            glfw::Key::F => Key::Key('F'),
            glfw::Key::G => Key::Key('G'),
            glfw::Key::H => Key::Key('H'),
            glfw::Key::I => Key::Key('I'),
            glfw::Key::J => Key::Key('J'),
            glfw::Key::K => Key::Key('K'),
            glfw::Key::L => Key::Key('L'),
            glfw::Key::M => Key::Key('M'),
            glfw::Key::N => Key::Key('N'),
            glfw::Key::O => Key::Key('O'),
            glfw::Key::P => Key::Key('P'),
            glfw::Key::Q => Key::Key('Q'),
            glfw::Key::R => Key::Key('R'),
            glfw::Key::S => Key::Key('S'),
            glfw::Key::T => Key::Key('T'),
            glfw::Key::U => Key::Key('U'),
            glfw::Key::V => Key::Key('V'),
            glfw::Key::W => Key::Key('W'),
            glfw::Key::X => Key::Key('X'),
            glfw::Key::Y => Key::Key('Y'),
            glfw::Key::Z => Key::Key('Z'),
            glfw::Key::LeftBracket => Key::LeftBracket,
            glfw::Key::RightBracket => Key::RightBracket,
            glfw::Key::Backslash => Key::Backslash,
            glfw::Key::Escape => Key::Escape,
            glfw::Key::Enter => Key::Enter,
            glfw::Key::Tab => Key::Tab,
            glfw::Key::Backspace => Key::Backspace,
            glfw::Key::Right => Key::Arrow(ArrowDirection::Right),
            glfw::Key::Left => Key::Arrow(ArrowDirection::Left),
            glfw::Key::Down => Key::Arrow(ArrowDirection::Down),
            glfw::Key::Up => Key::Arrow(ArrowDirection::Up),
            glfw::Key::CapsLock => Key::CapsLock,
            glfw::Key::NumLock => Key::NumLock,
            glfw::Key::F1 => Key::Function(1),
            glfw::Key::F2 => Key::Function(2),
            glfw::Key::F3 => Key::Function(3),
            glfw::Key::F4 => Key::Function(4),
            glfw::Key::F5 => Key::Function(5),
            glfw::Key::F6 => Key::Function(6),
            glfw::Key::F7 => Key::Function(7),
            glfw::Key::F8 => Key::Function(8),
            glfw::Key::F9 => Key::Function(9),
            glfw::Key::F10 => Key::Function(10),
            glfw::Key::F11 => Key::Function(11),
            glfw::Key::F12 => Key::Function(12),
            glfw::Key::F13 => Key::Function(13),
            glfw::Key::F14 => Key::Function(14),
            glfw::Key::F15 => Key::Function(15),
            glfw::Key::F16 => Key::Function(16),
            glfw::Key::F17 => Key::Function(17),
            glfw::Key::F18 => Key::Function(18),
            glfw::Key::F19 => Key::Function(19),
            glfw::Key::F20 => Key::Function(20),
            glfw::Key::F21 => Key::Function(21),
            glfw::Key::F22 => Key::Function(22),
            glfw::Key::F23 => Key::Function(23),
            glfw::Key::F24 => Key::Function(24),
            glfw::Key::F25 => Key::Function(25),
            glfw::Key::LeftShift => todo!(),
            glfw::Key::LeftControl => todo!(),
            glfw::Key::LeftAlt => todo!(),
            glfw::Key::RightShift => todo!(),
            glfw::Key::RightControl => todo!(),
            glfw::Key::RightAlt => todo!(),
            glfw::Key::Unknown | _ => Key::Unknown,
        }
    }
}

impl InputAction
{
    pub fn from_glfw_action(action : glfw::Action) -> Self
    {
        match action
        {
            Action::Release => InputAction::Release,
            Action::Press => InputAction::Press,
            Action::Repeat => InputAction::Repeat,
        }
    }
}