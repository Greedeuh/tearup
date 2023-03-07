use std::thread::spawn;
use tearup::{tearup, Context, ReadyChecksConfig, ReadyFn};

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
