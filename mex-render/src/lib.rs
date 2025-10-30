use std::any::Any;

use ratatui::{buffer::Buffer, layout::Rect};

pub struct Editor {
    loaded_files: Vec<String>,
}

pub struct Context<'a> {
    editor: &'a mut Editor,
}

pub trait Element: Any {
    fn render(&self, buffer: &mut Buffer, area: Rect, ctx: &mut Context);
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub struct Compositor {
    elements: Vec<Box<dyn Element>>,
}

impl Compositor {
    pub fn add_element(&mut self, element: impl Element) {
        self.elements.push(Box::new(element));
    }
    pub fn render(&self, buffer: &mut Buffer, area: Rect, ctx: &mut Context) {
        self.elements
            .iter()
            .map(|el| el.render(buffer, area, ctx))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use ratatui::widgets::{Block, List, Paragraph, Widget};

    use crate::{Compositor, Element};

    struct Input {}

    struct Tree {
        content: Vec<String>,
    }

    impl Element for Input {
        fn render(
            &self,
            buffer: &mut ratatui::prelude::Buffer,
            area: ratatui::prelude::Rect,
            ctx: &mut crate::Context,
        ) {
            Paragraph::new(">   ")
                .block(Block::bordered())
                .render(area, buffer);
        }
    }

    impl Element for Tree {
        fn render(
            &self,
            buffer: &mut ratatui::prelude::Buffer,
            area: ratatui::prelude::Rect,
            ctx: &mut crate::Context,
        ) {
            List::new(ctx.editor.loaded_files.clone()).render(area, buffer);
        }
    }

    #[test]
    fn build() {
        let tree = Tree { content: vec![] };
        assert_eq!(tree.type_name(), "mex_render::tests::Tree");
    }

    #[test]
    fn populate() {
        let mut comp = Compositor { elements: vec![] };
        comp.add_element(Tree { content: vec![] });
        comp.add_element(Input {});
        assert_eq!(comp.elements.len(), 1);
    }
}
