use std::{sync::Mutex, time::Duration};
use stopwatch::Stopwatch;

use lazy_static::lazy_static;
use tearup::{tearup, SequentialContextCombinator, SimpleContext};

use crate::helper::assert_around_100ms;

lazy_static! {
    static ref FIRST_CHECKPOINT: Mutex<Option<bool>> = Some(false).into();
    static ref SECOND_CHECKPOINT: Mutex<Option<bool>> = Some(false).into();
}

#[test]
fn it_pass_through_setup_then_teardown_of_both_contexts_one_after_the_other() {
    let stopwatch = Stopwatch::start_new();

    sequential();

    assert!(FIRST_CHECKPOINT.lock().unwrap().unwrap());
    assert!(SECOND_CHECKPOINT.lock().unwrap().unwrap());

    assert_around_100ms(&stopwatch);
}

type B = SequentialContextCombinator<FirstContext, SecondContext>;
#[tearup(B)]
fn sequential() {}

pub struct FirstContext;
impl SimpleContext for FirstContext {
    fn setup() -> Self {
        let mut checkpoint = FIRST_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(true);

        std::thread::sleep(Duration::from_millis(50));

        Self {}
    }

    fn teardown(&mut self) {}
}

pub struct SecondContext;
impl SimpleContext for SecondContext {
    fn setup() -> Self {
        let mut checkpoint = SECOND_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(true);

        std::thread::sleep(Duration::from_millis(50));

        Self {}
    }

    fn teardown(&mut self) {}
}

mod asyncc {
    use std::time::Duration;
    use stopwatch::Stopwatch;

    use lazy_static::lazy_static;
    use tearup::{async_trait, tearup, AsyncSequentialContextCombinator, AsyncSimpleContext};
    use tokio::{sync::Mutex, time::sleep};

    use crate::helper::assert_around_100ms;

    lazy_static! {
        static ref FIRST_CHECKPOINT: Mutex<Option<bool>> = Some(false).into();
        static ref SECOND_CHECKPOINT: Mutex<Option<bool>> = Some(false).into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown_of_both_contexts_one_after_the_other() {
        let stopwatch = Stopwatch::start_new();

        sequential().await;

        assert!(FIRST_CHECKPOINT.lock().await.unwrap());
        assert!(SECOND_CHECKPOINT.lock().await.unwrap());

        assert_around_100ms(&stopwatch);
    }

    type B = AsyncSequentialContextCombinator<FirstContext, SecondContext>;
    #[tearup(B)]
    async fn sequential() {}

    pub struct FirstContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for FirstContext {
        async fn setup() -> Self {
            let mut checkpoint = FIRST_CHECKPOINT.lock().await;
            *checkpoint = Some(true);

            sleep(Duration::from_millis(50)).await;

            Self {}
        }

        async fn teardown(&mut self) {}
    }

    pub struct SecondContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for SecondContext {
        async fn setup() -> Self {
            let mut checkpoint = SECOND_CHECKPOINT.lock().await;
            *checkpoint = Some(true);

            sleep(Duration::from_millis(50)).await;

            Self {}
        }

        async fn teardown(&mut self) {}
    }
}
