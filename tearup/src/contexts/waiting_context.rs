#[cfg(feature = "async")]
pub use asyncc::*;
use std::{
    any::Any,
    sync::{Arc, Mutex},
    thread::sleep,
};
use stopwatch::Stopwatch;

use crate::{ReadyChecksConfig, ReadyFn};

/// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
pub trait WaitingContext {
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup(ready: ReadyFn) -> Self
    where
        Self: Sized;

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(&mut self);

    /// Until the `setup` notify it's ready wait for a `duration` as many times needed with a `max`.
    /// So here we return this ReadyChecksConfig { duration: 100ms, max: 50} and can be overriden as you wish.
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms500()
    }

    fn wait_setup(&mut self, ready: Arc<Mutex<bool>>) {
        let ready_checks = self.ready_checks_config();
        let maxium_duration = ready_checks.maxium_duration();

        let ready = || *ready.lock().unwrap();

        let stopwatch = Stopwatch::start_new();
        while !ready() {
            if stopwatch.elapsed() >= maxium_duration {
                panic!("Setup has timeout, make sure to pass the 'ready: Arc<Mutex<bool>>' to true")
            }
            sleep(ready_checks.duration);
        }
    }

    /// Execute the test and catch panic
    fn test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
    where
        TestFn: FnOnce(),
        Self: Sized,
    {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(test))
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use futures::future::BoxFuture;
    pub use futures::future::FutureExt;
    use std::{
        any::Any,
        panic::AssertUnwindSafe,
        sync::{Arc, Mutex},
    };
    use stopwatch::Stopwatch;
    use tokio::time::sleep;

    use crate::{ReadyChecksConfig, ReadyFn};

    /// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
    #[async_trait]
    pub trait AsyncContext<'a>: Sync + Send {
        /// Will be executed before the test execution
        /// You should prepare all your test requirement here.
        /// Use the `ready` to notify that the test can start
        async fn setup(ready: ReadyFn) -> Self
        where
            Self: Sized;

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(&mut self);

        /// Until the `setup` notify it's ready wait for a `duration` as many times needed with a `max`.
        /// So here we return this ReadyChecksConfig { duration: 100ms, max: 50} and can be overriden as you wish.
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms500()
        }

        async fn wait_setup(&mut self, ready: Arc<Mutex<bool>>) {
            let ready_checks = self.ready_checks_config();
            let maxium_duration = ready_checks.maxium_duration();

            let ready = || *ready.lock().unwrap();

            let stopwatch = Stopwatch::start_new();
            while !ready() {
                if stopwatch.elapsed() >= maxium_duration {
                    panic!("Setup has timeout, make sure to pass the 'ready: Arc<Mutex<bool>>' to true")
                }
                sleep(ready_checks.duration).await;
            }
        }

        /// Actual test launch with panic catch
        async fn test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
        where
            TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
            Self: Sized,
        {
            AssertUnwindSafe(async move { test().await })
                .catch_unwind()
                .await
        }
    }
}
