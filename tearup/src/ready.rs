use std::{thread::sleep, time::Duration};

use crate::{PredicateFn, ReadyFn};
#[cfg(feature = "async")]
pub use asyncc::*;

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
