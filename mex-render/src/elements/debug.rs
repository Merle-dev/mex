use std::time::Duration;

use ratatui::widgets::{Paragraph, Widget};

use crate::Element;

pub struct DebugElement {}

impl Element for DebugElement {
    fn render(
        &mut self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &mut mex_app::Context,
    ) {
        Paragraph::new(format!(
            "{}",
            ctx.editor
                .messages
                .first()
                .map(|e| e.0.clone())
                .unwrap_or("".to_string())
        ))
        .render(area, buffer);
        if ctx
            .editor
            .messages
            .first()
            .is_some_and(|(_, age)| age.elapsed() > Duration::from_secs(3))
        {
            ctx.editor.messages.remove(0);
        }
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
