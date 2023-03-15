use tearup::{tearup, ReadyChecksConfig, ReadyFn, WaitingContext};

#[test]
#[should_panic]
fn it_pass_through_teardown() {
    teardown_panic()
}

struct TeardownPanicContext;
impl WaitingContext for TeardownPanicContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        ready();
        Self {}
    }

    fn teardown(&mut self) {
        panic!()
    }
}

#[tearup(TeardownPanicContext)]
fn teardown_panic() {}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use tearup::{tearup, AsyncWaitingContext, ReadyChecksConfig, ReadyFn};

    #[tokio::test]
    #[should_panic]
    async fn it_pass_through_teardown() {
        teardown_panic().await
    }

    struct TeardownPanicContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for TeardownPanicContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            ready();
            Self {}
        }

        async fn teardown(&mut self) {
            panic!()
        }
    }

    #[tearup(TeardownPanicContext)]
    async fn teardown_panic() {}
}
