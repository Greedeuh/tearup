use futures::future::BoxFuture;
use std::{sync::Mutex, time::SystemTime};
use stopwatch::Stopwatch;
use tearup::FutureExt;

pub type Checkpoint = Mutex<Option<SystemTime>>;
pub type AsyncCheckpoint = tokio::sync::Mutex<Option<SystemTime>>;

pub fn assert_order(checkpoint_before: &Checkpoint, checkpoint_after: &Checkpoint) {
    assert!(checkpoint_before.lock().unwrap().unwrap() < checkpoint_after.lock().unwrap().unwrap());
}

pub async fn assert_async_order(
    checkpoint_before: &AsyncCheckpoint,
    checkpoint_after: &AsyncCheckpoint,
) {
    assert!(checkpoint_before.lock().await.unwrap() < checkpoint_after.lock().await.unwrap());
}

#[allow(dead_code)]
pub fn assert_around_100ms<TestFn>(test: TestFn)
where
    TestFn: FnOnce(),
{
    let stopwatch = Stopwatch::start_new();

    test();

    assert_around_100ms_(&stopwatch);
}

#[allow(dead_code)]
pub async fn async_assert_around_100ms<'a, TestFn>(test: TestFn)
where
    TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
{
    let stopwatch = Stopwatch::start_new();

    test().await;

    assert_around_100ms_(&stopwatch);
}

#[allow(dead_code)]
pub fn assert_timeout_around_100ms<TestFn>(test: TestFn)
where
    TestFn: FnOnce(),
{
    let stopwatch = Stopwatch::start_new();

    let test_execution = std::panic::catch_unwind(std::panic::AssertUnwindSafe(test));
    assert!(test_execution.is_err());

    assert_around_100ms_(&stopwatch);
}

#[allow(dead_code)]
pub async fn async_assert_timeout_around_100ms<'a, TestFn>(test: TestFn)
where
    TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
{
    let stopwatch = Stopwatch::start_new();

    let test_execution = std::panic::AssertUnwindSafe(async move { test().await })
        .catch_unwind()
        .await;
    assert!(test_execution.is_err());

    assert_around_100ms_(&stopwatch);
}

fn assert_around_100ms_(stopwatch: &Stopwatch) {
    let ms = stopwatch.elapsed_ms();
    assert!(115 > ms, "stopwatch has {} elapsed ms > 115", ms);
    assert!(ms > 85, "stopwatch has {} elapsed ms < 85", ms);
}
