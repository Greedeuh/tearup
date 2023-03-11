pub use tearup_macro::{tearup, tearup_test};

use crate::{get_longest, split, Context, ReadyChecksConfig, ReadyFn};
#[cfg(feature = "async")]
pub use asyncc::*;

pub struct ContextCombinator<Context1: Context, Context2: Context> {
    context1: Context1,
    context2: Context2,
}

impl<Context1: Context, Context2: Context> Context for ContextCombinator<Context1, Context2> {
    fn setup(both_ready: ReadyFn) -> Self {
        let (ready1, ready2) = split(both_ready);

        let context1 = Context1::setup(ready1);
        let context2 = Context2::setup(ready2);

        Self { context1, context2 }
    }

    fn teardown(&mut self) {
        self.context1.teardown();
        self.context2.teardown();
    }

    fn ready_checks_config(&self) -> ReadyChecksConfig {
        get_longest(
            self.context1.ready_checks_config(),
            self.context2.ready_checks_config(),
        )
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    pub use tearup_macro::{tearup, tearup_test};

    use crate::{
        combinator::{get_longest, split},
        AsyncContext, ReadyChecksConfig, ReadyFn,
    };

    pub struct AsyncContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncContext<'a> + Send,
        for<'a> Context2: AsyncContext<'a> + Send,
    {
        context1: Context1,
        context2: Context2,
    }

    #[async_trait]
    impl<'b, Context1, Context2> AsyncContext<'b> for AsyncContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncContext<'a> + Send,
        for<'a> Context2: AsyncContext<'a> + Send,
    {
        async fn setup(both_ready: ReadyFn) -> Self {
            let (ready1, ready2) = split(both_ready);
            let context1 = Context1::setup(ready1).await;
            let context2 = Context2::setup(ready2).await;

            Self { context1, context2 }
        }

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(&mut self) {
            self.context1.teardown().await;
            self.context2.teardown().await;
        }

        fn ready_checks_config(&self) -> ReadyChecksConfig {
            get_longest(
                self.context1.ready_checks_config(),
                self.context2.ready_checks_config(),
            )
        }
    }
}
