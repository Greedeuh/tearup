use std::{thread::sleep, time::Duration};

#[cfg(feature = "async")]
pub use asyncc::*;

pub type PredicateFn = Box<dyn Fn() -> bool + Send + Sync>;

/// Periadically try the predicate waiting for the given duration.
///
/// Useful when you can't trigger a ready from your dependencies.
pub fn ready_when(predicate: PredicateFn, retry_interval: Duration) {
    while !predicate() {
        sleep(retry_interval)
    }
}

#[cfg(test)]
mod test {
    use super::ready_when;
    use std::time::Duration;
    use stopwatch::Stopwatch;

    #[test]
    fn it_wait_until_predicate_becames_true() {
        let stopwatch = Stopwatch::start_new();
        ready_when(
            Box::new(move || stopwatch.elapsed_ms() > 100),
            Duration::from_millis(10),
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
    use futures::future::BoxFuture;
    use std::time::Duration;
    use tokio::time::sleep;

    /// Periadically try the predicate waiting for the given duration.
    ///
    /// Useful when you can't trigger a ready from your dependencies.
    pub async fn async_ready_when<'a, PredicateFn>(
        mut predicate: PredicateFn,
        waiting_duration: Duration,
    ) where
        PredicateFn: FnMut() -> BoxFuture<'a, bool> + Send,
    {
        while !predicate().await {
            sleep(waiting_duration).await;
        }
    }

    #[cfg(test)]
    mod test {
        use super::async_ready_when;
        use futures::FutureExt;
        use std::time::Duration;
        use stopwatch::Stopwatch;

        #[tokio::test]
        async fn it_wait_until_predicate_becames_true() {
            let stopwatch = Stopwatch::start_new();
            async_ready_when(
                || async move { stopwatch.elapsed_ms() > 100 }.boxed(),
                Duration::from_millis(10),
            )
            .await;

            assert_around_100ms_(&stopwatch);
        }

        fn assert_around_100ms_(stopwatch: &Stopwatch) {
            let ms = stopwatch.elapsed_ms();
            assert!(115 > ms, "stopwatch has {} elapsed ms > 115", ms);
            assert!(ms > 85, "stopwatch has {} elapsed ms < 85", ms);
        }
    }
}
