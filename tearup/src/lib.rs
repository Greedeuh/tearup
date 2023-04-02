use anymap::{CloneAny, Map};
#[cfg(feature = "async")]
pub use async_trait::async_trait;
use std::time::Duration;
pub use tearup_macro::{tearup, tearup_test};

mod context;
pub use context::*;
mod context_combinator;
pub use context_combinator::*;
pub mod helper;
#[cfg(feature = "async")]
pub use asyncc::*;

#[derive(PartialEq, Debug)]
pub struct TimeoutError {
    pub duration: Duration,
    pub ready_checks_interval: Duration,
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
