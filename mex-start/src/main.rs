use std::time::Duration;

use anyhow::Result;
use mex_app::{Context, Editor, jobs::Jobs};
use mex_render::{
    Compositor,
    elements::{buffer::Buffer, debug::DebugElement, footer::Footer, which_key::WhichKey},
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
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

enum Cursor {
    Hiden,
    Shown((u16, u16)),
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            jobs: Jobs::new(),
            editor: Editor::new("./config.toml")?,
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
    let args: Vec<String> = std::env::args().into_iter().collect();

    app.compositor.add_element(DebugElement {}, |ide, layout| {
        layout.push(Constraint::Length(1), ide);
    });
    app.compositor.add_element(
        Buffer::new(args.get(1).cloned().as_deref())?,
        |ide, layout| {
            layout.push(Constraint::Fill(10), ide);
        },
    );
    app.compositor.add_element(WhichKey::new(), |ide, layout| {
        layout.push(Constraint::Percentage(15), ide);
    });
    app.compositor.add_element(Footer::new(), |ide, layout| {
        layout.push(Constraint::Length(1), ide);
    });
    // app.compositor.add_element(Explore::new(), |ide, layout| {
    //     layout.push(Location::Center((5, 6), (9, 10)), ide);
    // });

    app.terminal.show_cursor()?;
    while !app.editor.exit {
        let mut ctx = Context {
            editor: &mut app.editor,
        };
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) => {
                    app.compositor
                        .last_focused_element
                        .iter()
                        .rfind(|&id| {
                            app.compositor
                                .get(id)
                                .is_some_and(|element| element.is_visible())
                        })
                        .and_then(|id| app.compositor.elements.get_mut(id))
                        .and_then(|(element, rect)| {
                            element.capture_input(key_event, &mut ctx).zip(*rect)
                        })
                        .map(|(new_cursor_pos, rect)| {
                            ctx.editor.last_cursor_pos = Some((
                                new_cursor_pos.0.min(rect.width) + rect.x,
                                new_cursor_pos.1.min(rect.height) + rect.y,
                            ))
                        });
                }
                Event::Resize(width, height) => {
                    app.compositor
                        .calculate_areas(Rect::new(0, 0, width, height));
                }

                _ => {}
            }
        }

        let _ = app
            .terminal
            .draw(|frame: &mut Frame| {
                ctx.editor
                    .last_cursor_pos
                    .map(|pos| frame.set_cursor_position(pos));
                app.compositor
                    .render(frame.area(), frame.buffer_mut(), &mut ctx)
            })
            .map_err(|e| eprintln!("{}", e));
    }
    Ok(())
}
