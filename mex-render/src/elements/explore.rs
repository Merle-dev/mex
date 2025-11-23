use anyhow::Result;
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Clear, List, ListItem, Paragraph, Widget, Wrap},
};

use crate::Element;

pub struct Explore {
    query: String,
    search_result: Option<Vec<(DirEntry, usize, String)>>,
    visible: bool,
}

use ignore::{DirEntry, WalkBuilder, WalkState};
use regex::Regex;
use std::{path::Path, sync::mpsc};

impl Element for Explore {
    fn render(
        &mut self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &crate::Context,
    ) {
        Clear::default().render(area, buffer);
        let [left, display_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);
        let [search_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(left);
        Paragraph::new(self.query.clone())
            .bold()
            .wrap(Wrap::default())
            .block(
                Block::bordered()
                    .title("Find Word")
                    .title_alignment(ratatui::layout::Alignment::Center),
            )
            .render(search_area, buffer);
        let path = Path::new("./");
        let results = match &self.search_result {
            Some(result) => &result,
            None => {
                let result = self.parallel_search(&path).unwrap_or(vec![]);
                self.search_result = Some(result.clone());
                &self.search_result.clone().unwrap()
            }
        };
        List::new(results.iter().map(|(file, line_number, line)| {
            ListItem::new(format!("{}:{line_number}   {line}", file.path().display()))
        }))
        .block(Block::bordered())
        .render(list_area, buffer);
        Block::bordered().render(display_area, buffer);
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
    fn captures_input(&self) -> bool {
        true
    }
    fn capture_input(&mut self, event: ratatui::crossterm::event::KeyEvent) {
        match event.code {
            KeyCode::Char(key) => {
                self.query.push(key);
                self.search_result = None;
            }
            KeyCode::Backspace => {
                self.query
                    .pop()
                    .is_some()
                    .then(|| self.search_result = None);
            }
            KeyCode::Esc => self.visible = false,
            _ => (),
        };
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Explore {
    pub fn new() -> Self {
        Self {
            query: "".to_owned(),
            search_result: None,
            visible: true,
        }
    }

    fn parallel_search(&self, path: &std::path::Path) -> Result<Vec<(DirEntry, usize, String)>> {
        if self.query.is_empty() {
            return Ok(vec![]);
        }
        let regex = Regex::new(&self.query)?;
        let (tx, rx) = mpsc::channel();

        let walker = WalkBuilder::new(path)
            .hidden(false)
            .git_ignore(true)
            .build_parallel();

        walker.run(|| {
            let tx = tx.clone();
            let regex = regex.clone();
            Box::new(move |result| {
                let entry = match result {
                    Ok(entry) => entry,
                    Err(_) => return WalkState::Continue,
                };

                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        for (line_num, line) in content.lines().enumerate() {
                            if regex.is_match(line) {
                                let result = (entry.clone(), line_num + 1, line.trim().to_string());
                                if tx.send(result).is_err() {
                                    return WalkState::Quit;
                                }
                            }
                        }
                    }
                }

                WalkState::Continue
            })
        });

        drop(tx); // Drop the original sender to allow the receiver to know when no more messages will be sent.

        let results: Vec<(DirEntry, usize, String)> = rx.iter().collect();
        Ok(results)
    }
}
