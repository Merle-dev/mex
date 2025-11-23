use ratatui::widgets::{Paragraph, Widget};

use crate::Element;

pub struct Footer {}

impl Footer {
    pub fn new() -> Self {
        Footer {}
    }
}

impl Element for Footer {
    fn render(
        &mut self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &mex_core::Context,
    ) {
        Paragraph::new(format!(
            "{} | {}",
            ctx.editor.mode,
            ctx.editor
                .loaded_files
                .first()
                .map(|s| s.as_str())
                .unwrap_or("")
        ))
        .render(area, buffer);
        // self.update = false;
    }
    fn captures_input(&self) -> bool {
        false
    }
    fn is_visible(&self) -> bool {
        true
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
