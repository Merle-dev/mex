use std::fmt::Debug;

use ratatui::widgets::{Paragraph, Widget};

use crate::Element;

pub struct DebugElement<T: Debug> {
    pub text: T,
}

impl<T: Debug> DebugElement<T> {
    pub fn new(a: T) -> Self {
        Self { text: a }
    }
}

impl<T: Debug + 'static> Element for DebugElement<T> {
    fn render(
        &mut self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &mut mex_app::Context,
    ) {
        Paragraph::new(format!("{:?}", self.text)).render(area, buffer);
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
