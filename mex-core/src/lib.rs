use std::fmt::Display;

use futures::{future::BoxFuture, stream::FuturesUnordered};

#[derive(Debug, PartialEq, Eq, Hash)]
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
