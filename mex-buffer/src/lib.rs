use std::{
    fs::File,
    io::BufReader,
    ops::{Range, RangeBounds},
};

use anyhow::Result;
use ropey::Rope;

struct RopeWrapper {
    rope: Rope,
}

enum RopeInsert {
    Str(String),
    Char(char),
}

impl Into<RopeInsert> for String {
    fn into(self) -> RopeInsert {
        RopeInsert::Str(self)
    }
}
impl Into<RopeInsert> for char {
    fn into(self) -> RopeInsert {
        RopeInsert::Char(self)
    }
}

impl RopeWrapper {
    pub fn new(file: Option<&str>) -> Result<Self> {
        Ok(Self {
            rope: match file {
                Some(path) => Rope::from_reader(BufReader::new(File::open(path)?))?,
                None => Rope::new(),
            },
        })
    }
    fn insert(&mut self, x: usize, y: usize, value: impl Into<RopeInsert>) {
        let index = self.rope.line_to_char(y) + x;
        match value.into() {
            RopeInsert::Char(ch) => self.rope.insert_char(index, ch),
            RopeInsert::Str(str) => self.rope.insert(index, str.as_str()),
        }
    }
    fn replace(&mut self, x: Range<usize>, y: Range<usize>, value: impl Into<RopeInsert>) {
        let start_index = self.rope.line_to_char(y.start) + x.start;
        match value.into() {
            RopeInsert::Char(ch) => self.rope.insert_char(start_index, ch),
            RopeInsert::Str(str) => self.rope.insert(start_index, str.as_str()),
        };
        let end_index = self.rope.line_to_char(y.end) + x.end;
        self.rope.remove(start_index..end_index);
    }
}

#[cfg(test)]
mod test {
    use std::io::BufWriter;

    use crate::RopeWrapper;

    #[test]
    fn test() {
        let mut rw = RopeWrapper::new(Some("lorem.txt")).unwrap();
        rw.insert(10, 0, "FK".to_string());
        let text = String::from("Hello, World!");
        rw.replace(5..5 + text.len(), 2..3, text);
        let mut buffer = [0u8; 4096];
        rw.rope.write_to(BufWriter::new(buffer.as_mut())).unwrap();
        println!("{:#?}", String::from_utf8(buffer.to_vec()).unwrap());
    }
}
