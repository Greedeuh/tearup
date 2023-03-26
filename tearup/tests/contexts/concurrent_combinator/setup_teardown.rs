use lazy_static::lazy_static;
use std::time::{Duration, SystemTime};
use tearup::{tearup, ConcurrentContextCombinator, SimpleContext};

use crate::helper::{assert_around_100ms, assert_order, Checkpoint};

lazy_static! {
    static ref FIRST_SETUP_CHECKPOINT: Checkpoint = None.into();
    static ref FIRST_TEARDOWN_CHECKPOINT: Checkpoint = None.into();
    static ref SECOND_SETUP_CHECKPOINT: Checkpoint = None.into();
    static ref SECOND_TEARDOWN_CHECKPOINT: Checkpoint = None.into();
}

#[test]
fn it_pass_through_setup_then_teardown_of_both_contexts_concurrently() {
    assert_around_100ms(concurrent);

    assert_order(&FIRST_SETUP_CHECKPOINT, &FIRST_TEARDOWN_CHECKPOINT);
    assert_order(&SECOND_SETUP_CHECKPOINT, &SECOND_TEARDOWN_CHECKPOINT);
    assert_order(&SECOND_SETUP_CHECKPOINT, &FIRST_TEARDOWN_CHECKPOINT);
}

type B = ConcurrentContextCombinator<FirstContext, SecondContext>;
#[tearup(B)]
fn concurrent() {}

pub struct FirstContext;
impl SimpleContext for FirstContext {
    fn setup() -> Self {
        let mut checkpoint = FIRST_SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        std::thread::sleep(Duration::from_millis(100));

        Self {}
    }

    fn teardown(&mut self) {
        let mut checkpoint = FIRST_TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

pub struct SecondContext;
impl SimpleContext for SecondContext {
    fn setup() -> Self {
        let mut checkpoint = SECOND_SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        std::thread::sleep(Duration::from_millis(100));

        Self {}
    }

    fn teardown(&mut self) {
        let mut checkpoint = SECOND_TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

mod asyncc {
    use lazy_static::lazy_static;
    use std::time::{Duration, SystemTime};
    use tearup::{
        async_trait, tearup, AsyncConcurrentContextCombinator, AsyncSimpleContext, FutureExt,
    };
    use tokio::time::sleep;

    use crate::helper::{assert_async_order, async_assert_around_100ms, AsyncCheckpoint};

    lazy_static! {
        static ref FIRST_SETUP_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref FIRST_TEARDOWN_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref SECOND_SETUP_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref SECOND_TEARDOWN_CHECKPOINT: AsyncCheckpoint = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown_of_both_contexts_concurrently() {
        async_assert_around_100ms(move || { async move { concurrent().await } }.boxed()).await;

        assert_async_order(&FIRST_SETUP_CHECKPOINT, &FIRST_TEARDOWN_CHECKPOINT).await;
        assert_async_order(&SECOND_SETUP_CHECKPOINT, &SECOND_TEARDOWN_CHECKPOINT).await;
        assert_async_order(&SECOND_SETUP_CHECKPOINT, &FIRST_TEARDOWN_CHECKPOINT).await;
    }

    type B = AsyncConcurrentContextCombinator<FirstContext, SecondContext>;
    #[tearup(B)]
    async fn concurrent() {}

    pub struct FirstContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for FirstContext {
        async fn setup() -> Self {
            let mut checkpoint = FIRST_SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(100)).await;

            Self {}
        }

        async fn teardown(&mut self) {
            let mut checkpoint = FIRST_TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }

    pub struct SecondContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for SecondContext {
        async fn setup() -> Self {
            let mut checkpoint = SECOND_SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(100)).await;

            Self {}
        }

        async fn teardown(&mut self) {
            let mut checkpoint = SECOND_TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }
}
