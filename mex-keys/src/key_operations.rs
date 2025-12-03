use ratatui::crossterm::event::{KeyCode, MediaKeyCode, ModifierKeyCode};

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

        // Media keys
        "play" => Some(KeyOption::Specific(KeyCode::Media(MediaKeyCode::Play))),
        "pause_media" => Some(KeyOption::Specific(KeyCode::Media(MediaKeyCode::Pause))),
        "playpause" => Some(KeyOption::Specific(KeyCode::Media(MediaKeyCode::PlayPause))),
        "reverse" => Some(KeyOption::Specific(KeyCode::Media(MediaKeyCode::Reverse))),
        "stop" => Some(KeyOption::Specific(KeyCode::Media(MediaKeyCode::Stop))),
        "fastforward" => Some(KeyOption::Specific(KeyCode::Media(
            MediaKeyCode::FastForward,
        ))),
        "rewind" => Some(KeyOption::Specific(KeyCode::Media(MediaKeyCode::Rewind))),
        "tracknext" => Some(KeyOption::Specific(KeyCode::Media(MediaKeyCode::TrackNext))),
        "trackprevious" => Some(KeyOption::Specific(KeyCode::Media(
            MediaKeyCode::TrackPrevious,
        ))),
        "record" => Some(KeyOption::Specific(KeyCode::Media(MediaKeyCode::Record))),
        "lowervolume" => Some(KeyOption::Specific(KeyCode::Media(
            MediaKeyCode::LowerVolume,
        ))),
        "raisevolume" => Some(KeyOption::Specific(KeyCode::Media(
            MediaKeyCode::RaiseVolume,
        ))),
        "mutevolume" => Some(KeyOption::Specific(KeyCode::Media(
            MediaKeyCode::MuteVolume,
        ))),

        // Modifier keys
        "leftshift" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::LeftShift,
        ))),
        "leftcontrol" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::LeftControl,
        ))),
        "leftalt" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::LeftAlt,
        ))),
        "leftsuper" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::LeftSuper,
        ))),
        "lefthyper" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::LeftHyper,
        ))),
        "leftmeta" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::LeftMeta,
        ))),
        "rightshift" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::RightShift,
        ))),
        "rightcontrol" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::RightControl,
        ))),
        "rightalt" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::RightAlt,
        ))),
        "rightsuper" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::RightSuper,
        ))),
        "righthyper" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::RightHyper,
        ))),
        "rightmeta" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::RightMeta,
        ))),
        "isoLevel3Shift" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::IsoLevel3Shift,
        ))),
        "isoLevel5Shift" => Some(KeyOption::Specific(KeyCode::Modifier(
            ModifierKeyCode::IsoLevel5Shift,
        ))),
        f if f.starts_with('f') && f[1..].parse::<u8>().is_ok() => Some(KeyOption::Specific(
            KeyCode::F(f[1..].parse::<u8>().unwrap()),
        )),
        s if s.len() == 1 => Some(KeyOption::Specific(KeyCode::Char(
            s.chars().next().unwrap(),
        ))),

        // Not recognized
        _ => None,
    }
}
