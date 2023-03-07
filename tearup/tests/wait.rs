use std::thread::spawn;
use tearup::{tearup, Context, ReadyChecksConfig, ReadyFn};

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
