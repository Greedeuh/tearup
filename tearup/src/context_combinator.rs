pub use tearup_macro::{tearup, tearup_test};

use crate::{SharedContext, SimpleContext};
#[cfg(feature = "async")]
pub use asyncc::*;

pub struct ContextCombinator<Context1: SimpleContext, Context2: SimpleContext> {
    context1: Context1,
    context2: Context2,
}

impl<Context1: SimpleContext, Context2: SimpleContext> SimpleContext
    for ContextCombinator<Context1, Context2>
{
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup(shared_context: &mut SharedContext) -> Self {
        let context1 = Context1::launch_setup(shared_context);
        let context2 = Context2::launch_setup(shared_context);

        Self { context1, context2 }
    }

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(self, shared_context: &mut SharedContext) {
        self.context1.launch_teardown(shared_context);
        self.context2.launch_teardown(shared_context);
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    pub use tearup_macro::{tearup, tearup_test};

    use crate::{AsyncSharedContext, AsyncSimpleContext};

    pub struct AsyncContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncSimpleContext<'a> + Send,
        for<'a> Context2: AsyncSimpleContext<'a> + Send,
    {
        context1: Context1,
        context2: Context2,
    }

    #[async_trait]
    impl<Context1, Context2> AsyncSimpleContext<'_> for AsyncContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncSimpleContext<'a> + Send,
        for<'a> Context2: AsyncSimpleContext<'a> + Send,
    {
        async fn setup(shared_context: AsyncSharedContext) -> Self {
            let context1 = Context1::launch_setup(shared_context.clone()).await;
            let context2 = Context2::launch_setup(shared_context).await;
            Self { context1, context2 }
        }

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(mut self, shared_context: AsyncSharedContext) {
            self.context1.teardown(shared_context.clone()).await;
            self.context2.teardown(shared_context).await;
        }
    }
}
