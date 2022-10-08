use web_sys::MouseEvent;
// use gloo_console as console;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MouseButton {
    Left,
    Middle,
    Right,
    Other,
}

trait MouseButtoni16 {
    fn to_mouse_button(self) -> MouseButton;
}

impl MouseButtoni16 for i16 {
    fn to_mouse_button(self) -> MouseButton {
        match self {
            0 => { MouseButton::Left },
            1 => { MouseButton::Middle },
            2 => { MouseButton::Right },
            _ => { MouseButton::Other },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MouseState {
    Neither,
    Left,
    Right,
    Both,
}

impl MouseState {
    pub fn is_neither(self) -> bool {
        self == Self::Neither
    }

    pub fn is_left(self) -> bool {
        self == Self::Left
    }

    pub fn is_right(self) -> bool {
        self == Self::Right
    }

    pub fn is_both(self) -> bool {
        self == Self::Both
    }

    pub fn is_some(self) -> bool {
        !self.is_neither()
    }

    pub fn mouse_down(&self, event: MouseEvent) -> Self {
        let button = event.button().to_mouse_button();
        match self {
            Self::Neither => {
                match button {
                    MouseButton::Left => { Self::Left },
                    MouseButton::Right => { Self::Right },
                    _ => { Self::Neither },
                }
            },
            Self::Left => {
                match button {
                    MouseButton::Right => { Self::Both },
                    _ => { Self::Left },
                }
            },
            Self::Right => {
                match button {
                    MouseButton::Left => { Self::Both },
                    _ => { Self::Right },
                }
            },
            Self::Both => { Self::Both },
        }
    }

    pub fn mouse_up(&self, event: MouseEvent) -> Self {
        let button = event.button().to_mouse_button();
        match self {
            Self::Both => {
                match button {
                    MouseButton::Left => { Self::Right },
                    MouseButton::Right => { Self::Left },
                    _ => { Self::Both },
                }
            },
            Self::Left => {
                match button {
                    MouseButton::Left => { Self::Neither },
                    _ => { Self::Left },
                }
            },
            Self::Right => {
                match button {
                    MouseButton::Right => { Self::Neither },
                    _ => { Self::Right },
                }
            },
            Self::Neither => { Self::Neither },
        }
    }
}