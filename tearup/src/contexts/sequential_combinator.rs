pub use tearup_macro::{tearup, tearup_test};

use crate::SimpleContext;
#[cfg(feature = "async")]
pub use asyncc::*;

pub struct SequentialContextCombinator<Context1: SimpleContext, Context2: SimpleContext> {
    context1: Context1,
    context2: Context2,
}

impl<Context1: SimpleContext, Context2: SimpleContext> SimpleContext
    for SequentialContextCombinator<Context1, Context2>
{
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup() -> Self {
        let context1 = Context1::launch_setup();
        let context2 = Context2::launch_setup();

        Self { context1, context2 }
    }

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(self) {
        self.context1.launch_teardown();
        self.context2.launch_teardown();
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    pub use tearup_macro::{tearup, tearup_test};

    use crate::AsyncSimpleContext;

    pub struct AsyncSequentialContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncSimpleContext<'a> + Send,
        for<'a> Context2: AsyncSimpleContext<'a> + Send,
    {
        context1: Context1,
        context2: Context2,
    }

    #[async_trait]
    impl<Context1, Context2> AsyncSimpleContext<'_>
        for AsyncSequentialContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncSimpleContext<'a> + Send,
        for<'a> Context2: AsyncSimpleContext<'a> + Send,
    {
        async fn setup() -> Self {
            let context1 = Context1::launch_setup().await;
            let context2 = Context2::launch_setup().await;
            Self { context1, context2 }
        }

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(mut self) {
            self.context1.teardown().await;
            self.context2.teardown().await;
        }
    }
}
