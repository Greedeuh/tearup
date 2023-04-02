use std::time::Duration;

pub use anymap::AnyMap;
#[cfg(feature = "async")]
pub use async_trait::async_trait;
pub use tearup_macro::{tearup, tearup_test};

mod contexts;
pub use contexts::*;
pub mod helper;

#[derive(PartialEq, Debug)]
pub struct TimeoutError {
    pub duration: Duration,
    pub ready_checks_interval: Duration,
}
