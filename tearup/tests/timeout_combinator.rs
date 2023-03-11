use std::sync::Arc;

use tearup::{tearup, Context, ContextCombinator};
mod helper;
use helper::TooSlowContext;

use crate::helper::InstantContext;

#[test]
#[should_panic]
fn it_barely_timeout() {
    setup_barely_timeout()
}

struct TooSlowCombinedContext {
    contexts: Vec<Box<dyn Context>>,
}
impl ContextCombinator for TooSlowCombinedContext {
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
                Box::new(InstantContext::setup(ready1)),
                Box::new(TooSlowContext::setup(ready2)),
            ],
        }
    }

    fn size() -> u16 {
        2
    }
}

#[tearup(TooSlowCombinedContext)]
fn setup_barely_timeout() {}

struct TooSlowCombinedContextReversed {
    contexts: Vec<Box<dyn Context>>,
}
impl ContextCombinator for TooSlowCombinedContextReversed {
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
                Box::new(TooSlowContext::setup(ready1)),
                Box::new(InstantContext::setup(ready2)),
            ],
        }
    }

    fn size() -> u16 {
        2
    }
}

#[tearup(TooSlowCombinedContextReversed)]
fn setup_barely_timeout_reversed() {}
