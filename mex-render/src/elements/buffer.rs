use std::fmt::format;

use crate::Element;
use anyhow::Result;
use mex_buffer::RopeComponent;
use mex_core::{FilePosition, Mode, log};
use mex_keys::KeyBranch;
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    style::Stylize,
    widgets::{List, Paragraph, Widget},
};

pub struct Buffer {
    content: RopeComponent,
    cursor: FilePosition,
    last_rect: Option<Rect>,
    buffer: Option<Vec<String>>,
    scroll: usize,
}

impl Element for Buffer {
    fn render(
        &mut self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &mut mex_app::Context,
    ) {
        self.last_rect = Some(area);
        List::new(match &self.buffer {
            Some(buffer) => buffer.clone(),
            None => {
                let buffer = self
                    .content
                    .lines(self.scroll..self.scroll + area.height as usize)
                    .map(|line| {
                        line.lines()
                            .map(|a| String::from(a))
                            .collect::<Vec<String>>()
                    });
                self.buffer = buffer.clone();
                buffer.unwrap_or(vec![])
            }
        })
        .on_black()
        .render(area, buffer);
        Paragraph::new(format!(
            "{:?}|{}|{}|{}",
            self.cursor.y,
            self.scroll,
            self.content.len_lines(),
            self.cursor
                .y
                .saturating_sub((area.height as usize).saturating_sub(Self::BORDER_SPACING)),
        ))
        .render(
            Rect {
                x: 120,
                y: 20,
                width: 50,
                height: 1,
            },
            buffer,
        );
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
            KeyCode::Delete if ctx.editor.mode == Mode::Insert => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
                self.content
                    .delete(self.cursor.x, self.cursor.y, 1)
                    .unwrap();
            }
            KeyCode::Delete => ctx.editor.exit = true,
            KeyCode::Backspace if ctx.editor.mode == Mode::Insert => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
                self.content
                    .delete(self.cursor.x, self.cursor.y, 1)
                    .unwrap();
            }
            KeyCode::Char(char) if ctx.editor.mode == Mode::Insert => {
                self.content
                    .insert(self.cursor.x, self.cursor.y, char)
                    .unwrap();
                self.cursor.x += 1;
            }
            KeyCode::Left => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
            }
            KeyCode::Right => {
                self.cursor.x += 1;
            }
            KeyCode::Up => self.scroll(-1),
            KeyCode::Down => self.scroll(1),
            KeyCode::PageUp => self.scroll(-5),
            KeyCode::PageDown => self.scroll(5),
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
                    KeyBranch::Branches(b) => log(b),
                }),
        };
        self.buffer = None;
        Some((self.cursor.x as u16, (self.cursor.y - self.scroll) as u16))
    }
    fn is_visible(&self) -> bool {
        true
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Buffer {
    const BORDER_SPACING: usize = 5;
    pub fn new(file: Option<&str>) -> Result<Self> {
        Ok(Self {
            content: RopeComponent::new(file)?,
            cursor: FilePosition { x: 0, y: 0 },
            last_rect: None,
            buffer: None,
            scroll: 0,
        })
    }
    pub fn scroll(&mut self, n: isize) {
        self.cursor.y = self
            .cursor
            .y
            .saturating_add_signed(n)
            .clamp(0, self.content.len_lines() - 2);

        if let Some(rect) = self.last_rect {
            let height = rect.height as usize;
            let max_abs = self.content.len_lines().saturating_sub(height + 1);
            let top_limit = self.cursor.y.saturating_sub(Self::BORDER_SPACING);
            let bottom_limit = self.cursor.y.saturating_sub(
                height
                    .saturating_sub(Self::BORDER_SPACING)
                    .saturating_sub(1),
            );

            self.scroll = self
                .scroll
                .max(top_limit)
                .min(bottom_limit)
                .min(max_abs)
                .max(0);
        }

        // self.scroll = self
        //     .scroll
        //     .max(self.cursor.y - rect.height as usize)
        //     .min(self.content.len_lines() - rect.height as usize - 1)
        //     .min(self.cursor.y);
    }
}
