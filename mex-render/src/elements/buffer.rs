use std::{io::Read, time::Instant};

use crate::Element;
use mex_core::{Mode, log};
use mex_keys::KeyBranch;
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    widgets::{List, ListItem, Paragraph, Widget},
};

pub struct Buffer {
    content: RopeWrapper,
    index: usize,
    cursor: (usize, usize),
    last_position: (u16, u16),
}
impl Buffer {
    pub fn new(file: Option<String>) -> Self {
        Self {
            content: RopeWrapper::new(file),
            index: 0,
            cursor: (0, 0),
            last_position: (0, 0),
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
        self.last_position = (area.x, area.y);
        List::new(
            self.content
                .lines()
                .map(|line| line.chars().collect::<String>()),
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
        let get = |content: &mut Rope, index| -> Option<char> {
            (content.byte_len() >= index).then(|| content.byte(index) as char)
        };
        match event.code {
            KeyCode::Delete => ctx.editor.exit = true,
            KeyCode::Char(char) if ctx.editor.mode == Mode::Insert => {
                self.content.insert(self.index, char.to_string().as_str());
            }
            KeyCode::Left => {
                self.cursor.0 = self.cursor.0.saturating_sub(1);
                if self.index != 0 && get(&mut self.content, self.index - 1) != Some('\n') {
                    self.index -= 1;
                }
            }
            KeyCode::Right => {
                self.cursor.0 += 1;
                if get(&mut self.content, self.index + 1) != Some('\n') {
                    self.index += 1;
                }
            }
            KeyCode::Up => {
                self.cursor.1 = self.cursor.1.saturating_sub(1);
            }
            KeyCode::Down => {
                self.cursor.1 += 1;
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

        Some((
            self.cursor.0 as u16 + self.last_position.0,
            self.cursor.1 as u16 + self.last_position.1,
        ))
    }
    fn is_visible(&self) -> bool {
        true
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
