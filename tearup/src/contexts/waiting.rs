#[cfg(feature = "async")]
pub use asyncc::*;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use stopwatch::Stopwatch;

use crate::{ReadyChecksConfig, ReadyFn};

pub struct TimeGate {
    ready_flag: Arc<Mutex<bool>>,
    ready_checks: ReadyChecksConfig,
}

impl TimeGate {
    pub fn new() -> Self {
        TimeGate {
            ready_flag: Arc::new(Mutex::new(false)),
            ready_checks: ReadyChecksConfig::default(),
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
            sleep(self.ready_checks.duration);
        }
    }

    pub fn wait_signal_or_timeout(self, timeout: Duration) -> Result<(), TimeoutError> {
        let stopwatch = Stopwatch::start_new();
        let ready = || *self.ready_flag.lock().unwrap();

        while !ready() {
            if stopwatch.elapsed() >= timeout {
                return Err(TimeoutError {
                    duration: timeout,
                    ready_checks: self.ready_checks,
                });
            }
            sleep(self.ready_checks.duration);
        }

        Ok(())
    }
}
#[derive(PartialEq, Debug)]
pub struct TimeoutError {
    pub duration: Duration,
    pub ready_checks: ReadyChecksConfig,
}

impl Default for TimeGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };

    use stopwatch::Stopwatch;

    use crate::{ReadyChecksConfig, TimeGate, TimeoutError};

    #[test]
    fn it_waits_signal() {
        let stopwatch = Stopwatch::start_new();

        let gate = TimeGate::new();
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

        let gate = TimeGate::new();
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

        let gate = TimeGate::new();
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
                ready_checks: ReadyChecksConfig::default(),
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

#[cfg(feature = "async")]
mod asyncc {

    pub use futures::future::FutureExt;
    use std::sync::{Arc, Mutex};
    use stopwatch::Stopwatch;
    use tokio::time::sleep;

    use crate::{ready_state, AsyncSimpleContext, ReadyChecksConfig, ReadyFn};

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
