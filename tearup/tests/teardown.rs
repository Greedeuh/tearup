use tearup::{tearup, Context, ReadyChecksConfig, ReadyFn};

#[test]
#[should_panic]
fn it_pass_through_teardown() {
    teardown_panic()
}

struct TeardownPanicContext;
impl Context for TeardownPanicContext {
    fn ready_checks_config() -> ReadyChecksConfig {
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
    use tearup::{tearup, AsyncContext, ReadyChecksConfig, ReadyFn};

    #[tokio::test]
    #[should_panic]
    async fn it_pass_through_teardown() {
        teardown_panic().await
    }

    struct TeardownPanicContext;
    #[async_trait]
    impl AsyncContext<'_> for TeardownPanicContext {
        fn ready_checks_config() -> ReadyChecksConfig {
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
