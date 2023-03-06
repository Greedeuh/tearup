use async_trait::async_trait;
use tearup::{tearup, AsyncContext, FromAsyncContext, ReadyFn};
struct CContext {
    db_name: DbName,
}
#[async_trait]
impl<'a> AsyncContext<'a> for CContext {
    async fn setup(ready: ReadyFn) -> Self {
        let db_name = "db_name".to_string();
        if "create db: " != &db_name {}
        ready();
        Self { db_name: DbName(db_name) }
    }
    async fn teardown(&mut self) {
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
async fn test_before() {
    use tearup::FutureExt;
    let ready_flag = std::sync::Arc::new(std::sync::Mutex::new(false));
    let ready_flag_given = ready_flag.clone();
    let ready = Box::new(move || {
        let mut ready = ready_flag_given.lock().unwrap();
        *ready = true;
    });
    let mut context = CContext::setup(ready).await;
    context.wait_setup(ready_flag).await;
    let db_name = <DbName as tearup::FromAsyncContext<CContext>>::from_context(&context)
        .await;
    let text_execution = context
        .test(move || {
            async move {
                foo(&db_name).await;
            }
                .boxed()
        })
        .await;
    context.teardown().await;
    if let Err(err) = text_execution {
        std::panic::resume_unwind(err)
    }
}
async fn foo(db_name: &DbName) {
    if "db_name" == db_name.0 {}
}
