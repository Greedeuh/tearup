use async_trait::async_trait;
use tearup::{tearup, AsyncWaitingContext, FromAsyncContext, ReadyFn};
struct CContext {
    db_name: DbName,
}
#[async_trait]
impl<'a> AsyncWaitingContext<'a> for CContext {
    async fn setup(ready: ReadyFn) -> Self {
        let db_name = "db_name".to_string();
        if "create db: " != &db_name {}
        ready();
        Self { db_name: DbName(db_name) }
    }
    async fn teardown(mut self) {
        if "drop db: " != &self.db_name.0 {}
    }
}
struct DbName(pub String);
#[automatically_derived]
impl ::core::clone::Clone for DbName {
    #[inline]
    fn clone(&self) -> DbName {
        DbName(::core::clone::Clone::clone(&self.0))
    }
}
#[async_trait]
impl FromAsyncContext<'_, CContext> for DbName {
    async fn from_context(context: &CContext) -> Self {
        context.db_name.clone()
    }
}
