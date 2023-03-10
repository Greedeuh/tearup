#[cfg(feature = "async")]
pub use asyncc::*;
pub use tearup_macro::{tearup, tearup_test};

mod combinator;
pub use combinator::*;
mod ready;
pub use ready::*;
use std::{
    any::Any,
    sync::{Arc, Mutex},
    thread::sleep,
};

/// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
pub trait Context {
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

        let ready = || *ready.lock().unwrap();

        let mut checks_done = 0;
        while !ready() && checks_done <= ready_checks.maximum {
            checks_done += 1;
            if checks_done == ready_checks.maximum {
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

/// Trait to implement if you need to access a setup value in you test.
pub trait FromContext<C: Context> {
    fn from_context(context: &C) -> Self;
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

            let ready = || *ready.lock().unwrap();

            let mut checks_done = 0;
            while !ready() && checks_done < ready_checks.maximum {
                checks_done += 1;
                if checks_done == ready_checks.maximum {
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

    /// Trait to implement if you need to access a setup value in you test.
    #[async_trait]
    pub trait FromAsyncContext<'a, C: AsyncContext<'a>> {
        async fn from_context(context: &C) -> Self;
    }
}
