use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Write,
    sync::{LazyLock, Mutex},
};

struct Log {
    file: Option<Mutex<File>>,
}

impl Log {
    fn new() -> Self {
        Self {
            file: File::create("mex_log.txt")
                .map_err(|err| dbg!(err))
                .ok()
                .map(Mutex::new),
        }
    }
}

static LOG: LazyLock<Log> = LazyLock::new(|| Log::new());
pub fn log<T: Debug>(text: &T) {
    if let Some(file) = &LOG.file {
        let _ = file
            .lock()
            .unwrap()
            .write(format!("{:#?}\n\n\n", text).as_bytes())
            .map_err(|err| dbg!(err))
            .unwrap();
    }
}

pub fn log_self<T: Debug>(text: T) -> T {
    if let Some(file) = &LOG.file {
        let _ = file
            .lock()
            .unwrap()
            .write(format!("{:#?}\n\n\n", text).as_bytes())
            .map_err(|err| dbg!(err))
            .unwrap();
    }
    text
}

#[derive(Debug)]
pub struct FilePosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Insert,
    Select,
    Replace,
    MultiInsert,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Mode::Normal => f.write_str("normal"),
            Mode::Insert => f.write_str("insert"),
            Mode::Select => f.write_str("select"),
            Mode::Replace => f.write_str("replace"),
            Mode::MultiInsert => f.write_str("m-insert"),
        }
    }
}
