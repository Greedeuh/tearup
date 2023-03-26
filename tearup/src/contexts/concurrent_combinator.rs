use std::thread::spawn;

pub use tearup_macro::{tearup, tearup_test};

use crate::SimpleContext;
#[cfg(feature = "async")]
pub use asyncc::*;

pub struct ConcurrentContextCombinator<Context1: SimpleContext, Context2: SimpleContext> {
    context1: Context1,
    context2: Context2,
}

impl<Context1: SimpleContext + Send + 'static, Context2: SimpleContext> SimpleContext
    for ConcurrentContextCombinator<Context1, Context2>
{
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup() -> Self {
        let context1_handle = spawn(|| Context1::launch_setup());

        let context2 = Context2::launch_setup();
        let context1 = context1_handle.join().unwrap();

        Self { context1, context2 }
    }

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(self) {
        let context1 = self.context1;

        let context1_handle = spawn(|| context1.launch_teardown());
        self.context2.launch_teardown();

        context1_handle.join().unwrap();
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use futures::join;
    pub use tearup_macro::{tearup, tearup_test};

    use crate::AsyncSimpleContext;

    pub struct AsyncConcurrentContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncSimpleContext<'a> + Send,
        for<'a> Context2: AsyncSimpleContext<'a> + Send,
    {
        context1: Context1,
        context2: Context2,
    }

    #[async_trait]
    impl<Context1, Context2> AsyncSimpleContext<'_>
        for AsyncConcurrentContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncSimpleContext<'a> + Send,
        for<'a> Context2: AsyncSimpleContext<'a> + Send,
    {
        async fn setup() -> Self {
            let (context1, context2) = join!(Context1::launch_setup(), Context2::launch_setup());
            Self { context1, context2 }
        }

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(mut self) {
            join!(self.context1.teardown(), self.context2.teardown());
        }
    }
}
