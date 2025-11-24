use futures::{future::BoxFuture, stream::FuturesUnordered};
use mex_core::Mode;
use mex_keys::keymap::KeyMap;

pub struct Settings {
    key_map: KeyMap,
}

pub struct Editor {
    pub loaded_files: Vec<String>,
    pub settings: Settings,
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
