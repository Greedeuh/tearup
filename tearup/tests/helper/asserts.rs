use std::{sync::Mutex, time::SystemTime};

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
