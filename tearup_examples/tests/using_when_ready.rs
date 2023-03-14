use async_trait::async_trait;
use std::time::Duration;
use tearup::{
    async_ready_when, ready_when, tearup_test, AsyncContext, FutureExt, ReadyFn, WaitingContext,
};

#[tearup_test(AsyncReadyWhenContext)]
async fn setup_barely_timeout_with_ready_when() {}

struct AsyncReadyWhenContext;

#[async_trait]
impl AsyncContext<'_> for AsyncReadyWhenContext {
    async fn setup(ready: ReadyFn) -> Self {
        launch_server();

        async_ready_when(
            ready,
            || async move { ping_server().await.is_ok() }.boxed(),
            Duration::from_millis(100),
        )
        .await;

        Self {}
    }

    async fn teardown(&mut self) {}
}

fn launch_server() {}

async fn ping_server() -> Result<(), ()> {
    Ok(())
}

struct SyncReadyWhenContext;

#[async_trait]
impl WaitingContext for SyncReadyWhenContext {
    fn setup(ready: ReadyFn) -> Self {
        launch_server();

        ready_when(
            ready,
            Box::new(|| sync_ping_server().is_ok()),
            Duration::from_millis(100),
        );

        Self {}
    }

    fn teardown(&mut self) {}
}

fn sync_ping_server() -> Result<(), ()> {
    Ok(())
}
