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
fn it_barely_timeout() {
    setup_barely_timeout()
}

struct TooSlowContext;
impl Context for TooSlowContext {
    fn ready_checks_config() -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self::ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_after_max);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

#[tearup(TooSlowContext)]
fn setup_barely_timeout() {}

#[test]
#[should_panic]
fn it_barely_timeout_with_ready_when() {
    setup_barely_timeout_with_ready_when()
}

struct TooSlowReadyWhenContext;
impl Context for TooSlowReadyWhenContext {
    fn ready_checks_config() -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self::ready_checks_config();
            let just_after_max = config.maximum + 1;

            let count = Arc::new(AtomicUsize::new(1));
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
    use tearup::{tearup, AsyncContext, ReadyChecksConfig, ReadyFn};
    use tokio::{spawn, time::sleep};

    #[tokio::test]
    #[should_panic]
    async fn it_barely_timeout() {
        setup_barely_timeout().await
    }

    struct TooSlowContext;
    #[async_trait]
    impl AsyncContext<'_> for TooSlowContext {
        fn ready_checks_config() -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self::ready_checks_config();
                let just_after_max = (config.maximum + 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
            Self {}
        }

        async fn teardown(&mut self) {}
    }

    #[tearup(TooSlowContext)]
    async fn setup_barely_timeout() {}
}
