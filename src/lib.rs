use serde::{Deserialize, Serialize};
//use bitflags::bitflags;

#[cfg(unix)]
use enigo::{Enigo, MouseButton as EnigoMouseButton, 
    MouseControllable, Key, KeyboardControllable};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
// bitflags! {
//     #[derive(Serialize, Deserialize)]
//     pub struct Modifier: u32 {
//         const NONE = 0b00000000;
//         const CONTROL = 0b00000001;
//         const SHIFT = 0b00000010;
//         const ALT = 0b00000100;
//     }
// }
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventAction {
    KeyPress {
        key: String, 
    },
    Click { 
        x: i32, 
        y: i32,
        button: MouseButton,
    },
    Scroll {
        value: i32, // negative up, positive down
    },
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    pub action : EventAction,
    pub modifiers: u32,
}
impl Event {
    pub fn as_bytes(&mut self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    pub fn from_bytes(buffer: &[u8]) -> Self {
        bincode::deserialize(buffer).unwrap()
    }
}


#[cfg(unix)]
pub struct Input {
    enigo: Option<Enigo>,
}

#[cfg(unix)]
#[allow(dead_code)]
impl Input {
    pub fn new() -> Self {
        Self { 
            enigo: Some(Enigo::new()),
        }
    }
    pub fn focus(&mut self) {
        self.enigo.as_mut().unwrap().window_focus();
    }
    pub fn set_window(&mut self, window: i32) {
        self.enigo.as_mut().unwrap().set_window(window);
    }
    pub fn get_window_pid(&mut self) -> i32 {
        self.enigo.as_mut().unwrap().window_pid()
    }
    pub fn search_window_by_pid(&mut self, pid: i32) -> i32 {
        self.enigo.as_mut().unwrap().search_window_by_pid(pid)
    }
    pub fn mouse_click(&mut self, x: i32, y:i32, button: MouseButton, modifiers: u32) {
        let button = match button {
            MouseButton::Left => EnigoMouseButton::Left,
            MouseButton::Right => EnigoMouseButton::Right,
            MouseButton::Middle => EnigoMouseButton::Middle,
            _ => todo!()
        };
        self.enigo.as_mut().unwrap().mouse_move_to(x, y);
        self.enigo.as_mut().unwrap().mouse_click(button);
    }
    pub fn key_press(&mut self, key: &str, modifiers: u32) {
        println!(" key to match: {:?}", key);
        let k = match key {
            "Return" => Key::Return,
            "BackSpace" => Key::Backspace,
            "Delete" => Key::Delete,
            "Page_Up" => Key::PageUp,
            "Page_Down" => Key::PageDown,
            "Up" => Key::UpArrow,
            "Down" => Key::DownArrow,
            "Left" => Key::LeftArrow,
            "Right" => Key::RightArrow,
            "End" => Key::End,
            "Home" => Key::Home,
            "Tab" => Key::Tab,
            "Escape" => Key::Escape,
            c => Key::Layout(c.chars().next().unwrap()),
        };
        println!(" key detected: {:?}", k);
        self.enigo.as_mut().unwrap().key_click(k);

    }
}
