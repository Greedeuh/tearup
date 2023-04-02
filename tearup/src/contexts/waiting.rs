#[cfg(feature = "async")]
pub use asyncc::*;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use stopwatch::Stopwatch;

use crate::ReadyFn;

pub struct TimeGate {
    ready_flag: Arc<Mutex<bool>>,
    ready_checks_interval: Duration,
}

impl TimeGate {
    pub fn new(ready_checks_interval: Duration) -> Self {
        TimeGate {
            ready_flag: Arc::new(Mutex::new(false)),
            ready_checks_interval,
        }
    }

    pub fn notifier(&self) -> ReadyFn {
        let ready_flag = self.ready_flag.clone();

        Box::new(move || {
            let mut ready = ready_flag.lock().unwrap();
            *ready = true;
        })
    }

    pub fn wait_signal(self) {
        let ready = || *self.ready_flag.lock().unwrap();

        while !ready() {
            sleep(self.ready_checks_interval);
        }
    }

    pub fn wait_signal_or_timeout(self, timeout: Duration) -> Result<(), TimeoutError> {
        let stopwatch = Stopwatch::start_new();
        let ready = || *self.ready_flag.lock().unwrap();

        while !ready() {
            if stopwatch.elapsed() >= timeout {
                return Err(TimeoutError {
                    duration: timeout,
                    ready_checks_interval: self.ready_checks_interval,
                });
            }
            sleep(self.ready_checks_interval);
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct TimeoutError {
    pub duration: Duration,
    pub ready_checks_interval: Duration,
}

impl Default for TimeGate {
    fn default() -> Self {
        Self::new(Duration::from_millis(10))
    }
}

#[cfg(feature = "async")]
mod asyncc {

    use futures::future::BoxFuture;
    pub use futures::future::FutureExt;
    use std::{sync::Arc, time::Duration};
    use stopwatch::Stopwatch;
    use tokio::{sync::Mutex, time::sleep};

    use crate::TimeoutError;

    pub struct AsyncTimeGate {
        ready_flag: Arc<Mutex<bool>>,
        ready_checks_interval: Duration,
    }

    pub type AsyncReadyFn<'a> = Box<dyn Fn() -> BoxFuture<'a, ()> + Send + Sync>;

    impl AsyncTimeGate {
        pub fn new(ready_checks_interval: Duration) -> Self {
            AsyncTimeGate {
                ready_flag: Arc::new(Mutex::new(false)),
                ready_checks_interval,
            }
        }

        pub fn notifier<'a>(&self) -> AsyncReadyFn<'a> {
            let ready_flag = self.ready_flag.clone();

            Box::new(move || {
                let ready_flag = ready_flag.clone();
                Box::pin(async move {
                    let mut ready_flag = ready_flag.lock().await;
                    *ready_flag = true;
                })
            })
        }

        pub async fn wait_signal(self) {
            while !self.is_ready().await {
                sleep(self.ready_checks_interval).await;
            }
        }

        pub async fn wait_signal_or_timeout(self, timeout: Duration) -> Result<(), TimeoutError> {
            let stopwatch = Stopwatch::start_new();

            while !self.is_ready().await {
                if stopwatch.elapsed() >= timeout {
                    return Err(TimeoutError {
                        duration: timeout,
                        ready_checks_interval: self.ready_checks_interval,
                    });
                }
                sleep(self.ready_checks_interval).await;
            }

            Ok(())
        }

        async fn is_ready(&self) -> bool {
            *self.ready_flag.lock().await
        }
    }

    impl Default for AsyncTimeGate {
        fn default() -> Self {
            Self::new(Duration::from_millis(10))
        }
    }

    #[cfg(test)]
    mod test {
        use std::time::Duration;

        use stopwatch::Stopwatch;
        use tokio::{spawn, time::sleep};

        use crate::{AsyncTimeGate, TimeoutError};

        #[tokio::test]
        async fn it_waits_signal() {
            let stopwatch = Stopwatch::start_new();

            let gate = AsyncTimeGate::default();
            let ready = gate.notifier();

            spawn(async move {
                sleep(Duration::from_millis(100)).await;
                ready().await;
            })
            .await
            .unwrap();

            gate.wait_signal().await;
            assert_around_100ms_(&stopwatch);
        }

        #[tokio::test]
        async fn it_waits_signal_even_with_timeout_option() {
            let stopwatch = Stopwatch::start_new();

            let gate = AsyncTimeGate::default();
            let ready = gate.notifier();

            spawn(async move {
                sleep(Duration::from_millis(100)).await;
                ready().await;
            });

            assert!(gate
                .wait_signal_or_timeout(Duration::from_millis(115))
                .await
                .is_ok(),);
            assert_around_100ms_(&stopwatch);
        }

        #[tokio::test]
        async fn it_timeouts() {
            let stopwatch = Stopwatch::start_new();

            let gate = AsyncTimeGate::default();
            let ready = gate.notifier();

            spawn(async move {
                sleep(Duration::from_millis(100)).await;
                ready().await;
            });

            let timeout = Duration::from_millis(85);
            assert_eq!(
                gate.wait_signal_or_timeout(Duration::from_millis(85)).await,
                Err(TimeoutError {
                    duration: timeout,
                    ready_checks_interval: Duration::from_millis(10),
                })
            );
            assert_around_100ms_(&stopwatch);
        }

        fn assert_around_100ms_(stopwatch: &Stopwatch) {
            let ms = stopwatch.elapsed_ms();
            assert!(115 > ms, "stopwatch has {} elapsed ms > 115", ms);
            assert!(ms > 85, "stopwatch has {} elapsed ms < 85", ms);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };

    use stopwatch::Stopwatch;

    use crate::{TimeGate, TimeoutError};

    #[test]
    fn it_waits_signal() {
        let stopwatch = Stopwatch::start_new();

        let gate = TimeGate::default();
        let ready = gate.notifier();

        spawn(move || {
            sleep(Duration::from_millis(100));
            ready();
        });

        gate.wait_signal();
        assert_around_100ms_(&stopwatch);
    }

    #[test]
    fn it_waits_signal_even_with_timeout_option() {
        let stopwatch = Stopwatch::start_new();

        let gate = TimeGate::default();
        let ready = gate.notifier();

        spawn(move || {
            sleep(Duration::from_millis(100));
            ready();
        });

        assert!(gate
            .wait_signal_or_timeout(Duration::from_millis(115))
            .is_ok(),);
        assert_around_100ms_(&stopwatch);
    }

    #[test]
    fn it_timeouts() {
        let stopwatch = Stopwatch::start_new();

        let gate = TimeGate::default();
        let ready = gate.notifier();

        spawn(move || {
            sleep(Duration::from_millis(100));
            ready();
        });

        let timeout = Duration::from_millis(85);
        assert_eq!(
            gate.wait_signal_or_timeout(Duration::from_millis(85)),
            Err(TimeoutError {
                duration: timeout,
                ready_checks_interval: Duration::from_millis(10),
            })
        );
        assert_around_100ms_(&stopwatch);
    }

    fn assert_around_100ms_(stopwatch: &Stopwatch) {
        let ms = stopwatch.elapsed_ms();
        assert!(115 > ms, "stopwatch has {} elapsed ms > 115", ms);
        assert!(ms > 85, "stopwatch has {} elapsed ms < 85", ms);
    }
}
