use std::{fs::File, io::BufReader, ops::Range};

use anyhow::Result;
use ropey::{Rope, RopeSlice};

pub struct RopeComponent {
    rope: Rope,
}

pub enum RopeInsert {
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

impl RopeComponent {
    pub fn new(file: Option<&str>) -> Result<Self> {
        Ok(Self {
            rope: match file {
                Some(path) => Rope::from_reader(BufReader::new(File::open(path)?))?,
                None => Rope::new(),
            },
        })
    }
    pub fn insert(&mut self, x: usize, y: usize, value: impl Into<RopeInsert>) -> Result<()> {
        let index = self.rope.line_to_char(y) + x;
        match value.into() {
            RopeInsert::Char(ch) => self.rope.try_insert_char(index, ch),
            RopeInsert::Str(str) => self.rope.try_insert(index, str.as_str()),
        }?;
        Ok(())
    }
    pub fn replace(
        &mut self,
        x: Range<usize>,
        y: Range<usize>,
        value: impl Into<RopeInsert>,
    ) -> Result<()> {
        let start_index = self.rope.try_line_to_char(y.start)? + x.start;
        let end_index = self.rope.try_line_to_char(y.end)? + x.end;
        self.rope.try_remove(start_index..end_index)?;
        match value.into() {
            RopeInsert::Char(ch) => self.rope.try_insert_char(start_index, ch),
            RopeInsert::Str(str) => self.rope.try_insert(start_index, str.as_str()),
        }?;
        Ok(())
    }
    pub fn delete(&mut self, x: usize, y: usize, len: usize) -> Result<()> {
        let start_index = self.rope.try_line_to_char(y)? + x;
        self.rope.try_remove(start_index..start_index + len)?;
        Ok(())
    }
    pub fn index(&self, x: usize, y: usize) -> Option<char> {
        let index = self.rope.try_line_to_char(y).ok()? + x;
        (index < self.rope.len_bytes()).then(|| self.rope.byte(index) as char)
    }
    pub fn lines(&self, range: Range<usize>) -> Option<RopeSlice<'_>> {
        // range.map(|index| self.rope.line(index)).flatten()
        let start = self.rope.try_line_to_byte(range.start).ok()?;
        let end = self
            .rope
            .try_line_to_char(range.end)
            .ok()
            .unwrap_or(self.rope.len_bytes());
        self.rope.get_slice(start..end)
    }
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }
}

#[cfg(test)]
mod test {
    use crate::RopeComponent;

    #[test]
    fn test() {
        let mut rw = RopeComponent::new(Some("lorem.txt")).unwrap();
        rw.insert(10, 0, "FK".to_string()).unwrap();
        let text = String::from("Hello, World!");
        rw.replace(5..5 + text.len(), 2..3, text).unwrap();
    }
}
