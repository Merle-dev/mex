use futures::{
    StreamExt,
    future::{BoxFuture, FutureExt},
    stream::FuturesUnordered,
};

#[derive(Debug)]
enum Mode {
    Normal,
    Insert,
    Select,
}

struct Context {
    mode: Mode,
}

type Callback = Box<dyn FnOnce(&mut Context) + Send>;
type Job = BoxFuture<'static, Option<Callback>>;

struct Jobs {
    list: FuturesUnordered<Job>,
}

fn main() {
    let mut jobs = Jobs {
        list: FuturesUnordered::new(),
    };
    let call = Box::pin(async {
        let cb: Callback = Box::new(|ctx: &mut Context| println!("last"));
        Some(cb)
    });
    jobs.list.push(call);
    let mut cx = Context { mode: Mode::Normal };
    while let Some(callback) = smol::block_on(async { jobs.list.next().await.flatten() }) {
        callback(&mut cx);
    }
}
