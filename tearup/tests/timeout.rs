use std::thread::spawn;

use tearup::{tearup, Context, ReadyFn};

struct TimeoutContext;
impl Context for TimeoutContext {
    fn setup(_: ReadyFn) -> Self {
        Self {}
    }
    fn teardown(&mut self) {}
}

#[tearup(TimeoutContext)]
fn setup_timeout() {
    TimeoutContext::setup(Box::new(|| ()));
}

#[test]
#[should_panic]
fn it_timeout() {
    setup_timeout()
}

struct TooSlowContext;
impl Context for TooSlowContext {
    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self::ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_after_max);

            ready()
        });
        Self {}
    }
    fn teardown(&mut self) {}
}

#[tearup(TooSlowContext)]
fn setup_barely_timeout() {
    TimeoutContext::setup(Box::new(|| ()));
}

#[test]
#[should_panic]
fn it_barely_timeout() {
    setup_barely_timeout()
}

struct SlowContext;
impl Context for SlowContext {
    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self::ready_checks_config();
            let just_before_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_before_max);

            ready()
        });
        Self {}
    }
    fn teardown(&mut self) {}
}

#[tearup(SlowContext)]
fn setup_almost_timeout() {
    TimeoutContext::setup(Box::new(|| ()));
}

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}
