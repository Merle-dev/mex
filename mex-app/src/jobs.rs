use futures::{future::BoxFuture, stream::FuturesUnordered};

use crate::Context;

type JobFuture = BoxFuture<'static, Option<Callback>>;

pub type Callback = Box<dyn FnOnce(&mut Context) + Send>;
pub type Job = BoxFuture<'static, Option<Callback>>;

pub struct Jobs {
    threads: FuturesUnordered<JobFuture>,
}

impl Jobs {
    pub fn new() -> Self {
        Self {
            threads: FuturesUnordered::new(),
        }
    }
    pub fn spawn(&mut self, func: JobFuture) {
        self.threads.push(func);
    }
}

#[cfg(test)]
mod tests {
    use smol::future::FutureExt;
    async fn async_fun(opt: Option<Callback>) -> Option<Callback> {
        opt
    }
    use crate::jobs::{Callback, Jobs};
    fn create<F: Future<Output = Option<Callback>> + Send + 'static>(f: F) {
        let mut jobs = Jobs::new();
        jobs.spawn(f.boxed());
    }
    fn poll() {
        loop {
            let mut jobs = Jobs::new();
            jobs.spawn(async_fun(None).boxed());
        }
    }
}
