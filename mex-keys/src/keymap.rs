use std::{cmp::Ordering, collections::HashMap};

use anyhow::Result;
use mex_core::Mode;
use ratatui::crossterm::event::KeyCode;
use serde::Deserialize;
use toml::from_str;

use crate::key_operations::to_key;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum KeyOption {
    Num,
    Specific(KeyCode),
}

impl KeyOption {
    pub fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            KeyOption::Num => match *other {
                KeyOption::Num => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::Specific(skc) => match *other {
                KeyOption::Num => Ordering::Less,
                Self::Specific(okc) => {
                    let self_char = skc.to_string().chars().nth(0).unwrap();
                    let other_char = okc.to_string().chars().nth(0).unwrap();
                    match (self_char as u8) as i32 - (other_char as u8) as i32 {
                        ..0 => Ordering::Less,
                        0 => Ordering::Equal,
                        0.. => Ordering::Greater,
                    }
                }
            },
        }
    }
}

impl ToString for KeyOption {
    fn to_string(&self) -> String {
        match *self {
            KeyOption::Num => String::from("[@]"),
            KeyOption::Specific(ck) => ck.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyBranch {
    Branches(HashMap<KeyOption, KeyBranch>),
    Command(String),
}

pub fn get_unsafe(kb: &KeyBranch) -> &HashMap<KeyOption, KeyBranch> {
    match kb {
        KeyBranch::Branches(branch_ref) => branch_ref,
        KeyBranch::Command(_) => panic!("Core KeyMap can not be a Command"),
    }
}

pub fn add_kb(keymap: KeyBranch, path: &[KeyOption], command: String) -> KeyBranch {
    let (current_key, rest_of_path) = match path.split_first() {
        Some((key, rest)) => (key.clone(), rest),
        None => return keymap,
    };

    let mut inner_map = match keymap {
        KeyBranch::Branches(map) => map,
        KeyBranch::Command(_) => HashMap::new(),
    };

    if rest_of_path.is_empty() {
        inner_map.insert(current_key, KeyBranch::Command(command));
    } else {
        let next_branch = inner_map
            .remove(&current_key)
            .unwrap_or_else(|| KeyBranch::Branches(HashMap::new()));

        let modified_next_branch = add_kb(next_branch, rest_of_path, command);

        inner_map.insert(current_key, modified_next_branch);
    }

    KeyBranch::Branches(inner_map)
}

pub struct KeyMap(pub HashMap<Mode, KeyBranch>);

#[derive(Deserialize, Debug)]
struct CommandConfig {
    mode: String,
    keys: String,
    _multi_keys: Option<Vec<String>>,
}

impl KeyMap {
    pub fn from_config(path: &str) -> Result<Self> {
        let config_hashmap: HashMap<String, CommandConfig> =
            from_str(&std::fs::read_to_string(path)?)?;

        let mut hm: HashMap<Mode, KeyBranch> = HashMap::new();
        for (command, config) in config_hashmap.iter() {
            let key_seq = &Self::seperate(config.keys.clone());
            for mode_char in config.mode.split_whitespace().collect::<String>().chars() {
                let mode = match mode_char {
                    'i' => Mode::Insert,
                    's' => Mode::Select,
                    _ => Mode::Normal,
                };
                hm.get_mut(&mode)
                    .map(|kb| *kb = add_kb(kb.clone(), &key_seq, command.to_string()))
                    .or_else(|| {
                        Some({
                            hm.insert(
                                mode,
                                add_kb(
                                    KeyBranch::Branches(HashMap::new()),
                                    &key_seq,
                                    command.clone(),
                                ),
                            );
                        })
                    });
            }
        }

        Ok(Self(hm))
    }
    pub fn seperate(str: String) -> Vec<KeyOption> {
        str.chars()
            .fold((vec![], false), |(mut elements, inside), str| {
                if str == '{' && !inside {
                    elements.push(Ok(String::new()));
                    (elements, true)
                } else if str == '}' && inside {
                    (elements, false)
                } else if inside {
                    elements
                        .iter_mut()
                        .fold(None, |_acc, el| el.as_mut().ok())
                        .map(|last| last.push(str));
                    (elements, inside)
                } else {
                    elements.push(Err(str));
                    (elements, inside)
                }
            })
            .0
            .iter()
            .map(|result| match result {
                Ok(special) => to_key(special.clone()),
                Err(char) => to_key(char.to_string()),
            })
            .flatten()
            .collect()
    }
}
