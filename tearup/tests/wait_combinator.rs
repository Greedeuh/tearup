use std::sync::Arc;

use tearup::{tearup, Context, ContextCombinator};
mod helper;
use helper::SlowContext;

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}

struct SlowCombinedContext {
    contexts: Vec<Box<dyn Context>>,
}
impl ContextCombinator for SlowCombinedContext {
    fn contexts(&self) -> &Vec<Box<dyn Context>> {
        &self.contexts
    }

    fn contexts_mut(&mut self) -> &mut Vec<Box<dyn Context>> {
        &mut self.contexts
    }

    fn setup_all(splited_ready: tearup::SplitedReadyFn) -> Self {
        let splited_ready = Arc::new(splited_ready);

        let ready1 = {
            let splited_ready = splited_ready.clone();
            Box::new(move || splited_ready(0))
        };

        let ready2 = Box::new(move || splited_ready(1));

        Self {
            contexts: vec![
                Box::new(SlowContext::setup(ready1)),
                Box::new(SlowContext::setup(ready2)),
            ],
        }
    }

    fn size() -> u16 {
        2
    }
}

#[tearup(SlowCombinedContext)]
fn setup_almost_timeout() {}

#[cfg(feature = "async")]
mod asyncc {
    use tearup::{tearup, AsyncContext, AsyncContextCombinator};

    use crate::helper::AsyncSlowContext;

    #[tokio::test]
    async fn it_almost_timeout() {
        setup_almost_timeout().await
    }

    type A = AsyncContextCombinator<AsyncSlowContext, AsyncSlowContext>;
    #[tearup(A)]
    async fn setup_almost_timeout() {}
}
