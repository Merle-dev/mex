use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use futures::{StreamExt, stream::FuturesUnordered};
use mex_core::{Callback, Context, Editor, Jobs, Mode};
use mex_render::{Compositor, Element, elements::debug::DebugElement};
use ratatui::{DefaultTerminal, Frame, init};

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

fn main() {
    let mut app = App::new().unwrap();
    // let call = Box::pin(async {
    //     let cb: Callback = Box::new(|ctx: &mut Context| {
    //         pr
    //     });
    //     Some(cb)
    // });
    // app.jobs.list.push(call);
    app.compositor
        .add_element(DebugElement::new((false, 0u128)));

    let mut now = Instant::now();
    let mut dt = now.elapsed().as_nanos();
    while !app.editor.exit {
        let result: Option<Callback> =
            smol::block_on(async { app.jobs.list.next().await.flatten() });
        let mut ctx = Context {
            editor: &mut app.editor,
        };

        app.compositor
            .elements
            .iter_mut()
            .find(|element| {
                element.type_name() == std::any::type_name::<DebugElement<(bool, u128)>>()
            })
            .and_then(|mut_ref| {
                mut_ref
                    .as_any_mut()
                    .downcast_mut::<DebugElement<(bool, u128)>>()
            })
            .map(|element| element.text = (result.is_some(), dt));

        result.map(|callback| {
            callback(&mut ctx);
        });
        let _ = app
            .terminal
            .draw(|frame: &mut Frame| {
                app.compositor
                    .render(frame.area(), frame.buffer_mut(), &ctx)
            })
            .map_err(|e| eprintln!("{}", e));
        dt = now.elapsed().as_nanos();
        now = Instant::now();
    }
}
