#[cfg(feature = "async")]
pub use async_trait::async_trait;
use std::time::Duration;
pub use tearup_macro::{tearup, tearup_test};

mod context;
pub use context::*;
mod context_combinator;
pub use context_combinator::*;
pub mod helper;
mod shared_context;
pub use shared_context::*;

#[derive(PartialEq, Debug)]
pub struct TimeoutError {
    pub duration: Duration,
    pub ready_checks_interval: Duration,
}
