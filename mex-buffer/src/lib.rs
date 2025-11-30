use anyhow::Result;
use ropey::Rope;

struct RopeWrapper {
    index: usize,
    rope: Rope,
}

impl RopeWrapper {
    fn insert(&mut self, x: usize, y: usize, char: char) {
        self.rope.insert(char_idx, text);
    }
}
