pub use asyncc::*;
use std::thread::spawn;
use tearup::{Context, ReadyChecksConfig, ReadyFn};

pub struct InstantContext;
impl Context for InstantContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(&mut self) {}
}

pub struct TooSlowContext;
impl Context for TooSlowContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_after_max);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

#[cfg(feature = "async")]
pub mod asyncc {
    use async_trait::async_trait;
    use tearup::{AsyncContext, ReadyChecksConfig, ReadyFn};
    use tokio::{spawn, time::sleep};

    pub struct AsyncTooSlowContext;
    #[async_trait]
    impl AsyncContext<'_> for AsyncTooSlowContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum + 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
            Self {}
        }

        async fn teardown(&mut self) {}
    }
}
