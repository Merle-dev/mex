use crossterm::event::KeyCode;

use crate::keymap::KeyOption;

pub fn to_key(key_str: String) -> Option<KeyOption> {
    match key_str.to_lowercase().as_str() {
        // Special keys
        "num" => Some(KeyOption::Num),
        "enter" => Some(KeyOption::Specific(KeyCode::Enter)),
        "backspace" => Some(KeyOption::Specific(KeyCode::Backspace)),
        "left" => Some(KeyOption::Specific(KeyCode::Left)),
        "right" => Some(KeyOption::Specific(KeyCode::Right)),
        "up" => Some(KeyOption::Specific(KeyCode::Up)),
        "down" => Some(KeyOption::Specific(KeyCode::Down)),
        "home" => Some(KeyOption::Specific(KeyCode::Home)),
        "end" => Some(KeyOption::Specific(KeyCode::End)),
        "pageup" => Some(KeyOption::Specific(KeyCode::PageUp)),
        "pagedown" => Some(KeyOption::Specific(KeyCode::PageDown)),
        "tab" => Some(KeyOption::Specific(KeyCode::Tab)),
        "backtab" => Some(KeyOption::Specific(KeyCode::BackTab)),
        "delete" => Some(KeyOption::Specific(KeyCode::Delete)),
        "insert" => Some(KeyOption::Specific(KeyCode::Insert)),
        "esc" | "escape" => Some(KeyOption::Specific(KeyCode::Esc)),
        "capslock" => Some(KeyOption::Specific(KeyCode::CapsLock)),
        "scrolllock" => Some(KeyOption::Specific(KeyCode::ScrollLock)),
        "numlock" => Some(KeyOption::Specific(KeyCode::NumLock)),
        "printscreen" => Some(KeyOption::Specific(KeyCode::PrintScreen)),
        "pause" => Some(KeyOption::Specific(KeyCode::Pause)),
        "menu" => Some(KeyOption::Specific(KeyCode::Menu)),
        "keypadbegin" => Some(KeyOption::Specific(KeyCode::KeypadBegin)),
        " " | "space" => Some(KeyOption::Specific(KeyCode::Char(' '))),

        // Function keys
        "f1" => Some(KeyOption::Specific(KeyCode::F(1))),
        "f2" => Some(KeyOption::Specific(KeyCode::F(2))),
        "f3" => Some(KeyOption::Specific(KeyCode::F(3))),
        "f4" => Some(KeyOption::Specific(KeyCode::F(4))),
        "f5" => Some(KeyOption::Specific(KeyCode::F(5))),
        "f6" => Some(KeyOption::Specific(KeyCode::F(6))),
        "f7" => Some(KeyOption::Specific(KeyCode::F(7))),
        "f8" => Some(KeyOption::Specific(KeyCode::F(8))),
        "f9" => Some(KeyOption::Specific(KeyCode::F(9))),
        "f10" => Some(KeyOption::Specific(KeyCode::F(10))),
        "f11" => Some(KeyOption::Specific(KeyCode::F(11))),
        "f12" => Some(KeyOption::Specific(KeyCode::F(12))),

        // Media keys
        "play" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::Play,
        ))),
        "pause_media" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::Pause,
        ))),
        "playpause" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::PlayPause,
        ))),
        "reverse" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::Reverse,
        ))),
        "stop" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::Stop,
        ))),
        "fastforward" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::FastForward,
        ))),
        "rewind" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::Rewind,
        ))),
        "tracknext" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::TrackNext,
        ))),
        "trackprevious" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::TrackPrevious,
        ))),
        "record" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::Record,
        ))),
        "lowervolume" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::LowerVolume,
        ))),
        "raisevolume" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::RaiseVolume,
        ))),
        "mutevolume" => Some(KeyOption::Specific(KeyCode::Media(
            crossterm::event::MediaKeyCode::MuteVolume,
        ))),

        // Modifier keys
        "leftshift" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::LeftShift,
        ))),
        "leftcontrol" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::LeftControl,
        ))),
        "leftalt" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::LeftAlt,
        ))),
        "leftsuper" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::LeftSuper,
        ))),
        "lefthyper" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::LeftHyper,
        ))),
        "leftmeta" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::LeftMeta,
        ))),
        "rightshift" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::RightShift,
        ))),
        "rightcontrol" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::RightControl,
        ))),
        "rightalt" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::RightAlt,
        ))),
        "rightsuper" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::RightSuper,
        ))),
        "righthyper" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::RightHyper,
        ))),
        "rightmeta" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::RightMeta,
        ))),
        "isoLevel3Shift" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::IsoLevel3Shift,
        ))),
        "isoLevel5Shift" => Some(KeyOption::Specific(KeyCode::Modifier(
            crossterm::event::ModifierKeyCode::IsoLevel5Shift,
        ))),
        s if s.len() == 1 => Some(KeyOption::Specific(KeyCode::Char(
            s.chars().next().unwrap(),
        ))),

        // Not recognized
        _ => None,
    }
}
