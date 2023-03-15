use tearup::{tearup, Context, FromContext, ReadyFn};
struct CContext {
    db_name: DbName,
}
impl WaitingContext for CContext {
    fn setup(ready: ReadyFn) -> Self {
        let db_name = "db_name".to_string();
        if "create db: " != &db_name {}
        ready();
        Self { db_name: DbName(db_name) }
    }
    fn teardown(&mut self) {
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
impl FromContext<CContext> for DbName {
    fn from_context(context: &CContext) -> Self {
        context.db_name.clone()
    }
}
fn test_with_db_setup_and_teardown() {
    use tearup::{SimpleContext, WaitingContext};
    let mut context = CContext::launch_setup();
    let db_name = <DbName as tearup::FromContext<CContext>>::from_context(&context);
    let text_execution = context
        .launch_test(move || {
            if "db_name" == &db_name.0 {}
        });
    context.launch_teardown();
    if let Err(err) = text_execution {
        std::panic::resume_unwind(err)
    }
}
