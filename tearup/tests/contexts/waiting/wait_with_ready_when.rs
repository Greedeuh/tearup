use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{sleep, spawn},
    time::Duration,
};
use tearup::{ready_when, tearup, ReadyChecksConfig, ReadyFn, WaitingContext};

use crate::helper::assert_around_100ms;

#[test]
fn it_almost_timeout_with_ready_when() {
    assert_around_100ms(setup_almost_timeout_with_ready_when);
}

struct SlowReadyWhenContext;
impl WaitingContext for SlowReadyWhenContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_before_max = config.maximum - 1;

            sleep(Duration::from_millis(5));

            let count = Arc::new(AtomicUsize::new(1));
            let predicate = move || count.fetch_add(1, Ordering::SeqCst) == just_before_max;

            ready_when(ready, Box::new(predicate), config.duration);
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

#[tearup(SlowReadyWhenContext)]
fn setup_almost_timeout_with_ready_when() {}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::Duration,
    };
    use tearup::{
        async_ready_when, tearup, AsyncWaitingContext, FutureExt, ReadyChecksConfig, ReadyFn,
    };
    use tokio::{spawn, time::sleep};

    use crate::helper::async_assert_around_100ms;

    #[tokio::test]
    async fn it_almost_timeout_with_ready_when() {
        async_assert_around_100ms(move || {
            { async move { setup_almost_timeout_with_ready_when().await } }.boxed()
        })
        .await;
    }

    struct SlowReadyWhenContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for SlowReadyWhenContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_before_max = config.maximum - 1;

                let count = Arc::new(AtomicUsize::new(1));

                sleep(Duration::from_millis(5)).await;

                let predicate = {
                    move || {
                        let count = Arc::clone(&count);
                        async move { count.fetch_add(1, Ordering::SeqCst) == just_before_max }
                            .boxed()
                    }
                };

                async_ready_when(ready, predicate, config.duration).await;
            });
            Self {}
        }

        async fn teardown(&mut self) {}
    }

    #[tearup(SlowReadyWhenContext)]
    async fn setup_almost_timeout_with_ready_when() {}
}
