# tearup

A macro test helper to help you to write integration tests.

Basically:

- execute a `fn setup()` before your test
- execute a `fn teardown()` after the test end
- with a _panic catch_ and _wait_ mechanisms if you need some

## Install

Add the following to your `Cargo.toml`

```yaml
[dependencies]
tearup = "0.1"
```

## Usage

Your test will look like this:

```rust
#[tearup_test(DbContext)]
async fn is_should_do_that(mut db: DbConnection) {
    // assert something
}

#[tearup_test(WebContext)]
fn is_should_do_this(address: Address) {
    // assert something
}

type BothContext = ConcurrentContextCombinator<DbContext, AnotherContext>;
#[tearup_test(BothContext)]
fn is_should_do_this(mut db: DbConnection, address: Address) {
    // assert something
}
```

1. Choose your context (async versions exist too):
   - `SimpleContext`: for basic setup/teardown actions
   - `WaitingContext`: in case your setup/teardown are asyncronous and you need a polling/notification helper ([here an example waiting a rocket server to be up](/tearup_examples/waiting_rockets.rs))
2. Implement it, here the simple async implementation:

```rust
use async_trait::async_trait;
use tearup::{tearup_test, AsyncSimpleContext, FromAsyncContext};

// First define your context
struct YourContext {
    something_you_need_in_test: SomethingYouSetup,
}

// Second implement your setup/teardown
#[async_trait]
impl<'a> AsyncSimpleContext<'a> for YourContext {
    async fn setup() -> Self {
        /* --> do your stuff here <-- */
        Self { something_you_need_in_test: SomethingYouSetup{} }
    }

    async fn teardown(mut self) { /* --> clean your stuff here <-- */ }
}

// Optionnaly define some setup accessor
// if you need to access something from your setup (like db connection, seed, etc)
#[derive(Clone)]
pub struct SomethingYouSetup;
#[async_trait]
impl FromAsyncContext<'_, YourContext> for SomethingYouSetup {
    async fn from_context(context: &YourContext) -> Self {
        context.something_you_need_in_test.clone()
    }
}

// And write your tests !
#[tearup_test(YourContext)]
async fn is_should_do_that(mut something_you_need_in_test: SomethingYouSetup) {
    // assert something using something_you_need_in_test
}
```

3. Combine your contexts with (async versions exist too):
   - `SequentialContextCombinator`: executing setup/teardown one after the other, usefull when one context require the other
   - `ConcurrentContextCombinator`: executing setup/teardown actions in parallel

```rust
type BothContext = ConcurrentContextCombinator<YourContext, AnotherContext>;
#[tearup_test(BothContext)]
fn is_should_do_this(mut something_you_need_in_test: DbConnection, something_from_the_other_context: Address) {
    // assert something
}
```

[More examples here](/tearup_examples)
