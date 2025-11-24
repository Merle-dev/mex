use std::time::Duration;

use anyhow::Result;
use futures::stream::FuturesUnordered;
use mex_core::{Context, Editor, Jobs, Mode};
use mex_render::{
    Compositor, CompositorLayout, CompositorLayoutArea, Location,
    elements::{buffer::Buffer, explore::Explore, footer::Footer},
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    init,
    layout::{Constraint, Rect},
    restore,
};

struct App {
    jobs: Jobs,
    editor: Editor,
    compositor: Compositor,
    terminal: DefaultTerminal,
}

impl App {
    fn new() -> Result<Self, ()> {
        Ok(Self {
            jobs: Jobs {
                list: FuturesUnordered::new(),
            },
            editor: Editor {
                loaded_files: vec![],
                mode: Mode::Normal,
                exit: false,
            },
            compositor: Compositor::default(),
            terminal: init(),
        })
    }
}

impl Drop for App {
    fn drop(&mut self) {
        restore();
    }
}

fn main() -> Result<()> {
    let mut app = App::new().unwrap();

    app.compositor.add_element(Footer::new(), |ide, layout| {
        layout.push(Constraint::Length(1), ide);
    });
    app.compositor
        .add_element(Buffer::new(None), |ide, layout| {
            layout.push(Constraint::Fill(1), ide);
        });
    app.compositor.add_element(Footer::new(), |ide, layout| {
        layout.push(Constraint::Length(1), ide);
    });
    app.compositor.add_element(Explore::new(), |ide, layout| {
        layout.push(Location::Center((5, 6), (9, 10)), ide);
    });

    while !app.editor.exit {
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Enter {
                        app.editor.exit = true;
                    }
                    app.compositor
                        .last_focused_element
                        .iter()
                        .rfind(|&id| {
                            app.compositor
                                .get(id)
                                .is_some_and(|element| element.is_visible())
                        })
                        .and_then(|id| app.compositor.elements.get_mut(id))
                        .map(|(element, _)| element.capture_input(key_event));
                }
                Event::Resize(width, height) => {
                    app.compositor
                        .calculate_areas(Rect::new(0, 0, width, height));
                }

                _ => {}
            }
        }
        let ctx = Context {
            editor: &mut app.editor,
        };
        let _ = app
            .terminal
            .draw(|frame: &mut Frame| {
                app.compositor
                    .render(frame.area(), frame.buffer_mut(), &ctx)
            })
            .map_err(|e| eprintln!("{}", e));
    }
    Ok(())
}
