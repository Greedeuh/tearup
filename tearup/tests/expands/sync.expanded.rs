use tearup::{tearup, Context, FromContext, ReadyFn};
struct CContext {
    db_name: DbName,
}
impl Context for CContext {
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
    fn from_setup(context: &CContext) -> Self {
        context.db_name.clone()
    }
}
fn test_before() {
    let ready_flag = std::sync::Arc::new(std::sync::Mutex::new(false));
    let ready_flag_given = ready_flag.clone();
    let ready = Box::new(move || {
        let mut ready = ready_flag_given.lock().unwrap();
        *ready = true;
    });
    let mut context = CContext::setup(ready);
    let db_name = <DbName as tearup::FromContext<CContext>>::from_setup(&context);
    context
        .test(
            move || {
                if "db_name" == &db_name.0 {}
            },
            ready_flag,
        );
}
