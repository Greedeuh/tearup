use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::spawn,
};

use tearup::{ready_when, tearup, Context, ReadyChecksConfig, ReadyFn};

#[test]
fn it_notify_ready_when() {
    setup_almost_timeout()
}

struct SomeContext;
impl Context for SomeContext {
    fn ready_checks_config() -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self::ready_checks_config();
            let just_before_max = config.maximum - 1;

            let count = Arc::new(AtomicUsize::new(1));
            let predicate = move || count.fetch_add(1, Ordering::SeqCst) == just_before_max;

            ready_when(ready, Box::new(predicate), config.duration);
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

#[tearup(SomeContext)]
fn setup_almost_timeout() {}
