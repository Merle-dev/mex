use std::{io::Read, time::Instant};

use crate::Element;
use mex_core::Mode;
use mex_keys::KeyBranch;
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    widgets::{List, ListItem, Paragraph, Widget},
};

pub struct Buffer {
    lines: Vec<String>,
    cursor: (usize, usize),
}
impl Buffer {
    pub fn new(file: Option<String>) -> Self {
        let lines: Vec<_> = file
            .and_then(|name| std::fs::File::open(name).ok())
            .and_then(|mut a| {
                let mut buf = "".to_string();
                a.read_to_string(&mut buf)
                    .ok()
                    .map(|_| buf.lines().map(String::from).collect())
            })
            .unwrap_or(vec![]);
        Self {
            lines,
            cursor: (0, 0),
        }
    }
}

impl Element for Buffer {
    fn render(
        &mut self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &mut mex_app::Context,
    ) {
        List::new(
            self.lines
                .iter()
                .map(|line| ListItem::new(format!("{line}"))),
        )
        .render(area, buffer);
        Paragraph::new(format!("{:?}", self.cursor)).render(Rect::new(0, 30, 20, 1), buffer);
    }
    fn captures_input(&self) -> bool {
        true
    }
    fn capture_input(
        &mut self,
        event: ratatui::crossterm::event::KeyEvent,
        ctx: &mut mex_app::Context,
    ) -> Option<(u16, u16)> {
        match event.code {
            KeyCode::Delete => ctx.editor.exit = true,
            KeyCode::Backspace if ctx.editor.mode == Mode::Insert => {
                let _ = self
                    .lines
                    .get_mut(self.cursor.1)
                    .map(|line| line.remove(self.cursor.0.min(line.len().saturating_sub(1))));
                self.cursor.0 = self.cursor.0.saturating_sub(1);
            }
            KeyCode::Char(char) if ctx.editor.mode == Mode::Insert => {
                self.lines.get_mut(self.cursor.1).map(|line| {
                    line.insert(self.cursor.0.min(line.len().saturating_sub(1)), char);
                    self.cursor.0 += 1;
                });
            }
            KeyCode::Left => {
                self.cursor.0 = self.cursor.0.saturating_sub(1);
                return Some((self.cursor.0 as u16, self.cursor.1 as u16));
            }
            KeyCode::Right => {
                self.cursor.0 += 1;
                return Some((self.cursor.0 as u16, self.cursor.1 as u16));
            }
            other => ctx
                .editor
                .keymap_controller
                .compute_key(other)
                .iter()
                .flatten()
                .for_each(|result| match &result.0 {
                    KeyBranch::Command(cmd) => ctx.compute_command(cmd.clone()),
                    // KeyBranch::Command(cmd) => ctx.editor.messages.push((
                    //     format!(
                    //         "{cmd} {:?}",
                    //         result
                    //             .1
                    //             .iter()
                    //             .rev()
                    //             .enumerate()
                    //             .fold(0u32, |acc, (i, item)| acc
                    //                 + (*item as u32) * 10u32.pow(i as u32))
                    //     ),
                    //     Instant::now(),
                    // )),
                    KeyBranch::Branches(_) => (),
                }),
        };
        None
    }
    fn is_visible(&self) -> bool {
        true
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
