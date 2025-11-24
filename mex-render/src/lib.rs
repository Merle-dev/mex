use std::{any::Any, fmt::Debug};

use indexmap::IndexMap;
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout, Rect},
};

pub mod elements;

pub trait Element: Any {
    fn render(&mut self, buffer: &mut Buffer, area: Rect, ctx: &Context);
    fn is_visible(&self) -> bool;
    fn captures_input(&self) -> bool;
    fn capture_input(&mut self, _event: KeyEvent) {
        panic!(
            "{} has to implement capture_input because it's captures the event",
            self.type_name()
        )
    }
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone)]
pub enum Location {
    TopLeft(u16, u16),
    TopRight(u16, u16),
    BottomLeft(u16, u16),
    BottomRight(u16, u16),
    Center((u16, u16), (u16, u16)),
}

impl Into<RenderInfo> for Constraint {
    fn into(self) -> RenderInfo {
        RenderInfo::Inside(self)
    }
}

impl Into<RenderInfo> for Location {
    fn into(self) -> RenderInfo {
        RenderInfo::Float(self)
    }
}

#[derive(Clone)]
pub enum RenderInfo {
    Inside(Constraint),
    Float(Location),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CompositorElementId(usize);

#[derive(Clone)]
pub struct CompositorLayout {
    pub elements_area: Vec<(RenderInfo, CompositorLayoutArea)>,
    pub direction: Direction,
}

impl CompositorLayout {
    pub fn new(direction: Direction) -> Self {
        Self {
            elements_area: vec![],
            direction,
        }
    }
    pub fn push(&mut self, render_info: impl Into<RenderInfo>, id: CompositorLayoutArea) {
        self.elements_area.push((render_info.into(), id));
    }
}
#[derive(Clone)]
pub enum CompositorLayoutArea {
    CompositorLayout(CompositorLayout),
    Id(CompositorElementId),
}

pub struct Compositor {
    pub elements: IndexMap<CompositorElementId, (Box<dyn Element + Send + Sync>, Option<Rect>)>,
    pub last_focused_element: Vec<CompositorElementId>,
    pub element_layout: CompositorLayout,
    next_id: CompositorElementId,
}

impl Default for Compositor {
    fn default() -> Self {
        Self {
            elements: IndexMap::new(),
            element_layout: CompositorLayout::new(Direction::Vertical),
            last_focused_element: Vec::new(),
            next_id: CompositorElementId(0),
        }
    }
}

fn flaoting_rect(loc: Location, area: Rect) -> Rect {
    match loc {
        Location::TopLeft(width, height) => Rect::new(
            area.x,
            area.y,
            width.min(area.width),
            height.min(area.height),
        ),
        Location::TopRight(width, height) => Rect::new(
            area.x + area.width - width,
            area.y,
            width.min(area.width),
            height.min(area.height),
        ),
        Location::BottomLeft(width, height) => Rect::new(
            area.x,
            area.y + area.height - height,
            width.min(area.width),
            height.min(area.height),
        ),
        Location::BottomRight(width, height) => Rect::new(
            area.x + area.width - width,
            area.y + area.height - height,
            width.min(area.width),
            height.min(area.height),
        ),
        Location::Center(
            (numarator_width, denominator_width),
            (numarator_height, denominator_height),
        ) => {
            // height and width as numerator
            let app_to_rect_ration_width = (numarator_width as f32 / denominator_width as f32)
                .max(0.1)
                .min(1.0);
            let app_to_rect_ration_height = (numarator_height as f32 / denominator_height as f32)
                .max(0.1)
                .min(1.0);

            let width = (area.width as f32 * app_to_rect_ration_width) as u16;
            let height = (area.height as f32 * app_to_rect_ration_height) as u16;

            Rect::new(
                area.x + (area.width / 2).saturating_sub(width / 2),
                area.y + (area.height / 2).saturating_sub(height / 2),
                width,
                height,
            )
        }
    }
}
fn into_rects(direction: Direction, render_info: Vec<RenderInfo>, area: Rect) -> Vec<Rect> {
    let mut with_constraints = vec![];
    let mut floating = vec![];
    for i in render_info.iter() {
        match i {
            RenderInfo::Inside(c) => with_constraints.push(*c),
            RenderInfo::Float(loc) => floating.push(flaoting_rect(loc.clone(), area)),
        };
    }
    [
        Layout::new(direction, with_constraints)
            .split(area)
            .to_vec(),
        floating,
    ]
    .concat()
}

impl Compositor {
    pub fn add_element(
        &mut self,
        element: impl Element + Send + Sync,
        func: impl FnOnce(CompositorLayoutArea, &mut CompositorLayout),
    ) {
        element
            .captures_input()
            .then(|| self.last_focused_element.push(self.next_id));
        self.elements
            .insert(self.next_id, (Box::new(element), None));
        func(
            CompositorLayoutArea::Id(self.next_id),
            &mut self.element_layout,
        );
        self.next_id.0 += 1;
    }
    pub fn get(
        &self,
        id: &CompositorElementId,
    ) -> Option<&Box<dyn Element + Send + Sync + 'static>> {
        self.elements.get(id).map(|(e, _)| e)
    }
    pub fn get_mut(
        &mut self,
        id: &CompositorElementId,
    ) -> Option<&mut Box<dyn Element + Send + Sync + 'static>> {
        self.elements.get_mut(id).map(|(e, _)| e)
    }
    pub fn render(&mut self, area: Rect, buffer: &mut Buffer, ctx: &Context) {
        let updates: Vec<(&Box<dyn Element + Sync + Send>, bool)> = self
            .elements
            .iter()
            .map(|(_, (element, rect))| (element, rect.is_none()))
            .collect();

        if updates.iter().find(|(_, u)| *u).is_some() {
            self.calculate_areas(area);
        }
        let _: Vec<_> = self
            .elements
            .iter_mut()
            .filter(|(_, (element, _))| element.is_visible())
            .map(|(_, (element, element_area_opt))| {
                element_area_opt.map(|element_area| {
                    element.render(buffer, element_area, ctx);
                })
            })
            .collect();
    }
    pub fn calculate_areas(&mut self, area: Rect) {
        let _: Vec<_> = Self::calc_layout(&self.element_layout, area)
            .iter()
            .map(|(id, new_area)| {
                self.elements
                    .get_mut(id)
                    .map(|(_, area)| *area = Some(*new_area))
            })
            .collect();
    }
    fn calc_layout(layout: &CompositorLayout, area: Rect) -> Vec<(CompositorElementId, Rect)> {
        let (constraints, comp_areas): (Vec<RenderInfo>, Vec<CompositorLayoutArea>) =
            layout.elements_area.iter().cloned().unzip();
        into_rects(layout.direction, constraints, area)
            .iter()
            .zip(comp_areas)
            .map(|(rect, layout_area)| match layout_area {
                CompositorLayoutArea::Id(id) => vec![(id, rect.clone())],
                CompositorLayoutArea::CompositorLayout(layout) => Self::calc_layout(&layout, *rect),
            })
            .map(|a| a)
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{Element, elements::debug::DebugElement};

    #[test]
    fn name() {
        println!("{}", DebugElement { text: "" }.type_name())
    }
}
