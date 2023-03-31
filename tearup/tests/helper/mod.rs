use anymap::AnyMap;
use tearup::{SharedContext, SimpleContext};

mod asserts;
pub use asserts::*;
pub use asyncc::*;

pub struct FirstProof(pub String);
pub struct SecondProof(pub String);

pub struct FirstFullContext;
impl SimpleContext for FirstFullContext {
    fn setup(_shared_context: &mut SharedContext) -> Self {
        Self {}
    }

    fn teardown(self) {}

    fn public_context(&mut self) -> AnyMap {
        let mut public_context = AnyMap::new();
        public_context.insert(FirstProof("first_proof".to_owned()));
        public_context.insert(SecondProof("second_proof".to_owned()));
        public_context
    }
}

pub struct ThirdProof(pub String);
pub struct FourthProof(pub String);

pub struct SecondFullContext;
impl SimpleContext for SecondFullContext {
    fn setup(_shared_context: &mut SharedContext) -> Self {
        Self {}
    }

    fn teardown(self) {}

    fn public_context(&mut self) -> AnyMap {
        let mut public_context = AnyMap::new();
        public_context.insert(ThirdProof("third_proof".to_owned()));
        public_context.insert(FourthProof("fourth_proof".to_owned()));
        public_context
    }
}

#[cfg(feature = "async")]
pub mod asyncc {
    use async_trait::async_trait;
    use tearup::{ReadyChecksConfig, ReadyFn};
    use tokio::{spawn, time::sleep};
}
