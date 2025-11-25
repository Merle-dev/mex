use std::{collections::HashMap, rc::Rc};

use anyhow::{Result, anyhow};
use mex_core::Mode;
use ratatui::crossterm::event::KeyCode;

use crate::keymap::KeyOption;

mod key_operations;
mod keymap;
pub use keymap::{KeyBranch, KeyMap};

pub struct KeyMapWrapper {
    base: KeyMap,
    current: HashMap<KeyOption, KeyBranch>, /* Would be better a reference but to complicated and performance is negitable */
    stored_nums: Vec<u8>,
}

impl KeyMapWrapper {
    pub fn new(path: &str) -> Result<Self> {
        let base = KeyMap::from_config(path)?;
        let current = base
            .0
            .get(&Mode::Normal)
            .cloned()
            .and_then(|branch| match branch {
                KeyBranch::Branches(hm) => Some(hm),
                KeyBranch::Command(_) => None,
            })
            .ok_or(anyhow!("Somehow there are no normal mode implementation"))?;
        Ok(Self {
            base,
            current,
            stored_nums: vec![],
        })
    }
    fn return_to_base(&mut self) -> Result<()> {
        self.current = self
            .base
            .0
            .get(&Mode::Normal)
            .and_then(|branch| match branch {
                KeyBranch::Branches(hm) => Some(hm),
                KeyBranch::Command(_) => None,
            })
            .cloned()
            .ok_or(anyhow!("Somehow there are no normal mode implementation"))?;
        Ok(())
    }
    fn after_care(&mut self, branch: &KeyBranch) -> Result<()> {
        match branch {
            KeyBranch::Branches(new_hm) => self.current = new_hm.clone(),
            KeyBranch::Command(_) => self.return_to_base()?,
        };
        Ok(())
    }
    pub fn compute_key(&mut self, key: KeyCode) -> Result<Option<(KeyBranch, Rc<[u8]>)>> {
        if let Some(branch) = match key {
            KeyCode::Char(char) if (48..=57).contains(&(char as u8)) => {
                self.current.get(&KeyOption::Num).map(|branch| {
                    self.stored_nums.push(char as u8 - 48);
                    branch
                })
            }
            other => self.current.get(&KeyOption::Specific(other)),
        }
        .cloned()
        {
            self.after_care(&branch)?;
            Ok(Some((
                branch,
                Rc::from(std::mem::take(&mut self.stored_nums).into_boxed_slice()),
            )))
        } else {
            Ok(None)
        }
    }
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
