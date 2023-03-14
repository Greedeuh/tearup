use std::sync::Arc;
pub use tearup_macro::{tearup, tearup_test};

use crate::{n_times, ready_state, Context, ReadyChecksConfig, ReadyFn};
#[cfg(feature = "async")]
pub use asyncc::*;

pub struct ContextCombinator<Context1: Context, Context2: Context> {
    context1: Context1,
    context2: Context2,
}

impl<Context1: Context, Context2: Context> Context for ContextCombinator<Context1, Context2> {
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup(both_ready: ReadyFn) -> Self {
        let splited_ready = Arc::new(n_times(both_ready, 2));

        let context1 = {
            let splited_ready = splited_ready.clone();
            Context1::setup(Box::new(move || splited_ready(0)))
        };
        let context2 = Context2::setup(Box::new(move || splited_ready(1)));

        Self { context1, context2 }
    }

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(&mut self) {
        self.context1.teardown();
        self.context2.teardown();
    }

    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::get_longest(vec![
            self.context1.ready_checks_config(),
            self.context2.ready_checks_config(),
        ])
    }
}

pub struct SequentialContextCombinator<Context1: Context, Context2: Context> {
    context1: Context1,
    context2: Context2,
}

impl<Context1: Context, Context2: Context> Context
    for SequentialContextCombinator<Context1, Context2>
{
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup(both_ready: ReadyFn) -> Self {
        let (ready_flag, ready) = ready_state();

        let mut context1 = Context1::setup(ready);
        context1.wait_setup(ready_flag);

        let context2 = Context2::setup(both_ready);

        Self { context1, context2 }
    }

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(&mut self) {
        self.context1.teardown();
        self.context2.teardown();
    }

    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::get_longest(vec![
            self.context1.ready_checks_config(),
            self.context2.ready_checks_config(),
        ])
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use std::sync::Arc;

    use async_trait::async_trait;
    pub use tearup_macro::{tearup, tearup_test};

    use crate::{n_times, AsyncContext, ReadyChecksConfig, ReadyFn};

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
            let splited_ready = Arc::new(n_times(both_ready, 2));
            let context1 = {
                let splited_ready = splited_ready.clone();
                Context1::setup(Box::new(move || splited_ready(0))).await
            };
            let context2 = Context2::setup(Box::new(move || splited_ready(1))).await;

            Self { context1, context2 }
        }

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(&mut self) {
            self.context1.teardown().await;
            self.context2.teardown().await;
        }

        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::get_longest(vec![
                self.context1.ready_checks_config(),
                self.context2.ready_checks_config(),
            ])
        }
    }
}
