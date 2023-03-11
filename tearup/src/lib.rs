#[cfg(feature = "async")]
pub use asyncc::*;
use core::time::Duration;
pub use tearup_macro::{tearup, tearup_test};

use std::{
    any::Any,
    sync::{Arc, Mutex},
    thread::sleep,
};

/// Trait to implement to use the `#[tearup_test]` or `#[tearup]`
pub trait Context {
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup(ready: ReadyFn) -> Self;

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(&mut self);

    /// Until the `setup` notify it's ready wait for a `duration` as many times needed with a `max`.
    /// So here we return this ReadyChecksConfig { duration: 100ms, max: 50} and can be overriden as you wish.
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms500()
    }

    fn wait_setup(&mut self, ready: Arc<Mutex<bool>>) {
        let ready_checks = self.ready_checks_config();

        let ready = || *ready.lock().unwrap();

        let mut checks_done = 0;
        while !ready() && checks_done <= ready_checks.maximum {
            checks_done += 1;
            if checks_done == ready_checks.maximum {
                panic!("Setup has timeout, make sure to pass the 'ready: Arc<Mutex<bool>>' to true")
            }
            sleep(ready_checks.duration);
        }
    }

    /// Execute the test and catch panic
    fn test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
    where
        TestFn: FnOnce(),
    {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(test))
    }
}

pub struct ContextCombinator<Context1: Context, Context2: Context> {
    context1: Context1,
    context2: Context2,
}

impl<Context1: Context, Context2: Context> Context for ContextCombinator<Context1, Context2> {
    /// Will be executed before the test execution
    /// You should prepare all your test requirement here.
    /// Use the `ready` to notify that the test can start
    fn setup(both_ready: ReadyFn) -> Self {
        let both_ready = Arc::new(both_ready);

        let ready_flag1 = Arc::new(std::sync::Mutex::new(false));
        let ready_flag2 = Arc::new(std::sync::Mutex::new(false));

        let context1 = {
            let ready_flag1 = ready_flag1.clone();
            let ready_flag2 = ready_flag2.clone();
            let both_ready = both_ready.clone();

            let ready1 = Box::new(move || {
                let mut ready1 = ready_flag1.lock().unwrap();
                let ready2 = ready_flag2.lock().unwrap();
                *ready1 = true;
                if *ready2 {
                    both_ready();
                }
            });

            Context1::setup(ready1)
        };

        let ready2 = Box::new(move || {
            let ready1 = ready_flag1.lock().unwrap();
            let mut ready2 = ready_flag2.lock().unwrap();
            *ready2 = true;
            if *ready1 {
                both_ready();
            }
        });

        let context2 = Context2::setup(ready2);

        Self { context1, context2 }
    }

    fn ready_checks_config(&self) -> ReadyChecksConfig {
        let config1 = self.context1.ready_checks_config();
        let duration1 = config1.duration * config1.maximum.try_into().unwrap();

        let config2 = self.context2.ready_checks_config();
        let duration2 = config1.duration * config2.maximum.try_into().unwrap();

        if duration1 > duration2 {
            config1
        } else {
            config2
        }
    }

    /// Will be executed before the test execution even if the test has panicked
    /// You should do your clean up here.
    fn teardown(&mut self) {
        self.context1.teardown();
        self.context2.teardown();
    }
}

/// Trait to implement if you need to access a setup value in you test.
pub trait FromContext<C: Context> {
    fn from_context(context: &C) -> Self;
}

/// Periadically try the predicate waiting for the given duration.
/// When the predicate return `true` call the `ready` fn.
///
/// Useful when you can't trigger a ready from your dependencies.
pub fn ready_when(ready: ReadyFn, predicate: PredicateFn, waiting_duration: Duration) {
    while !predicate() {
        sleep(waiting_duration)
    }
    ready()
}

pub type PredicateFn = Box<dyn Fn() -> bool + Send + Sync>;

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use futures::future::BoxFuture;
    pub use futures::future::FutureExt;
    use std::{
        any::Any,
        panic::AssertUnwindSafe,
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tokio::time::sleep;

    use crate::{ReadyChecksConfig, ReadyFn};

    /// # Trait to implement to use the `#[tearup_test]` or `#[tearup]`
    #[async_trait]
    pub trait AsyncContext<'a>: Sync {
        /// Will be executed before the test execution
        /// You should prepare all your test requirement here.
        /// Use the `ready` to notify that the test can start
        async fn setup(ready: ReadyFn) -> Self;

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(&mut self);

        /// Until the `setup` notify it's ready wait for a `duration` as many times needed with a `max`.
        /// So here we return this ReadyChecksConfig { duration: 100ms, max: 50} and can be overriden as you wish.
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms500()
        }

        async fn wait_setup(&mut self, ready: Arc<Mutex<bool>>) {
            let ready_checks = self.ready_checks_config();

            let ready = || *ready.lock().unwrap();

            let mut checks_done = 0;
            while !ready() && checks_done < ready_checks.maximum {
                checks_done += 1;
                if checks_done == ready_checks.maximum {
                    panic!("Setup has timeout, make sure to pass the 'ready: Arc<Mutex<bool>>' to true")
                }
                sleep(ready_checks.duration).await;
            }
        }

        /// Actual test launch with panic catch
        async fn test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
        where
            TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
        {
            AssertUnwindSafe(async move { test().await })
                .catch_unwind()
                .await
        }
    }

    pub struct AsyncContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncContext<'a> + Send,
        for<'a> Context2: AsyncContext<'a> + Send,
    {
        context1: Context1,
        context2: Context2,
    }

    #[async_trait]
    impl<'b, Context1, Context2> AsyncContext<'b> for AsyncContextCombinator<Context1, Context2>
    where
        for<'a> Context1: AsyncContext<'a> + Send,
        for<'a> Context2: AsyncContext<'a> + Send,
    {
        /// Will be executed before the test execution
        /// You should prepare all your test requirement here.
        /// Use the `ready` to notify that the test can start
        async fn setup(both_ready: ReadyFn) -> Self {
            let both_ready = Arc::new(both_ready);

            let ready_flag1 = Arc::new(std::sync::Mutex::new(false));
            let ready_flag2 = Arc::new(std::sync::Mutex::new(false));

            let context1 = {
                let ready_flag1 = ready_flag1.clone();
                let ready_flag2 = ready_flag2.clone();
                let both_ready = both_ready.clone();

                let ready1 = Box::new(move || {
                    let mut ready1 = ready_flag1.lock().unwrap();
                    let ready2 = ready_flag2.lock().unwrap();
                    *ready1 = true;
                    if *ready2 {
                        both_ready();
                    }
                });

                Context1::setup(ready1).await
            };

            let ready2 = Box::new(move || {
                let ready1 = ready_flag1.lock().unwrap();
                let mut ready2 = ready_flag2.lock().unwrap();
                *ready2 = true;
                if *ready1 {
                    both_ready();
                }
            });

            let context2 = Context2::setup(ready2).await;

            Self { context1, context2 }
        }

        fn ready_checks_config(&self) -> ReadyChecksConfig {
            let config1 = self.context1.ready_checks_config();
            let duration1 = config1.duration * config1.maximum.try_into().unwrap();

            let config2 = self.context2.ready_checks_config();
            let duration2 = config1.duration * config2.maximum.try_into().unwrap();

            if duration1 > duration2 {
                config1
            } else {
                config2
            }
        }

        /// Will be executed before the test execution even if the test has panicked
        /// You should do your clean up here.
        async fn teardown(&mut self) {
            self.context1.teardown().await;
            self.context2.teardown().await;
        }
    }

    /// Trait to implement if you need to access a setup value in you test.
    #[async_trait]
    pub trait FromAsyncContext<'a, C: AsyncContext<'a>> {
        async fn from_context(context: &C) -> Self;
    }

    /// Periadically try the predicate waiting for the given duration.
    /// When the predicate return `true` call the `ready` fn.
    ///
    /// Useful when you can't trigger a ready from your dependencies.
    pub async fn async_ready_when<'a, PredicateFn>(
        ready: ReadyFn,
        mut predicate: PredicateFn,
        waiting_duration: Duration,
    ) where
        PredicateFn: FnMut() -> BoxFuture<'a, bool> + Send,
    {
        while !predicate().await {
            sleep(waiting_duration).await;
        }
        ready()
    }
}

pub type ReadyFn = Box<dyn Fn() + Send + Sync>;

pub struct ReadyChecksConfig {
    pub duration: Duration,
    pub maximum: usize,
}

impl ReadyChecksConfig {
    pub fn ms100() -> ReadyChecksConfig {
        ReadyChecksConfig {
            duration: Duration::from_millis(10),
            maximum: 10,
        }
    }

    pub fn ms500() -> ReadyChecksConfig {
        ReadyChecksConfig {
            duration: Duration::from_millis(50),
            maximum: 10,
        }
    }

    pub fn s1() -> ReadyChecksConfig {
        ReadyChecksConfig {
            duration: Duration::from_millis(100),
            maximum: 10,
        }
    }

    pub fn s2() -> ReadyChecksConfig {
        ReadyChecksConfig {
            duration: Duration::from_millis(200),
            maximum: 10,
        }
    }

    pub fn s5() -> ReadyChecksConfig {
        ReadyChecksConfig {
            duration: Duration::from_millis(500),
            maximum: 10,
        }
    }
}
