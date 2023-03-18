use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::spawn,
};
use tearup::{ready_when, tearup, ReadyChecksConfig, ReadyFn, WaitingContext};

use crate::helper::assert_timeout_around_100ms;

#[test]
fn it_barely_timeout_with_ready_when() {
    assert_timeout_around_100ms(setup_barely_timeout_with_ready_when);
}

struct TooSlowReadyWhenContext;
impl WaitingContext for TooSlowReadyWhenContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = config.maximum + 1;

            let count = Arc::new(AtomicUsize::new(0));
            let predicate = move || count.fetch_add(1, Ordering::SeqCst) == just_after_max;

            ready_when(ready, Box::new(predicate), config.duration);
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

#[tearup(TooSlowReadyWhenContext)]
fn setup_barely_timeout_with_ready_when() {}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tearup::{
        async_ready_when, tearup, AsyncWaitingContext, FutureExt, ReadyChecksConfig, ReadyFn,
    };
    use tokio::spawn;

    use crate::helper::async_assert_timeout_around_100ms;

    #[tokio::test]
    async fn it_barely_timeout_with_ready_when() {
        async_assert_timeout_around_100ms(move || {
            { async move { setup_barely_timeout_with_ready_when().await } }.boxed()
        })
        .await;
    }

    struct TooSlowReadyWhenContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for TooSlowReadyWhenContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = config.maximum + 1;

                let count = Arc::new(AtomicUsize::new(1));

                let predicate = {
                    move || {
                        let count = Arc::clone(&count);
                        async move { count.fetch_add(1, Ordering::SeqCst) == just_after_max }
                            .boxed()
                    }
                };

                async_ready_when(ready, predicate, config.duration).await;
            });
            Self {}
        }

        async fn teardown(&mut self) {}
    }

    #[tearup(TooSlowReadyWhenContext)]
    async fn setup_barely_timeout_with_ready_when() {}
}
