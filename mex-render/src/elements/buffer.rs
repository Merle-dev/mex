use crate::Element;
use anyhow::Result;
use mex_buffer::RopeComponent;
use mex_core::{FilePosition, Mode, log};
use mex_keys::KeyBranch;
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    widgets::{List, Paragraph, Widget},
};

pub struct Buffer {
    content: RopeComponent,
    cursor: FilePosition,
    cursor_extend: usize,
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
        .render(area, buffer);
        Paragraph::new(format!("{:?}", self.cursor_extend))
            .render(Rect::new(50, 30, 100, 1), buffer);
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
            KeyCode::Char(char) if ctx.editor.mode == Mode::Insert => {
                self.content
                    .insert(self.cursor.x, self.cursor.y, char)
                    .unwrap();
                self.cursor.x += 1;
            }
            _ => (),
        };
        ctx.editor
            .keymap_controller
            .compute_key(event.code)
            .ok()
            .flatten()
            .and_then(|result| match result {
                KeyBranch::Command { command, set_arg } => Some((command, set_arg)),
                KeyBranch::Branches(_) => None,
            })
            .map(|(result, args)| {
                let times = args.unwrap_or(1);
                match result.as_str() {
                    "delete" if ctx.editor.mode == Mode::Insert => {
                        self.cursor.x = self.cursor.x.saturating_sub(1);
                        let _ = self.content.delete(self.cursor.x, self.cursor.y, 1);
                    }
                    "remove" if ctx.editor.mode == Mode::Insert => self.backspace(),
                    "left" => self.horizontal(-(times as isize)),
                    "right" => self.horizontal(times as isize),
                    "up" => self.scroll(-(times as isize)),
                    "down" => self.scroll(times as isize),
                    "end_line" => {
                        self.horizontal(self.content.line_len(self.cursor.y) as isize);
                        self.cursor_extend = usize::MAX;
                    }
                    "start_line" => {
                        self.cursor.x = 0;
                        self.cursor_extend = 0;
                    }
                    _ => ctx.compute_command(result),
                };
            });
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
            cursor_extend: 0,
            last_rect: None,
            buffer: None,
            scroll: 0,
        })
    }
    pub fn backspace(&mut self) {
        let len = self.content.line_len(self.cursor.y);
        self.content
            .xy_to_index(self.cursor.x, self.cursor.y)
            .map(|index| index - 1)
            .map(|index| self.content.rope.try_remove(index..index + 1));
        if self.cursor.x != 0 {
            self.cursor.x -= 1;
        } else if self.cursor.y != 0 {
            self.cursor.x = len - 1;
            self.cursor.y -= 1;
        }
    }
    pub fn horizontal(&mut self, n: isize) {
        self.cursor.x = self
            .cursor
            .x
            .saturating_add_signed(n)
            .min(self.content.line_len(self.cursor.y) - 1);
        self.cursor_extend = self.cursor_extend.min(self.cursor.x);
    }

    pub fn scroll(&mut self, n: isize) {
        self.cursor.y = self
            .cursor
            .y
            .saturating_add_signed(n)
            .clamp(0, self.content.len_lines() - 2);

        self.cursor.x = self
            .cursor
            .x
            .max(self.cursor_extend)
            .min(self.content.line_len(self.cursor.y) - 1);

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
    }
}
