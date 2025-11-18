use std::any::Any;

use mex_core::Context;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::elements::explore::Explore;

pub mod elements;

pub enum Location {
    Center,
    Bottom,
    Right,
    Left,
    Top,
}

pub trait Element: Any {
    fn render(&self, buffer: &mut Buffer, area: Rect, ctx: &Context);
    fn update(&self) -> bool;
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Compositor {
    pub elements: Vec<Box<dyn Element>>,
}

impl Default for Compositor {
    fn default() -> Self {
        let mut r = Self { elements: vec![] };
        r.add_element(Explore::new());
        r
    }
}

impl Compositor {
    pub fn add_element(&mut self, element: impl Element) {
        self.elements.push(Box::new(element));
    }
    pub fn render(&self, area: Rect, buffer: &mut Buffer, ctx: &Context) {
        self.elements
            .iter()
            .filter(|el| el.update())
            .map(|el| el.render(buffer, area, ctx))
            .collect()
    }
    fn position(area: Rect) {}
}
#[cfg(test)]
mod test {
    use crate::{Element, elements::debug::DebugElement};

    #[test]
    fn name() {
        println!("{}", DebugElement { text: "" }.type_name())
    }
}
