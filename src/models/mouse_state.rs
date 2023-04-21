use web_sys::MouseEvent;
use crate::models::settings::ChordSetting;
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
    AfterBoth,
}

impl MouseState {
    pub fn is_neither(self) -> bool {
        self == Self::Neither
    }

    // pub fn is_both(self) -> bool {
    //     self == Self::Both
    // }

    pub fn is_chording(self, chord_setting: ChordSetting, cell_is_shown: bool) -> bool{
        match self {
            Self::Left => { cell_is_shown && chord_setting == ChordSetting::LeftClick },
            Self::Both => { true },
            _ => { false },
        }
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
            Self::Both | Self::AfterBoth => { Self::Both },
        }
    }

    pub fn mouse_up(&self, event: MouseEvent) -> Self {
        let button = event.button().to_mouse_button();
        match self {
            Self::Both => {
                match button {
                    MouseButton::Left | MouseButton::Right => { Self::AfterBoth },
                    _ => { Self::Both },
                }
            },
            Self::AfterBoth => { Self::Neither },
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