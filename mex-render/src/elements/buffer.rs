use crate::Element;
use mex_keys::KeyBranch;
use ratatui::{
    crossterm::event::KeyCode,
    widgets::{List, ListItem, Widget},
};

pub struct Buffer {}
impl Buffer {
    pub fn new(file: Option<String>) -> Self {
        Self {}
    }
}

impl Element for Buffer {
    fn render(
        &mut self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &mut mex_app::Context,
    ) {
        List::new((0..15).map(|i| ListItem::new(format!("{i}")))).render(area, buffer);
    }
    fn captures_input(&self) -> bool {
        true
    }
    fn capture_input(
        &mut self,
        event: ratatui::crossterm::event::KeyEvent,
        ctx: &mut mex_app::Context,
    ) {
        if event.code == KeyCode::Char('q') {
            ctx.editor.exit = true;
        }
        let _ = ctx
            .editor
            .keymap_controller
            .compute_key(event.code)
            .map(|err| dbg!(err));
    }
    fn is_visible(&self) -> bool {
        true
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
