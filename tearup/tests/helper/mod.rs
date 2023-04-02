use tearup::{SharedContext, SimpleContext};

mod asserts;
pub use asserts::*;
pub use asyncc::*;

#[derive(Clone)]
pub struct FirstProof(pub String);
#[derive(Clone)]
pub struct SecondProof(pub String);

pub struct FirstFullContext;
impl SimpleContext for FirstFullContext {
    fn setup(shared_context: &mut SharedContext) -> Self {
        shared_context.register(FirstProof("first_proof".to_owned()));
        shared_context.register(SecondProof("second_proof".to_owned()));
        Self {}
    }

    fn teardown(self, _shared_context: &mut SharedContext) {}
}

#[derive(Clone)]
pub struct ThirdProof(pub String);
#[derive(Clone)]
pub struct FourthProof(pub String);

pub struct SecondFullContext;
impl SimpleContext for SecondFullContext {
    fn setup(shared_context: &mut SharedContext) -> Self {
        let first = shared_context.get::<FirstProof>().unwrap().0;
        shared_context.register(ThirdProof(format!("ref_to_{}", first)));

        let second = shared_context.get::<SecondProof>().unwrap().0;
        shared_context.register(FourthProof(format!("another_ref_to_{}", second)));
        Self {}
    }

    fn teardown(self, _shared_context: &mut SharedContext) {}
}

#[cfg(feature = "async")]
pub mod asyncc {
    use tearup::{async_trait, AsyncSharedContext, AsyncSimpleContext};

    use crate::helper::{FirstProof, FourthProof, SecondProof, ThirdProof};

    pub struct AsyncFirstFullContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for AsyncFirstFullContext {
        async fn setup(shared_context: AsyncSharedContext) -> Self {
            shared_context
                .register(FirstProof("first_proof".to_owned()))
                .await;
            shared_context
                .register(SecondProof("second_proof".to_owned()))
                .await;
            Self {}
        }

        async fn teardown(self, _shared_context: AsyncSharedContext) {}
    }

    pub struct AsyncSecondFullContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for AsyncSecondFullContext {
        async fn setup(mut shared_context: AsyncSharedContext) -> Self {
            let first = shared_context.get::<FirstProof>().await.unwrap().0;
            shared_context
                .register(ThirdProof(format!("ref_to_{}", first)))
                .await;

            let second = shared_context.get::<SecondProof>().await.unwrap().0;
            shared_context
                .register(FourthProof(format!("another_ref_to_{}", second)))
                .await;
            Self {}
        }

        async fn teardown(self, _shared_context: AsyncSharedContext) {}
    }
}
