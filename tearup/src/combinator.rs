use std::sync::Arc;
pub use tearup_macro::{tearup, tearup_test};

use crate::{Context, ReadyChecksConfig, ReadyFn};
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

fn split(both_ready: ReadyFn) -> (ReadyFn, ReadyFn) {
    let both_ready = Arc::new(both_ready);

    let ready_flag1 = Arc::new(std::sync::Mutex::new(false));
    let ready_flag2 = Arc::new(std::sync::Mutex::new(false));

    let ready1 = {
        let ready_flag1 = ready_flag1.clone();
        let ready_flag2 = ready_flag2.clone();
        let both_ready = both_ready.clone();

        Box::new(move || {
            let mut ready1 = ready_flag1.lock().unwrap();
            let ready2 = ready_flag2.lock().unwrap();
            *ready1 = true;
            if *ready2 {
                both_ready();
            }
        })
    };

    let ready2 = Box::new(move || {
        let ready1 = ready_flag1.lock().unwrap();
        let mut ready2 = ready_flag2.lock().unwrap();
        *ready2 = true;
        if *ready1 {
            both_ready();
        }
    });
    (ready1, ready2)
}

fn get_longest(config1: ReadyChecksConfig, config2: ReadyChecksConfig) -> ReadyChecksConfig {
    let duration1 = config1.duration * config1.maximum.try_into().unwrap();

    let duration2 = config1.duration * config2.maximum.try_into().unwrap();

    if duration1 > duration2 {
        config1
    } else {
        config2
    }
}
