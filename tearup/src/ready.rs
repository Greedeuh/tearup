use std::sync::Arc;
use std::{thread::sleep, time::Duration};

#[cfg(feature = "async")]
pub use asyncc::*;

pub type ReadyFn = Box<dyn Fn() + Send + Sync>;
pub type PredicateFn = Box<dyn Fn() -> bool + Send + Sync>;

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

    pub fn get_longest(mut list: Vec<Self>) -> Self {
        list.sort_by_key(|a| a.maxium_duration());
        list.remove(0)
    }

    pub fn maxium_duration(&self) -> Duration {
        self.duration * self.maximum.try_into().unwrap()
    }
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

pub fn split(both_ready: ReadyFn) -> (ReadyFn, ReadyFn) {
    let both_ready = Arc::new(both_ready);

    let ready_flag1 = Arc::new(std::sync::Mutex::new(false));
    let ready_flag2 = Arc::new(std::sync::Mutex::new(false));

    let ready1 = {
        let ready_flag1 = ready_flag1.clone();
        let ready_flag2 = ready_flag2.clone();
        let both_ready = both_ready.clone();

        Box::new(move || {
            let mut ready1 = ready_flag1.lock().unwrap();
            let ready2 = ready_flag2.lock().unwrap();
            *ready1 = true;
            if *ready2 {
                both_ready();
            }
        })
    };

    let ready2 = Box::new(move || {
        let ready1 = ready_flag1.lock().unwrap();
        let mut ready2 = ready_flag2.lock().unwrap();
        *ready2 = true;
        if *ready1 {
            both_ready();
        }
    });
    (ready1, ready2)
}

pub type SplitedReadyFn = Box<dyn Fn(u16) + Send + Sync>;

pub fn n_times(all_ready: ReadyFn, n: u16) -> SplitedReadyFn {
    let all_ready = Arc::new(all_ready);
    let ready_flags = Arc::new(std::sync::Mutex::new(vec![false; n.into()]));

    Box::new(move |who_index| {
        let mut ready_flags = ready_flags.lock().unwrap();
        ready_flags[who_index as usize] = true;

        if ready_flags.iter().all(|x| *x) {
            all_ready()
        }
    })
}

pub fn get_longest(config1: ReadyChecksConfig, config2: ReadyChecksConfig) -> ReadyChecksConfig {
    let duration1 = config1.duration * config1.maximum.try_into().unwrap();

    let duration2 = config1.duration * config2.maximum.try_into().unwrap();

    if duration1 > duration2 {
        config1
    } else {
        config2
    }
}

#[cfg(feature = "async")]
mod asyncc {
    use std::time::Duration;

    use futures::future::BoxFuture;
    use tokio::time::sleep;

    use crate::ReadyFn;

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
