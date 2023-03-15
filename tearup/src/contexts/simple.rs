use std::any::Any;

#[cfg(feature = "async")]
pub use asyncc::*;

/// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
pub trait SimpleContext: Sized {
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup() -> Self
    where
        Self: Sized;

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(&mut self);

    fn launch_setup() -> Self {
        Self::setup()
    }

    fn launch_test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
    where
        TestFn: FnOnce(),
        Self: Sized,
    {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(test))
    }

    fn launch_teardown(&mut self) {
        self.teardown();
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use std::{any::Any, panic::AssertUnwindSafe};

    use async_trait::async_trait;
    use futures::future::BoxFuture;
    pub use futures::future::FutureExt;

    /// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
    #[async_trait]
    pub trait AsyncSimpleContext<'a>: Sync + Send + Sized {
        /// Will be executed before the test execution
        /// You should prepare all your test requirement here.
        /// Use the `ready` to notify that the test can start
        async fn setup() -> Self
        where
            Self: Sized;

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(&mut self);

        async fn launch_setup() -> Self
        where
            Self: Sized,
        {
            Self::setup().await
        }

        async fn launch_test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
        where
            TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
            Self: Sized,
        {
            AssertUnwindSafe(async move { test().await })
                .catch_unwind()
                .await
        }

        async fn launch_teardown(&mut self) {
            self.teardown().await;
        }
    }
}
