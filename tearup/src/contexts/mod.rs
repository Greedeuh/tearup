mod waiting_context;
use std::any::Any;

pub use waiting_context::*;

pub trait Context {
    fn launch_setup() -> Self;
    fn launch_teardown(&mut self);
    /// Execute the test and catch panic
    fn launch_test<TestFn>(&mut self, test: TestFn) -> Result<(), Box<dyn Any + Send>>
    where
        TestFn: FnOnce(),
        Self: Sized,
    {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(test))
    }
}
