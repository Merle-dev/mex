use mex_keys::{KeyBranch, KeyOption};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

use crate::Element;

pub struct WhichKey {
    visible: bool,
}

impl WhichKey {
    pub fn new() -> Self {
        Self { visible: true }
    }
}

impl Element for WhichKey {
    fn render(
        &mut self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &mut mex_app::Context,
    ) {
        // Block::bordered().render(area, buffer);
        let h = area.height.saturating_sub(2);
        let mut hm = ctx
            .editor
            .keymap_controller
            .current
            .iter()
            .collect::<Vec<_>>();
        hm.sort_by(|a, b| a.0.cmp(b.0));
        let items = hm
            .iter()
            .map(|(option, branch)| split_up(vec![option], branch))
            .flatten()
            .map(|item| ListItem::new(item))
            .collect::<Vec<_>>();
        let items_chunks = items.chunks(h as usize).collect::<Vec<_>>();
        let [border, show_area] = Layout::new(
            ratatui::layout::Direction::Vertical,
            [Constraint::Length(1), Constraint::Fill(1)],
        )
        .areas(area);
        Paragraph::new("─".repeat(border.width as usize)).render(border, buffer);
        let areas = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(1); items_chunks.len()],
        )
        .split(show_area);
        areas
            .into_iter()
            .zip(items_chunks)
            .for_each(|(part_area, keys)| {
                List::new(keys.to_vec())
                    .block(Block::new())
                    .render(*part_area, buffer)
            });
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
    fn captures_input(&self) -> bool {
        false
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn split_up(key_opt: Vec<&KeyOption>, key_branch: &KeyBranch) -> Vec<String> {
    match key_branch {
        KeyBranch::Command { command, .. } => vec![format!(
            "{} -> {command}",
            key_opt
                .into_iter()
                .map(keyopt_to_char)
                .collect::<Vec<_>>()
                .join(" ")
        )],
        KeyBranch::Branches(hm) => hm
            .iter()
            .map(|(a, b)| split_up([key_opt.clone(), vec![a]].concat(), b))
            .flatten()
            .collect(),
    }
}

fn keyopt_to_char(key_opt: &KeyOption) -> String {
    match *key_opt {
        KeyOption::Num => "[@]".to_string(),
        KeyOption::Specific(key) => key.to_string(),
    }
}
