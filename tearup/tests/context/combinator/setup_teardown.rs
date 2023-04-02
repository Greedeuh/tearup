use lazy_static::lazy_static;
use std::time::{Duration, SystemTime};
use tearup::{tearup, ContextCombinator, SharedContext, SimpleContext};

use crate::helper::{assert_order, Checkpoint};

lazy_static! {
    static ref FIRST_SETUP_CHECKPOINT: Checkpoint = None.into();
    static ref FIRST_TEARDOWN_CHECKPOINT: Checkpoint = None.into();
    static ref SECOND_SETUP_CHECKPOINT: Checkpoint = None.into();
    static ref SECOND_TEARDOWN_CHECKPOINT: Checkpoint = None.into();
}

#[test]
fn it_pass_through_setup_then_teardown_of_both_contexts_one_after_the_other() {
    sequential();

    assert_order(&FIRST_SETUP_CHECKPOINT, &SECOND_SETUP_CHECKPOINT);
    assert_order(&SECOND_SETUP_CHECKPOINT, &FIRST_TEARDOWN_CHECKPOINT);
    assert_order(&FIRST_TEARDOWN_CHECKPOINT, &SECOND_TEARDOWN_CHECKPOINT);
}

type B = ContextCombinator<FirstContext, SecondContext>;
#[tearup(B)]
fn sequential() {}

pub struct FirstContext;
impl SimpleContext for FirstContext {
    fn setup(_shared_context: &mut SharedContext) -> Self {
        let mut checkpoint = FIRST_SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        std::thread::sleep(Duration::from_millis(50));

        Self {}
    }

    fn teardown(self, _shared_context: &mut SharedContext) {
        let mut checkpoint = FIRST_TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

pub struct SecondContext;
impl SimpleContext for SecondContext {
    fn setup(_shared_context: &mut SharedContext) -> Self {
        let mut checkpoint = SECOND_SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        std::thread::sleep(Duration::from_millis(50));

        Self {}
    }

    fn teardown(self, _shared_context: &mut SharedContext) {
        let mut checkpoint = SECOND_TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

mod asyncc {
    use lazy_static::lazy_static;
    use std::time::{Duration, SystemTime};
    use tearup::{
        async_trait, tearup, AsyncContextCombinator, AsyncSharedContext, AsyncSimpleContext,
    };
    use tokio::time::sleep;

    use crate::helper::{assert_async_order, AsyncCheckpoint};

    lazy_static! {
        static ref FIRST_SETUP_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref FIRST_TEARDOWN_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref SECOND_SETUP_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref SECOND_TEARDOWN_CHECKPOINT: AsyncCheckpoint = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown_of_both_contexts_one_after_the_other() {
        sequential().await;

        assert_async_order(&FIRST_SETUP_CHECKPOINT, &SECOND_SETUP_CHECKPOINT).await;
        assert_async_order(&SECOND_SETUP_CHECKPOINT, &FIRST_TEARDOWN_CHECKPOINT).await;
        assert_async_order(&FIRST_TEARDOWN_CHECKPOINT, &SECOND_TEARDOWN_CHECKPOINT).await;
    }

    type B = AsyncContextCombinator<FirstContext, SecondContext>;
    #[tearup(B)]
    async fn sequential() {}

    pub struct FirstContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for FirstContext {
        async fn setup(_shared_context: AsyncSharedContext) -> Self {
            let mut checkpoint = FIRST_SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(50)).await;

            Self {}
        }

        async fn teardown(mut self, _shared_context: AsyncSharedContext) {
            let mut checkpoint = FIRST_TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }

    pub struct SecondContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for SecondContext {
        async fn setup(_shared_context: AsyncSharedContext) -> Self {
            let mut checkpoint = SECOND_SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(50)).await;

            Self {}
        }

        async fn teardown(mut self, _shared_context: AsyncSharedContext) {
            let mut checkpoint = SECOND_TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }
}
