use futures::{future::BoxFuture, stream::FuturesUnordered};

type JobFuture = BoxFuture<'static, Option<Callback>>;

type Callback = Box<dyn Fn(&mut Context)>;

struct Jobs {
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

struct App {
    jobs: Jobs,
}

#[cfg(test)]
mod tests {
    use smol::future::FutureExt;
    async fn async_fun(opt: Option<Callback>) -> Option<Callback> {
        opt
    }
    use crate::{Callback, Context, JobFuture, Jobs};
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
