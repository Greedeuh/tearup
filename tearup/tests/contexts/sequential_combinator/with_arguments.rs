use tearup::{tearup, SequentialContextCombinator};

use crate::helper::{
    FirstFullContext, FirstProof, FourthProof, SecondFullContext, SecondProof, ThirdProof,
};

#[test]
fn it_gives_access_to_variables() {
    it_uses_some_variables();
}

type Combination = SequentialContextCombinator<FirstFullContext, SecondFullContext>;

#[tearup(Combination)]
fn it_uses_some_variables(mut a: FirstProof, b: SecondProof, c: ThirdProof, d: FourthProof) {
    assert_eq!(a.0, "first_proof");
    a.0 = "mutable".to_owned();
    assert_eq!(a.0, "mutable");
    assert_eq!(b.0, "second_proof");
    assert_eq!(c.0, "ref_to_first_proof");
    assert_eq!(d.0, "another_ref_to_second_proof");
}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use lazy_static::lazy_static;
    use std::time::{Duration, SystemTime};
    use tearup::{tearup, AsyncSharedContext, AsyncSimpleContext};
    use tokio::time::sleep;

    use crate::helper::{assert_async_order, AsyncCheckpoint};

    lazy_static! {
        static ref SETUP_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref TEARDOWN_CHECKPOINT: AsyncCheckpoint = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown() {
        teardown_panic().await;

        assert_async_order(&SETUP_CHECKPOINT, &TEARDOWN_CHECKPOINT).await;
    }

    struct NiceContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for NiceContext {
        async fn setup(_shared_context: AsyncSharedContext) -> Self {
            let mut checkpoint = SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(10)).await;

            Self {}
        }

        async fn teardown(mut self, _shared_context: AsyncSharedContext) {
            let mut checkpoint = TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }

    #[tearup(NiceContext)]
    async fn teardown_panic() {}
}
