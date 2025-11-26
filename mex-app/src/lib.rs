use std::time::Instant;

use anyhow::Result;
use mex_core::Mode;
use mex_keys::KeyMapWrapper;

pub mod jobs;

pub struct Settings {}

pub struct Editor {
    pub loaded_files: Vec<String>,
    pub keymap_controller: KeyMapWrapper,
    pub settings: Settings,
    pub mode: Mode,
    pub exit: bool,
    pub messages: Vec<(String, Instant)>,
}

impl Editor {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            loaded_files: vec![],
            keymap_controller: KeyMapWrapper::new(path)?,
            settings: Settings {},
            mode: Mode::Normal,
            messages: vec![],
            exit: false,
        })
    }
}

pub struct Context<'a> {
    pub editor: &'a mut Editor,
}
