use lazy_static::lazy_static;
use std::{
    sync::Mutex,
    time::{Duration, SystemTime},
};
use tearup::{tearup, SequentialContextCombinator, SimpleContext};

use crate::helper::assert_around_100ms;

lazy_static! {
    static ref FIRST_SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    static ref FIRST_TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    static ref SECOND_SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    static ref SECOND_TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
}

#[test]
fn it_pass_through_setup_then_teardown_of_both_contexts_one_after_the_other() {
    assert_around_100ms(sequential);

    assert!(
        FIRST_SETUP_CHECKPOINT.lock().unwrap().unwrap()
            < SECOND_SETUP_CHECKPOINT.lock().unwrap().unwrap()
    );
    assert!(
        FIRST_TEARDOWN_CHECKPOINT.lock().unwrap().unwrap()
            < SECOND_TEARDOWN_CHECKPOINT.lock().unwrap().unwrap()
    );
}

type B = SequentialContextCombinator<FirstContext, SecondContext>;
#[tearup(B)]
fn sequential() {}

pub struct FirstContext;
impl SimpleContext for FirstContext {
    fn setup() -> Self {
        let mut checkpoint = FIRST_SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        std::thread::sleep(Duration::from_millis(50));

        Self {}
    }

    fn teardown(&mut self) {
        let mut checkpoint = FIRST_TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

pub struct SecondContext;
impl SimpleContext for SecondContext {
    fn setup() -> Self {
        let mut checkpoint = SECOND_SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        std::thread::sleep(Duration::from_millis(50));

        Self {}
    }

    fn teardown(&mut self) {
        let mut checkpoint = SECOND_TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

mod asyncc {
    use lazy_static::lazy_static;
    use std::time::{Duration, SystemTime};
    use tearup::{
        async_trait, tearup, AsyncSequentialContextCombinator, AsyncSimpleContext, FutureExt,
    };
    use tokio::{sync::Mutex, time::sleep};

    use crate::helper::async_assert_around_100ms;

    lazy_static! {
        static ref FIRST_SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
        static ref FIRST_TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
        static ref SECOND_SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
        static ref SECOND_TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown_of_both_contexts_one_after_the_other() {
        async_assert_around_100ms(move || { async move { sequential().await } }.boxed()).await;

        assert!(
            FIRST_SETUP_CHECKPOINT.lock().await.unwrap()
                < SECOND_SETUP_CHECKPOINT.lock().await.unwrap()
        );
        assert!(
            FIRST_TEARDOWN_CHECKPOINT.lock().await.unwrap()
                < SECOND_TEARDOWN_CHECKPOINT.lock().await.unwrap()
        );
    }

    type B = AsyncSequentialContextCombinator<FirstContext, SecondContext>;
    #[tearup(B)]
    async fn sequential() {}

    pub struct FirstContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for FirstContext {
        async fn setup() -> Self {
            let mut checkpoint = FIRST_SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(50)).await;

            Self {}
        }

        async fn teardown(&mut self) {
            let mut checkpoint = FIRST_TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }

    pub struct SecondContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for SecondContext {
        async fn setup() -> Self {
            let mut checkpoint = SECOND_SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(50)).await;

            Self {}
        }

        async fn teardown(&mut self) {
            let mut checkpoint = SECOND_TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }
}
