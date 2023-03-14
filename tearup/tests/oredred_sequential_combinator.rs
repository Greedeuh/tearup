use std::{sync::Mutex, thread::spawn, time::SystemTime};

use lazy_static::lazy_static;
use tearup::{tearup, ReadyChecksConfig, ReadyFn, SequentialContextCombinator, WaitingContext};

lazy_static! {
    static ref FIRST_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    static ref SECOND_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
}

#[test]
fn it_is_sequencial() {
    require_first_to_setup_second();

    let first_checkpoint = FIRST_CHECKPOINT.lock().unwrap();
    let second_checkpoint = SECOND_CHECKPOINT.lock().unwrap();
    assert!(first_checkpoint.unwrap() < second_checkpoint.unwrap());
}

type B = SequentialContextCombinator<FirstContext, SecondContext>;
#[tearup(B)]
fn require_first_to_setup_second() {}

pub struct FirstContext;
impl WaitingContext for FirstContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        let mut checkpoint = FIRST_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        spawn(move || {
            let config = Self {}.ready_checks_config();
            let just_after_max = (config.maximum - 1).try_into().unwrap();

            std::thread::sleep((config.duration * just_after_max) / 2);

            ready()
        });
        Self {}
    }

    fn teardown(&mut self) {}
}

pub struct SecondContext;
impl WaitingContext for SecondContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig::ms100()
    }

    fn setup(ready: ReadyFn) -> Self {
        let mut checkpoint = SECOND_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        ready();

        Self {}
    }

    fn teardown(&mut self) {}
}
