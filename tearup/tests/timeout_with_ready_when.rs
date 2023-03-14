use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::spawn,
};
use tearup::{ready_when, tearup, Context, ReadyChecksConfig, ReadyFn};

#[test]
#[should_panic]
fn it_barely_timeout_with_ready_when() {
    setup_barely_timeout_with_ready_when()
}

struct TooSlowReadyWhenContext;
impl Context for TooSlowReadyWhenContext {
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
    use tearup::{async_ready_when, tearup, AsyncContext, FutureExt, ReadyChecksConfig, ReadyFn};
    use tokio::spawn;

    #[tokio::test]
    #[should_panic]
    async fn it_barely_timeout_with_ready_when() {
        setup_barely_timeout_with_ready_when().await
    }

    struct TooSlowReadyWhenContext;
    #[async_trait]
    impl AsyncContext<'_> for TooSlowReadyWhenContext {
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
