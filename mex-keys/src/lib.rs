use std::{collections::HashMap, sync::mpsc::Sender, thread};

use anyhow::{Result, anyhow};
use mex_core::Mode;
use ratatui::crossterm::event::{self, KeyCode};

use crate::keymap::{KeyBranch, KeyOption, get_unsafe};
pub use keymap::KeyMap;

mod key_operations;
mod keymap;

fn run<'b, 'a: 'b>(
    keytree: &'a KeyMap,
    keytree_ref: &'b HashMap<KeyOption, KeyBranch>,
    mode: &mut Mode,
) -> Result<&'a HashMap<KeyOption, KeyBranch>> {
    Ok(match event::read()? {
        event::Event::Key(key) => compute_key(sender, keytree, keytree_ref, key.code, mode)?,
        _ => keytree_ref,
    })
}

fn compute_key<'a>(
    keytree: &'a KeyMap,
    keytree_ref: &'a HashMap<KeyOption, KeyBranch>,
    keycode: KeyCode,
    mode: &Mode,
) -> Result<&'a HashMap<KeyOption, KeyBranch>> {
    let t = match keytree_ref.get(&KeyOption::Specific(keycode)) {
        Some(KeyBranch::Command(cmd)) => keytree
            .0
            .get(mode)
            .ok_or(anyhow!("No Such Mode"))
            .map(get_unsafe),
        Some(KeyBranch::Branches(branch)) => Ok(branch),
        None => keytree
            .0
            .get(mode)
            .map(get_unsafe)
            .ok_or(anyhow!("No Such Mode")),
    };
    t
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ratatui::crossterm::event::KeyCode;

    use crate::keymap::{KeyBranch, KeyMap, KeyOption, add_kb};

    #[test]
    fn from_config() {
        assert!(KeyMap::from_config("../config.toml").is_ok());
    }

    #[test]
    fn seperate() {
        let vopts = KeyMap::seperate("{space}p".into());
        assert_eq!(
            vopts,
            vec![
                KeyOption::Specific(KeyCode::Char(' ')),
                KeyOption::Specific(KeyCode::Char('p'))
            ]
        );
        assert_eq!(
            KeyMap::seperate("{space}{num}p".into()),
            vec![
                KeyOption::Specific(KeyCode::Char(' ')),
                KeyOption::Num,
                KeyOption::Specific(KeyCode::Char('p'))
            ]
        );
    }
    #[test]
    fn tree() {
        let path = vec![
            KeyOption::Specific(KeyCode::Char(' ')),
            KeyOption::Num,
            KeyOption::Specific(KeyCode::Char('p')),
        ];
        let cmd = "print".to_string();
        let mut km = KeyBranch::Branches(HashMap::new());
        km = add_kb(km, &path, cmd.clone());
        assert_eq!(
            km,
            KeyBranch::Branches(HashMap::from([(
                path[0].clone(),
                KeyBranch::Branches(HashMap::from([(
                    path[1].clone(),
                    KeyBranch::Branches(HashMap::from([(
                        path[2].clone(),
                        KeyBranch::Command(cmd.clone())
                    )]))
                )]))
            )]))
        );
        let xpath = vec![
            KeyOption::Specific(KeyCode::Char(' ')),
            KeyOption::Specific(KeyCode::Tab),
            KeyOption::Specific(KeyCode::Char('x')),
        ];
        let xcmd = "remove".to_string();
        km = add_kb(km, &xpath, xcmd.clone());
        assert_eq!(
            km,
            KeyBranch::Branches(HashMap::from([(
                path[0].clone(),
                KeyBranch::Branches(HashMap::from([
                    (
                        path[1].clone(),
                        KeyBranch::Branches(HashMap::from([(
                            path[2].clone(),
                            KeyBranch::Command(cmd)
                        )]))
                    ),
                    (
                        xpath[1].clone(),
                        KeyBranch::Branches(HashMap::from([(
                            xpath[2].clone(),
                            KeyBranch::Command(xcmd)
                        )]))
                    ),
                ]))
            )]))
        );
    }
}
