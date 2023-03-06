use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::spawn,
};

use tearup::{ready_when, tearup, Context, ReadyChecksConfig, ReadyFn};

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}

struct SlowContext;
impl Context for SlowContext {
    fn ready_checks_config() -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self::ready_checks_config();
            let just_before_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_before_max);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

#[tearup(SlowContext)]
fn setup_almost_timeout() {}

#[test]
fn it_almost_timeout_with_ready_when() {
    setup_almost_timeout_with_ready_when()
}

struct SlowReadyWhenContext;
impl Context for SlowReadyWhenContext {
    fn ready_checks_config() -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self::ready_checks_config();
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
    use tearup::{tearup, AsyncContext, ReadyChecksConfig, ReadyFn};
    use tokio::{spawn, time::sleep};

    #[tokio::test]
    async fn it_almost_timeout() {
        setup_almost_timeout().await
    }

    struct SlowContext;
    #[async_trait]
    impl AsyncContext<'_> for SlowContext {
        fn ready_checks_config() -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self::ready_checks_config();
                let just_after_max = (config.maximum - 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
            Self {}
        }

        async fn teardown(&mut self) {}
    }

    #[tearup(SlowContext)]
    async fn setup_almost_timeout() {}
}
