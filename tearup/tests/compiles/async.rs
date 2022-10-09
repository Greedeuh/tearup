use async_trait::async_trait;
use tearup::{tearup, AsyncContext, FromAsyncContext, ReadyFn};

struct CContext {
    db_name: DbName,
}

#[async_trait]
impl<'a> AsyncContext<'a> for CContext {
    async fn setup(ready: ReadyFn) -> Self
    {
        let db_name = "db_name".to_string();
        if "create db: " != &db_name {};

        ready();
        
        Self {
            db_name: DbName(db_name),
        }
    }
    async fn teardown(&mut self) {
        if "drop db: " != &self.db_name.0 {};
    }
}

#[derive(Clone)]
struct DbName(pub String);

#[async_trait]
impl FromAsyncContext<'_, CContext> for DbName {
    async fn from_setup(context: &CContext) -> Self {
        context.db_name.clone()
    }
}

#[tearup(CContext)]
async fn test_before(db_name: DbName) {
    foo(&db_name).await;
}

async fn foo(db_name: &DbName) {
    if "db_name" == db_name.0 {};
}

fn main() {}