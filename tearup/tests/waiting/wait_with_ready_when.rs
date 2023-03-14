use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::spawn,
};
use tearup::{ready_when, tearup, ReadyChecksConfig, ReadyFn, WaitingContext};

#[test]
fn it_almost_timeout_with_ready_when() {
    setup_almost_timeout_with_ready_when()
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
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tearup::{
        async_ready_when, tearup, AsyncWaitingContext, FutureExt, ReadyChecksConfig, ReadyFn,
    };
    use tokio::spawn;

    #[tokio::test]
    #[should_panic]
    async fn it_almost_timeout_with_ready_when() {
        setup_almost_timeout_with_ready_when().await
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

    #[tearup(SlowReadyWhenContext)]
    async fn setup_almost_timeout_with_ready_when() {}
}
