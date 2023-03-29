use anymap::AnyMap;
use std::thread::spawn;
use tearup::{ReadyChecksConfig, ReadyFn, SimpleContext, WaitingContext};

mod asserts;
pub use asserts::*;
pub use asyncc::*;

pub struct FirstProof(pub String);
pub struct SecondProof(pub String);

pub struct FirstFullContext;
impl SimpleContext for FirstFullContext {
    fn setup() -> Self {
        Self {}
    }

    fn teardown(self) {}

    fn public_context(&mut self) -> AnyMap {
        let mut public_context = AnyMap::new();
        public_context.insert(FirstProof("first_proof".to_owned()));
        public_context.insert(SecondProof("second_proof".to_owned()));
        public_context
    }
}

pub struct ThirdProof(pub String);
pub struct FourthProof(pub String);

pub struct SecondFullContext;
impl SimpleContext for SecondFullContext {
    fn setup() -> Self {
        Self {}
    }

    fn teardown(self) {}

    fn public_context(&mut self) -> AnyMap {
        let mut public_context = AnyMap::new();
        public_context.insert(ThirdProof("third_proof".to_owned()));
        public_context.insert(FourthProof("fourth_proof".to_owned()));
        public_context
    }
}

pub struct InstantContext;
impl WaitingContext for InstantContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(self, ready: ReadyFn) {
        ready();
    }
}

pub struct TooSlowSetup;
impl WaitingContext for TooSlowSetup {
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

    fn teardown(self, ready: ReadyFn) {
        ready();
    }
}

pub struct TooSlowTeardown;
impl WaitingContext for TooSlowTeardown {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(self, ready: ReadyFn) {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_after_max);

            ready();
        });
    }
}

pub struct SlowSetup;
impl WaitingContext for SlowSetup {
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

    fn teardown(self, ready: ReadyFn) {
        ready();
    }
}

pub struct SlowTeardown;
impl WaitingContext for SlowTeardown {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(self, ready: ReadyFn) {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_before_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep(config.duration * just_before_max);

            ready()
        });
    }
}

pub struct HalfPlus1Setup;
impl WaitingContext for HalfPlus1Setup {
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

    fn teardown(self, ready: ReadyFn) {
        ready();
    }
}

pub struct HalfPlus1Teardown;
impl WaitingContext for HalfPlus1Teardown {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(self, ready: ReadyFn) {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum + 1).try_into().unwrap();

            std::thread::sleep((config.duration * just_after_max) / 2);

            ready()
        });
    }
}

pub struct HalfMinus1Setup;
impl WaitingContext for HalfMinus1Setup {
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

    fn teardown(self, ready: ReadyFn) {
        ready();
    }
}

pub struct HalfMinus1Teardown;
impl WaitingContext for HalfMinus1Teardown {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(self, ready: ReadyFn) {
        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep((config.duration * just_after_max) / 2);

            ready()
        });
    }
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

        async fn teardown(self, ready: ReadyFn) {
            ready()
        }
    }

    pub struct AsyncTooSlowSetup;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncTooSlowSetup {
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

        async fn teardown(self, ready: ReadyFn) {
            ready();
        }
    }

    pub struct AsyncTooSlowTeardown;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncTooSlowTeardown {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            ready();
            Self {}
        }

        async fn teardown(self, ready: ReadyFn) {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum + 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
        }
    }

    pub struct AsyncSlowSetup;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncSlowSetup {
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

        async fn teardown(self, ready: ReadyFn) {
            ready();
        }
    }

    pub struct AsyncSlowTeardown;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncSlowTeardown {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            ready();
            Self {}
        }

        async fn teardown(self, ready: ReadyFn) {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum - 1).try_into().unwrap();

                sleep(config.duration * just_after_max).await;

                ready();
            });
        }
    }

    pub struct AsyncHalfPlus1Setup;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncHalfPlus1Setup {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum - 1).try_into().unwrap();

                sleep(config.duration * just_after_max / 2).await;

                ready();
            });
            Self {}
        }

        async fn teardown(self, ready: ReadyFn) {
            ready();
        }
    }

    pub struct AsyncHalfMinus1Setup;
    #[async_trait]
    impl AsyncWaitingContext<'_> for AsyncHalfMinus1Setup {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }
        async fn setup(ready: ReadyFn) -> Self {
            spawn(async move {
                let config = Self {}.ready_checks_config();
                let just_after_max = (config.maximum - 1).try_into().unwrap();

                sleep(config.duration * just_after_max / 2).await;

                ready();
            });
            Self {}
        }

        async fn teardown(self, ready: ReadyFn) {
            ready();
        }
    }
}
