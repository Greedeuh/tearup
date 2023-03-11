pub use tearup_macro::{tearup, tearup_test};

use crate::{get_longest, n_times, split, Context, ReadyChecksConfig, ReadyFn, SplitedReadyFn};
#[cfg(feature = "async")]
pub use asyncc::*;

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

pub trait ContextCombinator {
    fn contexts(&self) -> &Vec<Box<dyn Context>>;
    fn contexts_mut(&mut self) -> &mut Vec<Box<dyn Context>>;
    fn setup_all(splited_ready: SplitedReadyFn) -> Self;
    fn size() -> u16;
}

impl<Combinator: ContextCombinator> Context for Combinator {
    fn setup(all_ready: ReadyFn) -> Self
    where
        Self: Sized,
    {
        let splited_ready = n_times(all_ready, Self::size());
        Self::setup_all(splited_ready)
    }

    fn teardown(&mut self) {
        self.contexts_mut()
            .iter_mut()
            .for_each(|context| context.teardown());
    }

    fn ready_checks_config(&self) -> ReadyChecksConfig {
        let configs = self
            .contexts()
            .iter()
            .map(|c| c.ready_checks_config())
            .collect();

        ReadyChecksConfig::get_longest(configs)
    }
}
