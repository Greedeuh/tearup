use std::any::Any;

use crate::SharedContext;
#[cfg(feature = "async")]
pub use asyncc::*;

/// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
pub trait SimpleContext: Sized {
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup(shared_context: &mut SharedContext) -> Self
    where
        Self: Sized;

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(self, _shared_context: &mut SharedContext);

    fn launch_setup(shared_context: &mut SharedContext) -> Self {
        Self::setup(shared_context)
    }

    fn launch_test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
    where
        TestFn: FnOnce(),
        Self: Sized,
    {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(test))
    }

    fn launch_teardown(self, shared_context: &mut SharedContext) {
        self.teardown(shared_context);
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use futures::future::BoxFuture;
    pub use futures::future::FutureExt;
    use std::{any::Any, panic::AssertUnwindSafe};

    use crate::AsyncSharedContext;

    /// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
    #[async_trait]
    pub trait AsyncSimpleContext<'a>: Sync + Send + Sized {
        /// Will be executed before the test execution
        /// You should prepare all your test requirement here.
        /// Use the `ready` to notify that the test can start
        async fn setup(shared_context: AsyncSharedContext) -> Self
        where
            Self: Sized;

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(mut self, shared_context: AsyncSharedContext);

        async fn launch_setup(shared_context: AsyncSharedContext) -> Self
        where
            Self: Sized,
        {
            Self::setup(shared_context).await
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

        async fn launch_teardown(mut self, shared_context: AsyncSharedContext) {
            self.teardown(shared_context).await;
        }
    }
}
