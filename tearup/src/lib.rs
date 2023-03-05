pub use asyncc::*;
use core::time;
use std::default;
pub use syncc::*;
pub use tearup_macro::{tearup, tearup_test};

pub type ReadyFn = Box<dyn Fn() + Send + Sync>;

pub struct ReadyChecksConfig {
    duration: time::Duration,
    maximum: usize,
}

impl Default for ReadyChecksConfig {
    fn default() -> Self {
        Self {
            duration: time::Duration::from_millis(100),
            maximum: 50,
        }
    }
}

mod syncc {
    use std::{
        any::Any,
        sync::{Arc, Mutex},
    };

    use crate::{ReadyChecksConfig, ReadyFn};

    /// # Trait to implement to use the `#[tearup_test]` or `#[tearup]`
    pub trait Context {
        /// Will be before the test execution
        /// use the `ready` to notify that the test can start
        fn setup(ready: ReadyFn) -> Self;

        /// Until the `setup` notify it's ready wait for a `duration` as many times needed with a `max`.
        /// So here we return this ReadyChecksConfig { duration: 100ms, max: 50} and can be overriden as you wish.
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::default()
        }

        fn wait_setup(&mut self, ready: Arc<Mutex<bool>>) {
            let ready_checks = self.ready_checks_config();

            let ready = || *ready.lock().unwrap();

            let mut checks_done = 0;
            while !ready() && checks_done < ready_checks.maximum {
                checks_done += 1;
                if checks_done == ready_checks.maximum {
                    panic!("Setup has timeout, make sure to pass the 'ready: Arc<Mutex<bool>>' to true")
                }
                std::thread::sleep(ready_checks.duration);
            }
        }

        /// 1) Wait to be ready
        /// 2) Execute the test (catching the panic)
        /// 3) teardown and finally ends (with catched panic if some)
        fn test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
        where
            TestFn: FnOnce(),
        {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(test))
        }

        /// Do your clean up here, this methos will be executed after the test (passing or failing)
        fn teardown(&mut self);
    }

    pub trait FromContext<C: Context> {
        fn from_setup(context: &C) -> Self;
    }
}

mod asyncc {
    use async_trait::async_trait;
    use core::time;
    use futures::future::BoxFuture;
    pub use futures::future::FutureExt;
    use std::{
        panic::AssertUnwindSafe,
        sync::{Arc, Mutex},
    };
    use tokio::time::sleep;

    use crate::ReadyFn;

    /// # Trait to implement to use the `#[tearup_test]` or `#[tearup]`
    #[async_trait]
    pub trait AsyncContext<'a>: Sync {
        /// Will be before the test execution
        /// use the `ready` to notify that the test can start
        async fn setup(ready: ReadyFn) -> Self;
        async fn ready(&self) -> bool {
            true
        }
        fn sleep_duration(&self) -> time::Duration {
            time::Duration::from_millis(100)
        }
        fn max_ready_checks(&self) -> usize {
            50
        }
        async fn test<TestFn>(&mut self, test: TestFn, ready: Arc<Mutex<bool>>)
        where
            TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
        {
            let ready = || *ready.lock().unwrap();

            let mut checks_done = 0;
            while !ready() && checks_done < self.max_ready_checks() {
                checks_done += 1;
                if checks_done == self.max_ready_checks() {
                    panic!("Setup has timeout, make sure to pass the 'ready: Arc<Mutex<bool>>' to true")
                }
                sleep(self.sleep_duration()).await;
            }

            let may_panicked = AssertUnwindSafe(async move { test().await })
                .catch_unwind()
                .await;

            self.teardown().await;

            if let Err(err) = may_panicked {
                std::panic::resume_unwind(err)
            }
        }

        /// Do your clean up here, this methos will be executed after the test (passing or failing)
        async fn teardown(&mut self);
    }

    #[async_trait]
    pub trait FromAsyncContext<'a, C: AsyncContext<'a>> {
        async fn from_setup(context: &C) -> Self;
    }
}
