use lazy_static::lazy_static;
use std::{sync::Mutex, time::SystemTime};
use tearup::{tearup, ReadyChecksConfig, ReadyFn, WaitingContext};

lazy_static! {
    static ref SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    static ref TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
}

#[test]
fn it_pass_through_teardown() {
    teardown_panic();

    let raw_setup_checkpoint = SETUP_CHECKPOINT.lock().unwrap().unwrap();
    let raw_teardown_checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap().unwrap();

    assert!(raw_setup_checkpoint < raw_teardown_checkpoint);
}

struct TeardownPanicContext;
impl WaitingContext for TeardownPanicContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        let mut checkpoint = SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
        ready();
        Self {}
    }

    fn teardown(&mut self) {
        let mut checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

#[tearup(TeardownPanicContext)]
fn teardown_panic() {}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use lazy_static::lazy_static;
    use std::{sync::Mutex, time::SystemTime};
    use tearup::{tearup, AsyncWaitingContext, ReadyChecksConfig, ReadyFn};

    lazy_static! {
        static ref SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
        static ref TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_teardown() {
        teardown_panic().await;

        let raw_setup_checkpoint = SETUP_CHECKPOINT.lock().unwrap().unwrap();
        let raw_teardown_checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap().unwrap();

        assert!(raw_setup_checkpoint < raw_teardown_checkpoint);
    }

    struct TeardownPanicContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for TeardownPanicContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms100()
        }

        async fn setup(ready: ReadyFn) -> Self {
            let mut checkpoint = SETUP_CHECKPOINT.lock().unwrap();
            *checkpoint = Some(SystemTime::now());
            ready();
            Self {}
        }

        async fn teardown(&mut self) {
            let mut checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap();
            *checkpoint = Some(SystemTime::now());
        }
    }

    #[tearup(TeardownPanicContext)]
    async fn teardown_panic() {}
}
