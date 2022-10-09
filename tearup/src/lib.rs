pub use asyncc::*;
pub use syncc::*;
pub use tearup_macro::{tearup, tearup_test};

pub type ReadyFn = Box<dyn Fn() + Send + Sync>;

mod syncc {
    use core::time;
    use std::sync::{Arc, Mutex};

    use crate::ReadyFn;

    /// # Trait to implement to use the `#[tearup_test]` or `#[tearup]`
    pub trait Context {
        /// Will be before the test execution
        /// use the `ready` to notify that the test can start
        fn setup(ready: ReadyFn) -> Self;

        /// Until the `setup` notify it's ready we will wait but not more than the duration this function will return
        fn max_ready_checks(&self) -> usize {
            50
        }

        fn ready_checks_sleep_duration(&self) -> time::Duration {
            time::Duration::from_millis(100)
        }

        fn test<TestFn>(&mut self, test: TestFn, ready: Arc<Mutex<bool>>)
        where
            TestFn: FnOnce(),
        {
            let ready = || *ready.lock().unwrap();

            let mut checks_done = 0;
            while !ready() && checks_done < self.max_ready_checks() {
                checks_done += 1;
                if checks_done == self.max_ready_checks() {
                    panic!("Setup has timeout, make sure to pass the 'ready: Arc<Mutex<bool>>' to true")
                }
                std::thread::sleep(self.ready_checks_sleep_duration());
            }

            let may_panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(test));

            self.teardown();

            if let Err(err) = may_panicked {
                std::panic::resume_unwind(err)
            }
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

    #[async_trait]
    pub trait AsyncContext<'a>: Sync {
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
