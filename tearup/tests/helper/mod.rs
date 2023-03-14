pub use asyncc::*;
use std::thread::spawn;
use tearup::{ReadyChecksConfig, ReadyFn, WaitingContext};

pub struct InstantContext;
impl WaitingContext for InstantContext {
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
impl WaitingContext for TooSlowContext {
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

pub struct SlowContext;
impl WaitingContext for SlowContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_before_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_before_max);

            ready()
        });

        Self {}
    }

    fn teardown(&mut self) {}
}

pub struct HalfPlus1Context;
impl WaitingContext for HalfPlus1Context {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep((config.duration * just_after_max) / 2);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

pub struct HalfMinus1Context;
impl WaitingContext for HalfMinus1Context {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep((config.duration * just_after_max) / 2);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

#[cfg(feature = "async")]
pub mod asyncc {
    use async_trait::async_trait;
    use tearup::{AsyncWaitingContext, ReadyChecksConfig, ReadyFn};
    use tokio::{spawn, time::sleep};

    pub struct AsyncInstantContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncInstantContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            ready();
            Self {}
        }

        async fn teardown(&mut self) {}
    }

    pub struct AsyncTooSlowContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncTooSlowContext {
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

    pub struct AsyncSlowContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncSlowContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum - 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
            Self {}
        }

        async fn teardown(&mut self) {}
    }
}
