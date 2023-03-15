mod waiting;
pub use waiting::*;
mod simple;
pub use simple::*;

// pub trait Context {
//     fn launch_setup() -> Self;
//     fn launch_teardown(&mut self);
//     /// Execute the test and catch panic
//     fn launch_test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
//     where
//         TestFn: FnOnce(),
//         Self: Sized,
//     {
//         std::panic::catch_unwind(std::panic::AssertUnwindSafe(test))
//     }
// }

// #[cfg(feature = "async")]
// mod asyncc {
//     use async_trait::async_trait;
//     use futures::{future::BoxFuture, FutureExt};
//     use std::{any::Any, panic::AssertUnwindSafe};

//     #[async_trait]
//     pub trait AsyncContext<'a> {
//         async fn launch_setup() -> Self;
//         async fn launch_teardown(&mut self);
//         /// Execute the test and catch panic
//         async fn launch_test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
//         where
//             TestFn: FnOnce() -> BoxFuture<'a, ()> + Send,
//             Self: Sized,
//         {
//             AssertUnwindSafe(async move { test().await })
//                 .catch_unwind()
//                 .await
//         }
//     }
// }
