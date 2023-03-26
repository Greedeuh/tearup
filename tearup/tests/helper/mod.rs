pub use asyncc::*;
use futures::future::BoxFuture;
use std::{sync::Mutex, thread::spawn, time::SystemTime};
use stopwatch::Stopwatch;
use tearup::{FutureExt, ReadyChecksConfig, ReadyFn, WaitingContext};

pub type Checkpoint = Mutex<Option<SystemTime>>;
pub type AsyncCheckpoint = tokio::sync::Mutex<Option<SystemTime>>;

pub fn assert_order(checkpoint_before: &Checkpoint, checkpoint_after: &Checkpoint) {
    assert!(checkpoint_before.lock().unwrap().unwrap() < checkpoint_after.lock().unwrap().unwrap());
}

pub async fn assert_async_order(
    checkpoint_before: &AsyncCheckpoint,
    checkpoint_after: &AsyncCheckpoint,
) {
    assert!(checkpoint_before.lock().await.unwrap() < checkpoint_after.lock().await.unwrap());
}

#[allow(dead_code)]
pub fn assert_around_100ms<TestFn>(test: TestFn)
where
    TestFn: FnOnce(),
{
    let stopwatch = Stopwatch::start_new();

    test();

    assert_around_100ms_(&stopwatch);
}

#[allow(dead_code)]
pub async fn async_assert_around_100ms<'a, TestFn>(test: TestFn)
where
    TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
{
    let stopwatch = Stopwatch::start_new();

    test().await;

    assert_around_100ms_(&stopwatch);
}

#[allow(dead_code)]
pub fn assert_timeout_around_100ms<TestFn>(test: TestFn)
where
    TestFn: FnOnce(),
{
    let stopwatch = Stopwatch::start_new();

    let test_execution = std::panic::catch_unwind(std::panic::AssertUnwindSafe(test));
    assert!(test_execution.is_err());

    assert_around_100ms_(&stopwatch);
}

#[allow(dead_code)]
pub async fn async_assert_timeout_around_100ms<'a, TestFn>(test: TestFn)
where
    TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
{
    let stopwatch = Stopwatch::start_new();

    let test_execution = std::panic::AssertUnwindSafe(async move { test().await })
        .catch_unwind()
        .await;
    assert!(test_execution.is_err());

    assert_around_100ms_(&stopwatch);
}

fn assert_around_100ms_(stopwatch: &Stopwatch) {
    let ms = stopwatch.elapsed_ms();
    assert!(115 > ms, "stopwatch has {} elapsed ms > 115", ms);
    assert!(ms > 85, "stopwatch has {} elapsed ms < 85", ms);
}

pub struct InstantContext;
impl WaitingContext for InstantContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(&mut self, ready: ReadyFn) {
        ready();
    }
}

pub struct TooSlowContext;
impl WaitingContext for TooSlowContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_after_max);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self, ready: ReadyFn) {
        ready();
    }
}

pub struct TooSlowTeardown;
impl WaitingContext for TooSlowTeardown {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(&mut self, ready: ReadyFn) {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_after_max);

            ready();
        });
    }
}

pub struct SlowContext;
impl WaitingContext for SlowContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_before_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_before_max);

            ready()
        });

        Self {}
    }

    fn teardown(&mut self, ready: ReadyFn) {
        ready();
    }
}

pub struct HalfPlus1Context;
impl WaitingContext for HalfPlus1Context {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep((config.duration * just_after_max) / 2);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self, ready: ReadyFn) {
        ready();
    }
}

pub struct HalfMinus1Context;
impl WaitingContext for HalfMinus1Context {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep((config.duration * just_after_max) / 2);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self, ready: ReadyFn) {
        ready();
    }
}

#[cfg(feature = "async")]
pub mod asyncc {
    use async_trait::async_trait;
    use tearup::{AsyncWaitingContext, ReadyChecksConfig, ReadyFn};
    use tokio::{spawn, time::sleep};

    pub struct AsyncInstantContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncInstantContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            ready();
            Self {}
        }

        async fn teardown(&mut self, ready: ReadyFn) {
            ready()
        }
    }

    pub struct AsyncTooSlowContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncTooSlowContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum + 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
            Self {}
        }

        async fn teardown(&mut self, ready: ReadyFn) {
            ready();
        }
    }

    pub struct AsyncTooSlowTeardown;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncTooSlowTeardown {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            ready();
            Self {}
        }

        async fn teardown(&mut self, ready: ReadyFn) {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum + 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
        }
    }

    pub struct AsyncSlowContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncSlowContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum - 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
            Self {}
        }

        async fn teardown(&mut self, ready: ReadyFn) {
            ready();
        }
    }

    pub struct AsyncHalfPlus1Context;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncHalfPlus1Context {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum - 1).try_into().unwrap();

                sleep(config.duration * just_after_max / 2).await;

                ready();
            });
            Self {}
        }

        async fn teardown(&mut self, ready: ReadyFn) {
            ready();
        }
    }

    pub struct AsyncHalfMinus1Context;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncHalfMinus1Context {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }
        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum - 1).try_into().unwrap();

                sleep(config.duration * just_after_max / 2).await;

                ready();
            });
            Self {}
        }

        async fn teardown(&mut self, ready: ReadyFn) {
            ready();
        }
    }
}
