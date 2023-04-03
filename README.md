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
tearup = "0.2"
```

## Usage

The macros `#[tearup(MyContext)]` executes the `setup` then the method annoted and finally the `teardown`.
The `#[tearup_test(MyContext)]` act the same + add the `#[test]` on the method.

```rust
#[tearup_test(WebContext)]
fn it_should_do_this(address: Address) {
    // assert something
}

#[tearup_test(DbContext)]
async fn it_should_do_that(mut db: DbConnection, address: Address) {
    // assert something
}
```

To do this you'll need to implement `Context` trait with both `setup` and `teardown` methods.

```rust
use async_trait::async_trait;
use tearup::tearup_test;

// Define your context
struct YourContext {
    something_you_need_in_teardown: SomethingYouSetup,
}

#[async_trait]
impl Context for YourContext {
    async fn setup(shared_context: &mut SharedContext) -> Self {
        /* --> do your setup here <-- */

        // Register struct that you want to access in your test
        shared_context.register(SomethingYouNeedInTest{});

        // You still can store things in your struct for the treardown step
        Self { something_you_need_in_teardown: SomethingYouSetup{} }
    }

    async fn teardown(mut self, shared_context: &mut SharedContext) {
        /* --> clean your stuff here <-- */

        // You still have access to the shared context
        shared_context.get::<SomethingYouNeedInTest>();
        // Same for self
        self.something_you_need_in_teardown;
    }
}

/// Type you need to access in test (registered in the SharedContext) must implement `Clone`
#[derive(Clone)]
struct SomethingYouNeedInTest;

struct SomethingYouSetup;
```

You can also combine your contexts with `ContextCombinator`:

```rust
type Both = ConcurrentContextCombinator<YourContext, AnotherContext>;
#[tearup_test(Both)]
fn it_should_do_this(mut something_you_need_in_test: DbConnection, something_from_the_other_context: Address) {
    // assert something
}

type MoreCombinaison = ConcurrentContextCombinator<YourContext, AnotherContext>;
#[tearup_test(MoreCombinaison)]
fn it_should_do_that(mut something_you_need_in_test: DbConnection, something_from_the_other_context: Address) {
    // assert something
}
```

## Examples

[More examples here](/tearup_examples/tests)
