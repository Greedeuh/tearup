use lazy_static::lazy_static;
use std::{
    sync::Mutex,
    thread::sleep,
    time::{Duration, SystemTime},
};
use tearup::{tearup, SimpleContext};

lazy_static! {
    static ref SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    static ref TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
}

#[test]
fn it_pass_through_setup_then_teardown() {
    teardown_panic();

    let raw_setup_checkpoint = SETUP_CHECKPOINT.lock().unwrap().unwrap();
    let raw_teardown_checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap().unwrap();

    assert!(raw_setup_checkpoint < raw_teardown_checkpoint);
}

struct NiceContext;
impl SimpleContext for NiceContext {
    fn setup() -> Self {
        let mut checkpoint = SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        sleep(Duration::from_millis(10));

        Self {}
    }

    fn teardown(&mut self) {
        let mut checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

#[tearup(NiceContext)]
fn teardown_panic() {}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use lazy_static::lazy_static;
    use std::time::{Duration, SystemTime};
    use tearup::{tearup, AsyncSimpleContext};
    use tokio::{sync::Mutex, time::sleep};

    lazy_static! {
        static ref SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
        static ref TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown() {
        teardown_panic().await;

        let raw_setup_checkpoint = SETUP_CHECKPOINT.lock().await.unwrap();
        let raw_teardown_checkpoint = TEARDOWN_CHECKPOINT.lock().await.unwrap();

        assert!(raw_setup_checkpoint < raw_teardown_checkpoint);
    }

    struct NiceContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for NiceContext {
        async fn setup() -> Self {
            let mut checkpoint = SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(10)).await;

            Self {}
        }

        async fn teardown(&mut self) {
            let mut checkpoint = TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }

    #[tearup(NiceContext)]
    async fn teardown_panic() {}
}
