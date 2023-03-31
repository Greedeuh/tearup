use anymap::AnyMap;
#[cfg(feature = "async")]
pub use asyncc::*;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
};
use stopwatch::Stopwatch;

use crate::{ready_state, ReadyChecksConfig, ReadyFn, SharedContext, SimpleContext};

/// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
pub trait WaitingContext: Sized {
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup(ready: ReadyFn) -> Self
    where
        Self: Sized;

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(self, ready: ReadyFn);

    /// Until the `setup` notify it's ready wait for a `duration` as many times needed with a `max`.
    /// So here we return this ReadyChecksConfig { duration: 100ms, max: 50} and can be overriden as you wish.
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms500()
    }

    fn public_context(&mut self) -> AnyMap {
        todo!()
    }
}

impl<T: WaitingContext> SimpleContext for T {
    fn setup(shared_context: &mut SharedContext) -> Self
    where
        Self: Sized,
    {
        let (ready_flag, ready) = ready_state();
        let context = Self::setup(ready);
        wait_setup(context.ready_checks_config(), ready_flag);
        context
    }

    fn teardown(self) {
        let conf = self.ready_checks_config();
        let (ready_flag, ready) = ready_state();
        self.teardown(ready);
        wait_setup(conf, ready_flag);
    }

    fn public_context(&mut self) -> AnyMap {
        self.public_context()
    }
}

fn wait_setup(ready_checks: ReadyChecksConfig, ready: Arc<Mutex<bool>>) {
    let maxium_duration = ready_checks.maxium_duration();

    let ready = || *ready.lock().unwrap();

    let stopwatch = Stopwatch::start_new();
    while !ready() {
        if stopwatch.elapsed() >= maxium_duration {
            panic!("Setup has timeout, make sure to call the 'ready' fn or raise up timeout.")
        }
        sleep(ready_checks.duration);
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use anymap::AnyMap;
    use async_trait::async_trait;
    pub use futures::future::FutureExt;
    use std::sync::{Arc, Mutex};
    use stopwatch::Stopwatch;
    use tokio::time::sleep;

    use crate::{ready_state, AsyncSimpleContext, ReadyChecksConfig, ReadyFn};

    /// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
    #[async_trait]
    pub trait AsyncWaitingContext<'a>: Sync + Send {
        /// Will be executed before the test execution
        /// You should prepare all your test requirement here.
        /// Use the `ready` to notify that the test can start
        async fn setup(ready: ReadyFn) -> Self
        where
            Self: Sized;

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(mut self, ready: ReadyFn);

        /// Until the `setup` notify it's ready wait for a `duration` as many times needed with a `max`.
        /// So here we return this ReadyChecksConfig { duration: 100ms, max: 50} and can be overriden as you wish.
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms500()
        }

        fn public_context(&mut self) -> AnyMap {
            AnyMap::new()
        }
    }

    #[async_trait]
    impl<'a, T: AsyncWaitingContext<'a>> AsyncSimpleContext<'a> for T {
        async fn setup() -> Self
        where
            Self: Sized,
        {
            let (ready_flag, ready) = ready_state();
            let context = Self::setup(ready).await;
            wait_setup(context.ready_checks_config(), ready_flag).await;
            context
        }

        async fn teardown(mut self) {
            let config = self.ready_checks_config();
            let (ready_flag, ready) = ready_state();
            self.teardown(ready).await;
            wait_setup(config, ready_flag).await;
        }

        fn public_context(&mut self) -> AnyMap {
            self.public_context()
        }
    }

    async fn wait_setup(ready_checks: ReadyChecksConfig, ready: Arc<Mutex<bool>>) {
        let maxium_duration = ready_checks.maxium_duration();

        let ready = || *ready.lock().unwrap();

        let stopwatch = Stopwatch::start_new();
        while !ready() {
            if stopwatch.elapsed() >= maxium_duration {
                panic!("Setup has timeout, make sure to pass the 'ready: Arc<Mutex<bool>>' to true")
            }
            sleep(ready_checks.duration).await;
        }
    }
}
