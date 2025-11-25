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
}

impl Editor {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            loaded_files: vec![],
            keymap_controller: KeyMapWrapper::new(path)?,
            settings: Settings {},
            mode: Mode::Normal,
            exit: false,
        })
    }
}

pub struct Context<'a> {
    pub editor: &'a mut Editor,
}
