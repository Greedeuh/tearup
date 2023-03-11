pub use tearup_macro::{tearup, tearup_test};

use crate::{n_times, Context, ReadyChecksConfig, ReadyFn, SplitedReadyFn};
#[cfg(feature = "async")]
pub use asyncc::*;

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

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    pub use tearup_macro::{tearup, tearup_test};

    use crate::{n_times, AsyncContext, ReadyChecksConfig, ReadyFn, SplitedReadyFn};

    #[async_trait]
    pub trait AsyncContextCombinator: Sync + Send {
        fn contexts(&self) -> &Vec<Box<dyn AsyncContext>>;
        fn contexts_mut(&mut self) -> &mut Vec<Box<dyn AsyncContext>>;
        async fn setup_all(splited_ready: SplitedReadyFn) -> Self;
        fn size() -> u16;
    }

    #[async_trait]
    impl<Combinator: AsyncContextCombinator> AsyncContext<'_> for Combinator {
        async fn setup(all_ready: ReadyFn) -> Self
        where
            Self: Sized,
        {
            let splited_ready = n_times(all_ready, Self::size());
            Self::setup_all(splited_ready).await
        }

        async fn teardown(&mut self) {
            for context in self.contexts_mut().iter_mut() {
                context.teardown().await;
            }
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
}
