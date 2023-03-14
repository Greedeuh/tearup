#[cfg(feature = "async")]
pub use async_trait::async_trait;
pub use tearup_macro::{tearup, tearup_test};

mod combinator;
mod contexts;
pub use combinator::*;
pub use contexts::*;
mod ready;
pub use ready::*;

/// Trait to implement if you need to access a setup value in you test.
pub trait FromContext<C: WaitingContext> {
    fn from_context(context: &C) -> Self;
}

/// Trait to implement if you need to access a setup value in you test.
#[cfg(feature = "async")]
#[async_trait]
pub trait FromAsyncContext<'a, C: AsyncWaitingContext<'a>> {
    async fn from_context(context: &C) -> Self;
}
