use std::fmt::Display;

use futures::{future::BoxFuture, stream::FuturesUnordered};

#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
    Select,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Mode::Normal => f.write_str("normal"),
            Mode::Insert => f.write_str("insert"),
            Mode::Select => f.write_str("select"),
        }
    }
}

pub struct Editor {
    pub loaded_files: Vec<String>,
    pub mode: Mode,
    pub exit: bool,
}

pub struct Context<'a> {
    pub editor: &'a mut Editor,
}

pub type Callback = Box<dyn FnOnce(&mut Context) + Send>;
pub type Job = BoxFuture<'static, Option<Callback>>;

pub struct Jobs {
    pub list: FuturesUnordered<Job>,
}
