use crate::Element;

pub struct Explore {
    query: String,
}

impl Element for Explore {
    fn render(
        &self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        ctx: &crate::Context,
    ) {
    }
    fn update(&self) -> bool {
        false
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Explore {
    pub fn new() -> Self {
        Self {
            query: "Search Query".to_owned(),
        }
    }
}
