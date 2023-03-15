#[cfg(feature = "async")]
use futures::future::BoxFuture;
use std::any::Any;
#[cfg(feature = "async")]
use std::panic::AssertUnwindSafe;

mod concurrent_combinator;
mod waiting;
pub use concurrent_combinator::*;
pub use waiting::*;
mod simple;
pub use simple::*;
mod sequential_combinator;
pub use sequential_combinator::*;

pub(crate) fn launch_test<TestFn>(test: TestFn) -> Result<(), Box<dyn Any + Send>>
where
    TestFn: FnOnce(),
{
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(test))
}

#[cfg(feature = "async")]
async fn async_launch_test<'a, TestFn>(test: TestFn) -> Result<(), Box<dyn Any + Send>>
where
    TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
{
    AssertUnwindSafe(async move { test().await })
        .catch_unwind()
        .await
}
