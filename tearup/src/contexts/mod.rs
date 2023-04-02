use anymap::{CloneAny, Map};
#[cfg(feature = "async")]
use futures::future::BoxFuture;
use std::any::Any;
#[cfg(feature = "async")]
use std::panic::AssertUnwindSafe;

mod simple;
pub use simple::*;
mod sequential_combinator;
#[cfg(feature = "async")]
pub use asyncc::*;
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

type AnyMap = Map<dyn CloneAny>;

pub struct SharedContext(AnyMap);

impl SharedContext {
    pub fn register<T: 'static + Clone>(&mut self, value: T) {
        self.0.insert(value);
    }

    pub fn get<T: 'static + Clone>(&mut self) -> Option<T> {
        self.0.get::<T>().cloned()
    }
}

impl Default for SharedContext {
    fn default() -> Self {
        Self(AnyMap::new())
    }
}

#[cfg(feature = "async")]
pub mod asyncc {
    use anymap::{CloneAny, Map};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    type AnymapSend = Map<dyn CloneAny + Send>;

    #[derive(Clone)]
    pub struct AsyncSharedContext(Arc<Mutex<AnymapSend>>);

    impl AsyncSharedContext {
        pub async fn register<T: 'static + Send + Clone>(&self, value: T) {
            let mut map = self.0.lock().await;
            map.insert(value);
        }

        pub async fn get<T: 'static + Send + Clone>(&mut self) -> Option<T> {
            self.0.lock().await.get::<T>().cloned()
        }
    }

    impl Default for AsyncSharedContext {
        fn default() -> Self {
            Self(Arc::new(Mutex::new(AnymapSend::new())))
        }
    }
}
