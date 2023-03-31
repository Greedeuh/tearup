use anymap::AnyMap;
#[cfg(feature = "async")]
use futures::future::BoxFuture;
use std::any::Any;
#[cfg(feature = "async")]
use std::panic::AssertUnwindSafe;

mod waiting;
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
pub(crate) async fn async_launch_test<'a, TestFn>(test: TestFn) -> Result<(), Box<dyn Any + Send>>
where
    TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
{
    AssertUnwindSafe(async move { test().await })
        .catch_unwind()
        .await
}

pub struct SharedContext(AnyMap);

impl SharedContext {
    pub fn register<T: 'static>(&mut self, value: T) {
        self.0.insert(value);
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut()
    }

    pub fn get<T: 'static>(&mut self) -> Option<&T> {
        self.0.get()
    }
}

impl Default for SharedContext {
    fn default() -> Self {
        Self(AnyMap::new())
    }
}
