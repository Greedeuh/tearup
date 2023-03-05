# tearup

A macro test helper to help you to write integration tests.

Basically:

- execute a `fn setup()` before your test
- execute a `fn teardown()` after the test end
- with a _wait_ and _panic catch_ mechanisms.

## Install

Add the following to your `Cargo.toml`

    [dependencies]
    tearup = "0.1"

## Usage

Your test will look like this:

```rust
#[tearup_test(DbContext)]
async fn is_should_do_that(mut db: DbConnection) {
    // assert something using something_you_need_in_test
}

#[tearup_test(AnotherContext)]
fn is_should_do_this(address: Address) {
    // assert something using something_you_need_in_test
}
```

How to implements:

```rust
use async_trait::async_trait;
use tearup::{tearup_test, AsyncContext, FromAsyncContext, ReadyFn};

// First define your context
struct YourContext {
    something_you_need_in_test: SomethingYouSetup,
}

// Second implement your setup/teardown
#[async_trait]
impl<'a> AsyncContext<'a> for YourContext {
    async fn setup(ready: ReadyFn) -> Self {
        /* --> do your stuff here <-- */

        ready(); // notify that your setup id ready

        Self { something_you_need_in_test: SomethingYouSetup{} }
    }

    async fn teardown(&mut self) { /* --> clean your stuff here <-- */ }
}

// Optionnaly define some setup accessor
// if you need to access something from your setup (like db connection, seed, etc)
#[derive(Clone)]
pub struct SomethingYouSetup;
#[async_trait]
impl FromAsyncContext<'_, YourContext> for SomethingYouSetup {
    async fn from_setup(context: &YourContext) -> Self {
        context.something_you_need_in_test.clone()
    }
}

// And write your tests !
#[tearup_test(YourContext)]
async fn is_should_do_that(mut something_you_need_in_test: SomethingYouSetup) {
    // assert something using something_you_need_in_test
}
```

[More examples here](/tearup_examples)
