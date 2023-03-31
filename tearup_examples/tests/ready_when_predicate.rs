use async_trait::async_trait;
use std::time::Duration;
use tearup::{
    async_ready_when, ready_when, tearup_test, AsyncSimpleContext, FutureExt, SharedContext,
    SimpleContext, TimeGate,
};

#[tearup_test(AsyncReadyWhenContext)]
async fn setup_barely_timeout_with_ready_when() {}

struct AsyncReadyWhenContext;

#[async_trait]
impl AsyncSimpleContext<'_> for AsyncReadyWhenContext {
    async fn setup() -> Self {
        launch_server();

        let gate = TimeGate::new();

        async_ready_when(
            gate.notifier(),
            || async move { ping_server().await.is_ok() }.boxed(),
            Duration::from_millis(100),
        )
        .await;

        Self {}
    }

    async fn teardown(mut self) {}
}

fn launch_server() {}

async fn ping_server() -> Result<(), ()> {
    Ok(())
}

struct SyncReadyWhenContext;

#[async_trait]
impl SimpleContext for SyncReadyWhenContext {
    fn setup(_shared_context: &mut SharedContext) -> Self {
        let gate = TimeGate::new();

        launch_server();

        ready_when(
            gate.notifier(),
            Box::new(|| sync_ping_server().is_ok()),
            Duration::from_millis(100),
        );

        Self {}
    }

    fn teardown(self) {}
}

fn sync_ping_server() -> Result<(), ()> {
    Ok(())
}
