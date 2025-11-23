use ratatui::widgets::{List, ListItem, Widget};

use crate::Element;

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
        ctx: &mex_core::Context,
    ) {
        List::new((0..15).map(|i| ListItem::new(format!("{i}")))).render(area, buffer);
    }
    fn captures_input(&self) -> bool {
        true
    }
    fn is_visible(&self) -> bool {
        true
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
